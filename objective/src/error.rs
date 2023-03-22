use std::fmt;

#[derive(Debug)]
pub struct AttributeError {
    message: String,
}

impl AttributeError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl fmt::Display for AttributeError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "AttributeError: {}", self.message)
    }
}

#[derive(Debug)]
pub struct IndexError {
    message: String,
}

impl IndexError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl fmt::Display for IndexError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "IndexError: {}", self.message)
    }
}

#[derive(Debug)]
pub struct TypeError {
    message: String,
}

impl TypeError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl fmt::Display for TypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "TypeError: {}", self.message)
    }
}
