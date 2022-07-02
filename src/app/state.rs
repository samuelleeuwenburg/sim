use super::grid::{Entity, Grid, Step, Trigger};
use super::input_state::InputMode;
use super::message::Message;
use super::user_interface::{Graphics, UserInterface};
use screech::{BasicTracker, Screech};

pub struct State {
    screech: Screech,
    grid: Grid,
    input_mode: InputMode,
    pub user_interface: UserInterface,
}

impl State {
    pub fn new(sample_rate: usize) -> Self {
        let buffer_size = 256;
        let tracker = Box::new(BasicTracker::<256>::new(buffer_size));
        let mut screech = Screech::with_tracker(tracker, sample_rate);

        // setup new output buffer
        screech.create_main_out("left_out");
        screech.create_main_out("right_out");

        State {
            screech,
            grid: Grid::new(),
            input_mode: InputMode::Command,
            user_interface: UserInterface::new(),
        }
    }

    pub fn sample(&mut self) -> (&[f32], &[f32]) {
        let mut sources = self.grid.get_mut_sources();

        if let Err(err) = self.screech.sample(&mut sources) {
            web_sys::console::log_1(&format!("unable to sample screech: {:?}", err).into());
        }

        (
            &self.screech.get_main_out("left_out").unwrap().samples,
            &self.screech.get_main_out("right_out").unwrap().samples,
        )
    }

    pub fn update_ui(&mut self) {
        self.user_interface.display_entities.clear();

        for e in self.grid.get_entities() {
            self.user_interface.display_entities.push(e.get_display());
        }
    }

    pub fn render_ui(&self, g: &dyn Graphics) {
        self.user_interface.render(g, &self.grid, self.input_mode)
    }

    pub fn process_messages(&mut self, messages: &[Message]) {
        for message in messages {
            self.process_message(message);
        }
    }

    pub fn process_message(&mut self, message: &Message) {
        match message {
            Message::SetInput(input) => self.user_interface.input = input.clone(),
            Message::ClearInput => self.user_interface.input.clear(),

            Message::ProcessInput => match self.input_mode {
                InputMode::Edit(pos) => (),
                _ => (),
            },

            Message::MoveTo(pos) => {
                self.user_interface.cursor = self
                    .user_interface
                    .cursor
                    .move_to(*pos)
                    .clamp(&self.grid.rect);

                self.process_message(&Message::UpdatePrompt);
            }

            Message::Move(pos) => {
                self.user_interface.cursor =
                    self.user_interface.cursor.add(*pos).clamp(&self.grid.rect);

                self.process_message(&Message::UpdatePrompt);
            }

            Message::MoveToEmpty => {
                let pos = self.grid.find_nearest_empty(&self.user_interface.cursor);
                self.process_message(&Message::MoveTo(pos));
            }

            Message::UpdatePrompt => {
                let entity = self.grid.get_entity(&self.user_interface.cursor);

                self.user_interface.prompt =
                    entity.map(|e| e.get_prompt()).unwrap_or_else(|| "".into());

                self.user_interface.settings = entity.map(|e| e.get_settings());
            }

            Message::AddStep => {
                if self.grid.is_empty(&self.user_interface.cursor) {
                    let mut step = Step::new(&mut self.screech);
                    step.set_position(self.user_interface.cursor);
                    self.grid.add_step(&mut self.screech, step);
                }
            }

            Message::AddTrigger => {
                if self.grid.is_empty(&self.user_interface.cursor) {
                    let mut trigger = Trigger::new(&mut self.screech);
                    trigger.set_position(self.user_interface.cursor);
                    self.grid.add_trigger(&mut self.screech, trigger);
                }
            }

            Message::DeleteEntity => {
                self.grid
                    .remove_entity(&mut self.screech, &self.user_interface.cursor);
            }

            Message::SwitchInputMode(mode) => {
                self.input_mode = *mode;
            }

            Message::JumpSetting(amount) => {
                match (self.input_mode, &self.user_interface.settings) {
                    (InputMode::Edit(pos), Some(settings)) => {
                        let new_pos =
                            ((pos + settings.len()) as i32 + amount) % settings.len() as i32;
                        self.input_mode = InputMode::Edit(new_pos as usize);
                    }
                    _ => (),
                }
            }
        }
    }
}
