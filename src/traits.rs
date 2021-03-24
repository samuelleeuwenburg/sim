use crate::effects::EffectErr;
use crate::stream::{Stream, StreamErr};

pub trait Playable {
    fn play(&mut self) -> Result<&Stream, StreamErr>;
    fn set_buffer_size(&mut self, buffer_size: usize) -> &mut Self;
}

pub trait Effect {
    fn process(&mut self, stream: &Stream) -> Result<Stream, EffectErr>;
}
