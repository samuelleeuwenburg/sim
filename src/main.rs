mod wave;
mod sample;

use std::fs;
use std::convert::Into;

use crate::wave::{parse_wave, Wave};
use crate::sample::Sample;

fn main() {
    let file = fs::read("./test_files/sine_mono.wav").unwrap();

    let wave: Wave = parse_wave(&file).unwrap();

    println!("{:?}\n", wave);

    let sample: Sample = wave.into();

    println!("{:?}", sample);
}

