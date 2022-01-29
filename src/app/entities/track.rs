use crate::app::grid::{Entity, Position};
use crate::app::user_interface::{Color, DisplayEntity};
use screech::basic::Track as T;
use screech::traits::{Source, Tracker};

pub struct Track {
    grid_position: Position,
    pub track: T,
}

impl Track {
    pub fn new(tracker: &mut dyn Tracker) -> Self {
        Track {
            grid_position: Position::origin(),
            track: T::new(tracker),
        }
    }
}

impl Entity for Track {
    fn set_position(&mut self, position: &Position) {
        self.grid_position.move_to(position);
    }

    fn get_position(&self) -> &Position {
        &self.grid_position
    }

    fn get_display(&self) -> DisplayEntity {
        DisplayEntity {
            position: self.grid_position,
            text: String::from("t"),
            color: Color::RGB(0, 255, 255),
        }
    }

    fn get_prompt(&self) -> String {
        "track".into()
    }

    fn get_mut_source(&mut self) -> Option<&mut dyn Source> {
        Some(&mut self.track as &mut dyn Source)
    }
}
