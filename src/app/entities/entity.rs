use super::{Modifier, Track, VCO};
use crate::app::grid::GridEntity;
use screech::traits::Source;

pub enum Entity {
    VCO(VCO),
    Track(Track),
    Modifier(Modifier),
}

impl Entity {
    pub fn get_mut_source(&mut self) -> Option<&mut dyn Source> {
        match self {
            Entity::VCO(o) => Some(&mut o.oscillator as &mut dyn Source),
            Entity::Track(t) => Some(&mut t.track as &mut dyn Source),
            _ => None,
        }
    }

    pub fn get_grid_entity(&self) -> &dyn GridEntity {
        match self {
            Entity::VCO(o) => o as &dyn GridEntity,
            Entity::Track(t) => t as &dyn GridEntity,
            Entity::Modifier(m) => m as &dyn GridEntity,
        }
    }

    // @TODO: implement finding nearby entities
    pub fn find_neighbouring(&self, entities: &[Entity]) -> Vec<&Entity> {
        vec![]
    }

    // @TODO: implement behaviour for processing entity
    pub fn process_entity(&mut self, entity: &Entity) {}
}
