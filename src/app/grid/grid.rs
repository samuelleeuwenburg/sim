use super::{Entity, EntityKind, Position, Rect, Step, Trigger};
use screech::core::ExternalSignal;
use screech::traits::Source;
use std::collections::HashMap;

enum GridEntity {
    Step(Step),
    Trigger(Trigger),
}

impl GridEntity {
    fn as_entity(&self) -> &dyn Entity {
        match self {
            GridEntity::Step(e) => e as &dyn Entity,
            GridEntity::Trigger(e) => e as &dyn Entity,
        }
    }

    fn as_mut_entity(&mut self) -> &mut dyn Entity {
        match self {
            GridEntity::Step(e) => e as &mut dyn Entity,
            GridEntity::Trigger(e) => e as &mut dyn Entity,
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
        self.connect();
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

        self.connect();
    }

    pub fn add_trigger(&mut self, trigger: Trigger) {
        self.entities
            .insert(*trigger.get_position(), GridEntity::Trigger(trigger));

        self.connect();
    }

    fn get_connections(&self, pos: Position) -> [Option<&GridEntity>; 4] {
        [
            self.entities.get(&Position::new(pos.x + 1, pos.y)),
            self.entities.get(&Position::new(pos.x - 1, pos.y)),
            self.entities.get(&Position::new(pos.x, pos.y + 1)),
            self.entities.get(&Position::new(pos.x, pos.y - 1)),
        ]
    }

    pub fn connect(&mut self) {
        let mut step_groups: Vec<Vec<(Position, ExternalSignal)>> = vec![];

        let mut steps: Vec<&Step> = self
            .entities
            .values()
            .filter_map(|e| match e {
                GridEntity::Step(s) => Some(s),
                _ => None,
            })
            .collect();

        steps.sort_by(|a, b| a.get_position().partial_cmp(b.get_position()).unwrap());

        let mut visited: HashMap<Position, bool> = HashMap::new();

        for step in steps.iter() {
            // skip already visited steps
            if visited.get(step.get_position()).is_some() {
                continue;
            }

            let mut pos = *step.get_position();
            visited.insert(pos, true);
            let mut group = vec![(pos, step.output)];

            // @TODO: this could be written less error prone
            'group: loop {
                for conn in self.get_connections(pos) {
                    if let Some(GridEntity::Step(step)) = conn {
                        if visited.get(step.get_position()).is_some() {
                            continue;
                        }

                        visited.insert(*step.get_position(), true);
                        group.push((*step.get_position(), step.output));
                        pos = *step.get_position();
                        continue 'group;
                    }
                }

                break;
            }

            step_groups.push(group);
        }

        // connect steps:
        for group in step_groups {
            let mut signal: Option<ExternalSignal> = None;

            for (pos, _) in group {
                if let Some(GridEntity::Step(step)) = self.entities.get_mut(&pos) {
                    step.clear_connections();

                    if let Some(s) = signal {
                        step.add_input(&s);
                    }

                    signal = Some(step.output);
                }
            }
        }

        // connect triggers to steps
        let mut trigger_connections: Vec<(Position, ExternalSignal)> = vec![];

        let triggers: Vec<&Trigger> = self
            .entities
            .values()
            .filter_map(|e| match e {
                GridEntity::Trigger(t) => Some(t),
                _ => None,
            })
            .collect();

        for trigger in triggers {
            for conn in self.get_connections(*trigger.get_position()) {
                if let Some(GridEntity::Step(step)) = conn {
                    trigger_connections.push((*step.get_position(), trigger.output));
                }
            }
        }

        for (pos, signal) in trigger_connections {
            if let Some(GridEntity::Step(step)) = self.entities.get_mut(&pos) {
                step.add_input(&signal);
            }
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

        let mut steps = [
            Step::new(&mut primary),
            Step::new(&mut primary),
            Step::new(&mut primary),
            Step::new(&mut primary),
            Step::new(&mut primary),
            Step::new(&mut primary),
            Step::new(&mut primary),
            Step::new(&mut primary),
            Step::new(&mut primary),
        ];

        // simple group
        steps[0].set_position(&Position::new(0, 0));
        steps[1].set_position(&Position::new(1, 0));
        steps[2].set_position(&Position::new(2, 0));
        steps[3].set_position(&Position::new(3, 0));
        // multiline group
        steps[4].set_position(&Position::new(2, 2));
        steps[5].set_position(&Position::new(3, 2));
        steps[6].set_position(&Position::new(3, 3));
        steps[7].set_position(&Position::new(2, 3));
        steps[8].set_position(&Position::new(2, 4));

        let mut grid = Grid::new();

        for step in steps {
            grid.add_step(step);
        }

        let mut sources: Vec<(usize, Vec<usize>)> = grid
            .get_mut_sources()
            .into_iter()
            .map(|s| (*s.get_source_id(), s.get_sources()))
            .collect();

        sources.sort();

        assert_eq!(
            sources,
            vec![
                // simple group
                (0, vec![]),
                (1, vec![0]),
                (2, vec![1]),
                (3, vec![2]),
                // multiline group
                (4, vec![]),
                (5, vec![4]),
                (6, vec![5]),
                (7, vec![6]),
                (8, vec![7]),
            ]
        );
    }

    #[test]
    fn test_trigger_steps() {
        let mut primary = Primary::<4>::new(4);

        let mut trigger = Trigger::new(&mut primary);
        trigger.set_position(&Position::new(0, 0));

        let mut steps = [Step::new(&mut primary)];

        steps[0].set_position(&Position::new(1, 0));

        let mut grid = Grid::new();

        grid.add_trigger(trigger);

        for step in steps {
            grid.add_step(step);
        }

        let mut sources: Vec<(usize, Vec<usize>)> = grid
            .get_mut_sources()
            .into_iter()
            .map(|s| (*s.get_source_id(), s.get_sources()))
            .collect();

        sources.sort();

        assert_eq!(sources, vec![(0, vec![]), (1, vec![0]),]);
    }
}
