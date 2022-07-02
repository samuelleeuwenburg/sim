use super::grid::{EntitySetting, Grid, Position};
use super::input_state::InputMode;

#[derive(Clone, Copy)]
pub enum Color {
    Rgb(u8, u8, u8),
    Rgba(u8, u8, u8, f32),
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Style {
    Fill,
    Stroke,
}

pub trait Graphics {
    fn clear(&self);
    fn get_viewport(&self) -> (u32, u32);
    fn draw_text(&self, color: Color, x: f32, y: f32, w: f32, h: f32, text: &str);
    fn draw_rect(&self, color: Color, style: Style, x: f32, y: f32, w: f32, h: f32);
}

pub struct DisplayEntity {
    pub position: Position,
    pub text: String,
    pub color: Color,
}

pub struct UserInterface {
    pub cursor: Position,
    pub grid_block_size: (f32, f32),
    pub settings: Option<Vec<EntitySetting>>,
    pub prompt: String,
    pub display_entities: Vec<DisplayEntity>,
    pub input: String,
}

impl UserInterface {
    pub fn new() -> Self {
        UserInterface {
            cursor: Position::new(0, 0),
            grid_block_size: (42.0, 64.0),
            settings: None,
            prompt: String::from(""),
            input: String::from(""),
            display_entities: vec![],
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

    pub fn render(&self, g: &dyn Graphics, grid: &Grid, input_mode: InputMode) {
        g.clear();
        self.render_background(g);
        self.render_grid(g, grid);
        self.render_entities(g, grid);
        self.render_cursor(g, grid);
        self.render_prompt(g, grid, input_mode);
        self.render_mode(g, grid, input_mode);
        self.render_input(g, grid);
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

    fn render_entity(&self, g: &dyn Graphics, grid: &Grid, entity: &DisplayEntity) {
        let (x, y) = self.get_grid_position(g, grid, entity.position.x, entity.position.y);
        let (w, h) = self.grid_block_size;
        g.draw_text(entity.color, x, y, w, h, &entity.text);
    }

    fn render_entities(&self, g: &dyn Graphics, grid: &Grid) {
        for entity in &self.display_entities {
            self.render_entity(g, grid, entity);
        }
    }

    fn render_cursor(&self, g: &dyn Graphics, grid: &Grid) {
        let (x, y) = self.get_grid_position(g, grid, self.cursor.x, self.cursor.y);
        let (w, h) = self.grid_block_size;
        g.draw_text(Color::Rgb(255, 255, 0), x, y, w, h, ".");
    }

    fn render_mode(&self, g: &dyn Graphics, grid: &Grid, input_mode: InputMode) {
        let (x, y) = self.get_grid_position(g, grid, grid.rect.width - 3, grid.rect.height + 2);
        let (w, h) = self.grid_block_size;

        g.draw_text(
            Color::Rgba(0, 255, 255, 1.0),
            x,
            y,
            w,
            h,
            &input_mode.get_prompt(),
        );
    }

    fn render_prompt(&self, g: &dyn Graphics, grid: &Grid, input_mode: InputMode) {
        let (x, y) = self.get_grid_position(g, grid, 0, grid.rect.height + 1);
        let (w, h) = self.grid_block_size;

        g.draw_text(Color::Rgba(0, 255, 255, 1.0), x, y, w, h, &self.prompt);

        if let Some(settings) = &self.settings {
            let mut cursor_x = self.prompt.len() as i32 + 1;
            let mut cursor_y = grid.rect.height + 1;

            for (index, setting) in settings.into_iter().enumerate() {
                let (x, y) = self.get_grid_position(g, grid, cursor_x, cursor_y);

                let value = setting.value.to_string();

                match input_mode {
                    InputMode::Edit(active_index) if index == active_index => {
                        g.draw_rect(
                            Color::Rgb(0, 255, 255),
                            Style::Fill,
                            x,
                            y,
                            w * (value.len() + setting.description.len()) as f32,
                            h,
                        );
                        g.draw_text(Color::Rgba(0, 0, 0, 1.0), x, y, w, h, &setting.description);
                        g.draw_text(
                            Color::Rgba(255, 255, 255, 1.0),
                            x + setting.description.len() as f32 * w,
                            y,
                            w,
                            h,
                            &value,
                        );
                    }
                    _ => {
                        g.draw_text(
                            Color::Rgba(255, 255, 255, 0.3),
                            x,
                            y,
                            w,
                            h,
                            &setting.description,
                        );
                        g.draw_text(
                            Color::Rgba(255, 255, 255, 0.4),
                            x + setting.description.len() as f32 * w,
                            y,
                            w,
                            h,
                            &value,
                        );
                    }
                };

                cursor_x += (setting.description.len() + value.len()) as i32 + 1;
                if cursor_x >= grid.rect.width {
                    cursor_x = 0;
                    cursor_y += 1;
                }
            }
        }
    }

    fn render_input(&self, g: &dyn Graphics, grid: &Grid) {
        let (x, y) = self.get_grid_position(g, grid, 0, grid.rect.height + 2);
        let (w, h) = self.grid_block_size;

        let mut prefix = String::from("> ");
        prefix.push_str(&self.input);

        g.draw_text(Color::Rgba(255, 255, 255, 0.3), x, y, w, h, &prefix);
    }
}
