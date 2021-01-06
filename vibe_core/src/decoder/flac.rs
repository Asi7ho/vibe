use std::{fs::File, io::Read, path::Path};

use claxon::FlacReader;

use crate::{AudioFormat, AudioInfo, Sample};

/// Decoder for FLAC files
pub struct FlacDecoder {
    reader: FlacReader<File>,
    sample_rate: u32,
    channels: usize,
}

impl FlacDecoder {
    /// Open WAV file and create a decoder
    #[inline]
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, ()> {
        let reader = FlacReader::open(path).map_err(|_| ())?;

        let sample_rate = reader.streaminfo().sample_rate;
        let channels = reader.streaminfo().channels as _;

        Ok(Self {
            reader: reader,
            sample_rate: sample_rate,
            channels: channels,
        })
    }

    /// Get the info
    #[inline]
    pub fn info(&self) -> AudioInfo {
        AudioInfo {
            format: AudioFormat::Flac,
            sample_rate: self.sample_rate,
            channels: self.channels,
        }
    }

    /// Create iterator over the samples
    #[inline]
    pub fn into_samples(self) -> Result<Box<dyn Iterator<Item = Result<Sample, ()>>>, ()> {
        let spec = self.reader.streaminfo();

        let current_block: Vec<i32> =
            Vec::with_capacity(spec.max_block_size as usize * spec.channels as usize);
        let max_sample_value = (i32::MAX >> (32 - spec.bits_per_sample)) as f32;

        Ok(Box::new(FlacSampleIterator {
            decoder: self.reader,
            current_block: current_block,
            current_block_len: 0,
            max_sample_value: max_sample_value,
            block_cursor: 0,
        }))
    }
}

struct FlacSampleIterator<R: Read> {
    decoder: FlacReader<R>,
    current_block: Vec<i32>,
    current_block_len: usize,
    max_sample_value: f32,
    block_cursor: usize,
}

impl<R: Read> Iterator for FlacSampleIterator<R> {
    type Item = Result<Sample, ()>;

    /// Iterate over the samples and convert it into f32 sample
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.block_cursor < self.current_block_len {
                let sample_float =
                    self.current_block[self.block_cursor] as f32 / self.max_sample_value;
                self.block_cursor += 1;
                return Some(Ok(sample_float));
            }

            self.block_cursor = 0;
            let block_buffer = std::mem::replace(&mut self.current_block, vec![]);
            match self.decoder.blocks().read_next_or_eof(block_buffer) {
                Ok(Some(block)) => {
                    self.current_block_len = block.len() as _;
                    self.current_block = block.into_buffer();
                }
                _ => return None,
            }
        }
    }
}
