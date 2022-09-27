pub mod ascii;

use ascii::{ASCII, UNKNOWN};
pub const CHARACTER_WIDTH: usize = 8;
pub const CHARACTER_HEIGHT: usize = 8;
use crate::Bitmap;

pub fn bitmap_from_char(c: char) -> Bitmap {
    let keycode = c as usize;
    if keycode >= 32 {
        let start = (keycode - 32) * CHARACTER_HEIGHT;
        let end = start + CHARACTER_HEIGHT;
        Bitmap::new(
            CHARACTER_WIDTH as i32,
            CHARACTER_HEIGHT as i32,
            &ASCII[start..end],
        )
    } else {
        Bitmap::new(CHARACTER_WIDTH as i32, CHARACTER_HEIGHT as i32, &UNKNOWN)
    }
}

// pub fn render_text() -> Image {
// }
