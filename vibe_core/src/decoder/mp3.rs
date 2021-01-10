use minimp3::{Decoder, Error, Frame};
use std::{fs::File, io::Read, path::Path};

use crate::{AudioFormat, AudioInfo, Sample};

///Decoder for MP3 files
pub struct Mp3Decoder {
    decoder: Decoder<File>,
    first_frame: Frame,
    sample_rate: u32,
    channels: usize,
}

impl Mp3Decoder {
    /// Open MP3 file and create a decoder
    #[inline]
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, ()> {
        let f = File::open(path).map_err(|_| ())?;

        let mut decoder = Decoder::new(f);

        let first_frame = decoder.next_frame().map_err(|_| ())?;
        let sample_rate = first_frame.sample_rate as u32;
        let channels = first_frame.channels;

        Ok(Mp3Decoder {
            decoder,
            first_frame,
            sample_rate,
            channels,
        })
    }

    /// Get the info
    #[inline]
    pub fn info(&self) -> AudioInfo {
        AudioInfo {
            format: AudioFormat::Mp3,
            sample_rate: self.sample_rate,
            channels: self.channels,
        }
    }

    /// Create iterator over the samples
    #[inline]
    pub fn into_samples(self) -> Result<Box<dyn Iterator<Item = Result<Sample, ()>>>, ()> {
        Ok(Box::new(Mp3SampleIterator {
            decoder: self.decoder,
            channels: self.channels,
            sample_rate: self.sample_rate,
            current_frame: self.first_frame,
            frame_cursor: 0,
        }))
    }
}

struct Mp3SampleIterator<R: Read> {
    decoder: Decoder<R>,
    channels: usize,
    sample_rate: u32,
    current_frame: Frame,
    frame_cursor: usize,
}

impl<R: Read> Iterator for Mp3SampleIterator<R> {
    type Item = Result<Sample, ()>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Read next frame in if current frame is exhausted
        if self.frame_cursor >= self.current_frame.data.len() {
            self.frame_cursor = 0;
            self.current_frame = loop {
                match self.decoder.next_frame() {
                    Ok(frame) => {
                        // Skip empty frames
                        if frame.data.len() == 0 {
                            continue;
                        }

                        // Make sure the sample rates and channels match
                        assert_eq!(frame.sample_rate as u32, self.sample_rate);
                        assert_eq!(frame.channels, self.channels);

                        break frame;
                    }
                    Err(Error::SkippedData) => continue,
                    Err(Error::Eof) => return None,
                    Err(e) => panic!("Error: {:?}", e),
                }
            };
        }

        let sample_float = self.current_frame.data[self.frame_cursor] as f32 / i16::MAX as f32;
        self.frame_cursor += 1;
        Some(Ok(sample_float))
    }
}
