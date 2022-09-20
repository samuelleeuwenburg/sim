mod bitmap;
mod color;
mod graphics;
mod image;

use crate::glyphs::ascii::bitmap_from_char;
use crate::{Grid, Input, InputState};
pub use bitmap::Bitmap;
pub use color::Color;
pub use graphics::Graphics;
pub use image::Image;

static DETAIL_VIEW_HEIGHT: i32 = 80;
static VIEW_BORDER: i32 = 1;
static VIEW_MARGIN: i32 = 4;

pub struct UserInterface {
    view_color: Color,
    grid_block_size: (i32, i32),
    font_size: (i32, i32),
    pub prompt: String,
}

impl UserInterface {
    pub fn new() -> Self {
        UserInterface {
	    view_color: Color::new(00, 209, 255, 255),
            grid_block_size: (8, 8),
            font_size: (8, 8),
            prompt: String::from("please type..."),
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
                }
                Input::Backspace => {
                    self.prompt.pop();
                }
                _ => (),
            }
        }
    }

    pub fn render(&mut self, g: &mut dyn Graphics, grid: &Grid) {
        g.clear();

        self.render_background(g);
	self.render_grid(g, grid);
	self.render_detail(g);
	self.render_prompt(g);
    }

    fn render_background(&mut self, g: &mut dyn Graphics) {
        let (w, h) = g.get_viewport();
        g.draw_rect(Color::new(0, 0, 0, 255), 0, 0, w, h);
    }

    fn render_grid(&mut self, g: &mut dyn Graphics, grid: &Grid) {
        let (vw, vh) = g.get_viewport();
        let (_, fh) = self.font_size;

	let grid_area_height = vh - DETAIL_VIEW_HEIGHT - fh;
    }

    fn render_detail(&mut self, g: &mut dyn Graphics) {
        let (vw, vh) = g.get_viewport();
        let (_, fh) = self.font_size;

	let x = VIEW_MARGIN;
	let y = vh - DETAIL_VIEW_HEIGHT - fh - VIEW_MARGIN * 2;
	let w = vw - VIEW_MARGIN * 2;
	let h = DETAIL_VIEW_HEIGHT;

        g.draw_rect(self.view_color, x, y, w, h);
        g.draw_rect(Color::new(0, 0, 0, 255), x + VIEW_BORDER, y + VIEW_BORDER, w - VIEW_BORDER * 2, h - VIEW_BORDER * 2);
    }

    fn render_prompt(&mut self, g: &mut dyn Graphics) {
        let (fw, fh) = self.font_size;
        let (_, vh) = g.get_viewport();
	let x = VIEW_MARGIN;
        let y = vh - fh - VIEW_MARGIN;

	let bitmap = bitmap_from_char('>');
	let char = Image::from_bitmap(&bitmap, Color::new(255, 255, 255, 255));
	g.draw_image(&char, x, y);

        for (i, c) in self.prompt.chars().enumerate() {
            let bitmap = bitmap_from_char(c);
            let char = Image::from_bitmap(&bitmap, Color::new(255, 255, 255, 255));
            g.draw_image(&char, VIEW_MARGIN + (i as i32 + 1) * fw, y);
        }
    }
}
