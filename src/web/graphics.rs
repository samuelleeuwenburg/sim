use crate::app::UserInterface;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub struct Graphics {
    pub canvas: HtmlCanvasElement,
    pub ctx: CanvasRenderingContext2d,
}

impl Graphics {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        canvas.set_width(800);
        canvas.set_height(640);

        Graphics { ctx, canvas }
    }

    pub fn render(&self, ui: &UserInterface) {
        let ctx = &self.ctx;

        let pos_x = ui.cursor.x;
        let pos_y = ui.cursor.y;

        let (grid_width, grid_height) = ui.grid_size;

        let height = self.canvas.height();
        let width = self.canvas.width();

        let block_width = 18;
        let block_height = 24;

        let origin_x = width / 2 - (grid_width * block_width) / 2;
        let origin_y = height / 2 - (grid_height * block_height) / 2;

        ctx.clear_rect(0.0, 0.0, width.into(), height.into());

        // render grid
        for x in 0..grid_width {
            for y in 0..grid_height {
                let px_x = origin_x + (x * block_width);
                let px_y = origin_y + (y * block_height);

                if x as i32 == pos_x && y as i32 == pos_y.into() {
                    ctx.fill_rect(
                        px_x.into(),
                        px_y.into(),
                        block_width.into(),
                        block_height.into(),
                    );
                }

                ctx.stroke_rect(
                    px_x.into(),
                    px_y.into(),
                    block_width.into(),
                    block_height.into(),
                );
            }
        }

        //// render oscillators
        //let state = unsafe { STATE.as_ref().unwrap() };
        //ctx.set_font("18px sans-serif");
        //ctx.set_text_baseline("hanging");

        //for vco in state.oscillators.iter() {
        //    let pos = vco.get_position();
        //    let px_x = origin_x + (pos.x as u32 * block_width);
        //    let px_y = origin_y + (pos.y as u32 * block_height);

        //    ctx.fill_text(&vco.get_display(), px_x.into(), px_y.into())
        //        .unwrap();
        //}

        //// render prompt
        //let prompt_y = origin_y + block_height * grid_height;
        //ctx.fill_text(&state.prompt, origin_x.into(), prompt_y.into())
        //    .unwrap();

        //// render input
        //let input_y = origin_y + block_height * (grid_height + 1);
        //let mut prefix = String::from("> ");
        //let input_string: String = input_state
        //    .input_buffer
        //    .clone()
        //    .iter()
        //    .map(|&digit| char::from_u32(digit).unwrap_or('?'))
        //    .collect();

        //prefix.push_str(&input_string);

        //ctx.fill_text(&prefix, origin_x.into(), input_y.into())
        //    .unwrap();
    }
}
