use super::message::Message;

pub struct InputState {
    pub input_buffer: Vec<u32>,
    pub message_buffer: Vec<Message>,
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            input_buffer: Vec::with_capacity(10),
            message_buffer: Vec::with_capacity(10),
        }
    }

    pub fn drain_messages(&mut self) -> Vec<Message> {
        self.message_buffer.drain(..).collect()
    }

    pub fn handle_input(&mut self, key_code: u32) -> &mut Self {
        self.input_buffer.push(key_code);

        if let Some(message) = Message::from_key_codes(&self.input_buffer) {
            match message {
                Message::Input(_) => (),
                _ => self.input_buffer.clear(),
            }

            self.message_buffer.push(message);
        }

        self
    }
}
