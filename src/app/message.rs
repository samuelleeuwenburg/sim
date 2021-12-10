use super::grid::GridPosition;

#[derive(Debug, Clone)]
pub enum Message {
    Move(GridPosition),
    AddOscillator,
    AddTrack,
    DeleteEntity,
    ClearInput,
    UpdatePrompt,
    Input(Vec<u32>),
}

impl Message {
    pub fn from_key_codes(codes: &[u32]) -> Option<Self> {
        match codes {
            &[.., 27] | &[.., 17, 219] => Some(Message::ClearInput),

            &[72] | &[37] => Some(Message::Move(GridPosition::new(-1, 0))), // left
            &[76] | &[39] => Some(Message::Move(GridPosition::new(1, 0))),  // right
            &[75] | &[38] => Some(Message::Move(GridPosition::new(0, -1))), // up
            &[74] | &[40] => Some(Message::Move(GridPosition::new(0, 1))),  // down

            &[16, 79] => Some(Message::AddOscillator),
            &[16, 84] => Some(Message::AddTrack),
            &[68, 68] => Some(Message::DeleteEntity),

            _ => Some(Message::Input(codes.into())),
        }
    }
}
