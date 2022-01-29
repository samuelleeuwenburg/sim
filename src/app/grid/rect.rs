use super::Position;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub position: Position,
    pub size: (i32, i32),
}

impl Rect {
    pub fn new(width: i32, height: i32, position: Position) -> Self {
        Rect {
            position,
            size: (width, height),
        }
    }

    pub fn intersects_position(&self, pos: &Position) -> bool {
        let (w, h) = self.size;

        pos.x >= self.position.x
            && pos.x <= self.position.x + w
            && pos.y >= self.position.y
            && pos.y <= self.position.y + h
    }
}
