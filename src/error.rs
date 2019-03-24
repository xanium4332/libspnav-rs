use std::{error, fmt};
use std::os::raw::c_int;

#[derive(Debug)]
pub enum Error {
    /// Generic error from the underlying C library
    GenericError(c_int),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::GenericError(i) => write!(f, "Generic error ({})", i),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

pub type Result<T> = std::result::Result<T, Error>;
