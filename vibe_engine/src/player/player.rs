use std::fs::File;

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

    pub fn play_audio(&mut self, path: &str) {
        if self.stream.is_some() {
            // self.stream.as_ref().unwrap().stop();
            self.stream = None;
        }

        let file = File::open(path).expect("File not found");
        let decoder = Decoder::new(file).expect("Decoding error");

        self.stream = Some(AudioStream::new::<f32, File>(decoder).unwrap());
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
