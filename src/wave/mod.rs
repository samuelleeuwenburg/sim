use std::convert::TryInto;

fn parse_wave_file(bytes: Vec<u8>) -> Result<(), &'static str> {
    let riff: u32 = bytes[0..4]
        .try_into()
        .map_err(|_| "can't read RIFF ")
        .map(|b| u32::from_le_bytes(b))?
        .swap_bytes();

    Err("incomplete implementation")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_wave_file() {
        let bytes = fs::read("test_files/sine_mono.wav").unwrap();

        parse_wave_file(bytes);

        assert_eq!(true, false);
    }
}

