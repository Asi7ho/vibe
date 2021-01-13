use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Stream, StreamConfig,
};
use std::{
    fs::File,
    io::{Read, Seek},
    time::Duration,
};
use vibe_core::decoder::Decoder;

pub struct OutputStream {
    stream: Stream,
    duration: Option<Duration>,
}

impl OutputStream {
    /// Returns a new stream using the default output device.
    pub fn new<T>(mut decoder: Decoder<File>) -> Result<Self, ()>
    where
        T: cpal::Sample,
    {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("no output device available");

        let config = device.default_output_config().unwrap();
        let config: StreamConfig = config.into();

        let channels = config.channels as usize;

        let info = decoder.info();
        let duration = info.duration();

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                    write_data(data, channels, decoder.by_ref())
                },
                err_fn,
            )
            .map_err(|_| ())?;

        Ok(Self { stream, duration })
    }

    pub fn play(self) {
        self.stream.play().unwrap();
        std::thread::sleep(self.duration.unwrap());
    }

    pub fn pause(self) {
        self.stream.pause().unwrap();
    }
}

fn write_data<T, R>(output: &mut [T], channels: usize, decoder: &mut Decoder<R>)
where
    T: cpal::Sample,
    R: Read + Seek,
{
    for frame in output.chunks_mut(channels) {
        let value = &decoder.next().unwrap().expect("error decoding value");
        let value: T = cpal::Sample::from::<f32>(value);
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
