use super::{Bitmap, Color};

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
                if byte >> (7 - i) & 0b1 == 0b1 {
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

    pub fn clear(&mut self, color: Color) {
        for c in self.data.iter_mut() {
            *c = color;
        }
    }

    pub fn scale(&mut self, factor: usize) {
        let height = self.height as usize * factor;
        let width = self.width as usize * factor;
        let new_length = width * height;
        let mut data = vec![Color::empty(); new_length];

        for (i, color) in self.data.iter().enumerate() {
            let offset_column = i * factor % width;
            let offset_row = (i / self.width as usize) * width * factor;
            let offset = offset_column + offset_row;

            for r in 0..factor {
                let row = r * width;
                for column in 0..factor {
                    data[offset + row + column] = *color;
                }
            }
        }

        self.width = width as i32;
        self.height = height as i32;
        self.data = data;
    }

    #[allow(dead_code)]
    pub fn to_ascii(&self) -> String {
        let mut ascii = String::from("");

        for color in self.data.iter() {
            if color == &Color::full() {
                ascii.push_str("1");
            } else if color == &Color::empty() {
                ascii.push_str("0");
            } else {
                ascii.push_str("?");
            }
        }

        ascii
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_scale_2() {
        let mut image = Image::new(1, 1);
        image.data[0] = Color::full();
        image.scale(2);

        assert_eq!(image.data.len(), 4);
        assert_eq!(
            &image.to_ascii(),
            "11\
	     11\
	    "
        );

        image.data[0] = Color::empty();
        image.data[1] = Color::full();
        image.data[2] = Color::full();
        image.data[3] = Color::empty();

        assert_eq!(
            &image.to_ascii(),
            "01\
	     10\
	    "
        );

        image.scale(2);
        assert_eq!(image.data.len(), 16);
        assert_eq!(
            &image.to_ascii(),
            "0011\
	     0011\
	     1100\
	     1100\
	    "
        );
    }

    #[test]
    fn test_image_scale_4() {
        let mut image = Image::new(2, 2);
        image.data[0] = Color::full();
        image.data[1] = Color::empty();
        image.data[2] = Color::empty();
        image.data[3] = Color::full();

        assert_eq!(
            &image.to_ascii(),
            "10\
	     01\
	    "
        );

        image.scale(4);

        assert_eq!(image.data.len(), 64);
        assert_eq!(
            &image.to_ascii(),
            "11110000\
	     11110000\
	     11110000\
	     11110000\
	     00001111\
	     00001111\
	     00001111\
	     00001111\
	    "
        );
    }

    #[test]
    fn test_image_from_bitmap() {
        let bitmap = Bitmap::new(8, 2, &[0b10101110, 0b01010111]);

        let image = Image::from_bitmap(&bitmap, Color::full());

        assert_eq!(image.width, 8);
        assert_eq!(image.height, 2);

        assert_eq!(
            &image.to_ascii(),
            "10101110\
	     01010111\
	    "
        );
    }

    #[test]
    fn test_layer() {
        let mut a = Image::new(4, 4);
        let mut b = Image::new(2, 2);

        a.clear(Color::new(255, 0, 0, 255));
        b.clear(Color::new(0, 255, 0, 255));

        a.layer(&b, 1, 1);

        assert_eq!(
            a.data,
            vec![
                Color::new(255, 0, 0, 255),
                Color::new(255, 0, 0, 255),
                Color::new(255, 0, 0, 255),
                Color::new(255, 0, 0, 255),
                Color::new(255, 0, 0, 255),
                Color::new(0, 255, 0, 255),
                Color::new(0, 255, 0, 255),
                Color::new(255, 0, 0, 255),
                Color::new(255, 0, 0, 255),
                Color::new(0, 255, 0, 255),
                Color::new(0, 255, 0, 255),
                Color::new(255, 0, 0, 255),
                Color::new(255, 0, 0, 255),
                Color::new(255, 0, 0, 255),
                Color::new(255, 0, 0, 255),
                Color::new(255, 0, 0, 255),
            ]
        );
    }
}
