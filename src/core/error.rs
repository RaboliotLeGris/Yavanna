use std::fmt;

#[derive(Debug)]
pub enum YavannaError{
    InvalidHour
}

impl std::error::Error for YavannaError {}

impl fmt::Display for YavannaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            YavannaError::InvalidHour => write!(f, "Not a valid time"),
        }
    }
}