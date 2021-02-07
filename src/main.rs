mod wave;
mod sample;
mod device;
mod stream;

use std::fs;
use std::convert::Into;
use std::sync::mpsc;

use crate::wave::{parse_wave, Wave};
use crate::sample::Sample;
use crate::device::get_stream;

fn main() {
    let (tx_buffer_ready, rx_buffer_ready) = mpsc::channel();
    let device = get_stream(tx_buffer_ready);

    let file = fs::read("./test_files/sine_mono.wav").unwrap();
    let wave: Wave = parse_wave(&file).unwrap();
    let mut sample: Sample = wave.into();

    loop {
        rx_buffer_ready.recv().unwrap();

        let buffer = device.create_stream();
        let _sample_stream = sample.get_stream(device.buffer_size);

        device.tx.send(buffer).unwrap();
    }
}
