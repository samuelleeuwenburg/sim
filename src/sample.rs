use std::convert::From;

use crate::wave::{Wave, Samples};
use crate::stream;
use crate::stream::Stream;

#[derive(Debug)]
pub struct Sample {
    pub name: String,
    stream: Stream,
    speed: f64,
    position: usize,
}

impl Sample {
    fn step(&mut self) {
        // wrap around
        self.position = if self.position >= self.stream.samples.len() - 1 {
            0
        } else {
            self.position + 1
        };
    }

    pub fn get_stream(&mut self, buffer_size: stream::BufferSize, channels: usize) -> Stream {
        let mut stream = Stream::empty(buffer_size, channels);

        for byte in stream.samples.iter_mut() {
            // step through the sample
            self.step();
            *byte = self.stream.samples.get(self.position).unwrap().clone();
        }

        stream
    }
}

impl From<Wave> for Sample {
    fn from(wave: Wave) -> Self {
        let num_channels = wave.format.num_channels as usize;

        let samples: Vec<f32> = match wave.data {
            Samples::BitDepth8(samples) => samples.into_iter().map(stream::u8_to_point).collect(),
            Samples::BitDepth16(samples) => samples.into_iter().map(stream::i16_to_point).collect(),
            Samples::BitDepth24(samples) => samples.into_iter().map(stream::i32_to_point).collect(),
        };

        let stream = Stream::from_samples(samples, num_channels);

        Sample {
	    name: wave.name.to_owned(),
            stream,
            speed: 1.0,
            position: 0,
        }
    }
}
