mod graphics;
mod color;
mod bitmap;
mod image;

use super::grid::Grid;
use crate::glyphs::ascii::bitmap_from_keycode;
pub use graphics::Graphics;
pub use color::Color;
pub use bitmap::Bitmap;
pub use image::Image;

pub struct UserInterface {
    pub grid_block_size: (i32, i32),
}

impl UserInterface {
    pub fn new() -> Self {
        UserInterface {
            grid_block_size: (8, 8),
        }
    }

    pub fn render(&mut self, g: &mut dyn Graphics, _grid: &Grid) {
        g.clear();

        self.render_background(g);

	let bitmap = bitmap_from_keycode(33);
	let char = Image::from_bitmap(&bitmap, Color::new(255, 255, 255, 255));
        g.draw_image(&char, 0, 0);
    }

    fn render_background(&mut self, g: &mut dyn Graphics) {
	let (w, h) = g.get_viewport();
        g.draw_rect(Color::new(0, 0, 0, 255), 0, 0, w, h);
    }
}
