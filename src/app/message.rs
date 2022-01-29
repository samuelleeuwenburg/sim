use super::entities::ModType;
use super::grid::{Entity, Position};
use super::input_state::{Input, InputState};

#[derive(Clone)]
pub enum EntityMsg {
    Track,
    VCO,
    Modifier(ModType),
}

#[derive(Clone)]
pub enum Message {
    Move(Position),
    MoveTo(Position),
    AddEntity(EntityMsg),
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
            (false, true, &[Input::C('O')]) => Some(Message::AddEntity(EntityMsg::VCO)),
            (false, true, &[Input::C('T')]) => Some(Message::AddEntity(EntityMsg::Track)),

            // modifiers
            (false, false, &[Input::C('s')]) => {
                Some(Message::AddEntity(EntityMsg::Modifier(ModType::S)))
            }
            (false, false, &[Input::C('i')]) => {
                Some(Message::AddEntity(EntityMsg::Modifier(ModType::I)))
            }
            (false, false, &[Input::C('f')]) => {
                Some(Message::AddEntity(EntityMsg::Modifier(ModType::F)))
            }
            (false, false, &[Input::C('p')]) => {
                Some(Message::AddEntity(EntityMsg::Modifier(ModType::P)))
            }
            (false, false, &[Input::C('v')]) => {
                Some(Message::AddEntity(EntityMsg::Modifier(ModType::V)))
            }

            (false, false, &[Input::C('d'), Input::C('d')]) => Some(Message::DeleteEntity),

            // update input prompt
            _ => {
                let string = input.iter().map(|i| i.to_char()).collect::<String>();
                Some(Message::SetInput(string))
            }
        }
    }
}
