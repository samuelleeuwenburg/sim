use super::{ConnType, Connector, Position};
use crate::app::modules::{Track, VCO};
use crate::app::user_interface::DisplayEntity;
use screech::core::ExternalSignal;
use screech::traits::Source;

pub enum EntityKind<'a> {
    VCO(&'a VCO),
    Track(&'a Track),
    Connector(&'a Connector),
}

pub trait Entity {
    fn set_position(&mut self, position: &Position);
    fn get_position(&self) -> &Position;
    fn get_display(&self) -> DisplayEntity;
    fn get_prompt(&self) -> String;

    fn as_kind(&self) -> EntityKind;
    fn as_mut_kind(&mut self) -> EntityKind;
}

pub trait Module: Entity + Source {
    fn get_signals(&self) -> Vec<ExternalSignal>;
    fn clear_signals(&mut self, signals: &[ExternalSignal]);
    fn process_signal(&mut self, connection: &[ConnType], signal: &ExternalSignal);
    fn as_mut_source(&mut self) -> &mut dyn Source;
    fn as_entity(&self) -> &dyn Entity;
    fn as_mut_entity(&mut self) -> &mut dyn Entity;
}
