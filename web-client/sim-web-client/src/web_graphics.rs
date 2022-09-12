use sim::{Color, Graphics, Image};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub enum RenderInstruction {
    Clear,
    Image(i32, i32, i32, i32, Vec<u8>),
    Rect(i32, i32, i32, i32, u8, u8, u8, u8),
}

pub struct WebGraphics {
    instructions: Vec<RenderInstruction>,
    viewport: (i32, i32),
}

impl WebGraphics {
    pub fn new(width: i32, height: i32) -> Self {

        WebGraphics {
	    instructions: vec![],
	    viewport: (width, height)
	}
    }

    pub fn get_instructions(&self) -> &[RenderInstruction] {
	&self.instructions
    }

    pub fn clear_instructions(&mut self) {
	self.instructions.clear();
    }
}

impl Graphics for WebGraphics {
    fn draw_image(&mut self, image: &Image, x: i32, y: i32) {
	let mut color_data: Vec<u8> = Vec::with_capacity(image.data.len() * 4);

	for color in &image.data {
	    color_data.push(color.red);
	    color_data.push(color.green);
	    color_data.push(color.blue);
	    color_data.push(color.alpha);
	};

	self.instructions.push(RenderInstruction::Image(x, y, image.width, image.height, color_data));
    }

    fn draw_rect(&mut self, c: Color, x: i32, y: i32, w: i32, h: i32) {
	self.instructions.push(RenderInstruction::Rect(x, y, w, h, c.red, c.green, c.blue, c.alpha));
    }

    fn get_viewport(&self) -> (i32, i32) {
	self.viewport
    }

    fn clear(&mut self) {
	self.instructions.push(RenderInstruction::Clear);
    }
}
