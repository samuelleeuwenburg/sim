use screech::basic::{Oscillator, Track};
use screech::core::{BasicTracker, Primary};
use screech::traits::{Tracker, Source};
use std::cell::RefCell;
use std::cmp::{max, min};
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    console, AudioBuffer, AudioContext, CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent,
    Window,
};

const BUFFER_SIZE: usize = 4800;

#[derive(Debug, Clone, Copy)]
struct GridPosition {
    x: i32,
    y: i32,
}

impl GridPosition {
    fn new(x: i32, y: i32) -> Self {
        GridPosition { x, y }
    }

    fn move_to(&mut self, pos: &Self) -> &mut Self {
	self.x = pos.x;
	self.y = pos.y;
	self
    }
}

trait GridEntity {
    fn set_position(&mut self, position: &GridPosition);
    fn get_position(&self) -> &GridPosition;
    fn get_display(&self) -> String;
    fn get_prompt(&self) -> String;
}

struct VCO {
    grid_position: GridPosition,
    oscillator: Oscillator,
}

impl VCO {
    fn new(tracker: &mut dyn Tracker) -> Self {
        VCO {
            grid_position: GridPosition::new(0, 0),
            oscillator: Oscillator::new(tracker),
        }
    }
}

impl GridEntity for VCO {
    fn set_position(&mut self, position: &GridPosition) {
	self.grid_position.move_to(position);
    }

    fn get_position(&self) -> &GridPosition {
	&self.grid_position
    }

    fn get_display(&self) -> String {
	String::from("O")
    }

    fn get_prompt(&self) -> String {
	format!("Oscillator @ {} freq", self.oscillator.frequency)
    }
}

struct State {
    primary: Primary<BUFFER_SIZE>,
    oscillators: Vec<VCO>,
    freq_pos: usize,
    tracks: Vec<Track>,
}

impl State {
    fn new(sample_rate: usize) -> Self {
        let tracker = Box::new(BasicTracker::<256, 8>::new());

        State {
            primary: Primary::with_tracker(tracker, sample_rate),
            oscillators: vec![],
            tracks: vec![],
            freq_pos: 1,
        }
    }

    fn sample(&mut self) -> &[f32; BUFFER_SIZE] {
        let mut a: Vec<&mut dyn Source> = self
            .oscillators
            .iter_mut()
            .map(|vco| &mut vco.oscillator as &mut dyn Source)
            .collect();

        let mut b: Vec<&mut dyn Source> = self
            .tracks
            .iter_mut()
            .map(|t| t as &mut dyn Source)
            .collect();

        a.append(&mut b);

        self.primary.sample(a).unwrap()
    }

    fn process_command(&mut self, command: &Command) {
        match command {
            Command::AddOscillator(pos) => {
                let mut vco = VCO::new(&mut self.primary);
		vco.set_position(&pos);
                let f = 110.0 * self.freq_pos as f32;

                vco.oscillator.output_saw();
                vco.oscillator.amplitude = 0.1;
                vco.oscillator.frequency = f;

                self.primary.add_monitor(vco.oscillator.output);
                self.oscillators.push(vco);
                self.freq_pos = if self.freq_pos >= 16 {
                    1
                } else {
                    self.freq_pos + 1
                };

                console::log_1(&format!("added osc! f: {}, {}", f, self.oscillators.len()).into());
            }
            Command::DeleteAt(position) => {
                let mut index = 0;
                for vco in self.oscillators.iter() {
		    let pos = vco.get_position();
                    if pos.x == position.x && pos.y == position.y {
                        self.oscillators.swap_remove(index);
                        console::log_1(&format!("removed osc!: {}", self.oscillators.len()).into());
                        break;
                    }

                    index += 1;
                }
            }
            _ => (),
        }
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

struct UIState {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    cursor_position: (i32, i32),
    grid_size: (u32, u32),
    prompt: String,
}

impl UIState {
    fn new(canvas: HtmlCanvasElement) -> Self {
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        canvas.set_width(800);
        canvas.set_height(640);

        UIState {
            ctx,
            canvas,
            cursor_position: (0, 0),
            grid_size: (32, 16),
            prompt: String::from("Oscillator example ..."),
        }
    }

    fn process_command(&mut self, command: &Command) {
        match command {
            Command::Move(pos) => {
                let (w, h) = self.grid_size;
                let new_x = self.cursor_position.0 + pos.x;
                let new_y = self.cursor_position.1 + pos.y;

                self.cursor_position.0 = max(0, min(w as i32 - 1, new_x));
                self.cursor_position.1 = max(0, min(h as i32 - 1, new_y));
            }
            _ => (),
        }
    }
}

#[derive(Debug)]
enum Command {
    Move(GridPosition),
    AddOscillator(GridPosition),
    DeleteAt(GridPosition),
}

struct InputState {
    input_buffer: Vec<u32>,
}

impl InputState {
    fn new() -> Self {
        InputState {
            input_buffer: Vec::with_capacity(10),
        }
    }

    fn process_input(&mut self, ui_state: &UIState) -> Option<Command> {
        let (x, y) = ui_state.cursor_position;

        let command = match self.input_buffer.as_slice() {
            &[.., 27] | &[.., 17, 219] => {
                self.input_buffer.clear();
                None
            }
            &[72] | &[37] => Some(Command::Move(GridPosition::new(-1, 0))), // left
            &[76] | &[39] => Some(Command::Move(GridPosition::new(1, 0))),  // right
            &[75] | &[38] => Some(Command::Move(GridPosition::new(0, -1))), // up
            &[74] | &[40] => Some(Command::Move(GridPosition::new(0, 1))),  // down
            &[16, 79] => Some(Command::AddOscillator(GridPosition::new(x, y))),
            &[68, 68] => Some(Command::DeleteAt(GridPosition::new(x, y))),
            _ => None,
        };

        if command.is_some() {
            self.input_buffer.clear();
        }

        command
    }
}

static mut STATE: Option<State> = None;
static mut AUDIO_STATE: Option<AudioState> = None;
static mut UI_STATE: Option<UIState> = None;
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

    unsafe { UI_STATE = Some(UIState::new(canvas)) };
    unsafe { AUDIO_STATE = Some(AudioState::new(audio_context)) };
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
    let mut audio_state = unsafe { AUDIO_STATE.as_mut().unwrap() };
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
    }
}

fn ui_tick() {
    let ui_state = unsafe { UI_STATE.as_mut().unwrap() };

    let ctx = &ui_state.ctx;

    let (pos_x, pos_y) = ui_state.cursor_position;
    let (grid_width, grid_height) = ui_state.grid_size;

    let height = ui_state.canvas.height();
    let width = ui_state.canvas.width();

    let block_width = 18;
    let block_height = 24;
    let origin_x = width / 2 - (grid_width * block_width) / 2;
    let origin_y = height / 2 - (grid_height * block_height) / 2;

    ctx.clear_rect(0.0, 0.0, width.into(), height.into());

    // render grid
    for x in 0..grid_width {
        for y in 0..grid_height {
            let px_x = origin_x + (x * block_width);
            let px_y = origin_y + (y * block_height);

            if x as i32 == pos_x && y as i32 == pos_y.into() {
                ctx.fill_rect(
                    px_x.into(),
                    px_y.into(),
                    block_width.into(),
                    block_height.into(),
                );
            }

            ctx.stroke_rect(
                px_x.into(),
                px_y.into(),
                block_width.into(),
                block_height.into(),
            );
        }
    }

    // render oscillators
    let state = unsafe { STATE.as_ref().unwrap() };
    ctx.set_font("18px sans-serif");
    ctx.set_text_baseline("hanging");

    for vco in state.oscillators.iter() {
	let pos = vco.get_position();
        let px_x = origin_x + (pos.x as u32 * block_width);
        let px_y = origin_y + (pos.y as u32 * block_height);

        ctx.fill_text(&vco.get_display(), px_x.into(), px_y.into()).unwrap();
    }

    // render prompt
    let prompt_y = origin_y + block_height * grid_height;
    ctx.fill_text(&ui_state.prompt, origin_x.into(), prompt_y.into())
        .unwrap();
}

fn handle_input(event: KeyboardEvent) {
    let input_state = unsafe { INPUT_STATE.as_mut().unwrap() };
    let ui_state = unsafe { UI_STATE.as_mut().unwrap() };

    input_state.input_buffer.push(event.key_code());

    if let Some(command) = input_state.process_input(&ui_state) {
        let state = unsafe { STATE.as_mut().unwrap() };

        ui_state.process_command(&command);
        state.process_command(&command);
    }

    console::log_1(&format!("input: {:?}", input_state.input_buffer).into());
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    let window = web_sys::window().expect("no global `window` exists");

    window
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn setup_callbacks(window: &Window) -> Result<(), JsValue> {
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

    let input_cb = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        event.prevent_default();
        handle_input(event);
    }) as Box<dyn FnMut(_)>);
    window.add_event_listener_with_callback("keydown", input_cb.as_ref().unchecked_ref())?;

    input_cb.forget();
    audio_cb.forget();

    Ok(())
}
