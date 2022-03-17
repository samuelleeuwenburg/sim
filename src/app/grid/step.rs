use crate::app::grid::{Entity, EntityKind, Position};
use crate::app::user_interface::{Color, DisplayEntity};
use screech::core::{ExternalSignal, Signal};
use screech::traits::{Source, Tracker};

#[derive(Debug)]
pub struct Step {
    pub output: ExternalSignal,
    grid_position: Position,
    frequency: f32,
    counter: f32,
    pub is_active: bool,
    pub input: Option<ExternalSignal>,
}

impl Step {
    pub fn new(tracker: &mut dyn Tracker) -> Self {
        Step {
            output: ExternalSignal::new(tracker.create_source_id(), 0),
            grid_position: Position::origin(),
            frequency: 1.5,
            counter: 0.0,
            is_active: true,
            input: None,
        }
    }
}

impl Source for Step {
    fn sample(&mut self, sources: &mut dyn Tracker, sample_rate: usize) {
        let increase_per_sample = 1.0 / sample_rate as f32 * self.frequency;

        // listen for gate at the input
        if let Some(i) = self.input.and_then(|i| sources.get_signal(&i)) {
            if i.get_point() >= &0.5 {
                self.is_active = true;
            }
        }

        if self.is_active {
            self.counter += increase_per_sample;
        }

        if self.counter >= 1.0 {
            self.counter = 0.0;
            self.is_active = false;
            sources.set_signal(&self.output, Signal::point(1.0));
        } else {
            sources.set_signal(&self.output, Signal::point(0.0));
        }
    }

    fn get_source_id(&self) -> &usize {
        self.output.get_source_id()
    }

    fn get_sources(&self) -> Vec<usize> {
        match self.input {
            Some(i) => vec![*i.get_source_id()],
            None => vec![],
        }
    }
}

impl Entity for Step {
    fn set_position(&mut self, position: &Position) {
        self.grid_position.move_to(position);
    }

    fn get_position(&self) -> &Position {
        &self.grid_position
    }

    fn get_display(&self) -> DisplayEntity {
        let color = if self.is_active {
            Color::Rgba(0, 255, 255, 1.0)
        } else {
            Color::Rgba(255, 255, 255, 0.5)
        };

        DisplayEntity {
            position: self.grid_position,
            text: String::from("."),
            color,
        }
    }

    fn get_prompt(&self) -> String {
        String::from("step")
    }

    fn as_kind(&self) -> EntityKind {
        EntityKind::Step(self)
    }

    fn as_mut_kind(&mut self) -> EntityKind {
        EntityKind::Step(self)
    }

    fn as_mut_source(&mut self) -> &mut dyn Source {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use screech::core::Primary;

    #[test]
    fn test_step_external_trigger() {
        let mut primary = Primary::<4>::new(4);
        primary.output_mono();

        let mut step_1 = Step::new(&mut primary);
        let mut step_2 = Step::new(&mut primary);

        step_1.is_active = true;
        step_2.input = Some(step_1.output);

        primary.add_monitor(step_1.output);

        assert_eq!(
            primary.sample(vec![&mut step_1, &mut step_2]).unwrap(),
            &[0.0, 0.0, 1.0, 0.0]
        );

        primary.remove_monitor(&step_1.output);
        primary.add_monitor(step_2.output);

        assert_eq!(
            primary.sample(vec![&mut step_1, &mut step_2]).unwrap(),
            &[1.0, 0.0, 0.0, 0.0]
        );
    }
}
