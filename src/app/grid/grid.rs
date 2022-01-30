use super::{ConnType, Connector, Entity, Module, Position, Rect};
use screech::core::ExternalSignal;
use screech::traits::Source;
use std::collections::HashMap;

enum GridEntity {
    Module(Box<dyn Module>),
    Connector(Connector),
}

impl GridEntity {
    fn as_entity(&self) -> &dyn Entity {
        match self {
            GridEntity::Module(e) => e.as_entity(),
            GridEntity::Connector(e) => e as &dyn Entity,
        }
    }

    fn as_mut_entity(&mut self) -> &mut dyn Entity {
        match self {
            GridEntity::Module(e) => e.as_mut_entity(),
            GridEntity::Connector(e) => e as &mut dyn Entity,
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

    fn find_connections(
        &self,
        module: &dyn Module,
    ) -> Option<Vec<(Vec<ConnType>, ExternalSignal)>> {
        // @TODO: implement
        None
    }

    pub fn update_connections(&mut self) {
        let mut connections: HashMap<usize, Vec<(Vec<ConnType>, ExternalSignal)>> = HashMap::new();

        for &module in self.get_modules().iter() {
            if let Some(conns) = self.find_connections(module) {
                connections.insert(*module.get_source_id(), conns);
            }
        }

        for module in self.get_mut_modules().iter_mut() {
            if let Some(conns) = connections.get(module.get_source_id()) {
                for (connection, signal) in conns {
                    module.process_signal(connection, signal);
                }
            }
        }
    }

    pub fn get_modules(&self) -> Vec<&dyn Module> {
        self.entities
            .values()
            .filter_map(|e| match e {
                GridEntity::Module(m) => Some(m),
                _ => None,
            })
            .map(|e| &**e as &dyn Module)
            .collect()
    }

    pub fn get_mut_modules(&mut self) -> Vec<&mut dyn Module> {
        self.entities
            .values_mut()
            .filter_map(|e| match e {
                GridEntity::Module(m) => Some(m),
                _ => None,
            })
            .map(|e| &mut **e as &mut dyn Module)
            .collect()
    }

    pub fn add_module(&mut self, module: Box<dyn Module>) {
        self.entities
            .insert(*module.get_position(), GridEntity::Module(module));
    }

    pub fn add_connector(&mut self, conn: Connector) {
        self.entities
            .insert(*conn.get_position(), GridEntity::Connector(conn));
    }

    pub fn get_mut_sources(&mut self) -> Vec<&mut dyn Source> {
        let mut sources = vec![];

        for module in self.get_mut_modules() {
            sources.push(module.as_mut_source());
        }

        sources
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

    pub fn get_entity(&self, pos: &Position) -> Option<&dyn Entity> {
        self.entities.get(pos).map(|e| e.as_entity())
    }

    pub fn get_mut_entity(&mut self, pos: &Position) -> Option<&mut dyn Entity> {
        self.entities.get_mut(pos).map(|e| e.as_mut_entity())
    }

    pub fn remove_entity(&mut self, pos: &Position) {
        self.entities.remove(pos);
    }
}
