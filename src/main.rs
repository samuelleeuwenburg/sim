mod wave;
mod sample;
mod device;

use std::fs;
use std::convert::Into;
use std::sync::mpsc;

use cpal::traits:: StreamTrait;

use crate::wave::{parse_wave, Wave};
use crate::sample::Sample;
use crate::device::get_stream;

fn main() {
    let (tx_buffer_ready, rx_buffer_ready) = mpsc::channel();
    let (tx_buffer, stream) = get_stream(tx_buffer_ready);

    stream.play().unwrap();

    let file = fs::read("./test_files/sine_mono.wav").unwrap();
    let wave: Wave = parse_wave(&file).unwrap();
    let mut sample: Sample = wave.into();

    loop {
        rx_buffer_ready.recv().unwrap();

        let mut buffer = [1.0; 2048];
        let buffer = sample.get_audio(&mut buffer);

        tx_buffer.send(*buffer).unwrap();
    }
}
