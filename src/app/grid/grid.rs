use super::{GridPosition, Rect};
use crate::app::entities::Entity;

pub struct Grid {
    pub rect: Rect,
    pub entities: Vec<Entity>,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            rect: Rect::new(33, 17, GridPosition::new(0, 0)),
            entities: Vec::with_capacity(256),
        }
    }
}
