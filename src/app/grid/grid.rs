use super::{Entity, Position, Rect, Step};
use screech::core::ExternalSignal;
use screech::traits::Source;
use std::collections::HashMap;

enum GridEntity {
    Step(Step),
}

impl GridEntity {
    fn as_entity(&self) -> &dyn Entity {
        match self {
            GridEntity::Step(e) => e as &dyn Entity,
        }
    }

    fn as_mut_entity(&mut self) -> &mut dyn Entity {
        match self {
            GridEntity::Step(e) => e as &mut dyn Entity,
        }
    }
}

pub struct Grid {
    pub rect: Rect,
    entities: HashMap<Position, GridEntity>,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            rect: Rect::new(33, 17, Position::origin()),
            entities: HashMap::new(),
        }
    }

    pub fn find_nearest_empty(&self, p: &Position) -> Position {
        let mut pos = p.clone();

        while self.entities.get(&pos).is_some() {
            if pos.x >= self.rect.width - 1 {
                pos.x = 0;
                pos.y += 1;

                if pos.y >= self.rect.height - 1 {
                    pos.y = 0;
                }
            } else {
                pos.x += 1;
            }
        }

        pos
    }

    pub fn is_empty(&self, pos: &Position) -> bool {
        !self.entities.contains_key(&pos)
    }

    pub fn remove_entity(&mut self, pos: &Position) {
        self.entities.remove(pos);
    }

    pub fn get_entity(&self, pos: &Position) -> Option<&dyn Entity> {
        self.entities.get(pos).map(|e| e.as_entity())
    }

    pub fn get_entities(&self) -> Vec<&dyn Entity> {
        self.entities.values().map(|e| e.as_entity()).collect()
    }

    pub fn get_mut_entities(&mut self) -> Vec<&mut dyn Entity> {
        self.entities
            .values_mut()
            .map(|e| e.as_mut_entity())
            .collect()
    }

    pub fn get_mut_steps(&mut self) -> Vec<&mut Step> {
        self.entities
            .values_mut()
            .filter_map(|e| match e {
                GridEntity::Step(s) => Some(s),
                _ => None,
            })
            .collect()
    }

    pub fn get_mut_sources(&mut self) -> Vec<&mut dyn Source> {
        let mut sources = vec![];

        for module in self.get_mut_entities() {
            sources.push(module.as_mut_source());
        }

        sources
    }

    pub fn add_step(&mut self, step: Step) {
        self.entities
            .insert(*step.get_position(), GridEntity::Step(step));

        self.connect_steps();
    }

    pub fn connect_steps(&mut self) {
        let mut steps = self.get_mut_steps();

        steps.sort_by(|a, b| a.get_position().partial_cmp(b.get_position()).unwrap());

        let mut signal: Option<ExternalSignal> = None;

        for step in steps.into_iter() {
            // clear input
            step.input = None;

            // check if it is connected
            step.input = signal;

            // set the next signal up
            signal = Some(step.output);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use screech::core::Primary;

    #[test]
    fn test_chain_steps() {
        let mut primary = Primary::<4>::new(4);
        primary.output_mono();

        let mut steps = [
            Step::new(&mut primary),
            Step::new(&mut primary),
            Step::new(&mut primary),
            Step::new(&mut primary),
        ];

        steps[0].set_position(&Position::new(0, 0));
        steps[0].is_active = true;
        steps[0].frequency = 1.5;

        steps[1].set_position(&Position::new(1, 0));
        steps[1].frequency = 1.5;

        steps[2].set_position(&Position::new(2, 0));
        steps[2].frequency = 1.5;

        steps[3].set_position(&Position::new(3, 0));
        steps[3].frequency = 1.5;

        primary.add_monitor(steps[0].output);
        primary.add_monitor(steps[2].output);
        primary.add_monitor(steps[3].output);

        let mut grid = Grid::new();

        for step in steps {
            grid.add_step(step)
        }

        assert_eq!(
            primary.sample(grid.get_mut_sources()).unwrap(),
            &[0.0, 0.0, 1.0, 0.0]
        );
        assert_eq!(
            primary.sample(grid.get_mut_sources()).unwrap(),
            &[0.0, 0.0, 1.0, 0.0]
        );
        assert_eq!(
            primary.sample(grid.get_mut_sources()).unwrap(),
            &[1.0, 0.0, 0.0, 0.0]
        );
    }
}
