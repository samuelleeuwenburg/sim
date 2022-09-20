mod bitmap;
mod color;
mod graphics;
mod image;

use crate::glyphs::ascii::bitmap_from_char;
use crate::{Grid, Input, InputState};
use crate::grid::Position;
pub use bitmap::Bitmap;
pub use color::Color;
pub use graphics::Graphics;
pub use image::Image;

static DETAIL_VIEW_HEIGHT: i32 = 80;
static VIEW_BORDER: i32 = 1;
static VIEW_MARGIN: i32 = 4;

#[derive(PartialEq)]
enum ActiveView {
    Grid,
    Detail,
}

pub struct UserInterface {
    select_color: Color,
    background_color: Color,
    grid_block_size: (i32, i32),
    font_size: (i32, i32),
    prompt: String,
    prompt_is_active: bool,
    active_view: ActiveView,
}

impl UserInterface {
    pub fn new() -> Self {
        UserInterface {
	    select_color: Color::new(00, 209, 255, 255),
	    background_color: Color::new(0, 0, 0, 255),
            grid_block_size: (8, 8),
            font_size: (8, 8),
            prompt: String::from(""),
	    prompt_is_active: false,
	    active_view: ActiveView::Grid,
        }
    }

    pub fn process_input(&mut self, grid: &mut Grid, input_state: &InputState) {
	for input in &input_state.buffer {
	    if self.prompt_is_active {
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
		    Input::Escape => {
			self.prompt_is_active = false;
		    }
		    _ => (),
		}
	    } else {
		match input {
		    Input::Char('>') => {
			self.prompt_is_active = true;
		    }
		    _ => (),
		}
		match self.active_view {
		    ActiveView::Grid => {
			match input {
			    Input::Char('l') | Input::Right => {
				grid.cursor_position = grid.cursor_position.add(Position::new(1, 0));
			    }
			    Input::Char('h') | Input::Left => {
				grid.cursor_position = grid.cursor_position.add(Position::new(-1, 0));
			    }
			    Input::Char('k') | Input::Up => {
				grid.cursor_position = grid.cursor_position.add(Position::new(0, -1));
			    }
			    Input::Char('j') | Input::Down => {
				grid.cursor_position = grid.cursor_position.add(Position::new(0, 1));
			    }
			    Input::Tab => {
				self.active_view = ActiveView::Detail;
			    }
			    _ => (),
			}
		    }
		    ActiveView::Detail => {
			match input {
			    Input::Tab => {
				self.active_view = ActiveView::Grid;
			    }
			    _ => (),
			}
		    }
		};
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

    fn render_grid(&self, g: &mut dyn Graphics, grid: &Grid) {
        let (vw, vh) = g.get_viewport();
        let (_, fh) = self.font_size;
	let (gw, gh) = self.grid_block_size;

	if self.active_view == ActiveView::Grid {
	    let x = VIEW_MARGIN;
	    let y = VIEW_MARGIN;
	    let w = vw - VIEW_MARGIN * 2;
	    let h = vh - DETAIL_VIEW_HEIGHT - fh - VIEW_MARGIN * 4;

	    let mut color = self.select_color;
	    if self.prompt_is_active {
		color.alpha = 128;
	    }

	    g.draw_rect(color, x, y, w, h);
            g.draw_rect(self.background_color, x + VIEW_BORDER, y + VIEW_BORDER, w - VIEW_BORDER * 2, h - VIEW_BORDER * 2);
	}

	let grid_width = vw - VIEW_MARGIN * 2 - VIEW_BORDER * 2;
	let grid_height = vh - DETAIL_VIEW_HEIGHT - fh - VIEW_MARGIN * 4 - VIEW_BORDER * 2;
	let grid_blocks_x = grid_width / gw;
	let grid_blocks_y = grid_height / gh;

	for y in 0..grid_blocks_y {
	    for x in 0..grid_blocks_x {
		let offset = VIEW_MARGIN + VIEW_BORDER;
		let pos_x = offset + x * gw;
		let pos_y = offset + y * gh;
		let pos = Position::new(x, y).add(grid.window_position);

		if let Some(image) = grid.get_image_for_pos(pos) {
		    g.draw_image(&image, pos_x, pos_y);
		} else {
		    let bitmap = bitmap_from_char('.');
		    let color = if y % 4 == 0 && x % 4 == 0 {
			Color::new(255, 255, 255, 128)
		    } else {
			Color::new(255, 255, 255, 64)
		    };
		    let char = Image::from_bitmap(&bitmap, color);
		    g.draw_image(&char, pos_x, pos_y);
		}
	    }
	}
    }

    fn render_detail(&self, g: &mut dyn Graphics) {
        let (vw, vh) = g.get_viewport();
        let (_, fh) = self.font_size;

	if self.active_view == ActiveView::Detail {
	    let x = VIEW_MARGIN;
	    let y = vh - DETAIL_VIEW_HEIGHT - fh - VIEW_MARGIN * 2;
	    let w = vw - VIEW_MARGIN * 2;
	    let h = DETAIL_VIEW_HEIGHT;

	    let mut color = self.select_color;
	    if self.prompt_is_active {
		color.alpha = 128;
	    }

	    g.draw_rect(color, x, y, w, h);
	    g.draw_rect(self.background_color, x + VIEW_BORDER, y + VIEW_BORDER, w - VIEW_BORDER * 2, h - VIEW_BORDER * 2);
	}
    }

    fn render_prompt(&mut self, g: &mut dyn Graphics) {
        let (fw, fh) = self.font_size;
        let (_, vh) = g.get_viewport();
	let x = VIEW_MARGIN;
        let y = vh - fh - VIEW_MARGIN;

	let text_color = if self.prompt_is_active {
	    Color::new(255, 255, 255, 255)
	} else {
	    Color::new(255, 255, 255, 128)
	};
	let bitmap = bitmap_from_char('>');
	let char = Image::from_bitmap(&bitmap, text_color);
	g.draw_image(&char, x, y);

        for (i, c) in self.prompt.chars().enumerate() {
            let bitmap = bitmap_from_char(c);
            let char = Image::from_bitmap(&bitmap, text_color);
            g.draw_image(&char, VIEW_MARGIN + (i as i32 + 1) * fw, y);
        }
    }
}
