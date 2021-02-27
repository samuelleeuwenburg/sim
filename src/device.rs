use std::sync::mpsc;

use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use cpal::{Sample, SampleFormat, StreamConfig, BufferSize as CpalBufferSize};
use cpal::traits::{StreamTrait, DeviceTrait, HostTrait};
use cpal::SupportedBufferSize;
use crate::stream;
use crate::stream::Stream;

pub fn get_device(buffer_size: usize, channels: usize) -> (mpsc::Receiver<usize>, Arc<Mutex<Stream>>, cpal::Stream) {
    let (tx_buffer_read, rx_buffer_read) = mpsc::channel();

    let sample_rate = 44_100;

    let buffer = Arc::new(Mutex::new(Stream::empty(buffer_size, channels)));
    let buffer_clone = Arc::clone(&buffer);

    let host = cpal::default_host();
    let cpal_device = host.default_output_device().expect("no output device available");

    let mut supported_configs_range = cpal_device.supported_output_configs()
        .expect("error while querying configs");

    let supported_config = supported_configs_range.find(|config| {
	config.channels() == (channels as u16) && config.sample_format() == SampleFormat::F32
    });

    let supported_config = supported_config
	.expect("no supported config")
	.with_sample_rate(cpal::SampleRate(sample_rate as u32));

    let sample_format = supported_config.sample_format();

    let mut config: StreamConfig = supported_config.into();
    config.buffer_size = CpalBufferSize::Fixed((buffer_size / channels) as u32);

    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

    let cpal_stream = match sample_format {
        SampleFormat::F32 => {
            println!("using f32 stream");

            cpal_device.build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
		    let buffer = buffer_clone.lock().unwrap();

		    if data.len() > buffer.samples.len() {
			panic!(
			    "trying to read samples ({}) not available in buffer ({})",
			    data.len(),
			    buffer.samples.len()
			);
		    }

		    for (i, sample) in data.iter_mut().enumerate() {
			let v = match buffer.samples.get(i) {
			    Some(v) => v,
			    None => panic!("failed getting value from buffer @ {}", i),
			};

			*sample = Sample::from(v);
		    }

		    tx_buffer_read.send(data.len()).unwrap();
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

    (rx_buffer_read, buffer, cpal_stream)
}

fn write_silence<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
    for sample in data.iter_mut() {
        *sample = Sample::from(&0.0);
    }
}

