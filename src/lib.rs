use screech::basic::{Oscillator, Track};
use screech::core::{BasicTracker, Primary};
use screech::traits::Source;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, AudioBuffer, AudioContext, Window};

const BUFFER_SIZE: usize = 480;

struct State {
    primary: Primary<BUFFER_SIZE>,
    oscillators: Vec<Oscillator>,
    tracks: Vec<Track>,
}

impl State {
    fn new(sample_rate: usize) -> Self {
        let tracker = Box::new(BasicTracker::<256>::new());

        State {
            primary: Primary::with_tracker(tracker, sample_rate),
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

struct AudioState {
    ctx: AudioContext,
    buffer_position: f64,
}

impl AudioState {
    fn new(ctx: AudioContext) -> Self {
        let buffer_position = ctx.current_time();
        AudioState {
            ctx,
            buffer_position,
        }
    }
}

static mut STATE: Option<State> = None;
static mut AUDIO_STATE: Option<AudioState> = None;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("no global `window` exists");
    let audio_context = web_sys::AudioContext::new().expect("no audiocontext available");

    // setup audio state
    let audio_state = AudioState::new(audio_context);
    unsafe { AUDIO_STATE = Some(audio_state) };

    // setup main state
    let mut state = State::new(48_000);

    let count = 100;

    for i in 0..count {
        let mut osc = Oscillator::new(&mut state.primary);
        osc.output_saw();
        osc.amplitude = 0.01;
        state.primary.add_monitor(&osc);

        osc.frequency = 110.0; // a

        osc.frequency += (i % count) as f32 * 0.01;

        state.oscillators.push(osc);
    }

    unsafe { STATE = Some(state) };

    // setup ticks
    setup_ticks(&window)?;

    Ok(())
}

fn create_buffer(ctx: &AudioContext) -> Result<AudioBuffer, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let performance = window
        .performance()
        .expect("performance should be available");

    let now = performance.now();

    let state = unsafe { STATE.as_mut().unwrap() };
    let channels = 2;
    let sample_rate = 48_000.0;
    let samples = state.sample();

    let mut left = [0.0; BUFFER_SIZE];
    let mut right = [0.0; BUFFER_SIZE];

    for (i, sample) in samples.into_iter().enumerate() {
        if i % 2 != 0 {
            left[i / 2] = sample;
        } else {
            right[(i + 1) / 2] = sample;
        }
    }

    let buffer = ctx.create_buffer(channels, BUFFER_SIZE as u32, sample_rate)?;
    buffer.copy_to_channel(&mut left, 0)?;
    buffer.copy_to_channel(&mut right, 1)?;

    console::log_1(&format!("wasm performance: {}ms", performance.now() - now).into());
    Ok(buffer)
}

fn audio_tick() {
    let mut audio_state = unsafe { AUDIO_STATE.as_mut().unwrap() };
    let current_time = audio_state.ctx.current_time();
    let buffer_size_in_s = BUFFER_SIZE as f64 / 48_000.0;

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
    }
}

fn ui_tick() {
    console::log_1(&format!("hello from ui!").into());
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    let window = web_sys::window().expect("no global `window` exists");

    window
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn setup_ticks(window: &Window) -> Result<(), JsValue> {
    let audio_cb = Closure::wrap(Box::new(audio_tick) as Box<dyn FnMut()>);
    let ui_f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let ui_g = ui_f.clone();

    *ui_g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        ui_tick();
        request_animation_frame(ui_f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(ui_g.borrow().as_ref().unwrap());

    window.set_interval_with_callback_and_timeout_and_arguments_0(
        audio_cb.as_ref().unchecked_ref(),
        0,
    )?;

    audio_cb.forget();

    Ok(())
}
