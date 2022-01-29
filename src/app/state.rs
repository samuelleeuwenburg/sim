use screech::core::{BasicTracker, Primary};
use screech::traits::Source;

use super::entities::{Modifier, Track, VCO};
use super::grid::{Entity, Grid};
use super::message::{EntityMsg, Message};
use super::user_interface::{Graphics, UserInterface};

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
        let sources: Vec<&mut dyn Source> = self
            .grid
            .entities
            .iter_mut()
            .filter_map(|e| e.get_mut_source())
            .collect();

        self.primary.sample(sources).unwrap()
    }

    pub fn update_ui(&mut self) {
        self.user_interface.display_entities.clear();

        for e in &self.grid.entities {
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

            Message::AddEntity(msg) => {
                if self.grid.get_entity(&self.user_interface.cursor).is_none() {
                    let mut entity: Box<dyn Entity> = match msg {
                        EntityMsg::VCO => Box::new(VCO::new(&mut self.primary)),
                        EntityMsg::Track => Box::new(Track::new(&mut self.primary)),
                        EntityMsg::Modifier(mod_type) => Box::new(Modifier::new(mod_type)),
                    };

                    entity.set_position(&self.user_interface.cursor);
                    self.grid.entities.push(entity);

                    self.process_message(&Message::UpdatePrompt);
                } else {
                    self.user_interface.prompt = "Space is occupied".into()
                }
            }

            Message::DeleteEntity => {
                let mut index = 0;

                for e in self.grid.entities.iter() {
                    let pos = e.get_position();

                    if pos == &self.user_interface.cursor {
                        self.grid.entities.swap_remove(index);
                        break;
                    }

                    index += 1;
                }

                self.user_interface.prompt.clear();
            }
        }
    }
}
