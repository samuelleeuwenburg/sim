use crate::app::grid::{ConnType, Entity, EntityKind, Module, Position};
use crate::app::user_interface::{Color, DisplayEntity};
use screech::basic::Oscillator;
use screech::core::ExternalSignal;
use screech::traits::{Source, Tracker};

pub struct VCO {
    grid_position: Position,
    pub oscillator: Oscillator,
}

impl VCO {
    pub fn new(tracker: &mut dyn Tracker) -> Self {
        let mut oscillator = Oscillator::new(tracker);
        oscillator.frequency = 440.0;

        VCO {
            grid_position: Position::origin(),
            oscillator,
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

    fn as_kind(&self) -> EntityKind {
        EntityKind::VCO(self)
    }

    fn as_mut_kind(&mut self) -> EntityKind {
        EntityKind::VCO(self)
    }
}

impl Source for VCO {
    fn sample(&mut self, sources: &mut dyn Tracker, sample_rate: usize) {
        self.oscillator.sample(sources, sample_rate)
    }

    fn get_source_id(&self) -> &usize {
        self.oscillator.get_source_id()
    }

    fn get_sources(&self) -> Vec<usize> {
        self.oscillator.get_sources()
    }
}

impl Module for VCO {
    fn as_mut_source(&mut self) -> &mut dyn Source {
        self
    }

    fn as_mut_entity(&mut self) -> &mut dyn Entity {
        self
    }

    fn as_entity(&self) -> &dyn Entity {
        self
    }

    fn get_signals(&self) -> Vec<ExternalSignal> {
        vec![self.oscillator.output]
    }

    fn clear_signals(&mut self, signals: &[ExternalSignal]) {}

    fn process_signal(&mut self, connection: &[ConnType], signal: &ExternalSignal) {}
}
