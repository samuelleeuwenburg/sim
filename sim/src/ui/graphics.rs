pub trait Graphics {
    fn clear(&self);
    fn get_viewport(&self) -> (u32, u32);
    fn draw_text(&self, color: Color, x: f32, y: f32, w: f32, h: f32, text: &str);
    fn draw_rect(&self, color: Color, style: Style, x: f32, y: f32, w: f32, h: f32);
}
