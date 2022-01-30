use crate::app::grid::{Entity, EntityKind, Position};
use crate::app::user_interface::{Color, DisplayEntity};

#[derive(Debug, Clone, Copy)]
pub enum ConnType {
    S,
    I,
    F,
    P,
}

impl ConnType {
    pub fn get_display(&self, &position: &Position) -> DisplayEntity {
        let text = match self {
            ConnType::S => String::from("s"),
            ConnType::I => String::from("i"),
            ConnType::F => String::from("f"),
            ConnType::P => String::from("p"),
        };

        let color = Color::RGB(0, 255, 255);

        DisplayEntity {
            position,
            color,
            text,
        }
    }

    pub fn get_prompt(&self) -> String {
        match self {
            ConnType::S => String::from("modifier s"),
            ConnType::I => String::from("modifier i"),
            ConnType::F => String::from("modifier f"),
            ConnType::P => String::from("modifier p"),
        }
    }
}

pub struct Connector {
    grid_position: Position,
    pub conn_type: ConnType,
}

impl Connector {
    pub fn new(mod_type: &ConnType) -> Self {
        Connector {
            grid_position: Position::origin(),
            conn_type: *mod_type,
        }
    }
}

impl Entity for Connector {
    fn set_position(&mut self, position: &Position) {
        self.grid_position.move_to(position);
    }

    fn get_position(&self) -> &Position {
        &self.grid_position
    }

    fn get_display(&self) -> DisplayEntity {
        self.conn_type.get_display(&self.grid_position)
    }

    fn get_prompt(&self) -> String {
        self.conn_type.get_prompt()
    }

    fn as_kind(&self) -> EntityKind {
        EntityKind::Connector(self)
    }

    fn as_mut_kind(&mut self) -> EntityKind {
        EntityKind::Connector(self)
    }
}
