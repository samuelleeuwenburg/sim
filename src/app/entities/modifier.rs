use crate::app::grid::{GridEntity, GridPosition};
use crate::app::user_interface::{Color, DisplayEntity};

#[derive(Debug, Clone, Copy)]
pub enum ModType {
    S,
    I,
    F,
    P,
    V,
}

impl ModType {
    pub fn get_display(&self, &position: &GridPosition) -> DisplayEntity {
        let text = match self {
            ModType::S => String::from("s"),
            ModType::I => String::from("i"),
            ModType::F => String::from("f"),
            ModType::P => String::from("p"),
            ModType::V => String::from("v"),
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
            ModType::S => String::from("modifier s"),
            ModType::I => String::from("modifier i"),
            ModType::F => String::from("modifier f"),
            ModType::P => String::from("modifier p"),
            ModType::V => String::from("modifier v"),
        }
    }
}

pub struct Modifier {
    grid_position: GridPosition,
    pub mod_type: ModType,
}

impl Modifier {
    pub fn new(mod_type: &ModType) -> Self {
        Modifier {
            grid_position: GridPosition::new(0, 0),
            mod_type: *mod_type,
        }
    }
}

impl GridEntity for Modifier {
    fn set_position(&mut self, position: &GridPosition) {
        self.grid_position.move_to(position);
    }

    fn get_position(&self) -> &GridPosition {
        &self.grid_position
    }

    fn get_display(&self) -> DisplayEntity {
        self.mod_type.get_display(&self.grid_position)
    }

    fn get_prompt(&self) -> String {
        self.mod_type.get_prompt()
    }
}