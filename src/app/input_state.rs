use super::command::Command;

pub struct InputState {
    pub input_buffer: Vec<u32>,
    pub command_buffer: Vec<Command>,
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            input_buffer: Vec::with_capacity(10),
            command_buffer: Vec::with_capacity(10),
        }
    }

    pub fn drain_commands(&mut self) -> Vec<Command> {
        // self.command_buffer.drain(..).collect()
        let commands = self.command_buffer.clone();
        self.command_buffer.clear();
        commands
    }

    pub fn handle_input(&mut self, key_code: u32) -> &mut Self {
        self.input_buffer.push(key_code);

        if let Some(command) = Command::from_key_codes(&self.input_buffer) {
            self.command_buffer.push(command);
            self.input_buffer.clear();
        }

        self
    }
}
