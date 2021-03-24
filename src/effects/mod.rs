use crate::stream::Stream;
use crate::traits::Effect;

#[derive(Debug)]
pub enum EffectErr {}

pub struct Mute;

impl Mute {
    pub fn new() -> Self {
        Mute
    }
}

impl Effect for Mute {
    fn process(&mut self, stream: &Stream) -> Result<Stream, EffectErr> {
        Ok(Stream::empty(stream.samples.len(), stream.channels))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mute() {
        let mut mute = Mute::new();
        let input = Stream::from_samples(vec![0.1, 0.2, 0.3, 0.4], 1);
        let output = mute.process(&input).unwrap();
        assert_eq!(output.samples, vec![0.0, 0.0, 0.0, 0.0]);
    }
}
