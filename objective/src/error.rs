#[derive(Debug)]
pub enum Error {
    AccessError(String),
    TypeError(String),
    AttributeError(String),
    IndexError(String),
    ValueError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", match self {
            Error::AccessError(message) => message,
            Error::TypeError(message) => message,
            Error::AttributeError(message) => message,
            Error::IndexError(message) => message,
            Error::ValueError(message) => message,
        })
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
