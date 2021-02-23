use crate::sample::Sample;

pub struct Track {
    pub sample: Option<Sample>
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
