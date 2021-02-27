use std::convert::TryFrom;
use std::path::Path;
use std::fs;

use crate::sample::Sample;
use crate::wave::{parse_wave, Wave};

#[derive(Debug)]
pub struct Track {
    pub sample: Option<Sample>,
}

impl Track {
    pub fn new() -> Self {
	Track { sample: None }
    }

    pub fn add_sample(mut self, sample: Sample) -> Self {
	self.sample = Some(sample);
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

	Ok(Track::new().add_sample(sample))
    }
}
