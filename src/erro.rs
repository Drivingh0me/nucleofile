use std::result;
use std::io;
use std::fmt;
use std::error;
use std::num::ParseIntError;

pub type Result<T> = result::Result<T, Err>;

// Maybe Use ".map_err(erro::Err::E)?" to bubble up error
#[derive(Debug)]
enum Err<E> {
    IoError(io::Error),
    ParseError(ParseIntError),
    Other(E),
}

impl<E: fmt::Display + fmt::Debug> fmt::Display for Err<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Err::IoError(e) => todo!,
            Err::ParseError(e) => todo!,
            Err::Other(e) => write!(f, "Generic error: {}", e),
        }
    }
}

impl<E: error:Error + 'static> error::Error for Err<E> {
    fn source(&self) -> Option<&(dyn Err + 'static)> {
        match self {
            Err::IoError(e) => Some(e),
            Err::ParseError(e) => Some(e),
            Err::Other(e) => Some(e),
        }
    }
}

impl<E> From<std::io::Error> for Err<E> {
    fn from(err: std::io::Error) -> Self {
        Err::IoError(err)
    }
}

impl<E> From<ParseIntError> for Err {
    fn from(err: ParseIntError) -> Self {
        Err::ParseError(err)
    }
}
