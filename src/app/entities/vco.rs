use crate::app::grid::{GridEntity, GridPosition};
use crate::app::user_interface::DisplayEntity;
use screech::basic::Oscillator;
use screech::traits::Tracker;

pub struct VCO {
    grid_position: GridPosition,
    pub oscillator: Oscillator,
}

impl VCO {
    pub fn new(tracker: &mut dyn Tracker) -> Self {
        VCO {
            grid_position: GridPosition::new(0, 0),
            oscillator: Oscillator::new(tracker),
        }
    }
}

impl GridEntity for VCO {
    fn set_position(&mut self, position: &GridPosition) {
        self.grid_position.move_to(position);
    }

    fn get_position(&self) -> &GridPosition {
        &self.grid_position
    }

    fn get_display(&self) -> DisplayEntity {
        DisplayEntity { position: self.grid_position, text: String::from("o") }
    }

    fn get_prompt(&self) -> String {
        format!("vco @ {} freq", self.oscillator.frequency)
    }
}

