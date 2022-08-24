mod setting;
mod step;
mod trigger;

// use crate::app::user_interface::DisplayEntity;
use crate::grid::Position;
use screech::traits::Source;
use screech::{Input, Output};
pub use setting::{Setting, SettingValue};
use step::Step;
use trigger::Trigger;

pub trait UpcastSource {
    fn as_mut_source(&mut self) -> &mut dyn Source;
}

impl<T: Source + Entity> UpcastSource for T {
    fn as_mut_source(&mut self) -> &mut dyn Source {
        self
    }
}

pub enum EntityKind<'a> {
    Step(&'a Step),
    Trigger(&'a Trigger),
}

pub enum EntityMutKind<'a> {
    Step(&'a mut Step),
    Trigger(&'a mut Trigger),
}

pub trait Entity: Source + UpcastSource {
    fn set_position(&mut self, position: Position);
    fn get_position(&self) -> Position;
    // fn get_display(&self) -> DisplayEntity;
    fn get_prompt(&self) -> String;
    fn get_settings(&self) -> Vec<Setting>;
    fn update_setting(&mut self, setting: &Setting);
    fn find_connections(
        &self,
        entity: &EntityKind,
        relative_position: Position,
    ) -> Vec<(Output, Input)>;

    fn as_kind(&self) -> EntityKind;
    fn as_mut_kind(&mut self) -> EntityMutKind;
}
