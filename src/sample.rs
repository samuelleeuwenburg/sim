use std::convert::From;

use crate::wave::{Wave, Samples};

type Point = f32;

type Stream = Vec<Point>;

#[derive(Debug)]
pub struct Sample {
    num_channels: usize,
    stream: Stream,
    speed: f64,
    position: usize,
}

impl Sample {
    pub fn get_audio<'a>(&mut self, buffer: &'a mut [f32;2048]) -> &'a [f32;2048] {
        for byte in buffer.iter_mut() {
            self.position = if self.position >= self.stream.len() - 1 {
                0
            } else {
                self.position + 1
            };

            *byte = self.stream.get(self.position).unwrap().clone();
        }

        buffer
    }
}

impl From<Wave> for Sample {
    fn from(wave: Wave) -> Self {
        let num_channels = wave.format.num_channels as usize;

        let stream: Stream = match wave.data {
            Samples::BitDepth8(samples) => samples.into_iter().map(u8_to_point).collect(),
            Samples::BitDepth16(samples) => samples.into_iter().map(i16_to_point).collect(),
            Samples::BitDepth24(samples) => samples.into_iter().map(i32_to_point).collect(),
        };

        let stream: Stream = match num_channels {
            1 => {
                let mut new_stream = vec![];
                for byte in stream {
                    new_stream.append(&mut vec![byte, byte]);
                }
                new_stream
            }
            2 => stream,
            _ => panic!("non supported channel size to create sample"),
        };

        Sample {
            num_channels,
            stream,
            speed: 1.0,
            position: 0,
        }
    }
}

fn u8_to_point(n: u8) -> Point {
    (n as f32 / u8::MAX as f32) * 2.0 - 1.0
}

fn i16_to_point(n: i16) -> Point {
    n as f32 / i16::MAX as f32
}

fn i32_to_point(n: i32) -> Point {
    n as f32 / i32::MAX as f32
}

#[cfg(test)]
mod tests {
    #![allow(overflowing_literals)]
    use super::*;

    #[test]
    fn test_u8_to_point() {
        assert_eq!(u8_to_point(u8::MIN), -1.0);
        assert_eq!(u8_to_point(0x80u8), 0.003921628);
        assert_eq!(u8_to_point(u8::MAX), 1.0);
    }

    #[test]
    fn test_i16_to_point() {
        assert_eq!(i16_to_point(i16::MIN + 1), -1.0);
        assert_eq!(i16_to_point(0i16), 0.0);
        assert_eq!(i16_to_point(i16::MAX), 1.0);
    }

    #[test]
    fn test_i32_to_point() {
        assert_eq!(i32_to_point(i32::MIN + 1), -1.0);
        assert_eq!(i32_to_point(0i32), 0.0);
        assert_eq!(i32_to_point(i32::MAX), 1.0);
    }
}
