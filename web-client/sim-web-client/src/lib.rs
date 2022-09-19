mod utils;
mod web_graphics;
use web_sys::console;
use js_sys::{Float32Array, Uint8ClampedArray};
use wasm_bindgen::__rt::core::{mem, slice};

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
pub fn allocate_u8_buffer(size: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

#[wasm_bindgen]
pub fn allocate_f32_buffer(size: usize) -> *mut f32 {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

#[wasm_bindgen]
pub fn get_u8_buffer(ptr: *mut u8, size: usize) -> Uint8ClampedArray {
    unsafe { Uint8ClampedArray::view(std::slice::from_raw_parts(ptr, size)) }
}

#[wasm_bindgen]
pub fn get_f32_buffer(ptr: *mut f32, size: usize) -> Float32Array {
    unsafe { Float32Array::view(std::slice::from_raw_parts(ptr, size)) }
}

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
pub fn process_input() {
    let mut audio = AUDIO.lock().unwrap();
    let mut grid = GRID.lock().unwrap();
    let mut input_state = INPUT.lock().unwrap();
}

#[wasm_bindgen]
pub fn sample(pointer: *mut f32, size: usize) {
    let mut audio = AUDIO.lock().unwrap();
    let mut grid = GRID.lock().unwrap();

    match (grid.as_mut(), audio.as_mut()) {
        (Some(mut grid), Some(audio)) => {
            let (l, r) = audio.sample(&mut grid);
	    let mut buffer = unsafe { slice::from_raw_parts_mut(pointer, size) };

	    assert_eq!(l.len() + r.len(), size);

	    for (i, s) in l.iter().enumerate() {
	    	buffer[i] = *s;
	    }

	    for (i, s) in l.iter().enumerate() {
	    	buffer[size / 2 + i] = *s;
	    }
        }
        _ => (),
    }
}

#[wasm_bindgen]
pub fn render_image(pointer: *mut u8, size: usize) {
    let mut ui = UI.lock().unwrap();
    let mut graphics = GRAPHICS.lock().unwrap();
    let grid = GRID.lock().unwrap();

    match (grid.as_ref(), ui.as_mut(), graphics.as_mut()) {
	(Some(grid), Some(ui), Some(graphics)) => {
	    ui.render(graphics, grid);

	    assert_eq!(size, graphics.canvas.data.len() * 4);

	    let mut buffer = unsafe { slice::from_raw_parts_mut(pointer, size) };

	    for (i, color) in graphics.canvas.data.iter().enumerate() {
		buffer[i * 4] = color.red;
		buffer[i * 4 + 1] = color.green;
		buffer[i * 4 + 2] = color.blue;
		buffer[i * 4 + 3] = color.alpha;
	    }
	}
	_ => (),
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
