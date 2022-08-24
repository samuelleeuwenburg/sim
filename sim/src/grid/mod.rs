pub mod position;
pub mod rect;

use crate::entity::Entity;
pub use position::Position;
pub use rect::Rect;
use screech::traits::Source;
use std::collections::HashMap;

pub struct Grid {
    pub rect: Rect,
    entities: HashMap<Position, Box<dyn Entity>>,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            rect: Rect::new(33, 17, Position::origin()),
            entities: HashMap::new(),
        }
    }

    pub fn get_mut_entities(&mut self) -> Vec<&mut Box<dyn Entity>> {
        self.entities.values_mut().collect()
    }
}
