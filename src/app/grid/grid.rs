use super::{ConnType, Connector, Entity, Module, Position, Rect};
use screech::core::ExternalSignal;
use screech::traits::Source;
use std::collections::HashMap;

pub struct Grid {
    pub rect: Rect,
    modules: Vec<Box<dyn Module>>,
    connectors: Vec<Connector>,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            rect: Rect::new(33, 17, Position::origin()),
            modules: Vec::with_capacity(256),
            connectors: Vec::with_capacity(256),
        }
    }

    fn find_connections(
        &self,
        module: &Box<dyn Module>,
    ) -> Option<Vec<(Vec<ConnType>, ExternalSignal)>> {
        // @TODO: implement
        None
    }

    pub fn update_connections(&mut self) {
        let mut connections: HashMap<usize, Vec<(Vec<ConnType>, ExternalSignal)>> = HashMap::new();

        for module in self.modules.iter() {
            if let Some(conns) = self.find_connections(&module) {
                connections.insert(*module.get_source_id(), conns);
            }
        }

        for module in self.modules.iter_mut() {
            if let Some(conns) = connections.get(module.get_source_id()) {
                for (connection, signal) in conns {
                    module.process_signal(connection, signal);
                }
            }
        }
    }

    pub fn add_module(&mut self, module: Box<dyn Module>) {
        self.modules.push(module);
    }

    pub fn add_connector(&mut self, conn: Connector) {
        self.connectors.push(conn);
    }

    pub fn get_mut_sources(&mut self) -> Vec<&mut dyn Source> {
        self.modules.iter_mut().map(|e| e.as_mut_source()).collect()
    }

    pub fn get_entities(&self) -> Vec<&dyn Entity> {
        let modules = self.modules.iter().map(|e| e.as_entity());
        let connectors = self.connectors.iter().map(|e| e as &dyn Entity);
        modules.chain(connectors).collect()
    }

    pub fn get_mut_entities(&mut self) -> Vec<&mut dyn Entity> {
        let modules = self.modules.iter_mut().map(|e| e.as_mut_entity());
        let connectors = self.connectors.iter_mut().map(|e| e as &mut dyn Entity);
        modules.chain(connectors).collect()
    }

    pub fn get_entity(&self, pos: &Position) -> Option<&dyn Entity> {
        self.get_entities()
            .into_iter()
            .find(|e| e.get_position() == pos)
    }

    pub fn get_mut_entity(&mut self, pos: &Position) -> Option<&mut dyn Entity> {
        self.get_mut_entities()
            .into_iter()
            .find(|e| e.get_position() == pos)
    }

    pub fn remove_entity(&mut self, pos: &Position) {
        // @TODO: call .clear_signals on Source entities
        self.modules.retain(|m| m.get_position() != pos);
        self.connectors.retain(|c| c.get_position() != pos);
    }
}
