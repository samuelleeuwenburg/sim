mod wave;
mod sample;
mod device;
mod stream;

use std::fs;
use std::convert::Into;
use std::sync::mpsc;

use crate::wave::{parse_wave, Wave};
use crate::sample::Sample;
use crate::device::get_device;
use crate::stream::Stream;

fn main() {
    let file = fs::read("./test_files/sine_mono.wav").unwrap();
    let wave: Wave = parse_wave(&file).unwrap();
    let mut sine_sample: Sample = wave.into();

    let file = fs::read("./test_files/c-strum.wav").unwrap();
    let wave: Wave = parse_wave(&file).unwrap();
    let mut guitar_sample: Sample = wave.into();

    let (tx_buffer_ready, rx_buffer_ready) = mpsc::channel();
    let device = get_device(tx_buffer_ready);

    loop {
        rx_buffer_ready.recv().unwrap();

        let _sine_stream = sine_sample.get_playback_stream(device.buffer_size, device.channels);
        let _guitar_stream = guitar_sample.get_playback_stream(device.buffer_size, device.channels);

        let buffer = Stream::empty(device.buffer_size * device.channels, device.channels);

        device.tx.send(buffer).unwrap();
    }
}
