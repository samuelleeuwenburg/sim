extern crate ncurses;
pub mod state;

use std::convert::From;
use crate::state::{State, Mode};
use crate::track::Track;
use ncurses::*;

pub struct WindowState {
    height: i32,
    width: i32,
}

pub fn setup() -> WindowState {
    initscr();
    keypad(stdscr(), true);
    noecho();
    raw();
    set_escdelay(0);
    nodelay(stdscr(), true);

    let mut state = WindowState {
	height: 0,
	width: 0,
    };

    getmaxyx(stdscr(), &mut state.height, &mut state.width);

    state
}

enum ViewAttribute {
    Bold,
}

struct View {
    string: String,
    attributes: Vec<ViewAttribute>,
}

impl From<String> for View {
    fn from(string: String) -> Self {
	View { string, attributes: vec![] }
    }
}

impl View {
    fn set_bold(&mut self) {
	self.attributes.push(ViewAttribute::Bold);
    }
}

pub fn get_input() -> Vec<i32> {
    let mut keys = vec![];

    let mut key = getch();
    while key != -1 {
	keys.push(key);
	key = getch();
    }

    keys
}

fn draw_at(x: i32, y: i32, view: &View){
    mvprintw(y, x, &view.string);
}

pub fn draw(window: &WindowState, state: &State, input_state: &state::InputState) {
    clear();

    let (x, y) = state.cursor_pos;
    mvprintw(y, x, "X");

    for track in &state.tracks {
	let (x, y) = track.position;
	let view = draw_track(&track);
	draw_at(x, y, &view);
    }

    if let Some(msg) = &state.error_message {
	draw_at(0, 0, &draw_error_message(&msg));
    }

    if let Some(msg) = &input_state.user_message {
	draw_at(0, window.height - 2, &draw_user_message(&msg));
    }

    draw_at(0, window.height - 1, &draw_input_mode(&input_state.input_buffer, &state.mode));

    refresh();
}

fn draw_track(track: &Track) -> View {
    let mut string = String::from("T ");

    let sample = match &track.sample {
	Some(sample) => {
	    let mut sample_string = String::from("s[");

	    let total_volume = sample.buffer.samples
		.iter()
		.filter(|&&s| s > 0.0)
		.fold(0.0, |s, sum| s + sum);

	    let average_volume = total_volume / sample.buffer.samples.len() as f32;

	    let volume = match average_volume {
		x if (0.0..0.2).contains(&x) => "▁",
		x if (0.2..0.4).contains(&x) => "▂",
		x if (0.4..0.6).contains(&x) => "▃",
		x if (0.6..0.8).contains(&x) => "▆",
		x if (0.8..1.0).contains(&x) => "▉",
		_ => " ",
	    };

	    let total_length = (sample.stream.samples.len() / sample.stream.channels / 44_100).to_string();
	    let pos = (sample.position / sample.stream.channels /  44_100).to_string();
	    sample_string.push_str(&pos);
	    sample_string.push_str("/");
	    sample_string.push_str(&total_length);
	    sample_string.push_str("] ");
	    sample_string.push_str(&volume);
	    sample_string
	}
	None => "-".to_string()
    };

    string.push_str(&sample);
    string.into()
}

fn draw_error_message(message: &str) -> View {
    let mut string = String::from("error: ");
    string.push_str(&message);
    string.into()
}

fn draw_user_message(message: &str) -> View {
    let mut string = String::from("> ");
    string.push_str(&message);
    string.into()
}

fn draw_input_mode(buffer: &Vec<i32>, mode: &Mode) -> View {
    let mut string = match mode {
	Mode::Normal => "normal ",
	Mode::Input =>  ":",
    }.to_string();

    let readable: String = buffer.iter().map(|&c| c as u8 as char).collect();
    string.push_str(&readable);

    string.into()
}

pub fn quit() {
    endwin();
}
