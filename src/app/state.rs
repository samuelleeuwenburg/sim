use super::grid::{Connector, Entity, Grid, Module};
use super::message::{Message, Modules};
use super::modules::{Track, VCO};
use super::user_interface::{Graphics, UserInterface};
use screech::core::{BasicTracker, Primary};

pub struct State<const BUFFER_SIZE: usize> {
    primary: Primary<BUFFER_SIZE>,
    grid: Grid,
    pub user_interface: UserInterface,
}

impl<const BUFFER_SIZE: usize> State<BUFFER_SIZE> {
    pub fn new(sample_rate: usize) -> Self {
        let tracker = Box::new(BasicTracker::<256, 8>::new());

        State {
            primary: Primary::with_tracker(tracker, sample_rate),
            grid: Grid::new(),
            user_interface: UserInterface::new(),
        }
    }

    pub fn sample(&mut self) -> &[f32; BUFFER_SIZE] {
        let sources = self.grid.get_mut_sources();
        self.primary.sample(sources).unwrap()
    }

    pub fn update_ui(&mut self) {
        self.user_interface.display_entities.clear();

        for e in self.grid.get_entities() {
            self.user_interface.display_entities.push(e.get_display());
        }
    }

    pub fn render_ui(&self, g: &dyn Graphics) {
        self.user_interface.render(g, &self.grid)
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

            Message::MoveTo(pos) => {
                self.user_interface
                    .cursor
                    .move_to(&pos)
                    .clamp(&self.grid.rect);

                self.process_message(&Message::UpdatePrompt);
            }

            Message::Move(pos) => {
                self.user_interface.cursor.add(&pos).clamp(&self.grid.rect);

                self.process_message(&Message::UpdatePrompt);
            }

            Message::UpdatePrompt => {
                self.user_interface.prompt = self
                    .grid
                    .get_entity(&self.user_interface.cursor)
                    .map(|e| e.get_prompt())
                    .unwrap_or_else(|| "".into());
            }

            Message::UpdateEntities => {
                self.grid.update_connections();
            }

            Message::AddConnector(conn_type) => {
                if self.grid.get_entity(&self.user_interface.cursor).is_none() {
                    let mut conn = Connector::new(conn_type);
                    conn.set_position(&self.user_interface.cursor);
                    self.grid.add_connector(conn);
                } else {
                    self.user_interface.prompt = "already occupied".into()
                }
            }

            Message::AddModule(msg) => {
                if self.grid.get_entity(&self.user_interface.cursor).is_none() {
                    let mut module: Box<dyn Module> = match msg {
                        Modules::VCO => Box::new(VCO::new(&mut self.primary)),
                        Modules::Track => Box::new(Track::new(&mut self.primary)),
                    };

                    module.set_position(&self.user_interface.cursor);
                    self.grid.add_module(module);

                    self.process_message(&Message::UpdatePrompt);
                    self.process_message(&Message::UpdateEntities);
                } else {
                    self.user_interface.prompt = "already occupied".into()
                }
            }

            Message::DeleteEntity => {
                self.grid.remove_entity(&self.user_interface.cursor);
                self.user_interface.prompt.clear();
            }
        }
    }
}
