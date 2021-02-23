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

fn main() {
    let ui_thread = thread::spawn(|| {
	ui::setup();

	loop {
	    ui::input();
	    ui::draw();

	    thread::sleep(time::Duration::from_millis(160));
	}
    });

    let audio_thread = thread::spawn(|| {
	let mut device = get_device();

	let file = fs::read("./test_files/p_16_stereo.wav").unwrap();
	let wave: Wave = parse_wave(&file).unwrap();
	let sample: Sample = wave.into();
	let example_track = Track::new().add_sample(sample);

	let mut tracks: Vec<Track> = vec![example_track];

	loop {
	    let buffer_size = device.buffer_size();

	    if buffer_size > 0 {
		let mut streams = vec![];

		for track in tracks.iter_mut() {
		    let sample  = track.sample.as_mut().unwrap();
		    let stream = sample.get_playback_stream(buffer_size, device.channels);
		    streams.push(stream);
		}

		let buffer = Stream::empty(buffer_size, device.channels)
		    .mix(&streams);

		device.send_buffer(buffer).unwrap();
	    }

	}
    });

    ui_thread.join().unwrap();
    audio_thread.join().unwrap();
}
