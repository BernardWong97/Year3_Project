//!
//! `AppError` is `App` error handler.
//!
//! # author: Mindaugas Sharskus
//! # date: 25-04-2019
//!

use std::{error, fmt, convert, io};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct AppError{
    err: String,
}

impl AppError {
    pub fn new(message: &str) -> Self {
        Self{ err: message.to_string() }
    }
}

impl error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.err)
    }
}

impl convert::From<io::Error> for AppError {
    fn from(er: io::Error) -> Self {
        Self{ err: er.to_string() }
    }
}

impl convert::From<String> for AppError {
    fn from(err: String) -> Self {
        Self{ err }
    }
}

impl convert::From<&str> for AppError {
    fn from(err: &str) -> Self {
        Self{ err: err.to_string() }
    }
}

impl convert::From<&dyn error::Error> for AppError {
    fn from(err: &error::Error) -> Self {
        Self{ err: err.to_string() }
    }
}