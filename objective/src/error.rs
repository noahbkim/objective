#[derive(Debug)]
pub enum Error {
    AccessError(String),
    TypeError(String),
    AttributeError(String),
    IndexError(String),
    ValueError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessError(message) => write!(f, "AccessError: {}", message),
            Error::TypeError(message) => write!(f, "TypeError: {}", message),
            Error::AttributeError(message) => write!(f, "AttributeError: {}", message),
            Error::IndexError(message) => write!(f, "IndexError: {}", message),
            Error::ValueError(message) => write!(f, "ValueError: {}", message),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
