use std::sync::mpsc;

use cpal::{Sample, SampleFormat, StreamConfig, BufferSize as CpalBufferSize};
use cpal::traits::{StreamTrait, DeviceTrait, HostTrait};
use cpal::SupportedBufferSize;
use crate::stream;
use crate::stream::Stream;

pub struct Device {
    pub tx: mpsc::Sender<Stream>,
    pub stream: Option<cpal::Stream>,
    pub buffer_size: stream::BufferSize,
    pub sample_rate: usize,
    pub channels: usize,
}

impl Device {
    fn new (channels: usize, buffer_size: usize, sample_rate: usize, tx: mpsc::Sender<Stream>) -> Self {
        Device {
            tx,
            channels,
            buffer_size,
            sample_rate,
            stream: None,
        }
    }

    pub fn create_stream(&self) -> Stream {
        Stream::empty(self.buffer_size, self.channels)
    }

    fn attach_cpal_stream(&mut self, cpal_stream: cpal::Stream) {
        self.stream = Some(cpal_stream);
    }
}

pub fn get_device(tx_buffer_ready: mpsc::Sender<bool>) -> Device {
    let (tx, rx_buffer) = mpsc::channel();
    let channels = 2;
    let buffer_size = 1024 * channels;
    let sample_rate = 44_100;

    let mut device = Device::new(channels, buffer_size, sample_rate, tx);

    println!("buffer_size: {} \nchannels: {} \nsample_rate: {}\n", buffer_size, channels, sample_rate);

    let host = cpal::default_host();
    let cpal_device = host.default_output_device().expect("no output device available");

    let mut supported_configs_range = cpal_device.supported_output_configs()
        .expect("error while querying configs");

    let supported_config = supported_configs_range.next()
        .expect("no supported config!")
        .with_sample_rate(cpal::SampleRate(sample_rate as u32));

    match supported_config.buffer_size() {
        SupportedBufferSize::Range { min, max } => println!("minimal buffer size: {} \nmaximum buffer size: {}\n", min, max),
        SupportedBufferSize::Unknown => println!("unknown buffer size support"),
    };

    let sample_format = supported_config.sample_format();

    let mut config: StreamConfig = supported_config.into();

    config.buffer_size = CpalBufferSize::Fixed((buffer_size / channels) as u32);
    config.channels = channels as u16;

    // initial empty buffer
    let buffer = device.create_stream();
    device.tx.send(buffer).unwrap();

    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

    let stream = match sample_format {
        SampleFormat::F32 => {
            println!("using f32 stream");

            cpal_device.build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    let buffer: Stream = match rx_buffer.try_recv() {
                        Ok(b) => b,
                        Err(_) => panic!("no buffer available for playback"),
                    };

                    for (i, sample) in data.iter_mut().enumerate() {
                        let v = match buffer.samples.get(i) {
                            Some(v) => v,
                            None => panic!("failed getting value from buffer @ {}", i),
                        };

                        *sample = Sample::from(v);
                    }

                    tx_buffer_ready.send(true).unwrap();
                },
                err_fn
           )
        },
        SampleFormat::I16 => {
            println!("using i16 stream");
            cpal_device.build_output_stream(&config, write_silence::<i16>, err_fn)
        },
        SampleFormat::U16 => {
            println!("using u16 stream");
            cpal_device.build_output_stream(&config, write_silence::<u16>, err_fn)
        },
    }.unwrap();

    stream.play().unwrap();

    device.attach_cpal_stream(stream);

    device
}

fn write_silence<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
    for sample in data.iter_mut() {
        *sample = Sample::from(&0.0);
    }
}

