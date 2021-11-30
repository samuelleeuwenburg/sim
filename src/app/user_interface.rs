use super::grid::GridPosition;

pub struct UserInterface {
    pub cursor: GridPosition,
    pub grid_size: (u32, u32),
    pub prompt: String,
}

impl UserInterface {
    pub fn new() -> Self {
        UserInterface {
            cursor: GridPosition::new(0, 0),
            grid_size: (32, 16),
            prompt: String::from("Oscillator example ..."),
        }
    }
}
