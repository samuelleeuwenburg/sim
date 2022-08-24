use super::Position;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub position: Position,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    pub fn new(width: i32, height: i32, position: Position) -> Self {
        Rect {
            position,
            width,
            height,
        }
    }

    #[allow(dead_code)]
    pub fn intersect(&self, rect: &Rect) -> bool {
        !(rect.position.x > self.position.x + self.width
            || rect.position.x + rect.width < self.position.x
            || rect.position.y > self.position.y + self.height
            || rect.position.y + rect.height < self.position.y)
    }

    #[allow(dead_code)]
    pub fn intersect_position(&self, pos: Position) -> bool {
        pos.x >= self.position.x
            && pos.x <= self.position.x + self.width
            && pos.y >= self.position.y
            && pos.y <= self.position.y + self.height
    }
}
