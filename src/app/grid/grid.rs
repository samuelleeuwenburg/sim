use super::{Entity, Position, Rect};

pub struct Grid {
    pub rect: Rect,
    pub entities: Vec<Box<dyn Entity>>,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            rect: Rect::new(33, 17, Position::origin()),
            entities: Vec::with_capacity(256),
        }
    }
}
