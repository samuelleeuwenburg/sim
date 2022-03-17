use super::{Position, Step};
use crate::app::user_interface::DisplayEntity;
use screech::traits::Source;

pub enum EntityKind<'a> {
    Step(&'a Step),
}

pub trait Entity: Source {
    fn set_position(&mut self, position: &Position);
    fn get_position(&self) -> &Position;
    fn get_display(&self) -> DisplayEntity;
    fn get_prompt(&self) -> String;

    fn as_kind(&self) -> EntityKind;
    fn as_mut_kind(&mut self) -> EntityKind;
    fn as_mut_source(&mut self) -> &mut dyn Source;
}
