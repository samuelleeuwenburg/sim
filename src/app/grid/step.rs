use crate::app::grid::{Entity, EntityKind, EntityMutKind, Position};
use crate::app::user_interface::{Color, DisplayEntity};
use screech::core::{ExternalSignal, Signal};
use screech::traits::{Source, Tracker};
use std::cmp;

#[derive(Debug)]
enum State {
    Charging,
    Discharging,
    Idle,
}

#[derive(Debug)]
pub struct Step {
    grid_position: Position,
    state: State,
    charge: usize,
    pub max_charge: usize,
    pub level: Signal,
    pub output: ExternalSignal,
    inputs: Vec<ExternalSignal>,
}

impl Step {
    pub fn new(tracker: &mut dyn Tracker) -> Self {
        Step {
            output: ExternalSignal::new(tracker.create_source_id(), 0),
            grid_position: Position::origin(),
            inputs: vec![],
            state: State::Idle,
            level: Signal::point(1.0),
            max_charge: 0,
            charge: 0,
        }
    }

    pub fn trigger(&mut self, charge: usize) -> &mut Self {
        self.charge = charge;
        self
    }

    pub fn clear_connections(&mut self) -> &mut Self {
        self.inputs.clear();
        self
    }

    pub fn add_input(&mut self, &signal: &ExternalSignal) -> &mut Self {
        self.inputs.push(signal);
        self
    }

    pub fn get_inputs(&self) -> &[ExternalSignal] {
        &self.inputs
    }
}

impl Source for Step {
    fn sample(&mut self, sources: &mut dyn Tracker, _sample_rate: usize) {
        // set output state
        self.state = match self
            .inputs
            .clone()
            .into_iter()
            .filter_map(|i| sources.get_signal(&i))
            .find(|s| s.get_point() >= &0.5)
        {
            Some(_) => State::Charging,
            _ => {
                if self.charge == 0 {
                    State::Idle
                } else {
                    State::Discharging
                }
            }
        };

        // set output
        match self.state {
            State::Discharging => sources.set_signal(&self.output, self.level),
            _ => sources.set_signal(&self.output, Signal::silence()),
        }

        // update state
        match self.state {
            State::Charging => self.charge += 1,
            State::Discharging => self.charge -= 1,
            _ => (),
        }

        // set max charge
        self.max_charge = cmp::max(self.charge, self.max_charge);
    }

    fn get_source_id(&self) -> &usize {
        self.output.get_source_id()
    }

    fn get_sources(&self) -> Vec<usize> {
        self.inputs
            .clone()
            .into_iter()
            .map(|i| *i.get_source_id())
            .collect()
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
        let alpha = (self.charge as f32 / self.max_charge as f32) / 2.0 + 0.5;

        let color = match self.state {
            State::Discharging => Color::Rgba(255, 255, 255, alpha),
            _ => Color::Rgba(255, 255, 255, 0.2),
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

    fn as_mut_kind(&mut self) -> EntityMutKind {
        EntityMutKind::Step(self)
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

        step_1.level = Signal::point(0.8);
        step_1.trigger(3);
        step_2.level = Signal::point(0.6);
        step_2.add_input(&step_1.output);

        primary.add_monitor(step_1.output);
        primary.add_monitor(step_2.output);

        assert_eq!(
            primary.sample(vec![&mut step_1, &mut step_2]).unwrap(),
            &[0.8, 0.8, 0.8, 0.6]
        );

        assert_eq!(
            primary.sample(vec![&mut step_1, &mut step_2]).unwrap(),
            &[0.6, 0.6, 0.0, 0.0]
        );
    }
}
