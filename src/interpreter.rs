use crate::error::Result;
use std::io;

pub fn run_interpreter() -> Result<()> {
    // println!("Enter Text: ");
    let mut input = String::new();
    let mut input_byt_len = io::stdin().read_line(&mut input)?;

    // println!("you entered: {}", input);
    Ok(())
}
