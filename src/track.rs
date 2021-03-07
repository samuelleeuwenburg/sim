use std::convert::TryFrom;
use std::path::Path;
use std::fs;

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
	    position: (0, 0)
	}
    }

    pub fn set_buffer_size(mut self, buffer_size: usize) -> Self {
	self.buffer.samples.resize_with(buffer_size, Default::default);
	self.sample = self.sample.map(|s| s.set_buffer_size(buffer_size));
	self
    }

    pub fn add_sample(mut self, sample: Sample) -> Self {
	self.sample = Some(sample);
	self
    }

    pub fn set_position(mut self, position: (i32, i32)) -> Self {
	self.position = position;
	self
    }
}

impl TryFrom<String> for Track {
    type Error = String;

    fn try_from(p: String) -> Result<Self, Self::Error> {
	let path = Path::new(&p);
	let file = fs::read(path).map_err(|e| format!("can't read track \"{}\" from path: {:?}", p,  e))?;
	let name = path.file_name().and_then(|osstr| osstr.to_str()).unwrap_or("<unnamed>");

	let wave: Wave = parse_wave(&file, &name).map_err(|e| format!("can't parse wave file: {}", e))?;
	let sample: Sample = wave.into();

	Ok(Track::new(sample.buffer.channels).add_sample(sample))
    }
}
