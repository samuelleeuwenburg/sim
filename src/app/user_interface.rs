use super::grid::{GridPosition, Rect};

pub enum Color {
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, f32),
}

pub enum Style {
    Fill,
    Stroke,
}

pub trait Graphics {
    fn clear(&self);
    fn get_viewport(&self) -> (u32, u32);
    fn draw_text(&self, color: Color, x: f32, y: f32, text: &str);
    fn draw_rect(&self, color: Color, style: Style, x: f32, y: f32, w: f32, h: f32);
}

pub struct DisplayEntity {
    pub position: GridPosition,
    pub text: String,
}

pub struct UserInterface {
    pub cursor: GridPosition,
    pub grid: Rect,
    pub grid_block_size: (f32, f32),
    pub prompt: String,
    pub display_entities: Vec<DisplayEntity>,
    pub input: String,
}

impl UserInterface {
    pub fn new() -> Self {
        UserInterface {
            cursor: GridPosition::new(0, 0),
            grid: Rect::new(33, 17, GridPosition::new(0, 0)),
            grid_block_size: (42.0, 64.0),
            prompt: String::from(""),
            input: String::from(""),
            display_entities: vec![],
        }
    }

    fn get_grid_position(&self, g: &dyn Graphics, x: i32, y: i32) -> (f32, f32) {
        let (width, height) = g.get_viewport();

        let (grid_width, grid_height) = self.grid.size;
        let (block_width, block_height) = self.grid_block_size;

        let origin_x = width as f32 / 2.0 - (grid_width as f32 * block_width) / 2.0;
        let origin_y = height as f32 / 2.0 - (grid_height as f32 * block_height) / 2.0;

        let px_x = origin_x + (x as f32 * block_width);
        let px_y = origin_y + (y as f32 * block_height);

        (px_x, px_y)
    }

    pub fn render(&self, g: &dyn Graphics) {
        g.clear();
        self.render_background(g);
        self.render_grid(g);
        self.render_entities(g);
        self.render_cursor(g);
        self.render_prompt(g);
        self.render_input(g);
    }

    fn render_background(&self, g: &dyn Graphics) {
        let (width, height) = g.get_viewport();

        g.draw_rect(
            Color::RGB(0, 0, 0),
            Style::Fill,
            0.0,
            0.0,
            width as f32,
            height as f32,
        );
    }

    fn render_grid(&self, g: &dyn Graphics) {
        let (grid_width, grid_height) = self.grid.size;

        for x in 0..grid_width {
            for y in 0..grid_height {
                let (px_x, px_y) = self.get_grid_position(g, x as i32, y as i32);
                let spacing = 4;

                if x % spacing == 0 && y % spacing == 0 {
                    g.draw_text(Color::RGBA(255, 255, 255, 0.2), px_x, px_y, ".");
                } else {
                    g.draw_text(Color::RGBA(255, 255, 255, 0.1), px_x, px_y, ".");
                }
            }
        }
    }

    fn render_entity(&self, g: &dyn Graphics, entity: &DisplayEntity) {
        let (x, y) = self.get_grid_position(g, entity.position.x, entity.position.y);
        g.draw_text(Color::RGB(255, 255, 255), x, y, &entity.text);
    }

    fn render_entities(&self, g: &dyn Graphics) {
        for entity in &self.display_entities {
            self.render_entity(g, entity);
        }
    }

    fn render_cursor(&self, g: &dyn Graphics) {
        let (x, y) = self.get_grid_position(g, self.cursor.x, self.cursor.y);
        g.draw_text(Color::RGB(255, 255, 0), x, y, ".");
    }

    fn render_prompt(&self, g: &dyn Graphics) {
        let (_, grid_height) = self.grid.size;
        let (x, y) = self.get_grid_position(g, 0, grid_height + 1);

        g.draw_text(Color::RGBA(255, 255, 255, 0.5), x, y, &self.prompt);
    }

    fn render_input(&self, g: &dyn Graphics) {
        let (_, grid_height) = self.grid.size;
        let (x, y) = self.get_grid_position(g, 0, grid_height + 2);

        let mut prefix = String::from("> ");
        prefix.push_str(&self.input);

        g.draw_text(Color::RGBA(255, 255, 255, 0.3), x, y, &prefix);
    }
}
