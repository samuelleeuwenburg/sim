use std::convert::TryInto;

#[derive(Debug, PartialEq)]
enum ChunkID {
    RIFF,
    FMT,
    LIST,
    DATA,
}

#[derive(Debug, Clone)]
struct WaveFormat {
    sample_rate: u32,
    bit_depth: u16,
    num_channels: u16,
}

#[derive(Debug)]
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
    let num_channels = bytes[2..4]
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

    Ok(WaveFormat { num_channels, sample_rate, bit_depth })
}

fn parse_data_chunk(format: &WaveFormat, bytes: &[u8]) -> Result<Vec<usize>, &'static str> {
    let sample_bytes = (format.bit_depth / 8) as usize;
    let slice_bytes = sample_bytes * (format.num_channels as usize);

    let mut pos = 0;
    let mut samples = vec![];

    loop {
        if pos + slice_bytes > bytes.len() {
            break;
        }

        for channel in 0..(format.num_channels as usize) {
            let offset = channel * sample_bytes;
            let start = offset + pos;
            let end = offset + pos + sample_bytes;

            println!("{}:{}", start, end);

            let sample = match format.bit_depth {
                8 => bytes[start..end].try_into().map(|b| u8::from_le_bytes(b) as usize),
                16 => bytes[start..end].try_into().map(|b| u16::from_le_bytes(b) as usize),
                _ => bytes[start..end].try_into().map(|b| u32::from_le_bytes(b) as usize),
            }.map_err(|_| "couldn't parse sample")?;

            samples.push(sample);
        }

        pos += slice_bytes;
    }

    Ok(samples)
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
            ChunkID::DATA => data = parse_data_chunk(&format.clone()?, &bytes[start..end]),
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

fn parse_wave_file(bytes: &[u8]) -> Result<Wave, &'static str> {
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

    parse_chunks(&bytes[12..file_size as usize + 8])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_wave_file_16_bit_stereo() {
        let bytes: [u8; 60] = [
            0x52, 0x49, 0x46, 0x46, // RIFF
            0x34, 0x00, 0x00, 0x00, // chunk size
            0x57, 0x41, 0x56, 0x45, // WAVE

            0x66, 0x6d, 0x74, 0x20, // fmt_
            0x10, 0x00, 0x00, 0x00, // chunk size
            0x01, 0x00,             // audio format
            0x02, 0x00,             // num channels
            0x22, 0x56, 0x00, 0x00, // sample rate
            0x88, 0x58, 0x01, 0x00, // byte rate
            0x04, 0x00,             // block align
            0x10, 0x00,             // bits per sample

            0x64, 0x61, 0x74, 0x61, // data
            0x10, 0x00, 0x00, 0x00, // chunk size
            0x00, 0x00, 0x00, 0x00, // sample 1 L+R
            0x24, 0x17, 0x1e, 0xf3, // sample 2 L+R
            0x3c, 0x13, 0x3c, 0x14, // sample 3 L+R
            0x16, 0xf9, 0x18, 0xf9, // sample 4 L+R
        ];

        let wave = parse_wave_file(&bytes).unwrap();

        assert_eq!(wave.format.sample_rate, 22050);
        assert_eq!(wave.format.bit_depth, 16);
        assert_eq!(wave.format.num_channels, 2);

        assert_eq!(wave.data, [
            0x0000, 0x0000, // sample 1 L+R
            0x1724, 0xf31e, // sample 2 L+R
            0x133c, 0x143c, // sample 3 L+R
            0xf916, 0xf918, // sample 4 L+R
        ]);
    }

    #[test]
    fn test_parse_wave_file_24_bit_mono() {
        let bytes: [u8; 60] = [
            0x52, 0x49, 0x46, 0x46, // RIFF
            0x34, 0x00, 0x00, 0x00, // chunk size
            0x57, 0x41, 0x56, 0x45, // WAVE

            0x66, 0x6d, 0x74, 0x20, // fmt_
            0x10, 0x00, 0x00, 0x00, // chunk size
            0x01, 0x00,             // audio format
            0x01, 0x00,             // num channels
            0x44, 0xac, 0x00, 0x00, // sample rate
            0x88, 0x58, 0x01, 0x00, // byte rate
            0x04, 0x00,             // block align
            0x20, 0x00,             // bits per sample

            0x64, 0x61, 0x74, 0x61, // data
            0x10, 0x00, 0x00, 0x00, // chunk size
            0x00, 0x00, 0x00, 0x00, // sample 1
            0x24, 0x17, 0x1e, 0xf3, // sample 2
            0x3c, 0x13, 0x3c, 0x14, // sample 3
            0x16, 0xf9, 0x18, 0xf9, // sample 4
        ];

        let wave = parse_wave_file(&bytes).unwrap();

        assert_eq!(wave.format.sample_rate, 44100);
        assert_eq!(wave.format.bit_depth, 32);
        assert_eq!(wave.format.num_channels, 1);

        assert_eq!(wave.data, [
            0x00000000, // sample 1
            0xf31e1724, // sample 2
            0x143c133c, // sample 3
            0xf918f916, // sample 4
        ]);
    }
}

