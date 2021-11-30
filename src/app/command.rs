use super::grid::GridPosition;

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Move(GridPosition),
    AddOscillator,
    DeleteOscillator,
    ClearInput,
}

impl Command {
    pub fn from_key_codes(codes: &[u32]) -> Option<Self> {
        match codes {
            &[.., 27] | &[.., 17, 219] => Some(Command::ClearInput),
            &[72] | &[37] => Some(Command::Move(GridPosition::new(-1, 0))), // left
            &[76] | &[39] => Some(Command::Move(GridPosition::new(1, 0))),  // right
            &[75] | &[38] => Some(Command::Move(GridPosition::new(0, -1))), // up
            &[74] | &[40] => Some(Command::Move(GridPosition::new(0, 1))),  // down

            &[16, 79] => Some(Command::AddOscillator),
            &[68, 68] => Some(Command::DeleteOscillator),
            _ => None,
        }
    }
}
