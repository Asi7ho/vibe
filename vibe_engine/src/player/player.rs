use std::{fs::File, time::Duration};

use vibe_core::decoder::Decoder;

use crate::stream::AudioStream;

#[derive(Clone)]
pub struct Player {
    stream: Option<AudioStream>,
    stream_duration: Option<Duration>,
}

impl Player {
    #[inline]
    pub fn new() -> Result<Self, ()> {
        Ok(Self {
            stream: None,
            stream_duration: None,
        })
    }

    pub fn create_stream(&mut self, path: &str) {
        let file = File::open(path).expect("File not found");
        let decoder = Decoder::new(file).expect("Decoding error");

        let info = decoder.info();
        let duration = info.duration();

        self.stream = Some(AudioStream::new::<f32, File>(decoder).unwrap());
        self.stream_duration = duration;
    }

    pub fn stream_duration(&self) -> Option<Duration> {
        self.stream_duration
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
