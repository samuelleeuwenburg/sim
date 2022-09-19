mod graphics;
mod color;
mod bitmap;
mod image;

use crate::glyphs::ascii::bitmap_from_char;
use crate::{Grid, InputState, Input};
pub use graphics::Graphics;
pub use color::Color;
pub use bitmap::Bitmap;
pub use image::Image;

pub struct UserInterface {
    pub grid_block_size: (i32, i32),
    pub prompt: String,
}

impl UserInterface {
    pub fn new() -> Self {
        UserInterface {
            grid_block_size: (8, 8),
	    prompt: String::from("!#\"abcdf"),
        }
    }

    pub fn process_input(&mut self, input: &InputState) {
	for input in &input.buffer {
	    match input {
		Input::Char(c) => {
		    self.prompt.push(*c);
		}
		Input::Enter => {
		    self.prompt.clear();
		},
		_ => (),
	    }
	}
    }

    pub fn render(&mut self, g: &mut dyn Graphics, _grid: &Grid) {
        g.clear();

        self.render_background(g);

	for (i, c) in self.prompt.chars().enumerate() {
	    let bitmap = bitmap_from_char(c);
	    let char = Image::from_bitmap(&bitmap, Color::new(255, 255, 255, 255));
	    g.draw_image(&char, i as i32 * 10, 0);
	}
    }

    fn render_background(&mut self, g: &mut dyn Graphics) {
	let (w, h) = g.get_viewport();
        g.draw_rect(Color::new(0, 0, 0, 255), 0, 0, w, h);
    }
}
