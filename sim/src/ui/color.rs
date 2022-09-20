#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Color {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn empty() -> Self {
        Color {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 0,
        }
    }

    pub fn full() -> Self {
        Color {
            red: 255,
            green: 255,
            blue: 255,
            alpha: 255,
        }
    }

    pub fn add(&mut self, color: Color) {
        if color.alpha == 255 {
            self.red = color.red;
            self.green = color.green;
            self.blue = color.blue;
            self.alpha = color.alpha;
        } else {
            let a0 = color.alpha as f32 / 255.0;
            let r0 = color.red as f32 / 255.0;
            let g0 = color.green as f32 / 255.0;
            let b0 = color.blue as f32 / 255.0;

            let a1 = self.alpha as f32 / 255.0;
            let r1 = self.red as f32 / 255.0;
            let g1 = self.green as f32 / 255.0;
            let b1 = self.blue as f32 / 255.0;

            let a01 = (1.0 - a0) * a1 + a0;
            let r01 = ((1.0 - a0) * a1 * r1 + a0 * r0) / a01;
            let g01 = ((1.0 - a0) * a1 * g1 + a0 * g0) / a01;
            let b01 = ((1.0 - a0) * a1 * b1 + a0 * b0) / a01;

            self.red = (r01 * 255.0) as u8;
            self.green = (g01 * 255.0) as u8;
            self.blue = (b01 * 255.0) as u8;
            self.alpha = (a01 * 255.0) as u8;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut color = Color::new(255, 0, 0, 255);
        color.add(Color::new(0, 255, 0, 255));

        assert_eq!(color, Color::new(0, 255, 0, 255));

        let mut color = Color::new(0, 255, 0, 127);
        color.add(Color::new(255, 0, 0, 127));

        assert_eq!(color, Color::new(169, 85, 0, 190));
    }
}
