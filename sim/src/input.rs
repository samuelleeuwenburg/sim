use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Input {
    Char(char),
    Enter,
    Tab,
    Space,
}

pub struct InputState {
    pub buffer: Vec<Input>,
    pub is_key_down: HashMap<Input, bool>,
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            buffer: vec![],
            is_key_down: HashMap::new(),
        }
    }

    pub fn key_down(&mut self, input: Input) {
        self.buffer.push(input);
        self.is_key_down.insert(input, true);
    }

    pub fn key_up(&mut self, input: Input) {
        self.is_key_down.insert(input, false);
    }

    pub fn clear_buffer(&mut self) {
	self.buffer.clear();
    }
}
