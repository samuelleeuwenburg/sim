use crate::app::grid::{GridEntity, GridPosition};
use crate::app::user_interface::{Color, DisplayEntity};
use screech::basic::Track as T;
use screech::traits::Tracker;

pub struct Track {
    grid_position: GridPosition,
    pub track: T,
}

impl Track {
    pub fn new(tracker: &mut dyn Tracker) -> Self {
        Track {
            grid_position: GridPosition::new(0, 0),
            track: T::new(tracker),
        }
    }
}

impl GridEntity for Track {
    fn set_position(&mut self, position: &GridPosition) {
        self.grid_position.move_to(position);
    }

    fn get_position(&self) -> &GridPosition {
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
}
