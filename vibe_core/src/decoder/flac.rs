use std::io::{Read, Seek, SeekFrom};
use std::time::Duration;

use claxon::FlacReader;

use crate::{AudioFormat, AudioInfo, Sample};

/// Decoder for FLAC files
pub struct FlacDecoder<R>
where
    R: Read + Seek,
{
    reader: FlacReader<R>,
    sample_rate: u32,
    channels: usize,
    duration: Option<Duration>,
    current_block: Vec<i32>,
    current_block_len: usize,
    current_block_channel_len: usize,
    max_sample_value: f32,
    block_cursor: usize,
}

impl<R> FlacDecoder<R>
where
    R: Read + Seek,
{
    /// Open WAV file and create a decoder
    #[inline]
    pub fn new(mut data: R) -> Result<Self, R> {
        if !is_flac(data.by_ref()) {
            return Err(data);
        }

        let reader = FlacReader::new(data).unwrap();

        let spec = reader.streaminfo();

        let sample_rate = reader.streaminfo().sample_rate;
        let channels = reader.streaminfo().channels as _;
        let current_block: Vec<i32> =
            Vec::with_capacity(spec.max_block_size as usize * spec.channels as usize);
        let current_block_len = 0;
        let current_block_channel_len = 1;
        let max_sample_value = (i32::MAX >> (32 - spec.bits_per_sample)) as f32;
        let block_cursor = 0;
        let duration = spec
            .samples
            .map(|s| Duration::from_millis(s * 1_000 / sample_rate as u64));

        Ok(Self {
            reader,
            sample_rate,
            channels,
            duration,
            current_block,
            current_block_len,
            current_block_channel_len,
            max_sample_value,
            block_cursor,
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
            format: AudioFormat::Flac,
            sample_rate: self.sample_rate,
            channels: self.channels,
            duration: self.duration(),
        }
    }
}

impl<R> Iterator for FlacDecoder<R>
where
    R: Read + Seek,
{
    type Item = Result<Sample, ()>;

    /// Iterate over the samples and convert it into f32 sample
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.block_cursor < self.current_block_len {
                let real_cursor = (self.block_cursor % self.channels as usize)
                    * self.current_block_channel_len
                    + self.block_cursor / self.channels as usize;

                let sample_float = self.current_block[real_cursor] as f32 / self.max_sample_value;

                self.block_cursor += 1;
                return Some(Ok(sample_float));
            }

            self.block_cursor = 0;
            let block_buffer = std::mem::replace(&mut self.current_block, vec![]);
            match self.reader.blocks().read_next_or_eof(block_buffer) {
                Ok(Some(block)) => {
                    self.current_block_len = block.len() as _;
                    self.current_block_channel_len = (block.len() / block.channels()) as usize;
                    self.current_block = block.into_buffer();
                }
                _ => return None,
            }
        }
    }
}

/// Returns true if the stream contains Flac data, then resets it to where it was.
fn is_flac<R>(mut data: R) -> bool
where
    R: Read + Seek,
{
    let stream_pos = data.seek(SeekFrom::Current(0)).unwrap();
    let is_flac = FlacReader::new(data.by_ref()).is_ok();
    data.seek(SeekFrom::Start(stream_pos)).unwrap();

    return is_flac;
}
