use std::fmt::{self, Debug, Formatter};
use std::io;
use std::num::ParseFloatError;
use std::result;

use self::Error::*;

pub type Result<T> = result::Result<T, Error>;

pub enum Error {
    Io(io::Error),
    ParseFloat(ParseFloatError),
    UnknownChar(char),
}

impl Debug for Error {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match *self {
            Io(ref error) => error.fmt(formatter),
            ParseFloat(ref error) => error.fmt(formatter),
            UnknownChar(char) => write!(formatter, "unknown char `{}`", char),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Io(error)
    }
}

impl From<ParseFloatError> for Error {
    fn from(error: ParseFloatError) -> Self {
        ParseFloat(error)
    }
}
