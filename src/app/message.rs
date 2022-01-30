use super::grid::ConnType;
use super::grid::Position;
use super::input_state::{Input, InputState};

#[derive(Clone)]
pub enum Modules {
    Track,
    VCO,
}

#[derive(Clone)]
pub enum Message {
    Move(Position),
    MoveTo(Position),
    AddConnector(ConnType),
    AddModule(Modules),
    DeleteEntity,
    ClearInput,
    UpdatePrompt,
    UpdateEntities,
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
    pub fn from_input(state: &InputState, input: &[Input]) -> Option<Self> {
        match (state.control, state.shift, input) {
            // input
            (true, false, &[.., Input::C('[')]) | (false, false, &[.., Input::Escape]) => {
                Some(Message::ClearInput)
            }

            // movement
            (false, false, &[Input::C('g'), Input::C('g')]) => {
                Some(Message::MoveTo(Position::new(0, 0)))
            } // move to origin
            (false, true, &[Input::C('G')]) => Some(Message::MoveTo(Position::new(1000, 1000))), // move to the end optimistically

            (false, false, &[Input::C('0')]) => Some(Message::Move(Position::new(-1000, 0))), // move to the start of line
            (false, true, &[Input::C('$')]) => Some(Message::Move(Position::new(1000, 0))), // move to the end of line
            (true, false, &[Input::C('u')]) => Some(Message::Move(Position::new(0, -8))), // move a "block" up
            (true, false, &[Input::C('d')]) => Some(Message::Move(Position::new(0, 8))), // move a "block" down
            (false, true, &[Input::C('{')]) => Some(Message::Move(Position::new(0, -4))), // move a "block" up
            (false, true, &[Input::C('}')]) => Some(Message::Move(Position::new(0, 4))), // move a "block" down
            (false, false, &[Input::C('b')]) => Some(Message::Move(Position::new(-4, 0))), // move a "block" backwards
            (false, false, &[Input::C('e')] | &[Input::C('w')]) => {
                Some(Message::Move(Position::new(4, 0)))
            } // move a "block" forward

            (false, false, &[Input::C('h')]) => Some(Message::Move(Position::new(-1, 0))), // left
            (false, false, &[Input::C('l')]) => Some(Message::Move(Position::new(1, 0))),  // right
            (false, false, &[Input::C('k')]) => Some(Message::Move(Position::new(0, -1))), // up
            (false, false, &[Input::C('j')]) => Some(Message::Move(Position::new(0, 1))),  // down

            (false, false, &[.., Input::C('h')]) => {
                Some(Message::Move(Position::new(-get_mult(&input), 0)))
            } // left
            (false, false, &[.., Input::C('l')]) => {
                Some(Message::Move(Position::new(get_mult(&input), 0)))
            } // right
            (false, false, &[.., Input::C('k')]) => {
                Some(Message::Move(Position::new(0, -get_mult(&input))))
            } // up
            (false, false, &[.., Input::C('j')]) => {
                Some(Message::Move(Position::new(0, get_mult(&input))))
            } // down

            // modules
            (false, true, &[Input::C('O')]) => Some(Message::AddModule(Modules::VCO)),
            (false, true, &[Input::C('M')]) => Some(Message::AddModule(Modules::Track)),

            // connectors
            (false, false, &[Input::C('s')]) => Some(Message::AddConnector(ConnType::S)),
            (false, false, &[Input::C('i')]) => Some(Message::AddConnector(ConnType::I)),
            (false, false, &[Input::C('f')]) => Some(Message::AddConnector(ConnType::F)),
            (false, false, &[Input::C('p')]) => Some(Message::AddConnector(ConnType::P)),

            (false, false, &[Input::C('d'), Input::C('d')]) => Some(Message::DeleteEntity),

            // update input prompt
            _ => {
                let string = input.iter().map(|i| i.to_char()).collect::<String>();
                Some(Message::SetInput(string))
            }
        }
    }
}
