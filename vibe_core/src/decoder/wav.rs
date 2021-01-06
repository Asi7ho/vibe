use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use hound::{SampleFormat, WavReader, WavSpec};

use crate::{AudioFormat, AudioInfo, Sample};

pub struct WavDecoder {
    reader: WavReader<BufReader<File>>,
    spec: WavSpec,
}

impl WavDecoder {
    #[inline]
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, ()> {
        let reader = WavReader::open(path).map_err(|_| ())?;
        Ok(Self {
            spec: reader.spec(),
            reader,
        })
    }

    #[inline]
    pub fn info(&self) -> AudioInfo {
        AudioInfo {
            format: AudioFormat::Wav,
            sample_rate: self.spec.sample_rate,
            channels: self.spec.channels as usize,
        }
    }

    pub fn into_samples(self) -> Result<Box<dyn Iterator<Item = Result<Sample, ()>>>, ()> {
        Ok(Box::new(WavSampleIterator {
            decoder: self.reader,
        }))
    }
}

struct WavSampleIterator<R: Read> {
    decoder: WavReader<R>,
}

impl<R: Read> Iterator for WavSampleIterator<R> {
    type Item = Result<Sample, ()>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let spec = self.decoder.spec();
        match (spec.sample_format, spec.bits_per_sample) {
            (SampleFormat::Int, 8) => self
                .decoder
                .samples::<i8>()
                .next()
                .map(|s| s.map(|x| x as f32 / i8::MAX as f32).map_err(|_| ())),
            (SampleFormat::Int, 16) => self
                .decoder
                .samples::<i16>()
                .next()
                .map(|s| s.map(|x| x as f32 / i16::MAX as f32).map_err(|_| ())),
            (SampleFormat::Int, 24) => {
                const MAX_I24: i32 = 0x7fffff;
                self.decoder
                    .samples::<i32>()
                    .next()
                    .map(|s| s.map(|x| x as f32 / MAX_I24 as f32).map_err(|_| ()))
            }
            (SampleFormat::Int, 32) => self
                .decoder
                .samples::<i32>()
                .next()
                .map(|s| s.map(|x| x as f32 / i32::MAX as f32).map_err(|_| ())),
            (SampleFormat::Float, 32) => self.decoder.samples().next().map(|s| s.map_err(|_| ())),
            (other_format, other_bps) => {
                panic!(
                    "Error wav: format '{}-bit {:?}' is not supported",
                    other_bps, other_format
                )
            }
        }
    }
}
