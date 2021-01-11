use std::fmt::Display;
use std::time::Duration;

/// Information about an opened audio file.
#[derive(Debug, Clone)]
pub struct AudioInfo {
    pub sample_rate: u32,
    pub channels: usize,
    pub format: AudioFormat,
    pub duration: Option<Duration>,
}

impl AudioInfo {
    /// Get the sample rate of the audio.
    #[inline]
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// Get the number of channels in the audio.
    #[inline]
    pub fn channels(&self) -> usize {
        self.channels
    }

    /// Get the original format of the audio.
    #[inline]
    pub fn format(&self) -> AudioFormat {
        self.format
    }

    /// Get the original format of the audio.
    #[inline]
    pub fn duration(&self) -> Option<Duration> {
        self.duration
    }

}

/// Indicates the format of an audio stream.
#[derive(Debug, Copy, Clone)]
pub enum AudioFormat {
    /// WAV format.
    Wav,
    /// Ogg Vorbis format.
    Ogg,
    /// MPEG Layer 3 format.
    Mp3,
    /// FLAC format.
    Flac,
}

impl Display for AudioFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioFormat::Wav => write!(f, "WAV"),
            AudioFormat::Ogg => write!(f, "OGG"),
            AudioFormat::Mp3 => write!(f, "MP3"),
            AudioFormat::Flac => write!(f, "FLAC"),
        }
    }
}
