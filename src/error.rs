//! Custom `Error` and `Result` types.

use std::fmt;

use std::result::Result as StdResult;

use std::error::Error as StdError;
use std::io::Error as IoError;
use hyper::Error as HttpError;
use serde_json::error::Error as FormatError;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(IoError),
    Http(HttpError),
    Format(FormatError),
    Raw(String)
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref error) => error.description(),
            Error::Http(ref error) => error.description(),
            Error::Format(ref error) => error.description(),
            Error::Raw(ref description) => description.as_ref()
        }
    }
    
    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Io(ref error) => Some(error),
            Error::Http(ref error) => Some(error),
            Error::Format(ref error) => Some(error),
            _ => None
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Self {
        Error::Io(error)
    }
}

impl From<HttpError> for Error {
    fn from(error: HttpError) -> Self {
        Error::Http(error)
    }
}

impl From<FormatError> for Error {
    fn from(error: FormatError) -> Self {
        Error::Format(error)
    }
}

impl From<String> for Error {
    fn from(description: String) -> Self {
        Error::Raw(description)
    }
}