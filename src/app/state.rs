use screech::core::{BasicTracker, Primary};
use screech::traits::Source;

use super::entities::{Track, VCO};
use super::message::Message;
use super::user_interface::{Graphics, UserInterface};
use super::GridEntity;

pub enum Entity {
    VCO(VCO),
    Track(Track),
}

impl Entity {
    pub fn get_mut_source(&mut self) -> &mut dyn Source {
        match self {
            Entity::VCO(o) => &mut o.oscillator as &mut dyn Source,
            Entity::Track(t) => &mut t.track as &mut dyn Source,
        }
    }

    pub fn get_grid_entity(&self) -> &dyn GridEntity {
        match self {
            Entity::VCO(o) => o as &dyn GridEntity,
            Entity::Track(t) => t as &dyn GridEntity,
        }
    }
}

pub struct State<const BUFFER_SIZE: usize> {
    primary: Primary<BUFFER_SIZE>,
    freq_pos: usize,
    entities: Vec<Entity>,
    pub user_interface: UserInterface,
}

impl<const BUFFER_SIZE: usize> State<BUFFER_SIZE> {
    pub fn new(sample_rate: usize) -> Self {
        let tracker = Box::new(BasicTracker::<256, 8>::new());

        State {
            primary: Primary::with_tracker(tracker, sample_rate),
            entities: Vec::with_capacity(256),
            freq_pos: 1,
            user_interface: UserInterface::new(),
        }
    }

    pub fn sample(&mut self) -> &[f32; BUFFER_SIZE] {
        let sources: Vec<&mut dyn Source> = self
            .entities
            .iter_mut()
            .map(|e| e.get_mut_source())
            .collect();

        self.primary.sample(sources).unwrap()
    }

    pub fn update_ui(&mut self) {
        self.user_interface.display_entities.clear();

        for e in &self.entities {
            self.user_interface
                .display_entities
                .push(e.get_grid_entity().get_display());
        }
    }

    pub fn render_ui(&self, g: &dyn Graphics) {
        self.user_interface.render(g)
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
                    .clamp(&self.user_interface.grid);

                self.process_message(&Message::UpdatePrompt);
            }

            Message::Move(pos) => {
                self.user_interface
                    .cursor
                    .add(&pos)
                    .clamp(&self.user_interface.grid);

                self.process_message(&Message::UpdatePrompt);
            }

            Message::UpdatePrompt => {
                self.user_interface.prompt = "".into();

                for e in &self.entities {
                    let e = e.get_grid_entity();

                    if e.get_position() == &self.user_interface.cursor {
                        self.user_interface.prompt = e.get_prompt();
                    }
                }
            }

            Message::AddOscillator => {
                let pos = self.user_interface.cursor;
                let mut vco = VCO::new(&mut self.primary);
                let f = 110.0 * self.freq_pos as f32;

                vco.set_position(&pos);
                vco.oscillator.output_saw();
                vco.oscillator.amplitude = 0.1;
                vco.oscillator.frequency = f;

                self.primary.add_monitor(vco.oscillator.output);
                self.entities.push(Entity::VCO(vco));

                self.freq_pos = if self.freq_pos >= 8 {
                    1
                } else {
                    self.freq_pos + 1
                };

                self.process_message(&Message::UpdatePrompt);
            }

            Message::AddTrack => {
                let pos = self.user_interface.cursor;
                let mut track = Track::new(&mut self.primary);

                track.set_position(&pos);

                self.entities.push(Entity::Track(track));

                self.process_message(&Message::UpdatePrompt);
            }

            Message::DeleteEntity => {
                let mut index = 0;

                for e in self.entities.iter() {
                    let e = e.get_grid_entity();
                    let pos = e.get_position();

                    if pos == &self.user_interface.cursor {
                        self.entities.swap_remove(index);
                        break;
                    }

                    index += 1;
                }
            }
        }
    }
}
