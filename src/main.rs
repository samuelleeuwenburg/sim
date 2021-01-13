mod wave;
mod sample;

use std::fs;
use std::convert::Into;
use std::sync::mpsc;

use cpal::{Sample, SampleFormat, StreamConfig, BufferSize};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::SupportedBufferSize;

use crate::wave::{parse_wave, Wave};
use crate::sample::Sample as S;

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("no output device available");

    let mut supported_configs_range = device.supported_output_configs()
        .expect("error while querying configs");

    let supported_config = supported_configs_range.next()
        .expect("no supported config?!")
        .with_sample_rate(cpal::SampleRate(44_100));

    match supported_config.buffer_size() {
        SupportedBufferSize::Range { min, max } => println!("m{} m{}", min, max),
        SupportedBufferSize::Unknown => println!("unknown buffer size support"),
    }

    let sample_format = supported_config.sample_format();

    let mut config: StreamConfig = supported_config.into();
    config.buffer_size = BufferSize::Fixed(1024);
    config.channels = 2;

    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

    let (tx_flag, rx_flag) = mpsc::channel();
    let (tx_buffer, rx_buffer) = mpsc::channel();

    let stream = match sample_format {
        SampleFormat::F32 => {
            println!("F32");

            device.build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {

                    let buffer: [f32;2048] = match rx_buffer.try_recv() {
                        Ok(b) => b,
                        Err(_) => [0.0; 2048],
                    };

                    for (i, sample) in data.iter_mut().enumerate() {
                        let v = buffer.get(i).unwrap().clone();
                        *sample = Sample::from(&v);
                    }

                    tx_flag.send(true).unwrap();

                },
                err_fn
           )
        },
        SampleFormat::I16 => {
            println!("I16");
            device.build_output_stream(&config, write_silence::<i16>, err_fn)
        },
        SampleFormat::U16 => {
            println!("U16");
            device.build_output_stream(&config, write_silence::<u16>, err_fn)
        },
    }.unwrap();

    stream.play().unwrap();

    let file = fs::read("./test_files/sine_mono.wav").unwrap();
    let file = fs::read("./test_files/c-strum.wav").unwrap();

    let wave: Wave = parse_wave(&file).unwrap();
    let mut sample: S = wave.into();

    loop {
        let _ = rx_flag.recv().unwrap();
        let mut buffer: [f32; 2048] = [1.0; 2048];

        let buffer = sample.get_audio(&mut buffer);

        tx_buffer.send(*buffer).unwrap();
    }
}

fn write_silence<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
    for sample in data.iter_mut() {
        *sample = Sample::from(&0.0);
    }
}
