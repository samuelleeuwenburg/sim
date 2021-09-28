use screech::basic::{Oscillator, Track};
use screech::core::Primary;
use screech::traits::Source;
use wasm_bindgen::prelude::*;
use web_sys::console;

struct State {
    primary: Primary<4800>,
    oscillators: Vec<Oscillator>,
    tracks: Vec<Track>,
}

impl State {
    fn new(sample_rate: usize) -> Self {
        State {
            primary: Primary::new(sample_rate),
            oscillators: vec![],
            tracks: vec![],
        }
    }

    fn sample(&mut self) -> Vec<f32> {
        let mut a: Vec<&mut dyn Source> = self
            .oscillators
            .iter_mut()
            .map(|o| o as &mut dyn Source)
            .collect();

        let mut b: Vec<&mut dyn Source> = self
            .tracks
            .iter_mut()
            .map(|t| t as &mut dyn Source)
            .collect();

        a.append(&mut b);

        self.primary.sample(a).unwrap()
    }
}

static mut STATE: Option<State> = None;

#[wasm_bindgen]
pub fn set_osc_output(shape: &str) {
    let state = unsafe { STATE.as_mut().unwrap() };

    for osc in state.oscillators.iter_mut() {
        // skip LFOs
        if osc.frequency < 50.0 {
            continue;
        }

        if shape == "saw" {
            osc.output_saw();
        }
        if shape == "square" {
            osc.output_square(0.5);
        }
        if shape == "triangle" {
            osc.output_triangle();
        }
    }
}

#[wasm_bindgen]
pub fn request_animation_frame() {}

#[wasm_bindgen]
pub fn request_buffer() -> Vec<f32> {
    let state = unsafe { STATE.as_mut().unwrap() };
    state.sample()
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    console::log_1(&"Hello from init".into());

    let mut state = State::new(48_000);
    let mut track = Track::new(&mut state.primary);
    let mut lfo = Oscillator::new(&mut state.primary);

    let count = 60;

    for i in 0..count {
        let mut osc = Oscillator::new(&mut state.primary);
        osc.output_saw();
        osc.amplitude = 0.01;
        track.add_input(&osc);

        osc.frequency = 440.0; // a

        osc.frequency += (i % count) as f32 * 0.05;
        osc.frequency /= 4.0;

        state.oscillators.push(osc);
    }

    lfo.frequency = 0.15;
    lfo.amplitude = 0.1;
    lfo.output_triangle();

    track.set_panning_cv(&lfo);

    state.primary.add_monitor(&track);
    state.oscillators.push(lfo);
    state.tracks.push(track);

    unsafe { STATE = Some(state) };

    Ok(())
}
