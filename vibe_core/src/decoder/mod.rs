use std::path::Path;

use crate::AudioInfo;
use crate::Sample;

#[cfg(feature = "mp3")]
mod mp3;
#[cfg(feature = "wav")]
mod wav;

/// Audio decoder
///
/// For now support only mp3
/// In the future will support FLAC, WAV, OGG and MP3
pub struct Decoder {
    decoder: FormatDecoder,
}

impl Decoder {
    /// Used to open an audio file, the right decoder is chosen by looking at the extention of the file
    #[inline]
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, ()> {
        Ok(Self {
            decoder: FormatDecoder::open(path)?,
        })
    }
}

impl Decoder {
    /// Get the info of the file (Format, Sampling rate and Number of channels)
    #[inline]
    pub fn info(&self) -> AudioInfo {
        self.decoder.info()
    }

    /// Return an iterator over the samples
    #[inline]
    pub fn into_samples(self) -> Result<SampleIterator, ()> {
        self.decoder.into_samples()
    }
}
/// Sample iterator to read the decoded samples (the channels are interleaved)
pub struct SampleIterator(Box<dyn Iterator<Item = Result<Sample, ()>>>);

impl Iterator for SampleIterator {
    type Item = Result<Sample, ()>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

/// Choose the right decoder
pub(crate) enum FormatDecoder {
    #[cfg(feature = "wav")]
    Wav(self::wav::WavDecoder),
    #[cfg(feature = "vorbis")]
    Vorbis(self::vorbis::VorbisDecoder),
    #[cfg(feature = "mp3")]
    Mp3(self::mp3::Mp3Decoder),
    #[cfg(feature = "flac")]
    Flac(self::flac::FlacDecoder),
}

impl FormatDecoder {
    #[inline]
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, ()> {
        // Check if feature exists
        macro_rules! get_decoder {
            ($in_ext:expr, $($ext:literal => requires $feature:literal for $init:expr),*) => {
                match $in_ext {
                    $(
                        #[cfg(feature = $feature)]
                        $ext => { return Ok($init) }
                        #[cfg(not(feature = $feature))]
                        $ext => { panic!("Feature not implemented yet") }
                    )*
                    other => panic!("Error: {:?}", other.to_owned())
                }
            }
        }

        // Look at the extention
        if let Some(ext) = path.as_ref().extension().and_then(|ext| ext.to_str()) {
            get_decoder!(ext,
                "wav" => requires "wav" for FormatDecoder::Wav(self::wav::WavDecoder::open(path)?),
                "ogg" => requires "vorbis" for FormatDecoder::Vorbis(self::vorbis::VorbisDecoder::open(path)?),
                "mp3" => requires "mp3" for FormatDecoder::Mp3(self::mp3::Mp3Decoder::open(path)?),
                "flac" => requires "flac" for FormatDecoder::Flac(self::flac::FlacDecoder::open(path)?)
            )
        }
        Err(())
    }

    #[inline]
    pub fn into_samples(self) -> Result<SampleIterator, ()> {
        match self {
            #[cfg(feature = "wav")]
            FormatDecoder::Wav(d) => Ok(SampleIterator(d.into_samples()?)),
            #[cfg(feature = "vorbis")]
            FormatDecoder::Vorbis(d) => Ok(SampleIterator(d.into_samples()?)),
            #[cfg(feature = "mp3")]
            FormatDecoder::Mp3(d) => Ok(SampleIterator(d.into_samples()?)),
            #[cfg(feature = "flac")]
            FormatDecoder::Flac(d) => Ok(SampleIterator(d.into_samples()?)),
        }
    }

    #[inline]
    pub fn info(&self) -> AudioInfo {
        match self {
            #[cfg(feature = "wav")]
            FormatDecoder::Wav(d) => d.info(),
            #[cfg(feature = "vorbis")]
            FormatDecoder::Vorbis(d) => d.info(),
            #[cfg(feature = "mp3")]
            FormatDecoder::Mp3(d) => d.info(),
            #[cfg(feature = "flac")]
            FormatDecoder::Flac(d) => d.info(),
        }
    }
}
