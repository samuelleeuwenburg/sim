use screech::basic::Track;
use screech::core::{BasicTracker, Primary};
use screech::traits::Source;

use super::command::Command;
use super::user_interface::UserInterface;
use super::vco::VCO;
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
            .map(|t| t as &mut dyn Source)
            .collect();

        a.append(&mut b);

        self.primary.sample(a).unwrap()
    }

    pub fn process_commands(&mut self, commands: &[Command]) {
        for command in commands {
            self.process_command(command);
        }
    }

    pub fn process_command(&mut self, command: &Command) {
        match command {
            Command::Move(pos) => {
                self.user_interface.cursor.add(&pos);
            }

            Command::AddOscillator => {
                let pos = self.user_interface.cursor;
                let mut vco = VCO::new(&mut self.primary);
                let f = 110.0 * self.freq_pos as f32;

                vco.set_position(&pos);
                vco.oscillator.output_saw();
                vco.oscillator.amplitude = 0.1;
                vco.oscillator.frequency = f;

                self.primary.add_monitor(vco.oscillator.output);
                self.oscillators.push(vco);

                self.freq_pos = if self.freq_pos >= 16 {
                    1
                } else {
                    self.freq_pos + 1
                };
            }

            Command::DeleteOscillator => {
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
            }
            _ => (),
        }
    }
}
