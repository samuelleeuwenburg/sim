use web_sys::AudioContext;

pub struct Audio {
    pub ctx: AudioContext,
    pub buffer_position: f64,
}

impl Audio {
    pub fn new(ctx: AudioContext) -> Self {
        let buffer_position = ctx.current_time();

        Audio {
            ctx,
            buffer_position,
        }
    }
}
