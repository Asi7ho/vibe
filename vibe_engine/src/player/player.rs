use std::io::{Read, Seek};

use vibe_core::decoder::Decoder;

use crate::stream::AudioStream;

#[derive(Clone)]
pub struct Player {
    stream: Option<AudioStream>,
}

impl Player {
    #[inline]
    /// Create a new empty player
    pub fn new() -> Self {
        Self { stream: None }
    }

    #[inline]
    /// Create a new stream inside the player
    pub fn create_stream<R>(&mut self, decoder: Decoder<R>)
    where
        R: Read + Seek + Send + 'static,
    {
        self.stream = Some(AudioStream::new::<f32, R>(decoder));
    }

    #[inline]
    /// Play the stream
    pub fn play_stream(&self) {
        if self.stream.is_some() {
            self.stream.as_ref().unwrap().play();
        }
    }

    #[inline]
    /// Pause the stream
    pub fn pause_stream(&self) {
        if self.stream.is_some() {
            self.stream.as_ref().unwrap().pause();
        }
    }

    #[inline]
    /// Stop the stream
    pub fn stop_stream(&self) {
        if self.stream.is_some() {
            self.stream.as_ref().unwrap().stop();
        }
    }
}
