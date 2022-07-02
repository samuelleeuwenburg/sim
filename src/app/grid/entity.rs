use super::{Position, Step, Trigger};
use crate::app::user_interface::DisplayEntity;
use screech::traits::Source;
use screech::{Input, Output};
use std::error::Error;
use std::fmt;

pub enum EntityKind<'a> {
    Step(&'a Step),
    Trigger(&'a Trigger),
}

pub enum EntityMutKind<'a> {
    Step(&'a mut Step),
    Trigger(&'a mut Trigger),
}

pub trait Entity: Source {
    fn set_position(&mut self, position: Position);
    fn get_position(&self) -> Position;
    fn get_display(&self) -> DisplayEntity;
    fn get_prompt(&self) -> String;
    fn get_settings(&self) -> Vec<EntitySetting>;
    fn update_setting(&mut self, setting: &EntitySetting);
    fn find_connections(
        &self,
        entity: &EntityKind,
        relative_position: Position,
    ) -> Vec<(Output, Input)>;

    fn as_kind(&self) -> EntityKind;
    fn as_mut_kind(&mut self) -> EntityMutKind;
    fn as_mut_source(&mut self) -> &mut dyn Source;
}

#[derive(Debug, Clone)]
pub enum EntitySettingValue {
    Float(f32),
    Integer(usize),
}

impl EntitySettingValue {
    pub fn to_string(&self) -> String {
        match self {
            EntitySettingValue::Float(s) => s.to_string(),
            EntitySettingValue::Integer(s) => s.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EntitySetting {
    pub value: EntitySettingValue,
    pub description: String,
}

impl EntitySetting {
    pub fn new(value: EntitySettingValue, description: &str) -> Self {
        EntitySetting {
            value,
            description: description.into(),
        }
    }

    pub fn try_update_value(&mut self, value: &str) -> Result<(), Box<dyn Error>> {
        match self.value {
            EntitySettingValue::Float(_) => {
                self.value = EntitySettingValue::Float(value.parse::<f32>()?)
            }
            EntitySettingValue::Integer(_) => {
                self.value = EntitySettingValue::Integer(value.parse::<usize>()?)
            }
        }

        Ok(())
    }
}
