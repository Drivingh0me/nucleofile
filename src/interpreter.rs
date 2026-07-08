use crate::error::Result;
use std::io::{self, Write};
use crate::compute;

// Rules:
// Set a variable with "let x = word".
// Let parenthases group things.
// Use @folder/file.ext to use a file.
// Use $ to dereference a variable. Eg. var = 2134\n $var * 222\n.
// Use # to comment rest of line.
// Build Vec with [x1,x2,x3] WITH NO SPACES. Begining with c makes column Vec,
//     Eg. c[x1,y1,z1]. Can also do r[x1], but r is assumed.
// Build Mtx with M[a1,a2,a3;b1,b2,b3;c1,c2,c3].
// Build Tns with T[a1,a2;b1,b2;;c1,c2;d1,d2].

pub fn run_interpreter() -> Result<()> {
    let mut stdout = io::stdout();
    let mut input = String::new();
    let mut words: Vec<&str> = Vec::with_capacity(32);
    let mut input_bytes: usize = 0;

    // REPL
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
        // Do variable substitution here?


        if let Some(word) = words.get(0) {
            if *word == "echo" {
                let response: Vec<&str> = words
                    .into_iter()
                    .skip(1)
                    .collect();
                print_response(&mut stdout, &response)?;
            }
            if *word == "test" {
                let vectors: Vec<&str> = words
                    .into_iter()
                    .skip(1)
                    .collect();
                let mut vector1: Vec<&str> = vectors.split(|&x| x == "x");
                let vector2: Vec<&str> = words
                    .into_iter()
                    .skip(1)
                    .collect();
                let product: Vec<String> = tool_vec_multiply(
                    vector1,
                    vector2)?;
                print_response(&mut stdout, &product)?;
            }
        }
    }

    Ok(())
}

// Accept two vectors from the user and do an operation with them
fn tool_vec_multiply(vector1: &str, vector2: &str) -> Result<Vec<String>> {
    let vector1: Vec<f64> = vector1.split(' ')
        .map(|x| x.parse::<f64>().unwrap_or(f64::NAN))
        .collect();
    let vector2: Vec<f64> = vector2.split(' ')
        .map(|x| x.parse::<f64>().unwrap_or(f64::NAN))
        .collect();

    let answer = compute::outer_product(&vector1, &vector2)?;
    let answer = compute::printable_object(&answer)?;
    Ok(answer)
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
    stdout_lock.write_all(response.as_bytes())?;
    stdout_lock.flush()?;
    Ok(())
}
