use crate::grid::Grid;
use screech::{BasicTracker, Screech};

pub struct Audio {
    screech: Screech,
}

impl Audio {
    pub fn new(sample_rate: usize, buffer_size: usize) -> Self {
        let tracker = Box::new(BasicTracker::<256>::new(buffer_size));
        let mut screech = Screech::with_tracker(tracker, sample_rate);

        // setup new output buffer
        screech.create_main_out("left_out");
        screech.create_main_out("right_out");

        Audio { screech }
    }

    pub fn sample(&mut self, grid: &mut Grid) -> (&[f32], &[f32]) {
        let mut sources = vec![];

        for module in grid.get_mut_entities() {
            sources.push(module.as_mut_source());
        }

        self.screech.sample(&mut sources).unwrap();

        (
            &self.screech.get_main_out("left_out").unwrap().samples,
            &self.screech.get_main_out("right_out").unwrap().samples,
        )
    }
}
