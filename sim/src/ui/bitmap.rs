pub struct Bitmap {
    pub width: i32,
    pub height: i32,
    /// one byte represents 8 pixels on the grid, each bit being a pixel
    pub data: Vec<u8>
}

impl Bitmap {
    pub fn new(width: i32, height: i32, data: &[u8]) -> Self {
        Bitmap {
            width,
            height,
            data: data.into(),
        }
    }

    pub fn empty(width: i32, height: i32) -> Self {
        Bitmap {
            width,
            height,
            data: Vec::with_capacity((width / 8 * height) as usize),
        }
    }
}
