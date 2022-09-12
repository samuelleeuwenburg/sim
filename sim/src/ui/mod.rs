mod graphics;

use super::grid::{Grid, Position};
pub use graphics::{Color, Image, Graphics};

pub struct UserInterface {
    pub cursor: Position,
    pub grid_block_size: (i32, i32),
}

impl UserInterface {
    pub fn new() -> Self {
        UserInterface {
            cursor: Position::new(0, 0),
            grid_block_size: (42, 64),
        }
    }

    pub fn render(&mut self, g: &mut dyn Graphics, _grid: &Grid) {
        g.clear();
        self.render_background(g);

	self.cursor.x += 1;
	self.cursor.y += 1;
	if self.cursor.x > 400 {
	    self.cursor.x = 0;
	    self.cursor.y = 0;
	}

	let mut image = Image::new(20, 20);

	image.data[0] = Color::new(100, 10, 255, 255);
	image.data[1] = Color::new(100, 10, 255, 255);
	image.data[2] = Color::new(100, 10, 255, 255);
	image.data[10] = Color::new(100, 10, 255, 255);
	image.data[11] = Color::new(100, 10, 255, 255);
	image.data[12] = Color::new(100, 10, 255, 255);

        g.draw_image(&image, self.cursor.x, self.cursor.y);
    }

    fn render_background(&self, g: &mut dyn Graphics) {
        let (width, height) = g.get_viewport();

        g.draw_rect(Color::new(0, 0, 0, 255), 0, 0, width, height);
    }
}
