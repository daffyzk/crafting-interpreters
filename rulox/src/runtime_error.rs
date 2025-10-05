use std::error::Error;
use std::fmt;

use crate::lox::Token;

#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String
}

impl RuntimeError {
    pub fn new(token: Token, message: String) -> Self {
        RuntimeError {
            token,
            message
        } 
    }
}

impl Error for RuntimeError {}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Runtime Error: {}", self.message)
    }
}
