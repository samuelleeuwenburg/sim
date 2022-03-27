use crate::app::grid::{Entity, EntityKind, EntityMutKind, Position};
use crate::app::user_interface::{Color, DisplayEntity};
use screech::core::{ExternalSignal, Signal};
use screech::traits::{Source, Tracker};

pub struct Trigger {
    grid_position: Position,
    bpm: f32,
    subdivision: f32,
    counter: f32,
    pub output: ExternalSignal,
}

impl Trigger {
    pub fn new(tracker: &mut dyn Tracker) -> Self {
        Trigger {
            grid_position: Position::origin(),
            output: ExternalSignal::new(tracker.create_source_id(), 0),
            bpm: 30.0,
            subdivision: 0.25,
            counter: 0.0,
        }
    }
}

impl Source for Trigger {
    fn sample(&mut self, sources: &mut dyn Tracker, sample_rate: usize) {
        let increase_per_sample = 1.0 / sample_rate as f32 * (self.bpm / 60.0);

        self.counter += increase_per_sample;

        if self.counter >= 1.0 {
            self.counter = 0.0;
        }

        if self.counter < self.subdivision {
            sources.set_signal(&self.output, Signal::point(1.0));
        } else {
            sources.set_signal(&self.output, Signal::point(0.0));
        }
    }

    fn get_source_id(&self) -> &usize {
        self.output.get_source_id()
    }

    fn get_sources(&self) -> Vec<usize> {
        vec![]
    }
}

impl Entity for Trigger {
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
            color: Color::Rgba(255, 255, 255, 1.0),
        }
    }

    fn get_prompt(&self) -> String {
        String::from("step")
    }

    fn as_kind(&self) -> EntityKind {
        EntityKind::Trigger(self)
    }

    fn as_mut_kind(&mut self) -> EntityMutKind {
        EntityMutKind::Trigger(self)
    }

    fn as_mut_source(&mut self) -> &mut dyn Source {
        self
    }
}
