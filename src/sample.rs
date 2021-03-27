use std::convert::From;

use crate::stream;
use crate::stream::{Stream, StreamErr};
use crate::traits::Playable;
use wavv::{Samples, Wave};

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
}

impl Playable for Sample {
    fn play(&mut self) -> Result<&Stream, StreamErr> {
        let sample_length = self.stream.len();

        for byte in self.buffer.samples.iter_mut() {
            *byte = match self.play_style {
                PlayStyle::Loop => self.stream.get_sample(self.position)?,
                PlayStyle::OneShot => {
                    if self.position >= sample_length {
                        0.0
                    } else {
                        self.stream.get_sample(self.position)?
                    }
                }
            };

            self.position = match self.play_style {
                PlayStyle::Loop => {
                    if self.position >= sample_length - 1 {
                        0
                    } else {
                        self.position + 1
                    }
                }
                PlayStyle::OneShot => {
                    if self.position >= sample_length - 1 {
                        sample_length
                    } else {
                        self.position + 1
                    }
                }
            };
        }

        Ok(&self.buffer)
    }

    fn set_buffer_size(&mut self, buffer_size: usize) -> &mut Self {
        self.buffer
            .samples
            .resize_with(buffer_size, Default::default);
        self
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

        Sample::new("<unnamed>".to_owned(), stream)
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
