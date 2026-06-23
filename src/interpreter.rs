use crate::error::Result;
use std::io::{self, Write};

pub fn run_interpreter() -> Result<()> {
    let mut stdout = io::stdout();
    let mut input = String::new();
    let mut input_bytes:usize = 0;
    loop {
        print_input_arrow(&mut stdout)?;
        input.clear();
        input_bytes = io::stdin().read_line(&mut input)?;

        if input == "quit\n" {
            break;
        }

        if input == "clear\n" {
            clear_screen(&mut stdout)?;
        }
    }

    Ok(())
}

fn print_input_arrow(std_out: &mut io::Stdout) -> Result<()> {
    let mut stdout_lock = std_out.lock();
    let input_arrow = b"\x1b[34m> \x1b[0m";
    stdout_lock.write_all(input_arrow)?;
    stdout_lock.flush()?;
    Ok(())
}

fn clear_screen(std_out: &mut io::Stdout) -> Result<()> {
    let mut stdout_lock = std_out.lock();
    let bytes = b"\x1b[2J";
    stdout_lock.write_all(bytes)?;
    stdout_lock.flush()?;
    Ok(())
}
