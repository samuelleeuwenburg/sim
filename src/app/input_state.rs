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

#[derive(Debug, Clone, Copy)]
pub enum InputMode {
    Command,
    Insert,
    Edit(usize),
}

impl InputMode {
    pub fn get_prompt(&self) -> &'static str {
        match self {
            InputMode::Command => "",
            InputMode::Insert => "INS",
            InputMode::Edit(_) => "EDT",
        }
    }
}

pub struct InputState {
    input_buffer: Vec<Input>,
    message_buffer: Vec<Message>,
    pub mode: InputMode,
    pub shift: bool,
    pub control: bool,
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            mode: InputMode::Command,
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
        for message in Message::from_input(self, &self.input_buffer) {
            match message {
                Message::SetInput(_) => (),
                Message::SwitchInputMode(mode) => {
                    self.mode = mode;
                    self.input_buffer.clear()
                }
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
            Input::Backspace => {
                self.input_buffer.pop();
            }
            Input::C(_) | Input::Escape | Input::Enter | Input::Tab => {
                self.input_buffer.push(input)
            }
            _ => (),
        }

        self.try_message();

        self
    }
}
