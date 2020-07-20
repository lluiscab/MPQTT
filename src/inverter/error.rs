use std::fmt::{self, Display};
use std::num::ParseIntError;
use std::str::Utf8Error;

/// Inverter result type alias.
pub type Result<T> = std::result::Result<T, Error>;

/// Inverter errors.
#[derive(Debug)]
pub enum Error {
    /// The command does not start with '(' or does not end with the checksum followed by '\r'.
    InvalidResponseFormat,
    /// The checksum does not match.
    InvalidResponseCheckSum,
    /// The payload format is invalid.
    InvalidResponsePayload,

    // Standard errors' encapsulations.
    Utf8(Utf8Error),
    Io(std::io::Error),
    ParseInt(ParseIntError),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Forward to debug print.
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<Utf8Error> for Error {
    fn from(error: Utf8Error) -> Self {
        Self::Utf8(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Self::ParseInt(error)
    }
}
