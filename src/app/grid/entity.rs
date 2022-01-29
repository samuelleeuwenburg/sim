use super::Position;
use crate::app::user_interface::DisplayEntity;
use screech::traits::Source;

pub trait Entity {
    fn set_position(&mut self, position: &Position);
    fn get_position(&self) -> &Position;
    fn get_display(&self) -> DisplayEntity;
    fn get_prompt(&self) -> String;

    fn get_mut_source(&mut self) -> Option<&mut dyn Source>;

    fn find_neighbouring<'a>(&self, entities: &'a [Box<dyn Entity>]) -> Vec<&'a Box<dyn Entity>> {
        let pos = self.get_position();

        entities
            .iter()
            .filter(|e| pos.is_adjacent(e.get_position()))
            .collect()
    }

    // @TODO: implement behaviour for processing entity
    // fn process_entity(&mut self, entity: &Entity) {}
}
