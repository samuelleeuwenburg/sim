use screech::oscillator::Oscillator;
use screech::primary::{Error, Primary};
use screech::track::Track;
use screech::traits::Source;
use wasm_bindgen::prelude::*;
use web_sys::console;

struct State {
    primary: Primary,
    oscillators: Vec<Oscillator>,
    tracks: Vec<Track>,
}

impl State {
    fn new(buffer_size: usize, sample_rate: usize) -> Self {
        State {
            primary: Primary::new(buffer_size, sample_rate),
            oscillators: vec![],
            tracks: vec![],
        }
    }

    fn sample(&mut self) -> Result<Vec<f32>, Error> {
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

        self.primary.sample(a)
    }

    // fn get_sources(&self) -> Vec<&mut dyn Source> {

    // 	a
    // }
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static mut STATE: Option<State> = None;

#[wasm_bindgen]
pub fn request_animation_frame() {}

#[wasm_bindgen]
pub fn request_buffer() -> Vec<f32> {
    let state = unsafe { STATE.as_mut().unwrap() };
    state.sample().unwrap()
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    console::log_1(&"Hello from init".into());

    let mut state = State::new(4800, 48_000);
    let mut track = Track::new(&mut state.primary);
    let mut lfo = Oscillator::new(&mut state.primary);

    let count = 3;

    for i in 0..count {
	let mut osc = Oscillator::new(&mut state.primary);
	osc.output_saw();
	track.add_input(&osc);

	if i % 2 == 0 {
	    osc.frequency = 440.0; // a
	}
	if i % 2 == 1 {
	    osc.frequency = 523.2511; // c
	}
	if i % 2 == 2 {
	    osc.frequency = 659.2551; // e
	}

	state.oscillators.push(osc);
    }


    lfo.frequency = 2.0;
    lfo.output_triangle();

    track.set_panning_cv(&lfo);

    state.primary.add_monitor(&track);
    state.oscillators.push(lfo);
    state.tracks.push(track);

    unsafe { STATE = Some(state) };

    Ok(())
}
