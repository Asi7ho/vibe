use std::io::{Read, Seek, SeekFrom};
use std::time::Duration;

use hound::{Error, SampleFormat, WavReader, WavSpec};

use crate::{info::DecoderError, AudioFormat, AudioInfo, Sample};

/// Decoder for WAV files
pub struct WavDecoder<R>
where
    R: Read + Seek,
{
    reader: WavReader<R>,
    spec: WavSpec,
}

impl<R> WavDecoder<R>
where
    R: Read + Seek,
{
    /// Open WAV file and create a decoder
    #[inline]
    pub fn new(mut data: R) -> Result<Self, R> {
        if !is_wav(data.by_ref()) {
            return Err(data);
        }

        let reader = WavReader::new(data).unwrap();
        let spec = reader.spec();

        Ok(Self { reader, spec })
    }

    /// Get duration audio file
    #[inline]
    fn duration(&self) -> Option<Duration> {
        let ms = self.reader.len() as u64 * 1000
            / (self.spec.channels as u64 * self.spec.sample_rate as u64);
        Some(Duration::from_millis(ms))
    }

    /// Get the info
    #[inline]
    pub fn info(&self) -> AudioInfo {
        AudioInfo {
            format: AudioFormat::Wav,
            sample_rate: self.spec.sample_rate,
            channels: self.spec.channels as usize,
            duration: self.duration(),
        }
    }
}

impl<R> Iterator for WavDecoder<R>
where
    R: Read + Seek,
{
    type Item = Result<Sample, DecoderError>;

    /// Get the next sample and convert it into f32 sample
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let spec = self.reader.spec();
        match (spec.sample_format, spec.bits_per_sample) {
            (SampleFormat::Int, 8) => self
                .reader
                .samples::<i8>()
                .next()
                .map(|s| s.map(|x| x as f32 / i8::MAX as f32).map_err(get_error)),
            (SampleFormat::Int, 16) => self
                .reader
                .samples::<i16>()
                .next()
                .map(|s| s.map(|x| x as f32 / i16::MAX as f32).map_err(get_error)),
            (SampleFormat::Int, 24) => {
                const MAX_I24: i32 = 0x7fffff;
                self.reader
                    .samples::<i32>()
                    .next()
                    .map(|s| s.map(|x| x as f32 / MAX_I24 as f32).map_err(get_error))
            }
            (SampleFormat::Int, 32) => self
                .reader
                .samples::<i32>()
                .next()
                .map(|s| s.map(|x| x as f32 / i32::MAX as f32).map_err(get_error)),
            (SampleFormat::Float, 32) => self
                .reader
                .samples::<f32>()
                .next()
                .map(|s| s.map_err(get_error)),
            (other_format, other_bps) => {
                panic!(
                    "wav: format '{}-bit {:?}' is not supported",
                    other_bps, other_format
                )
            }
        }
    }
}

/// Returns true if the stream contains Flac data, then resets it to where it was.
fn is_wav<R>(mut data: R) -> bool
where
    R: Read + Seek,
{
    let stream_pos = data.seek(SeekFrom::Current(0)).unwrap();
    let is_wav = WavReader::new(data.by_ref()).is_ok();
    data.seek(SeekFrom::Start(stream_pos)).unwrap();

    return is_wav;
}

fn get_error(error: Error) -> DecoderError {
    match error {
        Error::IoError(io_err) => DecoderError::IOError(io_err),
        Error::FormatError(fmt_err) => DecoderError::FormatError(format!("wav: {}", fmt_err)),
        Error::Unsupported => DecoderError::FormatError("wav: unsupported format".to_owned()),
        Error::InvalidSampleFormat => {
            DecoderError::FormatError("wav: invalid sample format".to_owned())
        }
        Error::TooWide => DecoderError::FormatError(
            "wav: decoded samples are too wide for destination type".to_owned(),
        ),
        _ => unreachable!(),
    }
}
