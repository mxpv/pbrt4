use std::num::{ParseFloatError, ParseIntError};

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    /// No more tokens.
    #[error("No tokens")]
    EndOfFile,

    /// Expected a token, but received `None`.
    #[error("Token expected, got end of stream")]
    NoToken,

    /// Token didn't pass basic validation checks.
    #[error("Invalid token")]
    InvalidToken,

    /// Failed to parse string to float.
    #[error("Unable to parse float")]
    ParseFloat(#[from] ParseFloatError),

    /// Failed to parse string to integer.
    #[error("Unable to parse number")]
    ParseInt(#[from] ParseIntError),

    /// Directive is unknown.
    #[error("Unsupported directive")]
    UnknownDirective,

    #[error("Expected string token")]
    InvalidString,

    /// Failed to parse option's `[ value ]`
    #[error("Unable to parse option value")]
    InvalidOptionValue,

    #[error("Unsupported coordinate system")]
    UnknownCoordinateSystem,
}
