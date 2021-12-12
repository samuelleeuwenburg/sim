use super::message::Message;

#[derive(Debug, Clone, Copy)]
pub enum Input {
    Shift,
    Alt,
    Control,
    Escape,
    Enter,
    Backspace,
    Tab,
    C(char),
}

impl Input {
    pub fn to_char(self) -> char {
        match self {
            Input::Shift => '^',
            Input::Alt => '!',
            Input::Control => '^',
            Input::Escape => '!',
            Input::Enter => '¬',
            Input::Backspace => '<',
            Input::Tab => '>',
            Input::C(c) => c,
        }
    }
}

pub struct InputState {
    input_buffer: Vec<Input>,
    message_buffer: Vec<Message>,
    pub shift: bool,
    pub control: bool,
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            input_buffer: Vec::with_capacity(10),
            message_buffer: Vec::with_capacity(10),
            shift: false,
            control: false,
        }
    }

    pub fn drain_messages(&mut self) -> Vec<Message> {
        self.message_buffer.drain(..).collect()
    }

    pub fn try_message(&mut self) -> &mut Self {
        if let Some(message) = Message::from_input(&self, &self.input_buffer) {
            match message {
                Message::SetInput(_) => (),
                _ => self.input_buffer.clear(),
            }

            self.message_buffer.push(message);
        }

        self
    }

    pub fn handle_keyup(&mut self, input: Input) -> &mut Self {
        match input {
            Input::Shift => self.shift = false,
            Input::Control => self.control = false,
            _ => (),
        }

        self
    }

    pub fn handle_keydown(&mut self, input: Input) -> &mut Self {
        match input {
            Input::Shift => self.shift = true,
            Input::Control => self.control = true,
            Input::Escape => self.input_buffer.push(input),
            Input::C(_) => self.input_buffer.push(input),
            _ => (),
        }

        self.try_message();

        self
    }
}
