use crate::error::Result;
use std::io::{self, Write};

pub fn run_interpreter() -> Result<()> {
    println!("Enter Text: ");
    // let mut stdout = io::stdout();
    // let prompt = b"Enter Text: ";
    // stdout.write_all(prompt)?;
    // stdout.flush()?;

    let mut input = String::new();
    let mut input_byt_len = io::stdin().read_line(&mut input)?;

    // println!("you entered: {}", input);
    Ok(())
}
