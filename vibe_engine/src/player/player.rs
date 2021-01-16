use std::io::{Read, Seek};

use vibe_core::decoder::Decoder;

use crate::stream::AudioStream;

#[derive(Clone)]
pub struct Player {
    stream: Option<AudioStream>,
}

impl Player {
    #[inline]
    pub fn new() -> Result<Self, ()> {
        Ok(Self { stream: None })
    }

    pub fn create_stream<R>(&mut self, decoder: Decoder<R>)
    where
        R: Read + Seek + Send + 'static,
    {
        self.stream = Some(AudioStream::new::<f32, R>(decoder).unwrap());
    }

    pub fn play_stream(&self) {
        if self.stream.is_some() {
            self.stream.as_ref().unwrap().play();
        }
    }

    pub fn pause_stream(&self) {
        if self.stream.is_some() {
            self.stream.as_ref().unwrap().pause();
        }
    }

    pub fn stop_stream(&self) {
        if self.stream.is_some() {
            self.stream.as_ref().unwrap().stop();
        }
    }
}
