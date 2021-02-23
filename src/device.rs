use std::sync::mpsc;

use cpal::{Sample, SampleFormat, StreamConfig, BufferSize as CpalBufferSize};
use cpal::traits::{StreamTrait, DeviceTrait, HostTrait};
use cpal::SupportedBufferSize;
use crate::stream;
use crate::stream::Stream;

pub struct Device {
    device_stream: Option<cpal::Stream>,
    stream: Stream,
    pub channels: usize,
    pub tx_buffer: mpsc::Sender<Stream>,
    rx_buffer_read: mpsc::Receiver<usize>,
}

impl Device {
    fn new (
	buffer_size: usize,
	channels: usize,
	tx_buffer: mpsc::Sender<Stream>,
	rx_buffer_read: mpsc::Receiver<usize>,
    ) -> Self {
        Device {
            device_stream: None,
            channels,
            tx_buffer,
	    rx_buffer_read,
	    stream: Stream::empty(buffer_size * channels, channels)
        }
    }

    fn attach_cpal_stream(&mut self, cpal_stream: cpal::Stream) {
        self.device_stream = Some(cpal_stream);
    }

    pub fn buffer_size(&mut self) -> usize {
	match self.rx_buffer_read.try_recv() {
	    Ok(bytes) => {
		// drop read bytes
		if self.stream.samples.len() < bytes {
		    panic!("tying to drain empty buffer!");
		}

		self.stream.samples.drain(0..bytes);

		bytes
	    }
	    Err(_) => 0,
	}
    }

    pub fn send_buffer(&mut self, stream: Stream) -> Result<(), mpsc::SendError<Stream>> {
	self.stream.samples.extend(stream.samples);
	self.tx_buffer.send(self.stream.clone())
    }
}

pub fn get_device() -> Device {
    let (tx_buffer_read, rx_buffer_read) = mpsc::channel();
    let (tx_buffer, rx_buffer) = mpsc::channel();

    let channels = 2;
    let buffer_size = 1024 * channels;
    let cpal_buffer_size = 512;
    let sample_rate = 44_100;

    let mut device = Device::new(buffer_size, channels, tx_buffer, rx_buffer_read);

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
    config.buffer_size = CpalBufferSize::Fixed(cpal_buffer_size);

    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

    let stream = match sample_format {
        SampleFormat::F32 => {
            println!("using f32 stream");

            cpal_device.build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    let buffer: Stream = match rx_buffer.try_recv() {
                        Ok(b) => b,
                        Err(_) => Stream::empty(data.len(), 1),
                    };

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

    stream.play().unwrap();

    device.attach_cpal_stream(stream);

    device
}

fn write_silence<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
    for sample in data.iter_mut() {
        *sample = Sample::from(&0.0);
    }
}

