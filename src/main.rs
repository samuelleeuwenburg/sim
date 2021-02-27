mod device;
mod sample;
mod stream;
mod ui;
mod wave;
mod track;

use std::fs;
use std::convert::Into;
use std::{thread, time};

use crate::device::get_device;
use crate::stream::Stream;
use crate::track::Track;
use crate::wave::{parse_wave, Wave};
use crate::sample::Sample;

use cpal::traits::StreamTrait;

fn main() {
    let buffer_size = 1024;
    let channels = 2;

    let (rx_buffer_read, buffer, device_stream) = get_device(buffer_size, channels);
    device_stream.play().unwrap();

    let ui_thread = thread::spawn(|| {
	ui::setup();

	loop {
	    ui::input();
	    ui::draw();

	    thread::sleep(time::Duration::from_millis(160));
	}
    });

    let audio_thread = thread::spawn(move || {
	let file = fs::read("./test_files/p_16_stereo.wav").unwrap();
	let wave: Wave = parse_wave(&file).unwrap();
	let sample: Sample = wave.into();
	let piano_track = Track::new().add_sample(sample);

	let file = fs::read("./test_files/f#_warm.wav").unwrap();
	let wave: Wave = parse_wave(&file).unwrap();
	let sample: Sample = wave.into();
	let guitar_track = Track::new().add_sample(sample);

	let mut tracks: Vec<Track> = vec![piano_track, guitar_track];

	loop {
	    match rx_buffer_read.recv() {
		Ok(used_samples) => {
		    let mut streams = vec![];

		    for track in tracks.iter_mut() {
			let sample  = track.sample.as_mut().unwrap();
			let stream = sample.get_stream(used_samples, channels);
			streams.push(stream);
		    }

		    let mut new_stream = Stream::empty(used_samples, channels)
			.mix(&streams);

		    // lock mutex
		    let mut buffer = buffer.lock().unwrap();
		    // remove read samples
		    buffer.samples.drain(..used_samples);
		    // add new samples
		    buffer.samples.append(&mut new_stream.samples);
		},
		Err(e) => println!("err: {}", e),
	    }
	}
    });

    ui_thread.join().unwrap();
    audio_thread.join().unwrap();
}
