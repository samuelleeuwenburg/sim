use screech::core::{BasicTracker, Primary};
use screech::traits::Source;

use super::entities::{Modifier, Track, VCO};
use super::grid::{Entity, Grid};
use super::message::Message;
use super::user_interface::{Graphics, UserInterface};

pub struct State<const BUFFER_SIZE: usize> {
    primary: Primary<BUFFER_SIZE>,
    grid: Grid,
    pub user_interface: UserInterface,
    freq_pos: usize,
}

impl<const BUFFER_SIZE: usize> State<BUFFER_SIZE> {
    pub fn new(sample_rate: usize) -> Self {
        let tracker = Box::new(BasicTracker::<256, 8>::new());

        State {
            primary: Primary::with_tracker(tracker, sample_rate),
            grid: Grid::new(),
            user_interface: UserInterface::new(),
            freq_pos: 1,
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
                self.user_interface.prompt = "".into();

                for e in &self.grid.entities {
                    if e.get_position() == &self.user_interface.cursor {
                        self.user_interface.prompt = e.get_prompt();
                    }
                }
            }

            Message::AddModifier(mod_type) => {
                let pos = self.user_interface.cursor;
                let mut modifier = Modifier::new(mod_type);
                modifier.set_position(&pos);
                self.grid.entities.push(Box::new(modifier));

                self.process_message(&Message::UpdatePrompt);
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
                self.grid.entities.push(Box::new(vco));

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

                self.grid.entities.push(Box::new(track));

                self.process_message(&Message::UpdatePrompt);
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
            }
        }
    }
}
