use std::result;
use std::io;
use std::fmt;
use std::error;
use std::num::ParseIntError;

// Make it visible in output what caller had what error
// Currently it just says the error, but not the caller.

pub type Result<T> = result::Result<T, Err>;

// Used AI to figure out the dyn + Send + Sync
#[derive(Debug)]
pub enum Err {
    IoError(io::Error),
    ParseError(ParseIntError),
    Other(Box<dyn error::Error + Send + Sync>),
}

impl fmt::Display for Err {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Err::IoError(e) => write!(f, "IoError: {}", e),
            Err::ParseError(e) => write!(f, "ParseError error: {}", e),
            Err::Other(e) => write!(f, "Generic error: {}", e),
        }
    }
}

// Used AI to figure this out, I was very confudled
impl error::Error for Err {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Err::IoError(e) => Some(e),
            Err::ParseError(e) => Some(e),
            Err::Other(e) => Some(e.as_ref()),
        }
    }
}

impl From<std::io::Error> for Err {
    fn from(err: std::io::Error) -> Self {
        Err::IoError(err)
    }
}

impl From<ParseIntError> for Err {
    fn from(err: ParseIntError) -> Self {
        Err::ParseError(err)
    }
}

// Testing
pub fn test_err(x: i32) -> Result<i32> {
    if x > 0 {Ok(x)}
    else {Err(Err::IoError(
        io::Error::new(io::ErrorKind::Other, "Caleb error message")))
    }
}

pub fn test_err2(x: i32) -> Result<u32> {
    let val = TryInto::<u32>::try_into(x)
        .map_err(|e| Err::Other(Box::new(e)))?;
    Ok(val)
}
