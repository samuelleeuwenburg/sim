use screech::core::{BasicTracker, Primary};
use screech::traits::Source;

use super::entities::{Track, VCO};
use super::message::Message;
use super::user_interface::{Graphics, UserInterface};
use super::GridEntity;

pub struct State<const BUFFER_SIZE: usize> {
    primary: Primary<BUFFER_SIZE>,
    freq_pos: usize,
    tracks: Vec<Track>,
    pub oscillators: Vec<VCO>,
    pub user_interface: UserInterface,
}

impl<const BUFFER_SIZE: usize> State<BUFFER_SIZE> {
    pub fn new(sample_rate: usize) -> Self {
        let tracker = Box::new(BasicTracker::<256, 8>::new());

        State {
            primary: Primary::with_tracker(tracker, sample_rate),
            oscillators: vec![],
            tracks: vec![],
            freq_pos: 1,
            user_interface: UserInterface::new(),
        }
    }

    pub fn sample(&mut self) -> &[f32; BUFFER_SIZE] {
        let mut a: Vec<&mut dyn Source> = self
            .oscillators
            .iter_mut()
            .map(|vco| &mut vco.oscillator as &mut dyn Source)
            .collect();

        let mut b: Vec<&mut dyn Source> = self
            .tracks
            .iter_mut()
            .map(|t| &mut t.track as &mut dyn Source)
            .collect();

        a.append(&mut b);

        self.primary.sample(a).unwrap()
    }

    pub fn update_ui(&mut self) {
        self.user_interface.display_entities.clear();

        for osc in &self.oscillators {
            self.user_interface.display_entities.push(osc.get_display());
        }

        for track in &self.tracks {
            self.user_interface
                .display_entities
                .push(track.get_display());
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

                for osc in &self.oscillators {
                    if osc.get_position() == &self.user_interface.cursor {
                        self.user_interface.prompt = osc.get_prompt();
                        break;
                    }
                }

                for track in &self.tracks {
                    if track.get_position() == &self.user_interface.cursor {
                        self.user_interface.prompt = track.get_prompt();
                        break;
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
                self.oscillators.push(vco);

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

                self.tracks.push(track);

                self.process_message(&Message::UpdatePrompt);
            }

            Message::DeleteEntity => {
                let position = self.user_interface.cursor;

                let mut index = 0;

                for vco in self.oscillators.iter() {
                    let pos = vco.get_position();
                    if pos.x == position.x && pos.y == position.y {
                        self.oscillators.swap_remove(index);
                        break;
                    }

                    index += 1;
                }

                index = 0;

                for track in self.tracks.iter() {
                    let pos = track.get_position();
                    if pos.x == position.x && pos.y == position.y {
                        self.tracks.swap_remove(index);
                        break;
                    }

                    index += 1;
                }
            }
        }
    }
}
