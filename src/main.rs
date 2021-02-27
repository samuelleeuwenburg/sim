mod device;
mod sample;
mod stream;
mod ui;
mod wave;
mod track;

use std::convert::TryInto;
use std::thread;
use std::time;
use std::sync::{Arc, Mutex, mpsc};

use crate::device::get_device;
use crate::track::Track;
use crate::stream::Stream;

use cpal::traits::StreamTrait;

enum AudioMessage {
    AddTrack(Track),
}

fn main() {
    let buffer_size = 1024;
    let channels = 2;
    let sample_rate = 44_100;

    let (rx_buffer, buffer, device_stream) = get_device(buffer_size, channels, sample_rate);
    device_stream.play().unwrap();

    let (tx_audio, rx_audio) = mpsc::channel::<AudioMessage>();
    let (tx_ui, rx_ui) = mpsc::channel::<ui::Message>();

    // crude hardcoded ui state shared between threads
    let ui_state = Arc::new(Mutex::new(ui::InputView { message: "select a file".to_owned(), input: None }));

    let audio_thread = thread::spawn(move || {
	let mut tracks: Vec<Track> = vec![];

	loop {
	    if let Ok(used_samples) = rx_buffer.try_recv() {
		let mut streams = vec![];

		for track in tracks.iter_mut() {
		    let sample  = track.sample.as_mut().unwrap();
		    let stream = sample.get_stream(used_samples, channels);
		    streams.push(stream);
		}

		let mut new_stream = Stream::empty(used_samples, channels)
		    .mix(&streams);

		let mut buffer = buffer.lock().unwrap();
		// remove read samples
		buffer.samples.drain(..used_samples);
		// add new samples
		buffer.samples.append(&mut new_stream.samples);
	    };

	    if let Ok(msg) = rx_audio.try_recv() {
		match msg {
		    AudioMessage::AddTrack(track) => tracks.push(track),
		}
	    }
	}
    });

    let ui_thread = thread::spawn(move || {
	ui::setup();

	loop {
	    ui::input(tx_ui.clone());

	    let state = &*ui_state.lock().unwrap();
	    ui::draw(&state);

	    thread::sleep(time::Duration::from_millis(160));
	}
    });

    let message_thread = thread::spawn(move || {
	loop {
	    if let Ok(msg) = rx_ui.recv() {
		match msg {
		    ui::Message::AddTrack(path) => {
			match path.try_into() {
			    Ok(track) => tx_audio.send(AudioMessage::AddTrack(track)).unwrap(),
			    Err(e) => println!("{}", e),
			}
		    },
		}
	    }
	}
    });

    audio_thread.join().unwrap();
    ui_thread.join().unwrap();
    message_thread.join().unwrap();
}
