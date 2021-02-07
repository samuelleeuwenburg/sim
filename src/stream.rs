pub type BufferSize = usize;
pub type Point = f32;
pub type Stream = Vec<Point>;

pub fn get_stream(size: BufferSize) -> Stream {
    vec![0.0; size]
}

pub fn combine_streams(streams: Vec<Stream>) -> Stream {
    let size = streams.iter().max_by(|x, y| x.len().cmp(&y.len())).unwrap().len();

    get_stream(size)
        .iter()
        .enumerate()
        .map(|(i, _)| streams.iter().fold(0.0, |xs, x| xs + x.get(i).unwrap_or(&0.0)))
        .collect()
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

#[cfg(test)]
mod tests {
    #![allow(overflowing_literals)]
    use super::*;

    #[test]
    fn test_combine_streams() {
        assert_eq!(
            combine_streams(vec![
                vec![-1.0, -0.5, 0.0, 0.5, 1.0],
            ]),
                vec![-1.0, -0.5, 0.0, 0.5, 1.0],
        );

        assert_eq!(
            combine_streams(vec![
                vec![1.0, 0.2, 1.0, 1.0, 0.2],
                vec![0.0, 0.0, 0.0, 0.0, 0.0],
            ]),
                vec![1.0, 0.2, 1.0, 1.0, 0.2],
        );

        assert_eq!(
            combine_streams(vec![
                vec![0.1, 0.0, -0.1, -0.2, -0.3],
                vec![0.2, 0.1, 0.0, -0.1, -0.2],
                vec![0.3, 0.2, 0.1, 0.0, -0.1],
            ]),
                vec![0.6, 0.3, 0.0, -0.3, -0.6],
        );

        assert_eq!(
            combine_streams(vec![
                vec![0.1, 0.0, -0.1, -0.2, -0.3],
                vec![0.2, 0.1, 0.0],
                vec![0.3],
            ]),
                vec![0.6, 0.1, -0.1, -0.2, -0.3],
        );
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
