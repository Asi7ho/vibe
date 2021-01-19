use std::{error::Error, fmt::Display};

/// An error encountered while decoding an audio file.
#[derive(Debug)]
pub enum DecoderError {
    /// I/O error.
    IOError(std::io::Error),
    /// Error specific to the audio format.
    FormatError(String),
    /// The decoder could not read a complete frame or sample, possibly due to an EOF.
    IncompleteData,
}

impl Error for DecoderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl Display for DecoderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecoderError::IOError(err) => write!(f, "IO error: {}", err),
            DecoderError::FormatError(err) => write!(f, "format error: {}", err),
            DecoderError::IncompleteData => write!(f, "incomplete data"),
        }
    }
}
