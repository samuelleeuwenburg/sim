use crate::app::user_interface::{Color, Graphics, Style};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub struct WebGraphics {
    pub canvas: HtmlCanvasElement,
    pub ctx: CanvasRenderingContext2d,
}

impl WebGraphics {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        canvas.set_width(800);
        canvas.set_height(640);

        WebGraphics { ctx, canvas }
    }

    fn set_fill_style(&self, color: &Color) {
        let value = match color {
            Color::RGB(r, g, b) => format!("rgb({}, {}, {})", r, g, b),
            Color::RGBA(r, g, b, a) => format!("rgba({}, {}, {}, {})", r, g, b, a),
        };

        self.ctx.set_fill_style(&JsValue::from_str(&value));
        self.ctx.set_stroke_style(&JsValue::from_str(&value));
    }
}

impl Graphics for WebGraphics {
    fn draw_text(&self, color: Color, x: f32, y: f32, text: &str) {
        self.set_fill_style(&color);
        self.ctx.fill_text(text, x as f64, y as f64).unwrap();
    }

    fn draw_rect(&self, color: Color, style: Style, x: f32, y: f32, w: f32, h: f32) {
        self.set_fill_style(&color);

        match style {
            Style::Fill => self.ctx.fill_rect(x as f64, y as f64, w as f64, h as f64),
            Style::Stroke => self.ctx.stroke_rect(x as f64, y as f64, w as f64, h as f64),
        };
    }

    fn get_viewport(&self) -> (u32, u32) {
        (self.canvas.width(), self.canvas.height())
    }

    fn clear(&self) {
        self.ctx.clear_rect(
            0.0,
            0.0,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        );
    }
}
