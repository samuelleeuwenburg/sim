pub mod position;
pub mod rect;

use crate::entity::Entity;
pub use position::Position;
pub use rect::Rect;

pub struct Grid {
    pub cursor_position: Position,
    pub window_position: Position,
    entities: Vec<Box<dyn Entity>>,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            cursor_position: Position::origin(),
            window_position: Position::origin(),
            entities: vec![],
        }
    }

    pub fn get_mut_entities(&mut self) -> Vec<&mut Box<dyn Entity>> {
        self.entities.iter_mut().collect()
    }
}
