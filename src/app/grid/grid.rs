use super::{Entity, EntityKind, Position, Rect, Step, Trigger};
use screech::traits::Source;
use screech::Screech;
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

    pub fn remove_entity(&mut self, screech: &mut Screech, pos: &Position) {
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

    pub fn get_mut_sources(&mut self) -> Vec<&mut dyn Source> {
        let mut sources = vec![];

        for module in self.get_mut_entities() {
            sources.push(module.as_mut_source());
        }

        sources
    }

    pub fn add_step(&mut self, screech: &mut Screech, step: Step) {
        let pos = step.get_position();
        self.entities.insert(pos, GridEntity::Step(step));

        self.connect(screech, &pos);
    }

    pub fn add_trigger(&mut self, screech: &mut Screech, trigger: Trigger) {
        let pos = trigger.get_position();
        self.entities.insert(pos, GridEntity::Trigger(trigger));

        self.connect(screech, &pos);
    }

    pub fn connect(&mut self, screech: &mut Screech, pos: &Position) {
        let entity = self
            .entities
            .get(&pos)
            .expect("missing target entity when making connections");

        let entities: Vec<&GridEntity> = [
            Position::new(pos.x, pos.y - 1), // north
            Position::new(pos.x + 1, pos.y), // east
            Position::new(pos.x, pos.y + 1), // south
            Position::new(pos.x - 1, pos.y), // west
        ]
        .iter()
        .filter_map(|pos| self.entities.get(&pos))
        .collect();

        let mut conns = vec![];

        for e in entities {
            let relative_pos = e
                .as_entity()
                .get_position()
                .subtract(entity.as_entity().get_position());

            conns.append(
                &mut entity
                    .as_entity()
                    .find_connections(&e.as_entity().as_kind(), relative_pos),
            );

            conns.append(
                &mut e
                    .as_entity()
                    .find_connections(&entity.as_entity().as_kind(), relative_pos.invert()),
            );
        }

        for (output, input) in conns {
            web_sys::console::log_1(&format!("{:?} : {:?}", output, input).into());
            screech.connect_signal(&output, &input);
        }
    }
}
