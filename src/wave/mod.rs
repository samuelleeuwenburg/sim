use std::convert::TryInto;

#[derive(Debug, PartialEq)]
enum ChunkID {
    RIFF,
    FMT,
    LIST,
    DATA,
}

struct WaveFormat {
    sample_rate: u32,
    bit_depth: u16,
    channels: u16,
}

struct Wave {
    format: WaveFormat,
    data: Vec<usize>,
}

fn parse_chunk_id(id: [u8;4]) -> Result<ChunkID, &'static str> {
    match id {
        [b'R', b'I', b'F', b'F'] => Ok(ChunkID::RIFF),
        [b'f', b'm', b't', b' '] => Ok(ChunkID::FMT),
        [b'L', b'I', b'S', b'T'] => Ok(ChunkID::LIST),
        [b'd', b'a', b't', b'a'] => Ok(ChunkID::DATA),
        _ => Err("unknown chunk id"),
    }
}

fn parse_fmt_chunk(bytes: &[u8]) -> Result<WaveFormat, &'static str> {
    let channels = bytes[2..4]
        .try_into()
        .map_err(|_| "can't read num of channels")
        .map(|b| u16::from_le_bytes(b))?;

    let sample_rate = bytes[4..8]
        .try_into()
        .map_err(|_| "can't read sample rate")
        .map(|b| u32::from_le_bytes(b))?;

    let bit_depth = bytes[14..16]
        .try_into()
        .map_err(|_| "can't read bits per sample")
        .map(|b| u16::from_le_bytes(b))?;

    Ok(WaveFormat { channels, sample_rate, bit_depth })
}

fn parse_data_chunk(bytes: &[u8]) -> Result<Vec<usize>, &'static str> {
    unimplemented!();
}

fn parse_chunks(bytes: &[u8]) -> Result<Wave, &'static str> {
    let mut pos = 0;

    let mut format = Err("no fmt chunk found");
    let mut data = Err("no data chunk found");

    loop {
        if pos + 8 > bytes.len() {
            break;
        }

        let chunk_id = bytes[pos..pos + 4]
            .try_into()
            .map_err(|_| "can't read chunk id")
            .and_then(|b| parse_chunk_id(b))?;

        let chunk_size = bytes[pos + 4..pos + 8]
            .try_into()
            .map_err(|_| "can't read chunk size")
            .map(|b| u32::from_le_bytes(b))?;

        let start = pos + 8;
        let end = pos + 8 + chunk_size as usize;

        println!("{:?} {}", chunk_id, chunk_size);

        match chunk_id {
            ChunkID::FMT => format = parse_fmt_chunk(&bytes[start..end]),
            ChunkID::DATA => data = parse_data_chunk(&bytes[start..end]),
            _ => (),
        };

        pos += chunk_size as usize + 8;
    }

    let wave = Wave {
        format: format?,
        data: data?,
    };

    Ok(wave)
}

fn parse_wave_file(bytes: &[u8]) -> Result<(), &'static str> {
    let riff = bytes[0..4]
        .try_into()
        .map_err(|_| "can't read chunk id")
        .and_then(|b| parse_chunk_id(b))?;

    let file_size = bytes[4..8]
        .try_into()
        .map_err(|_| "can't read chunk size")
        .map(|b| u32::from_le_bytes(b))?;

    if riff != ChunkID::RIFF {
        return Err("no RIFF chunk id found at the start of file")
    }

    parse_chunks(&bytes[12..file_size as usize + 8])?;

    Err("incomplete implementation")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_wave_file() {
        let bytes = fs::read("test_files/sine_mono.wav").unwrap();

        parse_wave_file(&bytes);

        assert_eq!(true, false);
    }
}

