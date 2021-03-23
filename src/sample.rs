use std::convert::From;

use crate::wave::{Wave, Samples};
use crate::stream;
use crate::stream::Stream;

#[derive(Debug)]
enum PlayStyle {
    OneShot,
    Loop,
}

#[derive(Debug)]
pub struct Sample {
    pub name: String,
    pub buffer: Stream,
    pub stream: Stream,
    pub position: usize,
    speed: f64,
    play_style: PlayStyle,
}

impl Sample {
    fn new(name: String, stream: Stream) -> Self {
	let channels = stream.channels;

        Sample {
	    name,
            stream,
	    buffer: Stream::empty(0, channels),
            speed: 1.0,
            position: 0,
	    play_style: PlayStyle::Loop,
        }
    }

    pub fn set_buffer_size(&mut self, buffer_size: usize) -> &mut Self {
	self.buffer.samples.resize_with(buffer_size, Default::default);
	self
    }

    // fill buffer with new samples
    pub fn play(&mut self) -> Result<&Stream, String> {
	let sample_length = self.stream.samples.len();

        for byte in self.buffer.samples.iter_mut() {
	    let value: Option<&f32> = match self.play_style {
		PlayStyle::Loop => self.stream.samples.get(self.position),
		PlayStyle::OneShot => if self.position >= sample_length { Some(&0.0) } else { self.stream.samples.get(self.position) }
	    };

	    *byte = value.ok_or(format!("can't get sample for {} @ {}", self.name, self.position))?.clone();

	    self.position = match self.play_style {
		PlayStyle::Loop => if self.position >= sample_length - 1 { 0 } else { self.position + 1 },
		PlayStyle::OneShot => if self.position >= sample_length - 1 { sample_length } else { self.position + 1 },
	    };
        }

	Ok(&self.buffer)
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

        Sample::new(wave.name.to_owned(), stream)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_loop_buffer_smaller_than_sample() {
        let stream = Stream::from_samples(vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8], 1);
	let mut sample = Sample::new("foo".to_owned(), stream);
	sample.set_buffer_size(5);

	let buffer = sample.play().unwrap();
	assert_eq!(buffer.samples, vec![0.0, 0.1, 0.2, 0.3, 0.4]);

	let buffer = sample.play().unwrap();
	assert_eq!(buffer.samples, vec![0.5, 0.6, 0.7, 0.8, 0.0]);

	let buffer = sample.play().unwrap();
	assert_eq!(buffer.samples, vec![0.1, 0.2, 0.3, 0.4, 0.5]);
    }

    #[test]
    fn test_play_loop_buffer_larger_than_sample() {
        let stream = Stream::from_samples(vec![0.0, 0.1, 0.2, 0.3, 0.4], 1);
	let mut sample = Sample::new("foo".to_owned(), stream);
	sample.set_buffer_size(8);

	let buffer = sample.play().unwrap();
	assert_eq!(buffer.samples, vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.0, 0.1, 0.2]);

	let buffer = sample.play().unwrap();
	assert_eq!(buffer.samples, vec![0.3, 0.4, 0.0, 0.1, 0.2, 0.3, 0.4, 0.0]);
    }

    #[test]
    fn test_play_oneshot() {
        let stream = Stream::from_samples(vec![0.0, 0.1, 0.2, 0.3, 0.4], 1);
	let mut sample = Sample::new("foo".to_owned(), stream);
	sample.set_buffer_size(8);
	sample.play_style = PlayStyle::OneShot;

	let buffer = sample.play().unwrap();
	assert_eq!(buffer.samples, vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.0, 0.0, 0.0]);

	let buffer = sample.play().unwrap();
	assert_eq!(buffer.samples, vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    }
}
