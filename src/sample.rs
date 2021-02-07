use std::convert::From;

use crate::wave::{Wave, Samples};
use crate::stream;
use crate::stream::Stream;

#[derive(Debug)]
pub struct Sample {
    num_channels: usize,
    stream: Stream,
    speed: f64,
    position: usize,
}

impl Sample {
    pub fn get_stream(&mut self, buffer_size: stream::BufferSize) -> Stream {
        let mut stream = stream::get_stream(buffer_size);

        for byte in stream.iter_mut() {
            self.position = if self.position >= self.stream.len() - 1 {
                0
            } else {
                self.position + 1
            };

            *byte = self.stream.get(self.position).unwrap().clone();
        }

        stream
    }
}

impl From<Wave> for Sample {
    fn from(wave: Wave) -> Self {
        let num_channels = wave.format.num_channels as usize;

        let stream: Stream = match wave.data {
            Samples::BitDepth8(samples) => samples.into_iter().map(stream::u8_to_point).collect(),
            Samples::BitDepth16(samples) => samples.into_iter().map(stream::i16_to_point).collect(),
            Samples::BitDepth24(samples) => samples.into_iter().map(stream::i32_to_point).collect(),
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
