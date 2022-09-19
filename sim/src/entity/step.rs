use super::{Entity, EntityKind, EntityMutKind, Setting, SettingValue};
use crate::grid::Position;
use crate::Image;
use screech::traits::{Source, Tracker};
use screech::{Input, Output, Screech};
use std::cmp;

#[derive(Debug, PartialEq)]
enum State {
    Charging,
    Discharging,
    Idle,
}

#[derive(Debug)]
pub struct Step {
    id: usize,
    grid_position: Position,
    state: State,
    charge: usize,
    pub max_charge: usize,
    pub level: f32,
    pub output: Output,
    input: Input,
}

impl Step {
    pub fn new(screech: &mut Screech) -> Self {
        let id = screech.create_source_id();

        Step {
            id,
            output: screech.init_output(&id, "output"),
            input: screech.init_input(&id, "input"),
            grid_position: Position::origin(),
            state: State::Idle,
            level: 1.0,
            max_charge: 0,
            charge: 0,
        }
    }

    pub fn trigger(&mut self, charge: usize) -> &mut Self {
        self.charge = charge;
        self
    }
}

impl Source for Step {
    fn sample(&mut self, tracker: &mut dyn Tracker, _sample_rate: usize) {
        let buffer_size = *tracker.get_buffer_size();
        let mut signal_in = vec![0.0; buffer_size];

        for input in tracker.get_input(&self.input).unwrap().into_iter() {
            let buffer = tracker.get_output(&input).unwrap();

            for i in 0..*tracker.get_buffer_size() {
                signal_in[i] = if signal_in[i] >= buffer.samples[i] {
                    signal_in[i]
                } else {
                    buffer.samples[i]
                };
            }
        }

        let signal = tracker.get_mut_output(&self.output).unwrap();

        for (i, s) in signal.samples.iter_mut().enumerate() {
            self.state = match (signal_in[i], self.charge) {
                (v, _) if v >= 0.5 => State::Charging,
                (_, c) if c > 0 => State::Discharging,
                _ => State::Idle,
            };

            *s = match self.state {
                State::Discharging => 1.0,
                _ => 0.0,
            };

            match self.state {
                State::Charging => self.charge += 1,
                State::Discharging => self.charge -= 1,
                _ => (),
            };

            self.max_charge = cmp::max(self.charge, self.max_charge);
        }
    }

    fn get_source_id(&self) -> &usize {
        &self.id
    }
}

impl Entity for Step {
    fn set_position(&mut self, position: Position) {
        self.grid_position = self.grid_position.move_to(position);
    }

    fn get_position(&self) -> Position {
        self.grid_position
    }

    fn get_grid_display(&self) -> Option<Image> {
        None
    }

    fn get_detail_display(&self) -> Option<Image> {
        None
    }

    // fn get_settings(&self) -> Vec<Setting> {
    //     vec![Setting::new(SettingValue::Integer(self.charge), "cap")]
    // }

    //fn update_setting(&mut self, _setting: &Setting) {}

    //fn find_connections(
    //    &self,
    //    entity: &EntityKind,
    //    relative_position: Position,
    //) -> Vec<(Output, Input)> {
    //    let mut conns = vec![];

    //    match (entity, relative_position) {
    //        (EntityKind::Trigger(trigger), Position { x: 0, y: -1 })
    //        | (EntityKind::Trigger(trigger), Position { x: -1, y: 0 }) => {
    //            conns.push((trigger.output, self.input));
    //        }
    //        (EntityKind::Step(step), Position { x: 0, y: -1 })
    //        | (EntityKind::Step(step), Position { x: -1, y: 0 }) => {
    //            conns.push((step.output, self.input));
    //        }
    //        _ => (),
    //    }

    //    conns
    //}

    fn as_kind(&self) -> EntityKind {
        EntityKind::Step(self)
    }

    fn as_mut_kind(&mut self) -> EntityMutKind {
        EntityMutKind::Step(self)
    }
}
