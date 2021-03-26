use std::convert::TryFrom;
use std::error;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

use crate::sample::Sample;
use crate::stream::{Stream, StreamErr};
use crate::traits::Playable;
use crate::wave;
use crate::wave::{parse_wave, Wave};

#[derive(Debug)]
pub enum Error {
    BadFile(io::Error),
    CantParseFile(wave::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl error::Error for Error {}

impl From<wave::Error> for Error {
    fn from(err: wave::Error) -> Self {
        Error::CantParseFile(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::BadFile(err)
    }
}

pub struct Track {
    pub sample: Option<Sample>,
    pub buffer: Stream,
    pub position: (i32, i32),
}

impl Track {
    pub fn new(channels: usize) -> Self {
        Track {
            sample: None,
            buffer: Stream::empty(0, channels),
            position: (0, 0),
        }
    }

    pub fn add_sample(&mut self, sample: Sample) -> &mut Self {
        self.sample = Some(sample);
        self
    }

    pub fn set_position(&mut self, position: (i32, i32)) -> &mut Self {
        self.position = position;
        self
    }
}

impl Playable for Track {
    fn play(&mut self) -> Result<&Stream, StreamErr> {
        let sample_stream = match self.sample.as_mut() {
            Some(sample) => Some(sample.play()?),
            None => None,
        };

        for (i, byte) in self.buffer.samples.iter_mut().enumerate() {
            *byte = sample_stream
                .and_then(|s| s.samples.get(i))
                .unwrap_or(&0.0)
                .clone();
        }

        Ok(&self.buffer)
    }

    fn set_buffer_size(&mut self, buffer_size: usize) -> &mut Self {
        self.buffer
            .samples
            .resize_with(buffer_size, Default::default);
        self.sample.as_mut().map(|s| s.set_buffer_size(buffer_size));
        self
    }
}

impl TryFrom<String> for Track {
    type Error = Error;

    fn try_from(p: String) -> Result<Self, Self::Error> {
        let path = Path::new(&p);
        let file = fs::read(path)?;

        let name = path
            .file_name()
            .and_then(|osstr| osstr.to_str())
            .unwrap_or("<unnamed>");

        let wave: Wave = parse_wave(&file, &name)?;
        let sample: Sample = wave.into();

        let mut track = Track::new(sample.buffer.channels);
        track.add_sample(sample);

        Ok(track)
    }
}
