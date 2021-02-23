mod device;
mod sample;
mod stream;
mod ui;
mod wave;
mod track;

use std::convert::Into;
use std::fs;
use std::sync::{mpsc};
use std::{thread, time};

use crate::device::get_device;
use crate::sample::Sample;
use crate::stream::Stream;
use crate::wave::{parse_wave, Wave};
use crate::track::Track;

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
	let (tx_buffer_ready, rx_buffer_ready) = mpsc::channel();
	let device = get_device(tx_buffer_ready);

	loop {
	    rx_buffer_ready.recv().unwrap();
	    let buffer = Stream::empty(device.buffer_size * device.channels, device.channels);

	    device.tx.send(buffer).unwrap();
	}
    });

    ui_thread.join().unwrap();
    audio_thread.join().unwrap();
}
