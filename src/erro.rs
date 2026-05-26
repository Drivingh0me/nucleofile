use std::result;
use std::io;
use std::fmt::{Debug, Display};
use std::error;

pub type Result<T> = result::Result<T, Err>;

// Use ".map_err(erro::Err::E)?" to bubble up error
#[derive(Debug)]
enum Err {
    ValueError(E),
    TaskFailed(E),
}
