use minimp3::{Decoder, Error, Frame};
use std::io::{Read, Seek, SeekFrom};
use std::time::Duration;

use crate::{AudioFormat, AudioInfo, Sample};

///Decoder for MP3 files
pub struct Mp3Decoder<R>
where
    R: Read + Seek,
{
    decoder: Decoder<R>,
    channels: usize,
    sample_rate: u32,
    duration: Option<Duration>,
    current_frame: Frame,
    frame_cursor: usize,
}

impl<R> Mp3Decoder<R>
where
    R: Read + Seek,
{
    /// Open MP3 file and create a decoder
    #[inline]
    pub fn new(mut data: R) -> Result<Self, R> {
        if !is_mp3(data.by_ref()) {
            return Err(data);
        }

        let duration = compute_duration(data.by_ref());

        let mut decoder = Decoder::new(data);

        let current_frame = decoder.next_frame().unwrap();
        let sample_rate = current_frame.sample_rate as u32;
        let channels = current_frame.channels;
        let frame_cursor = 0;

        Ok(Mp3Decoder {
            decoder,
            channels,
            sample_rate,
            duration,
            current_frame,
            frame_cursor,
        })
    }

    /// Get duration audio file
    #[inline]
    fn duration(&self) -> Option<Duration> {
        self.duration
    }

    /// Get the info
    #[inline]
    pub fn info(&self) -> AudioInfo {
        AudioInfo {
            format: AudioFormat::Mp3,
            sample_rate: self.sample_rate,
            channels: self.channels,
            duration: self.duration(),
        }
    }
}

impl<R> Iterator for Mp3Decoder<R>
where
    R: Read + Seek,
{
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

fn is_mp3<R>(mut data: R) -> bool
where
    R: Read + Seek,
{
    let stream_pos = data.seek(SeekFrom::Current(0)).unwrap();
    let is_mp3 = Decoder::new(data.by_ref()).next_frame().is_ok();
    data.seek(SeekFrom::Start(stream_pos)).unwrap();

    return is_mp3;
}

/// Compute duration
fn compute_duration<R>(mut data: R) -> Option<Duration>
where
    R: Read + Seek,
{
    let stream_pos = data.seek(SeekFrom::Current(0)).unwrap();

    let mut decoder = Decoder::new(data.by_ref());

    let mut duration: u64 = 0;
    loop {
        match decoder.next_frame() {
            Ok(Frame {
                data,
                sample_rate,
                channels,
                ..
            }) => duration += data.len() as u64 * 1_000 / (channels as u64 * sample_rate as u64),
            Err(Error::Eof) => break,
            Err(e) => panic!("{:?}", e),
        }
    }

    data.seek(SeekFrom::Start(stream_pos)).unwrap();
    return Some(Duration::from_millis(duration));
}
