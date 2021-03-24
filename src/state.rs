use std::convert::TryInto;
use std::sync::{Arc, Mutex};

use crate::track::Track;
use crate::traits::Playable;
use crate::ui::state::{ArgCommand, SimpleCommand};

#[derive(Debug)]
pub enum Mode {
    Normal,
    Input,
}

#[derive(Debug, PartialEq)]
pub enum Flag {
    Quit,
}

type Position = (i32, i32);

pub struct State {
    pub buffer_size: usize,
    pub channels: usize,
    pub sample_rate: usize,
    pub cursor_pos: Position,
    pub view_pos: Position,
    pub tracks: Vec<Track>,
    pub mode: Mode,
    pub flags: Vec<Flag>,
    pub error_message: Option<String>,
}

impl State {
    pub fn new(buffer_size: usize, channels: usize, sample_rate: usize) -> Self {
        State {
            buffer_size,
            channels,
            sample_rate,
            cursor_pos: (0, 0),
            view_pos: (0, 0),
            tracks: vec![],
            mode: Mode::Normal,
            flags: vec![],
            error_message: None,
        }
    }
}

#[derive(Debug)]
pub enum Message {
    SetMode(Mode),
    Move(Position),
    RunSimpleCommand(SimpleCommand),
    RunCommand(ArgCommand, Vec<i32>),
}

pub fn handle_message(msg: Message, state: &Arc<Mutex<State>>) -> () {
    match msg {
        Message::SetMode(mode) => {
            let mut state = state.lock().unwrap();
            state.mode = mode;
        }

        Message::RunSimpleCommand(command) => {
            handle_simple_command(&command, state);
            let mut state = state.lock().unwrap();
            state.mode = Mode::Normal;
        }

        Message::RunCommand(command, input) => {
            handle_arg_command(&command, &input, state);
            let mut state = state.lock().unwrap();
            state.mode = Mode::Normal;
        }

        Message::Move(pos) => {
            let mut state = state.lock().unwrap();
            let (delta_x, delta_y) = pos;
            let (x, y) = state.cursor_pos;

            state.cursor_pos = (x + delta_x, y + delta_y);
        }
    }
}

pub fn handle_arg_command(command: &ArgCommand, input: &Vec<i32>, state: &Arc<Mutex<State>>) -> () {
    match command {
        ArgCommand::AddTrack => {
            let path: String = input.iter().map(|&c| c as u8 as char).collect();

            let track: Result<Track, String> = path.to_owned().try_into();

            match track {
                Ok(track) => {
                    let mut state = state.lock().unwrap();
                    let buffer_size = state.buffer_size;

                    let mut track = track;
                    track.set_buffer_size(buffer_size);
                    track.set_position(state.cursor_pos);

                    state.tracks.push(track);
                }
                Err(e) => {
                    let mut state = state.lock().unwrap();
                    state.error_message = Some(e.to_string());
                }
            }
        }
    }
}

pub fn handle_simple_command(command: &SimpleCommand, state: &Arc<Mutex<State>>) -> () {
    match command {
        SimpleCommand::Quit => {
            let mut state = state.lock().unwrap();
            state.flags.push(Flag::Quit);
        }
    }
}
