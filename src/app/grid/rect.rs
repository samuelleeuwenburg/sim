use super::GridPosition;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub position: GridPosition,
    pub size: (i32, i32),
}

impl Rect {
    pub fn new(width: i32, height: i32, position: GridPosition) -> Self {
        Rect { position, size: (width, height) }
    }
}
