use crate::error::Result;
use std::io::{self, Write};

pub fn run_interpreter() -> Result<()> {
    let mut stdout = io::stdout();
    let mut input = String::new();
    let mut words: Vec<&str> = Vec::with_capacity(32);
    let mut input_bytes: usize = 0;

    loop {
        print_input_arrow(&mut stdout)?;
        input.clear();
        input_bytes = io::stdin().read_line(&mut input)?;

        if input == "quit\n" {
            break;
        }

        if input == "clear\n" {
            clear_screen(&mut stdout)?;
            continue;
        }

        if input == "help\n" {
            print_help(&mut stdout)?;
            continue;
        }

        words = input.split(' ').collect();

        if let Some(word) = words.get(0) {
            if *word == "echo" {
                let response: Vec<&str> = words
                    .into_iter()
                    .skip(1)
                    .collect();
                print_response(&mut stdout, &response)?;
            }
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
    let bytes = b"\x1b[2J\x1b[H";
    stdout_lock.write_all(bytes)?;
    stdout_lock.flush()?;
    Ok(())
}

fn print_help(std_out: &mut io::Stdout) -> Result<()> {
    let mut stdout_lock = std_out.lock();
    let bytes = b"Help not implemented\n";
    stdout_lock.write_all(bytes)?;
    stdout_lock.flush()?;
    Ok(())
}

fn print_response(
    std_out: &mut io::Stdout,
    response: &Vec<&str>) -> Result<()> {
    let mut stdout_lock = std_out.lock();
    let response_arrow = b"\x1b[32m> \x1b[0m";
    stdout_lock.write_all(response_arrow)?;

    let response: String = response.join(" ");
    let response_bytes: &[u8] = response.as_bytes();

    stdout_lock.write_all(&response_bytes)?;
    stdout_lock.flush()?;
    Ok(())
}
