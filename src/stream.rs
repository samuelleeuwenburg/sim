pub type BufferSize = usize;

pub type Point = f32;

#[derive(Debug, PartialEq, Clone)]
pub struct Stream {
    pub samples: Vec<Point>,
    pub channels: usize,
}

pub fn u8_to_point(n: u8) -> Point {
    (n as f32 / u8::MAX as f32) * 2.0 - 1.0
}

pub fn i16_to_point(n: i16) -> Point {
    n as f32 / i16::MAX as f32
}

pub fn i32_to_point(n: i32) -> Point {
    n as f32 / i32::MAX as f32
}

impl Stream {
    pub fn empty(size: BufferSize, channels: usize) -> Self {
        Stream { samples: vec![0.0; size], channels }
    }

    pub fn from_samples(samples: Vec<Point>, channels: usize) -> Self {
        Stream { samples, channels }
    }

    pub fn mix(mut self, streams: &Vec<Stream>) -> Self {
        for (i, sample) in self.samples.iter_mut().enumerate() {
            *sample = streams.iter().fold(sample.clone(), |xs, x| xs + x.samples.get(i).unwrap_or(&0.0));
        }

        self
    }

    pub fn _amplify(mut self, db: f32) -> Self {
        let ratio = 10_f32.powf(db / 20.0);

        for sample in self.samples.iter_mut() {
            *sample = (sample.clone() * ratio).clamp(-1.0, 1.0);
        }

        self
    }
}


#[cfg(test)]
mod tests {
    #![allow(overflowing_literals)]
    use super::*;

    #[test]
    fn test_mix() {
        let samples = vec![-1.0, -0.5, 0.0, 0.5, 1.0];
        let streams = vec![];
        let stream = Stream::from_samples(samples, 1).mix(&streams);
        assert_eq!(stream.samples, vec![-1.0, -0.5, 0.0, 0.5, 1.0]);

        let samples = vec![1.0, 0.2, 1.0, 1.0, 0.2];
        let streams = vec![Stream::from_samples(vec![0.0, 0.0, 0.0, 0.0, 0.0], 1)];
        let stream = Stream::from_samples(samples, 1).mix(&streams);
        assert_eq!(stream.samples, vec![1.0, 0.2, 1.0, 1.0, 0.2]);

        let samples = vec![0.1, 0.0, -0.1, -0.2, -0.3];
        let streams = vec![
            Stream::from_samples(vec![0.2, 0.1, 0.0, -0.1, -0.2], 1),
            Stream::from_samples(vec![0.3, 0.2, 0.1, 0.0, -0.1], 1),
        ];
        let stream = Stream::from_samples(samples, 1).mix(&streams);
        assert_eq!(stream.samples, vec![0.6, 0.3, 0.0, -0.3, -0.6]);

        let samples = vec![0.1, 0.0, -0.1, -0.2, -0.3];
        let streams = vec![
            Stream::from_samples(vec![0.2, 0.1, 0.0], 1),
            Stream::from_samples(vec![0.3], 1),
        ];
        let stream = Stream::from_samples(samples, 1).mix(&streams);
        assert_eq!(stream.samples, vec![0.6, 0.1, -0.1, -0.2, -0.3]);
    }

    #[test]
    fn test_amplify() {
        let stream = Stream::empty(1, 1).amplify(6.0);
        assert_eq!(stream.samples, vec![0.0]);

        // 6 dBs should roughly double / half
        let stream = Stream::from_samples(vec![0.1, 0.25, 0.3, -0.1, -0.4], 1).amplify(6.0);
        let rounded_samples: Vec<Point> = stream.samples.iter().map(|x| (x * 10.0).round() / 10.0).collect::<Vec<Point>>();
        assert_eq!(rounded_samples, vec![0.2, 0.5, 0.6, -0.2, -0.8]);

        let stream = Stream::from_samples(vec![0.4, 0.5, 0.8, -0.3, -0.6], 1).amplify(-6.0);
        let rounded_samples: Vec<Point> = stream.samples.iter().map(|x| (x * 100.0).round() / 100.0).collect::<Vec<Point>>();
        assert_eq!(rounded_samples, vec![0.2, 0.25, 0.4, -0.15, -0.3]);

        // clamp the value
        let stream = Stream::from_samples(vec![0.1, 0.4, 0.6, -0.2, -0.3, -0.5], 1).amplify(12.0);
        let rounded_samples: Vec<Point> = stream.samples.iter().map(|x| (x * 100.0).round() / 100.0).collect::<Vec<Point>>();
        assert_eq!(rounded_samples, vec![0.4, 1.0, 1.0, -0.8, -1.0, -1.0]);
    }


    #[test]
    fn test_u8_to_point() {
        assert_eq!(u8_to_point(u8::MIN), -1.0);
        assert_eq!(u8_to_point(0x80u8), 0.003921628);
        assert_eq!(u8_to_point(u8::MAX), 1.0);
    }

    #[test]
    fn test_i16_to_point() {
        assert_eq!(i16_to_point(i16::MIN + 1), -1.0);
        assert_eq!(i16_to_point(0i16), 0.0);
        assert_eq!(i16_to_point(i16::MAX), 1.0);
    }

    #[test]
    fn test_i32_to_point() {
        assert_eq!(i32_to_point(i32::MIN + 1), -1.0);
        assert_eq!(i32_to_point(0i32), 0.0);
        assert_eq!(i32_to_point(i32::MAX), 1.0);
    }
}
