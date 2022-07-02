use super::{Position, Step, Trigger};
use crate::app::user_interface::DisplayEntity;
use screech::traits::Source;
use screech::{Input, Output};

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
    fn find_connections(
        &self,
        entity: &EntityKind,
        relative_position: Position,
    ) -> Vec<(Output, Input)>;

    fn as_kind(&self) -> EntityKind;
    fn as_mut_kind(&mut self) -> EntityMutKind;
    fn as_mut_source(&mut self) -> &mut dyn Source;
}

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
}
