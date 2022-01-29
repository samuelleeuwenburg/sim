use super::Rect;
use std::cmp;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    pub fn origin() -> Self {
        Position { x: 0, y: 0 }
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

    pub fn is_adjacent(&self, pos: &Self) -> bool {
        let rect = Rect::new(3, 3, Position::new(pos.x - 1, pos.y - 1));

        rect.intersect_position(pos) && pos != self
    }

    pub fn clamp(&mut self, rect: &Rect) -> &mut Self {
        let x_min = rect.position.x;
        let x_max = rect.position.x + rect.width - 1;
        let y_min = rect.position.y;
        let y_max = rect.position.y + rect.height - 1;

        self.x = cmp::min(cmp::max(self.x, x_min), x_max);
        self.y = cmp::min(cmp::max(self.y, y_min), y_max);

        self
    }
}
