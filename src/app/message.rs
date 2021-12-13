use super::grid::GridPosition;
use super::input_state::{Input, InputState};
    use web_sys::console;

#[derive(Debug, Clone)]
pub enum Message {
    Move(GridPosition),
    MoveTo(GridPosition),
    AddOscillator,
    AddTrack,
    DeleteEntity,
    ClearInput,
    UpdatePrompt,
    SetInput(String),
}

fn get_mult(input: &[Input]) -> i32 {
    input.iter()
        .rev()
        .enumerate()
        .filter_map(|(index, input)| {
            match input {
                Input::C(c) => c.to_digit(10).map(|n| n * 10u32.pow(index as u32 - 1)),
                _ => None,
            }
        })
        .sum::<u32>() as i32
}

impl Message {
    pub fn from_input(state: &InputState, input: &[Input]) -> Option<Self> {
        match (state.control, state.shift, input) {
            // input
            (true, false, &[.., Input::C('[')]) | (false, false, &[.., Input::Escape]) => Some(Message::ClearInput),

            // movement
            (false, false, &[Input::C('g'), Input::C('g')]) => Some(Message::MoveTo(GridPosition::new(0, 0))), // move to origin
            (false, true, &[Input::C('G')]) => Some(Message::MoveTo(GridPosition::new(1000, 1000))), // move to the end optimistically

            (false, false,  &[Input::C('0')]) => Some(Message::Move(GridPosition::new(-1000, 0))), // move to the start of line
            (false, true,  &[Input::C('$')]) => Some(Message::Move(GridPosition::new(1000, 0))), // move to the end of line
            (true, false,  &[Input::C('u')]) => Some(Message::Move(GridPosition::new(0, -8))), // move a "block" up
            (true, false,  &[Input::C('d')]) => Some(Message::Move(GridPosition::new(0, 8))),   // move a "block" down
            (false, true,  &[Input::C('{')]) => Some(Message::Move(GridPosition::new(0, -4))), // move a "block" up
            (false, true,  &[Input::C('}')]) => Some(Message::Move(GridPosition::new(0, 4))),   // move a "block" down
            (false, false,  &[Input::C('b')]) => Some(Message::Move(GridPosition::new(-4, 0))), // move a "block" backwards
            (false, false,  &[Input::C('e')] | &[Input::C('w')]) => Some(Message::Move(GridPosition::new(4, 0))),   // move a "block" forward

            (false, false, &[Input::C('h')]) => Some(Message::Move(GridPosition::new(-1, 0))), // left
            (false, false, &[Input::C('l')]) => Some(Message::Move(GridPosition::new(1, 0))),  // right
            (false, false, &[Input::C('k')]) => Some(Message::Move(GridPosition::new(0, -1))), // up
            (false, false, &[Input::C('j')]) => Some(Message::Move(GridPosition::new(0, 1))),  // down

            (false, false, &[.., Input::C('h')]) => Some(Message::Move(GridPosition::new(-get_mult(&input), 0,))), // left
            (false, false, &[.., Input::C('l')]) => Some(Message::Move(GridPosition::new(get_mult(&input), 0,))), // right
            (false, false, &[.., Input::C('k')]) => Some(Message::Move(GridPosition::new(0, -get_mult(&input),))), // up
            (false, false, &[.., Input::C('j')]) => Some(Message::Move(GridPosition::new(0, get_mult(&input),))), // down

            (false, true, &[Input::C('O')]) => Some(Message::AddOscillator),
            (false, true, &[Input::C('T')]) => Some(Message::AddTrack),
            (false, false, &[Input::C('d'), Input::C('d')]) => Some(Message::DeleteEntity),

            // update input prompt
            _ => {
                let string = input.iter().map(|i| i.to_char()).collect::<String>();
                Some(Message::SetInput(string))
            }
        }
    }
}

