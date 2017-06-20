use std::error::Error as StdError;
use std::fmt::Error as FmtError;
use std::fmt::{Display, Formatter};
use std::io::Error as IoError;
use hyper::error::Error as HyperError;
use hyper::status::StatusCode;
use serde_json::error::Error as JsonError;

#[derive(Debug)]
pub enum Error {
    Io(IoError),
    Json(JsonError),
    Http(HyperError),
    HttpStatus(StatusCode)
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Error::Io(e)
    }
}

impl From<JsonError> for Error {
    fn from(e: JsonError) -> Self {
        Error::Json(e)
    }
}

impl From<HyperError> for Error {
    fn from(e: HyperError) -> Self {
        Error::Http(e)
    }
}

impl From<StatusCode> for Error {
    fn from(e: StatusCode) -> Self {
        Error::HttpStatus(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match *self {
            Error::Io(ref e) => Display::fmt(e, f),
            Error::Json(ref e) => Display::fmt(e, f),
            Error::Http(ref e) => Display::fmt(e, f),
            Error::HttpStatus(e) => f.write_fmt(format_args!("Unexpected status code: {}", e))
        }
   }
}

impl StdError for Error {
    fn description(&self) -> &str {
        "HipChat client error"
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Io(ref e) => Some(e),
            Error::Json(ref e) => Some(e),
            Error::Http(ref e) => Some(e),
            Error::HttpStatus(_) => None
        }
    }
}
