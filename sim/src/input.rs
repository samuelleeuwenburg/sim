use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Input {
    Char(char),
    Enter,
    Tab,
    Escape,
    Space,
    Shift,
    Backspace,
    Alt,
    Control,
    Meta,
    Up,
    Right,
    Down,
    Left,
}

pub struct InputState {
    pub buffer: Vec<Input>,
    is_key_down_map: HashMap<Input, bool>,
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            buffer: vec![],
            is_key_down_map: HashMap::new(),
        }
    }

    pub fn is_key_down(&self, input: Input) -> bool {
	*self.is_key_down_map.get(&input).unwrap_or(&false)
    }

    pub fn key_down(&mut self, input: Input) {
        self.buffer.push(input);
        self.is_key_down_map.insert(input, true);
    }

    pub fn key_up(&mut self, input: Input) {
        self.is_key_down_map.insert(input, false);
    }

    pub fn clear_buffer(&mut self) {
        self.buffer.clear();
    }
}
