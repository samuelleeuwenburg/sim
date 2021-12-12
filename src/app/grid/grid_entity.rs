use super::GridPosition;
use crate::app::user_interface::DisplayEntity;

pub trait GridEntity {
    fn set_position(&mut self, position: &GridPosition);
    fn get_position(&self) -> &GridPosition;
    fn get_display(&self) -> DisplayEntity;
    fn get_prompt(&self) -> String;
}
