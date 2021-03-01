use std::convert::TryInto;
use std::sync::{Arc, Mutex};

use crate::track::Track;

#[derive(Debug)]
pub enum Mode {
    Normal,
}

#[derive(Debug)]
pub struct State {
    pub buffer_size: usize,
    pub channels: usize,
    pub sample_rate: usize,
    pub cursor_pos: (u32, u32),
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
    AddTrack(String),
}

pub fn handle_input(keys: &[i32]) -> Option<Message> {
    match keys {
	// A
	&[65] => Some(Message::AddTrack("./test_files/f#_warm.wav".to_owned())),
	_ => None,
    }
}

pub fn handle_message(msg: Message, state: &Arc<Mutex<State>>) -> () {
    match msg {
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
    }
}
