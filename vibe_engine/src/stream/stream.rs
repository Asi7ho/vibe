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

#[derive(Clone)]
pub struct AudioStream {
    tx_stream: Sender<Controls>,
}

impl AudioStream {
    #[inline]
    /// Returns a new thread containing a stream.
    pub fn new<T, R>(decoder: Decoder<R>) -> Result<Self, ()>
    where
        T: cpal::Sample,
        R: Read + Seek + Send + 'static,
    {
        let (tx, rx) = unbounded();

        std::thread::spawn(move || {
            let stream = create_stream::<T, R>(decoder);

            stream.pause().unwrap();

            println!("Wait for reception");
            while let Ok(res) = rx.recv() {
                println!("{:?}", res);
                match res {
                    Controls::Pause => {
                        stream.pause().unwrap();
                    }
                    Controls::Play => {
                        stream.play().unwrap();
                    }
                    Controls::Stop => {
                        stream.pause().unwrap();
                        drop(stream);
                        break;
                    }
                }
            }
        });

        Ok(Self { tx_stream: tx })
    }

    #[inline]
    /// Send Play command
    pub fn play(&self) {
        self.tx_stream.send(Controls::Play).unwrap();
    }

    #[inline]
    /// Send Pause command
    pub fn pause(&self) {
        self.tx_stream.send(Controls::Pause).unwrap()
    }

    #[inline]
    /// Send Stop command
    pub fn stop(&self) {
        self.tx_stream.send(Controls::Stop).unwrap()
    }
}

fn write_data<T, R>(output: &mut [T], channels: usize, ended: &mut bool, decoder: &mut Decoder<R>)
where
    T: cpal::Sample,
    R: Read + Seek,
{
    for frame in output.chunks_mut(channels) {
        let value = &decoder.next();

        if (*value).is_none() {
            *ended = true;
        }

        if *ended {
            let value: T = cpal::Sample::from::<f32>(&0.0);
            for sample in frame.iter_mut() {
                *sample = value;
            }
        } else {
            let value = &value.unwrap().expect("Error value");
            let value: T = cpal::Sample::from::<f32>(value);
            for sample in frame.iter_mut() {
                *sample = value;
            }
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

    let mut ended = false;

    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                write_data(data, channels, &mut ended, decoder.by_ref())
            },
            err_fn,
        )
        .unwrap();

    println!("Stream built");
    stream
}
