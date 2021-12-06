use super::user_interface::DisplayEntity;

#[derive(Debug, Clone, Copy)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

impl GridPosition {
    pub fn new(x: i32, y: i32) -> Self {
        GridPosition { x, y }
    }

    pub fn move_to(&mut self, pos: &Self) -> &mut Self {
        self.x = pos.x;
        self.y = pos.y;
        self
    }

    pub fn add(&mut self, pos: &Self) -> &mut Self {
        self.x += pos.x;
        self.y += pos.y;
        self
    }
}

pub trait GridEntity {
    fn set_position(&mut self, position: &GridPosition);
    fn get_position(&self) -> &GridPosition;
    fn get_display(&self) -> DisplayEntity;
    fn get_prompt(&self) -> String;
}
