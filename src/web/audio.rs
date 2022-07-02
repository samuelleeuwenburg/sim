use wasm_bindgen::prelude::*;
use web_sys::{AudioBuffer, AudioContext};

pub struct WebAudio {
    pub ctx: AudioContext,
    pub buffer_position: f64,
    pub buffer_size: usize,
}

impl WebAudio {
    pub fn new(ctx: AudioContext, buffer_size: usize) -> Self {
        let buffer_position = ctx.current_time();

        WebAudio {
            ctx,
            buffer_position,
            buffer_size,
        }
    }

    pub fn buffer_size_in_seconds(&self) -> f64 {
        self.buffer_size as f64 / 48_000.0 / 2.0
    }

    pub fn move_buffer_size_forward(&mut self) -> &mut Self {
        self.buffer_position += self.buffer_size_in_seconds();
        self
    }

    pub fn needs_new_buffer(&self) -> bool {
        let current_time = self.ctx.current_time();

        current_time + self.buffer_size_in_seconds() >= self.buffer_position
    }

    pub fn queue_new_buffer(&mut self, samples: (&[f32], &[f32])) -> Result<(), JsValue> {
        // set next buffer_position
        self.move_buffer_size_forward();

        // get buffer and queue
        let buffer = self.create_buffer_from_samples(samples)?;
        let source = self.ctx.create_buffer_source()?;

        source.set_buffer(Some(&buffer));
        source.connect_with_audio_node(&self.ctx.destination())?;
        source.start_with_when(self.buffer_position)?;

        Ok(())
    }

    pub fn create_buffer_from_samples(
        &self,
        (left, right): (&[f32], &[f32]),
    ) -> Result<AudioBuffer, JsValue> {
        let channels = 2;
        let sample_rate = 48_000.0;

        let buffer = self
            .ctx
            .create_buffer(channels, self.buffer_size as u32, sample_rate)?;

        buffer.copy_to_channel(&left, 0)?;
        buffer.copy_to_channel(&right, 1)?;

        Ok(buffer)
    }
}
