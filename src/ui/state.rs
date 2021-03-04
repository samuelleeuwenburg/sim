use std::convert::{TryInto, TryFrom};
use std::sync::mpsc;
use crate::state::{Message, Mode, State};

#[derive(Debug, Clone)]
pub enum Command {
    Simple(SimpleCommand),
    Arg(ArgCommand),
}

#[derive(Debug, Clone)]
pub enum SimpleCommand {
    Quit,
}

#[derive(Debug, Clone)]
pub enum ArgCommand {
    AddTrack,
}

impl ArgCommand {
    pub fn get_user_message(&self, input: &[i32]) -> String {
	match self {
	    ArgCommand::AddTrack => {
		// @TODO: turn into re-usable struct
		let args = vec!["path", " title"];

		let input = input
		    .into_iter()
		    .map(|&c| c as u8 as char)
		    .collect::<String>();

		let input_args: Vec<&str> = input.split(":").collect();

		let mut msg = String::from("");

		for (i, arg) in args.into_iter().enumerate() {
		    let input = input_args.get(i);
		    let mut message = arg.to_owned();

		    let string = match input {
			Some(&"") | None => {
			    message.push_str("[!]");
			    message
			}
			Some(input) => {
			    message.push_str("[g]");

			    let length = if input.len() > message.len() { input.len() - message.len() } else { 0 };
			    let whitespace: String = vec![" "; length].into_iter().collect();
			    message.push_str(&whitespace);

			    message
			}
		    };

		    msg.push_str(string.as_str());
		}

		msg
	    }
	}
    }
}

impl TryFrom<&[i32]> for Command {
    type Error = String;

    fn try_from(input: &[i32]) -> Result<Self, Self::Error> {
	let readable: String = input.iter().map(|&c| c as u8 as char).collect();

	match readable.as_str() {
	    "q" | "qa" | "qa!" | "q!" | "exit" => Ok(Command::Simple(SimpleCommand::Quit)),
	    "add_track" => Ok(Command::Arg(ArgCommand::AddTrack)),
	    _ => Err(format!("unknown command: {:?}", input).to_owned()),
	}
    }
}

#[derive(Debug, PartialEq)]
pub enum Arg {
    Path(String),
    Title(String),
}

#[derive(Debug)]
pub struct InputState {
    pub user_message: Option<String>,
    pub input_buffer: Vec<i32>,
    pub active_command: Option<ArgCommand>,
}

impl InputState {
    pub fn new() -> Self {
	InputState { user_message: None, input_buffer: vec![], active_command: None }
    }

    pub fn reset(&mut self) {
	self.user_message = None;
	self.input_buffer = vec![];
	self.active_command =  None;
    }
}

fn get_multiplier(input: &[i32]) -> Result<i32, String> {
    input.iter()
	.map(|&c| c as u8 as char)
	.collect::<String>()
	.parse::<i32>()
	.map_err(|e| format!("can't get input multiplier E:{:?}\n for:{:?}", e, input))
}


pub fn handle_input(input_state: &mut InputState, state: &State, tx: &mpsc::Sender<Message>) -> Result<(), String> {
    match state.mode {
	Mode::Normal => handle_input_mode_normal(input_state, &state, tx),
	Mode::Input => handle_input_mode_input(input_state, &state, tx),
    }
}

pub fn handle_input_mode_normal(
    input_state: &mut InputState,
    state: &State,
    tx: &mpsc::Sender<Message>
) -> Result<(), String> {
    let input = &mut input_state.input_buffer;

    let msg = match input.as_slice() {
	&[.., 27] => {
	    input.drain(..);
	    None
	}

	&[58] => Some(Message::SetMode(Mode::Input)),

	&[104] | &[.., 104] => Some(Message::Move((-get_multiplier(&input[..input.len() - 1]).unwrap_or(1), 0))),
	&[108] | &[.., 108] => Some(Message::Move((get_multiplier(&input[..input.len() - 1]).unwrap_or(1), 0))),
	&[106] | &[.., 106] => Some(Message::Move((0, get_multiplier(&input[..input.len() - 1]).unwrap_or(1)))),
	&[107] | &[.., 107] => Some(Message::Move((0, -get_multiplier(&input[..input.len() - 1]).unwrap_or(1)))),

	_ => None,
    };

    match msg {
	Some(msg) => {
	    input.drain(..);
	    tx.send(msg).map_err(|e| e.to_string())
	}
	None => Ok(()),
    }
}


pub fn handle_input_mode_input(
    input_state: &mut InputState,
    state: &State,
    tx: &mpsc::Sender<Message>
) -> Result<(), String> {
    // if let Some(command @ Command::Arg(_)) = &input_state.command {
    // 	let input = &mut input_state.input_buffer;
    // 	input_state.user_message = Some(command.get_user_message(&input[..]));
    // };

    
    let msg = match input_state.input_buffer.as_slice() {
	&[.., 27] | &[.., 3] => {
	    input_state.reset();
	    Some(Message::SetMode(Mode::Normal))
	}
	&[.., 127] => {
	    input_state.input_buffer.resize_with(
		input_state.input_buffer.len().saturating_sub(2),
		Default::default
	    );
	    None
	}
	&[.., 10] => {
	    // if let Some(command) = &input_state.active_command {
	    // 	let msg = Message::RunCommand(command.clone(), input_state.input_buffer.clone());
	    // 	input_state.reset();
	    // 	Some(msg)
	    // } else {

	    let input = &mut input_state.input_buffer;
	    let command: Result<Command, String> = input[..input.len() - 1].try_into();

	    match command {
		Ok(Command::Simple(command)) => {
		    Some(Message::RunSimpleCommand(command.clone()))
		}
		Ok(Command::Arg(command)) => {
		    let message = command.get_user_message(&input[..input.len() - 1]);
		    input_state.active_command = Some(command);
		    input.drain(..);
		    input_state.user_message = Some(message);
		    None
		}
		Err(_) => {
		    input_state.reset();
		    let message = "uknown command".to_owned();
		    input_state.user_message = Some(message);
		    Some(Message::SetMode(Mode::Normal))
		}
	    }
		// if let Ok(cmd @ Command::Simple(_)) = command {
		//     Some(Message::RunCommand(cmd.clone(), input_state.input_buffer.clone()))
		// } else {
		//     match command {
		// 	Ok(command) => {
		// 	    let message = command.get_user_message(&input[..input.len() - 1]);
		// 	    input_state.command = Some(command);
		// 	    input.drain(..);
		// 	    input_state.user_message = Some(message);
		// 	    None
		// 	},
		// 	Err(_) => {
		// 	    input_state.reset();
		// 	    let message = "uknown command".to_owned();
		// 	    input_state.user_message = Some(message);
		// 	    Some(Message::SetMode(Mode::Normal))
		// 	}
		//     }
		// }
	    // }
	}
	_ => None,
    };

    match msg {
	Some(msg) => tx.send(msg).map_err(|e| e.to_string()),
	None => Ok(()),
    }
}