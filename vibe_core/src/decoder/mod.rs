use std::io::{Read, Seek};

use crate::{info::DecoderError, AudioInfo, Sample};

#[cfg(feature = "flac")]
mod flac;
#[cfg(feature = "mp3")]
mod mp3;
#[cfg(feature = "vorbis")]
mod ogg;
#[cfg(feature = "wav")]
mod wav;

/// Audio decoder
///
/// For now support only FLAC, WAV, OGG and MP3
pub struct Decoder<R>
where
    R: Read + Seek,
{
    decoder: FormatDecoder<R>,
}

impl<R> Decoder<R>
where
    R: Read + Seek,
{
    /// Used to open an audio file, the right decoder is chosen by looking at the extention of the file
    #[inline]
    pub fn new(data: R) -> Result<Self, R> {
        Ok(Self {
            decoder: FormatDecoder::new(data)?,
        })
    }

    #[inline]
    pub fn info(&self) -> AudioInfo {
        self.decoder.info()
    }
}

impl<R> Iterator for Decoder<R>
where
    R: Read + Seek,
{
    type Item = Result<Sample, DecoderError>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.decoder.next()
    }
}

/// Choose the right decoder
pub(crate) enum FormatDecoder<R>
where
    R: Read + Seek,
{
    #[cfg(feature = "wav")]
    Wav(self::wav::WavDecoder<R>),
    #[cfg(feature = "vorbis")]
    Vorbis(self::ogg::VorbisDecoder<R>),
    #[cfg(feature = "mp3")]
    Mp3(self::mp3::Mp3Decoder<R>),
    #[cfg(feature = "flac")]
    Flac(self::flac::FlacDecoder<R>),
}

impl<R> FormatDecoder<R>
where
    R: Read + Seek,
{
    #[inline]
    pub fn new(data: R) -> Result<Self, R> {
        #[cfg(feature = "wav")]
        let data = match self::wav::WavDecoder::new(data) {
            Ok(decoder) => {
                return Ok(FormatDecoder::Wav(decoder));
            }
            Err(err) => err,
        };

        #[cfg(feature = "flac")]
        let data = match self::flac::FlacDecoder::new(data) {
            Ok(decoder) => {
                return Ok(FormatDecoder::Flac(decoder));
            }
            Err(err) => err,
        };

        #[cfg(feature = "vorbis")]
        let data = match self::ogg::VorbisDecoder::new(data) {
            Ok(decoder) => {
                return Ok(FormatDecoder::Vorbis(decoder));
            }
            Err(err) => err,
        };

        #[cfg(feature = "mp3")]
        let data = match self::mp3::Mp3Decoder::new(data) {
            Ok(decoder) => {
                return Ok(FormatDecoder::Mp3(decoder));
            }
            Err(err) => err,
        };

        Err(data)
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

impl<R> Iterator for FormatDecoder<R>
where
    R: Read + Seek,
{
    type Item = Result<Sample, DecoderError>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            #[cfg(feature = "wav")]
            FormatDecoder::Wav(d) => d.next(),
            #[cfg(feature = "vorbis")]
            FormatDecoder::Vorbis(d) => d.next(),
            #[cfg(feature = "flac")]
            FormatDecoder::Flac(d) => d.next(),
            #[cfg(feature = "mp3")]
            FormatDecoder::Mp3(d) => d.next(),
        }
    }
}
