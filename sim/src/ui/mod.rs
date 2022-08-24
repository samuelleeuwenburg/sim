mod graphics;

use super::grid::{Grid, Position};
use graphics::Graphics;

pub struct UserInterface {
    pub cursor: Position,
    pub grid_block_size: (f32, f32),
}

impl UserInterface {
    pub fn new() -> Self {
        UserInterface {
            cursor: Position::new(0, 0),
            grid_block_size: (42.0, 64.0),
        }
    }

    fn get_grid_position(&self, g: &dyn Graphics, grid: &Grid, x: i32, y: i32) -> (f32, f32) {
        let (width, height) = g.get_viewport();

        let (block_width, block_height) = self.grid_block_size;

        let origin_x = width as f32 / 2.0 - (grid.rect.width as f32 * block_width) / 2.0;
        let origin_y = height as f32 / 2.0 - (grid.rect.height as f32 * block_height) / 2.0;

        let px_x = origin_x + (x as f32 * block_width);
        let px_y = origin_y + (y as f32 * block_height);

        (px_x, px_y)
    }

    pub fn render(&self, g: &dyn Graphics, grid: &Grid) {
        g.clear();
        self.render_background(g);
        self.render_grid(g, grid);
        self.render_cursor(g, grid);
    }

    fn render_background(&self, g: &dyn Graphics) {
        let (width, height) = g.get_viewport();

        g.draw_rect(
            Color::Rgb(0, 0, 0),
            Style::Fill,
            0.0,
            0.0,
            width as f32,
            height as f32,
        );
    }

    fn render_grid(&self, g: &dyn Graphics, grid: &Grid) {
        let (w, h) = self.grid_block_size;

        for x in 0..grid.rect.width {
            for y in 0..grid.rect.height {
                let (px_x, px_y) = self.get_grid_position(g, grid, x as i32, y as i32);
                let spacing = 4;

                if x % spacing == 0 && y % spacing == 0 {
                    g.draw_text(Color::Rgba(255, 255, 255, 0.2), px_x, px_y, w, h, ".");
                } else {
                    g.draw_text(Color::Rgba(255, 255, 255, 0.08), px_x, px_y, w, h, ".");
                }
            }
        }
    }

    fn render_cursor(&self, g: &dyn Graphics, grid: &Grid) {
        let (x, y) = self.get_grid_position(g, grid, self.cursor.x, self.cursor.y);
        let (w, h) = self.grid_block_size;
        g.draw_text(Color::Rgb(255, 255, 0), x, y, w, h, ".");
    }
}
