use std::convert::TryInto;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::cmp;

use crate::track::Track;

#[derive(Debug)]
pub enum Mode {
    Normal,
    Input,
}

type Position = (i32, i32);

#[derive(Debug)]
pub struct State {
    pub buffer_size: usize,
    pub channels: usize,
    pub sample_rate: usize,
    pub cursor_pos: Position,
    pub tracks: Vec<Track>,
    pub mode: Mode,
}

impl State {
    pub fn new(buffer_size: usize, channels: usize, sample_rate: usize) -> Self {
	State {
	    buffer_size,
	    channels,
	    sample_rate,
	    cursor_pos: (0,0),
	    tracks: vec![],
	    mode: Mode::Normal,
	}
    }
}

#[derive(Debug)]
pub enum Message {
    Quit,
    SetMode(Mode),
    Move(Position),
    AddTrack(String),
}


fn get_multiplier(input: &[i32]) -> Result<i32, String> {
    input.iter()
	.map(|&c| c as u8 as char)
	.collect::<String>()
	.parse::<i32>()
	.map_err(|e| format!("can't get input multiplier E:{:?}\n for:{:?}", e, input))
}


pub fn handle_input(input: &mut Vec<i32>, state: &State, tx: &mpsc::Sender<Message>) {
    match state.mode {
	Mode::Normal => handle_input_mode_normal(input, &state, tx),
	Mode::Input => handle_input_mode_input(input, &state, tx),
    }
}

pub fn handle_input_mode_normal(input: &mut Vec<i32>, state: &State, tx: &mpsc::Sender<Message>) {
    let msg = match input.as_slice() {
	&[.., 27] => {
	    input.drain(..);
	    None
	}

	&[.., 3] => Some(Message::Quit),

	&[58] => Some(Message::SetMode(Mode::Input)),

	&[104] | &[.., 104] => Some(Message::Move((-get_multiplier(&input[..input.len() - 1]).unwrap_or(1), 0))),
	&[108] | &[.., 108] => Some(Message::Move((get_multiplier(&input[..input.len() - 1]).unwrap_or(1), 0))),
	&[106] | &[.., 106] => Some(Message::Move((0, get_multiplier(&input[..input.len() - 1]).unwrap_or(1)))),
	&[107] | &[.., 107] => Some(Message::Move((0, -get_multiplier(&input[..input.len() - 1]).unwrap_or(1)))),

	_ => None,
    };

    if let Some(msg) = msg {
	input.drain(..);
	tx.send(msg).unwrap();
    }
}


pub fn handle_input_mode_input(input: &mut Vec<i32>, state: &State, tx: &mpsc::Sender<Message>) {
    let msg = match input.as_slice() {
	&[.., 27] | &[.., 3] => {
	    input.drain(..);
	    Some(Message::SetMode(Mode::Normal))
	}
	&[.., 127] => {
	    input.resize_with(
		input.len().saturating_sub(2),
		Default::default
	    );
	    None
	}
	_ => None,
    };

    if let Some(msg) = msg {
	tx.send(msg).unwrap();
    }
}


pub fn handle_message(msg: Message, state: &Arc<Mutex<State>>) -> () {
    match msg {
	Message::Quit => (),
	Message::SetMode(mode) => {
	    let mut state = state.lock().unwrap();
	    state.mode = mode;
	}
	Message::AddTrack(path) => {
	    let track: Result<Track, String> = path.try_into();

	    match track {
		Ok(track) => {
		    let mut state = state.lock().unwrap();
		    let buffer_size = state.buffer_size;
		    state.tracks.push(track.set_buffer_size(buffer_size));
		}
		Err(e) => println!("{}", e),
	    }
	}

	Message::Move(pos) => {
	    let mut state = state.lock().unwrap();
	    let (delta_x, delta_y) = pos;
	    let (x, y) = state.cursor_pos;

	    state.cursor_pos = (x + delta_x, y + delta_y);
	}
    }
}
