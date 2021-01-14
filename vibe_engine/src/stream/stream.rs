use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Stream, StreamConfig,
};
use crossbeam::channel::{unbounded, Sender};
use std::io::{Read, Seek};
use vibe_core::decoder::Decoder;

#[derive(Debug)]
pub enum Controls {
    Pause,
    Play,
    Stop,
}

pub struct AudioStream {
    tx_stream: Sender<Controls>,
}

impl AudioStream {
    /// Returns a new stream using the default output device.
    pub fn new<T, R>(decoder: Decoder<R>) -> Result<Self, ()>
    where
        T: cpal::Sample,
        R: Read + Seek + Send + 'static,
    {
        let (tx, rx) = unbounded();

        // let info = decoder.info();
        // let duration = info.duration().unwrap();

        std::thread::spawn(move || {
            let stream = create_stream::<T, R>(decoder);

            stream.play().unwrap();

            while let Ok(res) = rx.recv() {
                match res {
                    Controls::Pause => {
                        println!("Pause");
                        stream.pause().unwrap();
                    }
                    Controls::Play => {
                        println!("Play");
                        stream.play().unwrap();
                    }
                    Controls::Stop => {
                        println!("Stop");
                        stream.pause().unwrap();
                        drop(stream);
                        break;
                    }
                }
            }
        });

        Ok(Self { tx_stream: tx })
    }

    pub fn play(&self) {
        self.tx_stream.send(Controls::Play).unwrap();
    }

    pub fn pause(&self) {
        self.tx_stream.send(Controls::Pause).unwrap()
    }

    pub fn stop(&self) {
        self.tx_stream.send(Controls::Stop).unwrap()
    }
}

fn write_data<T, R>(output: &mut [T], channels: usize, decoder: &mut Decoder<R>)
where
    T: cpal::Sample,
    R: Read + Seek,
{
    for frame in output.chunks_mut(channels) {
        let value = &decoder.next().unwrap().expect("Error value");
        let value: T = cpal::Sample::from::<f32>(value);
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}

fn create_stream<T, R>(mut decoder: Decoder<R>) -> Stream
where
    T: cpal::Sample,
    R: Read + Seek + Send + 'static,
{
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");

    println!("Device: {:?}", device.name().unwrap());

    let config = device.default_output_config().unwrap();
    let config: StreamConfig = config.into();

    let channels = 1;

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                write_data(data, channels, decoder.by_ref())
            },
            err_fn,
        )
        .unwrap();

    stream
}
