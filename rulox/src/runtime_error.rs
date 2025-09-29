use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct RuntimeError {
    message: String
}

impl Error for RuntimeError {}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Runtime Error: {}", self.message)
    }
}
