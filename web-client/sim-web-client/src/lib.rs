mod utils;

use sim::{Audio, Grid};
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static AUDIO: Mutex<Option<Audio>> = Mutex::new(None);
static GRID: Mutex<Option<Grid>> = Mutex::new(None);

#[wasm_bindgen]
pub fn init_sim(sample_rate: usize, buffer_size: usize) {
    let mut audio = AUDIO.lock().unwrap();
    let _ = audio.insert(Audio::new(sample_rate, buffer_size));

    let mut grid = GRID.lock().unwrap();
    let _ = grid.insert(Grid::new());
}

#[wasm_bindgen]
pub fn sample() -> Vec<f32> {
    let mut audio = AUDIO.lock().unwrap();
    let mut grid = GRID.lock().unwrap();

    let mut samples = vec![];

    match (grid.as_mut(), audio.as_mut()) {
        (Some(mut grid), Some(audio)) => {
            let (l, r) = audio.sample(&mut grid);
            samples.extend(l);
            samples.extend(r);
        }
        _ => (),
    }

    samples
}
