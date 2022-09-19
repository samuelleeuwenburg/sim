use super::{Color, Bitmap};

#[derive(Clone, Default)]
pub struct Image {
    pub width: i32,
    pub height: i32,
    pub data: Vec<Color>,
}

impl Image {
    pub fn new(width: i32, height: i32) -> Self {
        Image {
            width,
            height,
            data: vec![Color::empty(); (width * height) as usize],
        }
    }


    pub fn from_bitmap(bitmap: &Bitmap, color: Color) -> Self {
	let mut data = Vec::with_capacity(bitmap.data.len() * 8);

	for byte in &bitmap.data {
	    for i in 0..8 {
		if byte >> (7 - i)  & 0b1 == 0b1 {
		    // if the byte is positive set the color of the bitmap
		    data.push(color);
		} else {
		    // for empty bytes set a transparant pixel
		    data.push(Color::empty());
		}
	    }
	}

	Image {
	    width: bitmap.width,
	    height: bitmap.height,
	    data,
	}
    }

    pub fn layer(&mut self, image: &Image, x: i32, y: i32) {
	for (i, &color) in image.data.iter().enumerate() {
	    let i = i as i32;

	    let offset = x + y * self.width;
	    let row = (i / image.width) * self.width;
	    let column = i % image.width;

	    // only if the position is in the current image buffer add the colors
	    if let Some(c) = self.data.get_mut((offset + row + column) as usize) {
		c.add(color);
	    }
	}
    }

    pub fn enlarge(&mut self, factor: usize) {
	let mut data = Vec::with_capacity(self.data.len() * factor);
	self.height *= factor as i32;
	self.width *= self.width * factor as i32;

	for (i, color) in data.iter_mut().enumerate() {
	    *color = *self.data.get(i / factor).unwrap();
	}

	self.data = data;
    }

    pub fn clear(&mut self, color: Color) {
	for c in self.data.iter_mut() {
	    *c = color;
	}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_from_bitmap() {
	let bitmap = Bitmap::new(8, 2, &[
	    0b10101110,
	    0b01010111,
	]);

	let image = Image::from_bitmap(&bitmap, Color::full());

	assert_eq!(image.width, 8);
	assert_eq!(image.height, 2);

	assert_eq!(image.data, vec![
	    Color::full(),
	    Color::empty(),
	    Color::full(),
	    Color::empty(),
	    Color::full(),
	    Color::full(),
	    Color::full(),
	    Color::empty(),
	    Color::empty(),
	    Color::full(),
	    Color::empty(),
	    Color::full(),
	    Color::empty(),
	    Color::full(),
	    Color::full(),
	    Color::full(),
	]);
    }

    #[test]
    fn test_layer() {
	let mut a = Image::new(4, 4);
	let mut b = Image::new(2, 2);

	a.clear(&Color::new(255, 0, 0, 255));
	b.clear(&Color::new(0, 255, 0, 255));

	a.layer(&b, 1, 1);

	assert_eq!(a.data, vec![
	    Color::new(255, 0, 0, 255), Color::new(255, 0, 0, 255), Color::new(255, 0, 0, 255), Color::new(255, 0, 0, 255),
	    Color::new(255, 0, 0, 255), Color::new(0, 255, 0, 255), Color::new(0, 255, 0, 255), Color::new(255, 0, 0, 255),
	    Color::new(255, 0, 0, 255), Color::new(0, 255, 0, 255), Color::new(0, 255, 0, 255), Color::new(255, 0, 0, 255),
	    Color::new(255, 0, 0, 255), Color::new(255, 0, 0, 255), Color::new(255, 0, 0, 255), Color::new(255, 0, 0, 255),
	]);
    }
}
