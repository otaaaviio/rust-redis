use std::fmt;

#[derive(Debug)]
pub enum AppError {
    InvalidExpirationValue,
    WrongNumberOfArgumentsError,
    InvalidPattern
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InvalidExpirationValue => write!(f, "ERR value is not an integer or out of range"),
            AppError::WrongNumberOfArgumentsError => write!(f, "ERR wrong number of arguments for command"),
            AppError::InvalidPattern => write!(f, "ERR invalid pattern"),
        }
    }
}

impl std::error::Error for AppError {}