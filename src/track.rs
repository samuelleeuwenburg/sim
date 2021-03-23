use std::convert::TryFrom;
use std::fs;
use std::path::Path;

use crate::sample::Sample;
use crate::stream::Stream;
use crate::wave::{parse_wave, Wave};

#[derive(Debug)]
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

    pub fn set_buffer_size(&mut self, buffer_size: usize) -> &mut Self {
        self.buffer
            .samples
            .resize_with(buffer_size, Default::default);
        self.sample.as_mut().map(|s| s.set_buffer_size(buffer_size));
        self
    }

    pub fn add_sample(&mut self, sample: Sample) -> &mut Self {
        self.sample = Some(sample);
        self
    }

    pub fn play(&mut self) -> Result<&Stream, String> {
        let buffer_size = self.buffer.samples.len();

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

    pub fn set_position(&mut self, position: (i32, i32)) -> &mut Self {
        self.position = position;
        self
    }
}

impl TryFrom<String> for Track {
    type Error = String;

    fn try_from(p: String) -> Result<Self, Self::Error> {
        let path = Path::new(&p);
        let file =
            fs::read(path).map_err(|e| format!("can't read track \"{}\" from path: {:?}", p, e))?;
        let name = path
            .file_name()
            .and_then(|osstr| osstr.to_str())
            .unwrap_or("<unnamed>");

        let wave: Wave =
            parse_wave(&file, &name).map_err(|e| format!("can't parse wave file: {}", e))?;
        let sample: Sample = wave.into();

        let mut track = Track::new(sample.buffer.channels);
        track.add_sample(sample);

        Ok(track)
    }
}
