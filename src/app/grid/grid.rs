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

    pub fn get_entity(&self, pos: &Position) -> Option<&Box<dyn Entity>> {
        self.entities.iter().find(|e| e.get_position() == pos)
    }

    pub fn is_occupied(&self, pos: &Position) -> bool {
        self.rect.intersect_position(&pos)
    }
}
