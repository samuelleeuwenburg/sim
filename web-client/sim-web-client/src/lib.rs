mod utils;
mod web_graphics;
use web_sys::console;

use sim::{Audio, Grid, Input, InputState, UserInterface};
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use web_graphics::WebGraphics;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static AUDIO: Mutex<Option<Audio>> = Mutex::new(None);
static GRID: Mutex<Option<Grid>> = Mutex::new(None);
static UI: Mutex<Option<UserInterface>> = Mutex::new(None);
static GRAPHICS: Mutex<Option<WebGraphics>> = Mutex::new(None);
static INPUT: Mutex<Option<InputState>> = Mutex::new(None);

#[wasm_bindgen]
pub fn init_sim(sample_rate: usize, buffer_size: usize, width: i32, height: i32) {
    let mut audio = AUDIO.lock().unwrap();
    let _ = audio.insert(Audio::new(sample_rate, buffer_size));

    let mut grid = GRID.lock().unwrap();
    let _ = grid.insert(Grid::new());

    let mut ui = UI.lock().unwrap();
    let _ = ui.insert(UserInterface::new());

    let mut graphics = GRAPHICS.lock().unwrap();
    let _ = graphics.insert(WebGraphics::new(width, height));

    let mut input = INPUT.lock().unwrap();
    let _ = input.insert(InputState::new());
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

#[wasm_bindgen]
pub fn get_render_instructions() -> Option<String> {
    let mut ui = UI.lock().unwrap();
    let mut graphics = GRAPHICS.lock().unwrap();
    let grid = GRID.lock().unwrap();

    match (grid.as_ref(), ui.as_mut(), graphics.as_mut()) {
        (Some(grid), Some(ui), Some(graphics)) => {
            ui.render(graphics, grid);

            let json = serde_json::to_string(graphics.get_instructions()).unwrap();

            graphics.clear_instructions();

            Some(json)
        }
        _ => None,
    }
}

#[wasm_bindgen]
pub fn handle_key_down(input: String) {
    let mut input_state = INPUT.lock().unwrap();

    if let Some(input_state) = input_state.as_mut() {
        match input.as_ref() {
            "Tab" => input_state.key_down(Input::Tab),
            "Space" => input_state.key_down(Input::Space),
            "Enter" => input_state.key_down(Input::Enter),
            _ => {
                if let Some(c) = input.chars().next() {
                    input_state.key_down(Input::Char(c));
                }
            }
        }

        console::log_1(&input.into());
    }
}

#[wasm_bindgen]
pub fn handle_key_up(input: String) {
    let mut input_state = INPUT.lock().unwrap();

    if let Some(input_state) = input_state.as_mut() {
        match input.as_ref() {
            "Tab" => input_state.key_up(Input::Tab),
            "Space" => input_state.key_up(Input::Space),
            "Enter" => input_state.key_up(Input::Enter),
            _ => {
                if let Some(c) = input.chars().next() {
                    input_state.key_up(Input::Char(c));
                }
            }
        }

        console::log_1(&input.into());
    }
}
