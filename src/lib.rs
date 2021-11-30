mod app;
mod web;

use app::{InputState, State};

use std::cell::RefCell;
use std::f64;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, AudioBuffer, AudioContext, HtmlCanvasElement, KeyboardEvent, Window};

const BUFFER_SIZE: usize = 4800;

static mut STATE: Option<State<BUFFER_SIZE>> = None;
static mut AUDIO: Option<web::Audio> = None;
static mut GRAPHICS: Option<web::Graphics> = None;
static mut INPUT_STATE: Option<InputState> = None;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("no global `window` exists");
    let document = web_sys::window()
        .unwrap()
        .document()
        .expect("no document exists");
    let audio_context = web_sys::AudioContext::new().expect("no audiocontext available");

    let canvas = document
        .get_element_by_id("ui")
        .expect("#ui canvas element not found");
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    unsafe { GRAPHICS = Some(web::Graphics::new(canvas)) };
    unsafe { AUDIO = Some(web::Audio::new(audio_context)) };
    unsafe { INPUT_STATE = Some(InputState::new()) };
    unsafe { STATE = Some(State::new(48_000)) };

    setup_callbacks(&window)?;

    Ok(())
}

fn create_buffer(ctx: &AudioContext) -> Result<AudioBuffer, JsValue> {
    // let window = web_sys::window().expect("no global `window` exists");
    // let performance = window
    //     .performance()
    //     .expect("performance should be available");
    // let now = performance.now();

    let state = unsafe { STATE.as_mut().unwrap() };
    let channels = 2;
    let sample_rate = 48_000.0;
    let samples = state.sample();

    let mut left = [0.0; BUFFER_SIZE / 2];
    let mut right = [0.0; BUFFER_SIZE / 2];

    for (i, sample) in samples.into_iter().enumerate() {
        if i % 2 != 0 {
            left[i / 2] = *sample;
        } else {
            right[(i + 1) / 2] = *sample;
        }
    }

    let buffer = ctx.create_buffer(channels, BUFFER_SIZE as u32 / 2, sample_rate)?;
    buffer.copy_to_channel(&mut left, 0)?;
    buffer.copy_to_channel(&mut right, 1)?;

    // console::log_1(&format!("wasm performance: {}ms", performance.now() - now).into());
    Ok(buffer)
}

fn audio_tick() {
    let mut audio_state = unsafe { AUDIO.as_mut().unwrap() };

    // handle buffer
    let current_time = audio_state.ctx.current_time();
    let buffer_size_in_s = BUFFER_SIZE as f64 / 48_000.0 / 2.0;

    if current_time + buffer_size_in_s >= audio_state.buffer_position {
        // set next buffer_position
        audio_state.buffer_position += buffer_size_in_s;

        // get buffer and queue
        let buffer = create_buffer(&audio_state.ctx).expect("could not create buffer");
        let source = audio_state
            .ctx
            .create_buffer_source()
            .expect("could not create source buffer");

        source.set_buffer(Some(&buffer));
        source
            .connect_with_audio_node(&audio_state.ctx.destination())
            .expect("can't connect source to destination");
        source
            .start_with_when(audio_state.buffer_position)
            .expect("couldn't start audio");
    } else {
        // if the audio buffer has no priority, process commands
        let state = unsafe { STATE.as_mut().unwrap() };
        let input_state = unsafe { INPUT_STATE.as_mut().unwrap() };

        // handle input
        for command in input_state.command_buffer.iter() {
            state.process_command(command);
        }

        // clear input
        input_state.command_buffer.clear();
    }
}

fn handle_input(event: KeyboardEvent) {
    let input_state = unsafe { INPUT_STATE.as_mut().unwrap() };
    let state = unsafe { STATE.as_ref().unwrap() };

    input_state.input_buffer.push(event.key_code());

    if let Some(command) = input_state.process_input(&state.user_interface.cursor) {
        input_state.command_buffer.push(command);
    }

    console::log_1(&format!("input: {:?}", input_state.input_buffer).into());
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    let window = web_sys::window().expect("no global `window` exists");

    window
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn graphics_tick() {
    let graphics = unsafe { GRAPHICS.as_mut().unwrap() };
    let state = unsafe { STATE.as_mut().unwrap() };

    graphics.render(&state.user_interface);
}

fn setup_callbacks(window: &Window) -> Result<(), JsValue> {
    let audio_cb = Closure::wrap(Box::new(audio_tick) as Box<dyn FnMut()>);
    let ui_f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let ui_g = ui_f.clone();

    *ui_g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        graphics_tick();
        request_animation_frame(ui_f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(ui_g.borrow().as_ref().unwrap());

    window.set_interval_with_callback_and_timeout_and_arguments_0(
        audio_cb.as_ref().unchecked_ref(),
        0,
    )?;

    let input_cb = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        event.prevent_default();
        handle_input(event);
    }) as Box<dyn FnMut(_)>);
    window.add_event_listener_with_callback("keydown", input_cb.as_ref().unchecked_ref())?;

    input_cb.forget();
    audio_cb.forget();

    Ok(())
}
