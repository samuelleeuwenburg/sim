mod app;
mod web;

use app::{Input, InputState, State};
use web::{WebAudio, WebGraphics};

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;
use web_sys::{DragEvent, HtmlCanvasElement, KeyboardEvent, Window};

const BUFFER_SIZE: usize = 4800;

static mut STATE: Option<State> = None;
static mut AUDIO: Option<WebAudio> = None;
static mut GRAPHICS: Option<WebGraphics> = None;
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

    unsafe { GRAPHICS = Some(WebGraphics::new(canvas)) };
    unsafe { AUDIO = Some(WebAudio::new(audio_context, BUFFER_SIZE)) };
    unsafe { INPUT_STATE = Some(InputState::new()) };
    unsafe { STATE = Some(State::new(48_000)) };

    setup_callbacks(&window)?;

    Ok(())
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    let window = web_sys::window().expect("no global `window` exists");

    window
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn audio_loop() {
    let audio_state = unsafe { AUDIO.as_mut().unwrap() };

    if audio_state.needs_new_buffer() {
        // @TODO: lock and collect samples from the main state
        let state = unsafe { STATE.as_mut().unwrap() };
        let samples = state.sample();

        audio_state
            .queue_new_buffer(samples)
            .expect("can't queue new buffer");
    }
}

fn state_loop() {
    let state = unsafe { STATE.as_mut().unwrap() };
    let input_state = unsafe { INPUT_STATE.as_mut().unwrap() };

    let messages = input_state.drain_messages();
    state.process_messages(&messages);
}

fn get_input(event: KeyboardEvent) -> Input {
    match event.key().as_str() {
        "Shift" => Input::Shift,
        "Control" => Input::Control,
        "Alt" => Input::Alt,
        "Backspace" => Input::Backspace,
        "Escape" => Input::Escape,
        "Tab" => Input::Tab,
        "Enter" => Input::Enter,
        c => Input::C(c.chars().next().unwrap_or('?')),
    }
}

fn handle_keydown(event: KeyboardEvent) {
    let input_state = unsafe { INPUT_STATE.as_mut().unwrap() };
    input_state.handle_keydown(get_input(event));
}

fn handle_keyup(event: KeyboardEvent) {
    let input_state = unsafe { INPUT_STATE.as_mut().unwrap() };
    input_state.handle_keyup(get_input(event));
}

fn graphics_loop() {
    let graphics = unsafe { GRAPHICS.as_mut().unwrap() };
    let state = unsafe { STATE.as_mut().unwrap() };

    state.update_ui();
    state.render_ui(graphics);
}

fn handle_drop(event: DragEvent) {
    event.prevent_default();
    let data_transfer = event.data_transfer().unwrap();
    console::log_1(&data_transfer.into());
}

fn setup_callbacks(window: &Window) -> Result<(), JsValue> {
    let drag_cb = Closure::wrap(
        Box::new(move |event: web_sys::DragEvent| event.prevent_default()) as Box<dyn FnMut(_)>,
    );
    let drop_cb = Closure::wrap(Box::new(handle_drop) as Box<dyn FnMut(_)>);
    let audio_cb = Closure::wrap(Box::new(audio_loop) as Box<dyn FnMut()>);
    let state_cb = Closure::wrap(Box::new(state_loop) as Box<dyn FnMut()>);
    let ui_f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let ui_g = ui_f.clone();

    *ui_g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        graphics_loop();
        request_animation_frame(ui_f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(ui_g.borrow().as_ref().unwrap());

    // @TODO: thread if possible?
    window.set_interval_with_callback_and_timeout_and_arguments_0(
        audio_cb.as_ref().unchecked_ref(),
        0,
    )?;

    window.set_interval_with_callback_and_timeout_and_arguments_0(
        state_cb.as_ref().unchecked_ref(),
        100,
    )?;

    window.set_ondrop(Some(drop_cb.as_ref().unchecked_ref()));
    window.set_ondragover(Some(drag_cb.as_ref().unchecked_ref()));

    let keyup_cb = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        event.prevent_default();
        handle_keyup(event);
    }) as Box<dyn FnMut(_)>);

    let keydown_cb = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        event.prevent_default();
        handle_keydown(event);
    }) as Box<dyn FnMut(_)>);

    window.add_event_listener_with_callback("keydown", keydown_cb.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("keyup", keyup_cb.as_ref().unchecked_ref())?;

    drag_cb.forget();
    drop_cb.forget();
    keyup_cb.forget();
    keydown_cb.forget();
    audio_cb.forget();
    state_cb.forget();

    Ok(())
}
