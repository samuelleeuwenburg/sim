use std::sync::mpsc;

use cpal::{Sample, SampleFormat, StreamConfig, BufferSize};
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::SupportedBufferSize;

pub fn get_stream(tx_buffer_ready: mpsc::Sender<bool>) -> (mpsc::Sender<[f32;2048]>, cpal::Stream) {
    let (tx_buffer, rx_buffer) = mpsc::channel();

    let host = cpal::default_host();
    let device = host.default_output_device().expect("no output device available");

    let mut supported_configs_range = device.supported_output_configs()
        .expect("error while querying configs");

    let supported_config = supported_configs_range.next()
        .expect("no supported config!")
        .with_sample_rate(cpal::SampleRate(44_100));

    match supported_config.buffer_size() {
        SupportedBufferSize::Range { min, max } => println!("minimal buffer size: {} \nmaximum buffer size: {}", min, max),
        SupportedBufferSize::Unknown => println!("unknown buffer size support"),
    }

    let sample_format = supported_config.sample_format();

    let mut config: StreamConfig = supported_config.into();
    println!("using buffer size: 1024");
    config.buffer_size = BufferSize::Fixed(1024);
    println!("number of channels: 2");
    config.channels = 2;

    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

    let stream = match sample_format {
        SampleFormat::F32 => {
            println!("using f32 stream");

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

                    tx_buffer_ready.send(true).unwrap();
                },
                err_fn
           )
        },
        SampleFormat::I16 => {
            println!("using i16 stream");
            device.build_output_stream(&config, write_silence::<i16>, err_fn)
        },
        SampleFormat::U16 => {
            println!("using u16 stream");
            device.build_output_stream(&config, write_silence::<u16>, err_fn)
        },
    }.unwrap();

    (tx_buffer, stream)
}

fn write_silence<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
    for sample in data.iter_mut() {
        *sample = Sample::from(&0.0);
    }
}

