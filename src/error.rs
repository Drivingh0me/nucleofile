use thiserror::Error;
use std::io;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Io error: {0}")]
    Io(#[from] io::Error),
    #[error("Parse Int Error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Try From Int Error: {0}")]
    TryIntErr(#[from] std::num::TryFromIntError),
}

// Testing
pub fn test_err(x: i32) -> Result<u32> {
    Ok(TryInto::<u32>::try_into(x)?)
}
