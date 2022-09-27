pub mod position;
pub mod rect;

use crate::entity::Entity;
use crate::{Color, Image};
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
            window_position: Position::new(-8, -4),
            entities: vec![],
        }
    }

    pub fn get_mut_entities(&mut self) -> Vec<&mut Box<dyn Entity>> {
        self.entities.iter_mut().collect()
    }

    pub fn get_image_for_pos(&self, pos: Position) -> Option<Image> {
	if self.cursor_position == pos {
	    let mut image = Image::new(4, 4);
	    image.clear(Color::new(251, 255, 38, 255));
	    Some(image)
	} else {
	    for entity in self.entities.iter() {
		if entity.get_position() == pos {
		    return entity.get_grid_display();
		}
	    }

	    None
	}
    }
}
