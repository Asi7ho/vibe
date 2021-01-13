use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, StreamConfig, SupportedStreamConfig,
};
use std::io::{BufReader, Read, Seek};
use vibe_core::decoder::Decoder;

pub struct OutputStream {
    device: Device,
    config: SupportedStreamConfig,
}

impl OutputStream {
    /// Returns a new stream using the default output device.
    pub fn new() -> Result<Self, ()> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("no output device available");

        let config = device.default_output_config().unwrap();

        Ok(Self { device, config })
    }

    pub fn run<T, R>(self, data: R) -> Result<(), ()>
    where
        T: cpal::Sample,
        R: Read + Seek + Send + 'static,
    {
        let mut decoder = Decoder::new(BufReader::new(data)).ok().unwrap();

        let device = self.device;
        let config: StreamConfig = self.config.into();

        let channels = config.channels as usize;
        let info = decoder.info();

        let duration = info.duration().unwrap();

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

        stream.play().map_err(|_| ())?;

        std::thread::sleep(duration);

        Ok(())
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
