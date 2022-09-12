#[derive(Default, Copy, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
	Color { red, green, blue, alpha }
    }
}

#[derive(Clone, Default)]
pub struct Image {
    pub width: i32,
    pub height: i32,
    pub data: Vec<Color>,
}

impl Image {
    pub fn new(width: i32, height: i32) -> Self {
        Image {
            width,
            height,
            data: vec![Default::default(); (width * height) as usize],
        }
    }
}

pub trait Graphics {
    fn clear(&mut self);
    fn get_viewport(&self) -> (i32, i32);
    fn draw_rect(&mut self, color: Color, x: i32, y: i32, w: i32, h: i32);
    fn draw_image(&mut self, image: &Image, x: i32, y: i32);
}
