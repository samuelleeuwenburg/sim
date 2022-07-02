use super::grid::Position;
use super::input_state::{Input, InputMode, InputState};

#[derive(Clone)]
pub enum Message {
    SwitchInputMode(InputMode),
    JumpSetting(i32),
    ProcessInput,
    Move(Position),
    MoveTo(Position),
    MoveToEmpty,
    AddStep,
    AddTrigger,
    DeleteEntity,
    ClearInput,
    UpdatePrompt,
    SetInput(String),
}

fn get_mult(input: &[Input]) -> i32 {
    input
        .iter()
        .rev()
        .enumerate()
        .filter_map(|(index, input)| match input {
            Input::C(c) => c.to_digit(10).map(|n| n * 10u32.pow(index as u32 - 1)),
            _ => None,
        })
        .sum::<u32>() as i32
}

impl Message {
    pub fn from_input(state: &InputState, input: &[Input]) -> Vec<Self> {
        let commands = match state.mode {
            InputMode::Command => Self::mode_command(state, input),
            InputMode::Insert => Self::mode_insert(state, input),
            InputMode::Edit(_) => Self::mode_edit(state, input),
        };

        match commands.as_slice() {
            [] => {
                let string = input.iter().map(|i| i.to_char()).collect::<String>();
                vec![Message::SetInput(string)]
            }
            _ => commands,
        }
    }

    fn mode_command(state: &InputState, input: &[Input]) -> Vec<Self> {
        match (state.control, state.shift, input) {
            // clear
            (true, false, &[.., Input::C('[')]) | (false, false, &[.., Input::Escape]) => {
                vec![Message::ClearInput]
            }

            // change modes
            (false, false, &[.., Input::C('i')]) => {
                vec![Message::SwitchInputMode(InputMode::Insert)]
            }

            (false, false, &[Input::Tab]) => {
                vec![Message::SwitchInputMode(InputMode::Edit(0))]
            }

            // movement
            (false, false, &[Input::C('g'), Input::C('g')]) => {
                vec![Message::MoveTo(Position::new(0, 0))]
            } // move to origin
            (false, true, &[Input::C('G')]) => vec![Message::MoveTo(Position::new(1000, 1000))], // move to the end optimistically

            (false, false, &[Input::C('0')]) => vec![Message::Move(Position::new(-1000, 0))], // move to the start of line
            (false, true, &[Input::C('$')]) => vec![Message::Move(Position::new(1000, 0))], // move to the end of line
            (true, false, &[Input::C('u')]) => vec![Message::Move(Position::new(0, -8))], // move a "block" up
            (true, false, &[Input::C('d')]) => vec![Message::Move(Position::new(0, 8))], // move a "block" down
            (false, true, &[Input::C('{')]) => vec![Message::Move(Position::new(0, -4))], // move a "block" up
            (false, true, &[Input::C('}')]) => vec![Message::Move(Position::new(0, 4))], // move a "block" down
            (false, false, &[Input::C('b')]) => vec![Message::Move(Position::new(-4, 0))], // move a "block" backwards
            (false, false, &[Input::C('e')] | &[Input::C('w')]) => {
                vec![Message::Move(Position::new(4, 0))]
            } // move a "block" forward

            (false, false, &[Input::C('h')]) => vec![Message::Move(Position::new(-1, 0))], // left
            (false, false, &[Input::C('l')]) => vec![Message::Move(Position::new(1, 0))],  // right
            (false, false, &[Input::C('k')]) => vec![Message::Move(Position::new(0, -1))], // up
            (false, false, &[Input::C('j')]) => vec![Message::Move(Position::new(0, 1))],  // down

            (false, false, &[.., Input::C('h')]) => {
                vec![Message::Move(Position::new(-get_mult(input), 0))]
            } // left
            (false, false, &[.., Input::C('l')]) => {
                vec![Message::Move(Position::new(get_mult(input), 0))]
            } // right
            (false, false, &[.., Input::C('k')]) => {
                vec![Message::Move(Position::new(0, -get_mult(input)))]
            } // up
            (false, false, &[.., Input::C('j')]) => {
                vec![Message::Move(Position::new(0, get_mult(input)))]
            } // down

            // deletion
            (false, false, &[Input::C('d'), Input::C('d')]) => vec![Message::DeleteEntity],

            _ => vec![],
        }
    }

    fn mode_insert(state: &InputState, input: &[Input]) -> Vec<Self> {
        match (state.control, state.shift, input) {
            // exit mode
            (true, false, &[.., Input::C('[')]) | (false, false, &[.., Input::Escape]) => {
                vec![Message::SwitchInputMode(InputMode::Command)]
            }

            // movement
            (false, false, &[Input::C(' ')]) => vec![Message::Move(Position::new(1, 0))],
            (false, false, &[Input::Enter]) => vec![Message::Move(Position::new(-10000, 1))],
            (false, true, &[Input::Enter]) => vec![Message::Move(Position::new(0, 1))],

            // entities
            (false, false, &[Input::C('.')]) => {
                vec![Message::AddStep, Message::MoveToEmpty]
            }
            (false, false, &[Input::C('t')]) => {
                vec![Message::AddTrigger, Message::MoveToEmpty]
            }
            _ => vec![],
        }
    }

    fn mode_edit(state: &InputState, input: &[Input]) -> Vec<Self> {
        match (state.control, state.shift, input) {
            // exit mode
            (true, false, &[.., Input::C('[')]) | (false, false, &[.., Input::Escape]) => {
                vec![Message::SwitchInputMode(InputMode::Command)]
            }

            // navigate settings
            (false, false, &[Input::Tab]) => vec![Message::JumpSetting(1)],
            (false, true, &[Input::Tab]) => vec![Message::JumpSetting(-1)],

            (false, false, &[Input::Enter]) => vec![Message::ProcessInput],

            _ => vec![],
        }
    }
}
