use std::convert::From;

use crate::wave::{Wave, Samples};

type Point = f64;
type Stream = Vec<Point>;

#[derive(Debug)]
pub struct Sample {
    num_channels: usize,
    speed: f64,
    stream: Stream,
}

fn u8_to_point(n: u8) -> Point {
    (n as f64 / u8::MAX as f64) * 2.0 - 1.0
}

fn i16_to_point(n: i16) -> Point {
    n as f64 / i16::MAX as f64
}

fn u32_to_point(n: u32) -> Point {
    ((n << 8) as f64 / u16::MAX as f64) * 2.0 - 1.0
}

impl From<Wave> for Sample {
    fn from(wave: Wave) -> Self {
        let num_channels = wave.format.num_channels as usize;

        let stream = match wave.data {
            Samples::BitDepth8(samples) => samples.into_iter().map(u8_to_point).collect(),
            Samples::BitDepth16(samples) => samples.into_iter().map(i16_to_point).collect(),
            Samples::BitDepth24(samples) => samples.into_iter().map(u32_to_point).collect(),
        };

        Sample { num_channels, stream, speed: 1.0 }
    }
}
