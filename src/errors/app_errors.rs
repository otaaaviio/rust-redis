use std::fmt;
use std::io::Error;

#[derive(Debug)]
pub enum AppError {
    InvalidExpirationValue,
    WrongNumberOfArgumentsError,
    InvalidPattern,
    FileError(Error),
    InvalidFileFormat,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InvalidExpirationValue => write!(f, "ERR value is not an integer or out of range"),
            AppError::WrongNumberOfArgumentsError => write!(f, "ERR wrong number of arguments for command"),
            AppError::InvalidPattern => write!(f, "ERR invalid pattern"),
            AppError::FileError(e) => write!(f, "ERR file error: {}", e),
            AppError::InvalidFileFormat => write!(f, "ERR invalid file format"),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::FileError(e) => Some(e),
            _ => None,
        }
    }
}