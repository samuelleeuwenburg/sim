use super::Rect;
use std::cmp;

#[derive(Debug, Clone, Copy, PartialEq)]
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

    pub fn clamp(&mut self, rect: &Rect) -> &mut Self {
        let (w, h) = rect.size;
        let x_min = rect.position.x;
        let x_max = rect.position.x + w - 1;
        let y_min = rect.position.y;
        let y_max = rect.position.y + h - 1;

        self.x = cmp::min(cmp::max(self.x, x_min), x_max);
        self.y = cmp::min(cmp::max(self.y, y_min), y_max);

        self
    }
}
