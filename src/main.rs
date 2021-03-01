mod device;
mod sample;
mod stream;
mod ui;
mod wave;
mod track;
mod state;

use std::thread;
use std::time;
use std::sync::{Arc, Mutex, mpsc};

use cpal::traits::StreamTrait;

use crate::device::get_device;
use crate::stream::Stream;
use crate::state::{State, Message, handle_message};

fn main() {
    let buffer_size = 1024;
    let channels = 2;
    let sample_rate = 44_100;

    let state = Arc::new(Mutex::new(State::new(buffer_size, channels, sample_rate)));
    let state_audio = state.clone();
    let state_ui = state.clone();

    let (rx_buffer, buffer, device_stream) = get_device(buffer_size, channels, sample_rate);

    device_stream.play().unwrap();

    let (tx, rx) = mpsc::channel::<Message>();


    let audio_thread = thread::spawn(move || {
	loop {
	    let used_samples = rx_buffer.recv().unwrap();
	    let mut buffer = buffer.lock().unwrap();

	    // remove read samples
	    buffer.samples.drain(..used_samples);

	    if buffer.samples.len() < buffer_size {
		let mut streams = vec![];
		let mut state = state_audio.lock().unwrap();

		for track in state.tracks.iter_mut() {
		    let sample = track.sample.as_mut().unwrap();
		    let buffer = sample.play().unwrap();
		    streams.push(buffer);
		}

		let mut new_stream = Stream::empty(buffer_size, channels)
		    .mix(&streams);

		// add new samples
		buffer.samples.append(&mut new_stream.samples);
	    }
	}
    });

    let ui_thread = thread::spawn(move || {
	ui::setup();

	loop {
	    let state = state_ui.lock().unwrap();

	    ui::input(&state, &tx);
	    ui::draw(&state);

	    drop(state);

	    thread::sleep(time::Duration::from_millis(160));
	}
    });

    let message_thread = thread::spawn(move || {
	loop {
	    match rx.recv() {
		Ok(msg) => handle_message(msg, &state),
		Err(_) => (),
	    }
	}
    });


    audio_thread.join().unwrap();
    ui_thread.join().unwrap();
    message_thread.join().unwrap();
}
