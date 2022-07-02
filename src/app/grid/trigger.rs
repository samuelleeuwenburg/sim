use crate::app::grid::{
    Entity, EntityKind, EntityMutKind, EntitySetting, EntitySettingValue, Position,
};
use crate::app::user_interface::{Color, DisplayEntity};
use screech::traits::{Source, Tracker};
use screech::{Input, Output, Screech};

pub struct Trigger {
    id: usize,
    grid_position: Position,
    bpm: f32,
    subdivision: f32,
    counter: f32,
    pub output: Output,
}

impl Trigger {
    pub fn new(screech: &mut Screech) -> Self {
        let id = screech.create_source_id();

        Trigger {
            id,
            grid_position: Position::origin(),
            output: screech.init_output(&id, "output"),
            bpm: 480.0,
            subdivision: 0.25,
            counter: 0.0,
        }
    }
}

impl Source for Trigger {
    fn sample(&mut self, tracker: &mut dyn Tracker, sample_rate: usize) {
        let signal = tracker.get_mut_output(&self.output).unwrap();
        let increase_per_sample = 1.0 / sample_rate as f32 * (self.bpm / 60.0);

        for s in signal.samples.iter_mut() {
            self.counter += increase_per_sample;

            if self.counter >= 1.0 {
                self.counter = 0.0;
            }

            *s = if self.counter < self.subdivision {
                1.0
            } else {
                0.0
            };
        }
    }

    fn get_source_id(&self) -> &usize {
        self.output.get_source_id()
    }
}

impl Entity for Trigger {
    fn set_position(&mut self, position: Position) {
        self.grid_position = self.grid_position.move_to(position);
    }

    fn get_position(&self) -> Position {
        self.grid_position
    }

    fn get_display(&self) -> DisplayEntity {
        let alpha = if self.counter < self.subdivision {
            1.0
        } else {
            0.6
        };

        DisplayEntity {
            position: self.grid_position,
            text: String::from("t"),
            color: Color::Rgba(120, 200, 255, alpha),
        }
    }

    fn get_settings(&self) -> Vec<EntitySetting> {
        vec![
            EntitySetting::new(EntitySettingValue::Float(self.bpm), "bpm"),
            EntitySetting::new(EntitySettingValue::Float(self.subdivision), "div"),
        ]
    }

    fn update_setting(&mut self, setting: &EntitySetting) {
        match (&setting.value, setting.description.as_str()) {
            (EntitySettingValue::Float(v), "bpm") => self.bpm = *v,
            (EntitySettingValue::Float(v), "div") => self.subdivision = *v,
            _ => (),
        }
    }

    fn get_prompt(&self) -> String {
        String::from("t")
    }

    fn find_connections(
        &self,
        _entity: &EntityKind,
        _relative_position: Position,
    ) -> Vec<(Output, Input)> {
        vec![]
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
