use sim::{Color, Graphics, Image};

pub struct WebGraphics {
    canvas: Image,
    scale: usize,
}

impl WebGraphics {
    pub fn new(width: i32, height: i32, scale: usize) -> Self {
        WebGraphics {
            canvas: Image::new(width / scale as i32, height / scale as i32),
            scale,
        }
    }

    pub fn render_image(&self) -> Image {
        let mut image = self.canvas.clone();
        image.scale(self.scale);
        image
    }
}

impl Graphics for WebGraphics {
    fn draw_image(&mut self, image: &Image, x: i32, y: i32) {
        self.canvas.layer(&image, x, y);
    }

    fn draw_rect(&mut self, c: Color, x: i32, y: i32, w: i32, h: i32) {
        let mut rect = Image::new(w, h);
        rect.clear(c);
        self.canvas.layer(&rect, x, y);
    }

    fn get_viewport(&self) -> (i32, i32) {
        (self.canvas.width, self.canvas.height)
    }

    fn clear(&mut self) {
        self.canvas.clear(Color::empty());
    }
}
