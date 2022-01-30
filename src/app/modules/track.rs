use crate::app::grid::{ConnType, Entity, EntityKind, Module, Position};
use crate::app::user_interface::{Color, DisplayEntity};
use screech::basic::Track as T;
use screech::core::ExternalSignal;
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
            text: String::from("m"),
            color: Color::RGB(255, 100, 100),
        }
    }

    fn get_prompt(&self) -> String {
        "track".into()
    }

    fn as_kind(&self) -> EntityKind {
        EntityKind::Track(self)
    }

    fn as_mut_kind(&mut self) -> EntityKind {
        EntityKind::Track(self)
    }
}

impl Source for Track {
    fn sample(&mut self, sources: &mut dyn Tracker, sample_rate: usize) {
        self.track.sample(sources, sample_rate)
    }

    fn get_source_id(&self) -> &usize {
        self.track.get_source_id()
    }

    fn get_sources(&self) -> Vec<usize> {
        self.track.get_sources()
    }
}

impl Module for Track {
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
        vec![self.track.output]
    }

    fn clear_signals(&mut self, signals: &[ExternalSignal]) {
        for signal in signals {
            self.track.remove_input(signal);
        }
    }

    fn process_signal(&mut self, connection: &[ConnType], signal: &ExternalSignal) {
        match connection {
            &[ConnType::S, ConnType::I] => {
                self.track.add_input(*signal);
            }
            _ => (),
        };
    }
}
