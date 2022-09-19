use super::{Color, Image};

pub trait Graphics {
    fn clear(&mut self);
    fn get_viewport(&self) -> (i32, i32);
    fn draw_rect(&mut self, color: Color, x: i32, y: i32, w: i32, h: i32);
    fn draw_image(&mut self, image: &Image, x: i32, y: i32);
}

