use crate::app::grid::{Entity, Position};
use crate::app::user_interface::{Color, DisplayEntity};
use screech::basic::Oscillator;
use screech::traits::{Source, Tracker};

pub struct VCO {
    grid_position: Position,
    pub oscillator: Oscillator,
}

impl VCO {
    pub fn new(tracker: &mut dyn Tracker) -> Self {
        VCO {
            grid_position: Position::origin(),
            oscillator: Oscillator::new(tracker),
        }
    }
}

impl Entity for VCO {
    fn set_position(&mut self, position: &Position) {
        self.grid_position.move_to(position);
    }

    fn get_position(&self) -> &Position {
        &self.grid_position
    }

    fn get_display(&self) -> DisplayEntity {
        DisplayEntity {
            position: self.grid_position,
            text: String::from("o"),
            color: Color::RGB(255, 255, 255),
        }
    }

    fn get_prompt(&self) -> String {
        format!("vco @ {} freq", self.oscillator.frequency)
    }

    fn get_mut_source(&mut self) -> Option<&mut dyn Source> {
        Some(&mut self.oscillator as &mut dyn Source)
    }
}
