use crate::error::{Result, Error};
use std::io::{self, Write};
use crate::compute;
use std::collections::HashMap;

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

// Order of operations:
// Check for exact match commands ->
// Sub vaiables -> Perform actions/tools in order -> Simplify words with,
// results from actions/tools -> Enact keywords.

pub fn run_interpreter() -> Result<()> {
    let mut stdout = io::stdout();
    let mut input = String::new();
    let mut words: Vec<String> = Vec::with_capacity(32);
    let mut input_bytes: usize = 0;
    let mut variable_dict: HashMap<String, String> = HashMap::new();

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

        words = input.split(' ')
            .map(|s| s.to_string())
            .collect();
        // Variable substitution
        if let Some(()) = sub_variables(&variable_dict, &mut words) {
            let var_warning: String = String::from(
                "\x1b[31mUndefined variable:"
            );
            let reset_color: String = String::from(
                "\x1b[0m\n"
            );
            let mut not_a_var: Vec<&str> = Vec::new();
            not_a_var.push(&var_warning);
            not_a_var.extend(words.iter().map(|s| s.as_str()));
            not_a_var.push(&reset_color);
            print_response(&mut stdout, &not_a_var)?;
            continue;
        }


        // Must be first word tools.
        if let Some(word) = words.get(0) {
            if *word == "echo" {
                let response: Vec<&str> = words
                    .iter()
                    .skip(1)
                    .map(String::as_str)
                    .collect();
                print_response(&mut stdout, &response)?;
                continue;
            }
            if *word == "let" {
                let expression: Vec<&str> = words
                    .iter()
                    .skip(1)
                    .map(String::as_str)
                    .collect();
                set_variables(
                    &mut variable_dict,
                    &expression
                )?;
                // dbg!(&variable_dict);
                continue;
            }
        }
    }
    Ok(())
}

fn set_variables<'a>(
    variable_dict: &'a mut HashMap<String, String>,
    expression: &Vec<&str>) -> Result<()> {
    // expression must be: x = y, where y can be evaluated by tools.
    // y can have spaces. must have spaces between x and + and y.
    // x must have no spaces.
    valid_assignment(&expression)?;
    let key = match expression.get(0) {
        Some(k) => k,
        None => return Err(Error::ItemNotFound(String::from("No key"))),
    };

    let val: String = expression[2..].join(" ").trim_end().to_string();

    // let val = match expression.get(2) {
    //     Some(v) => v.trim_end(),
    //     None => return Err(Error::ItemNotFound(String::from("No key"))),
    // };
    // variable_dict.insert(key.to_string(), val.to_string());

    variable_dict.insert(key.to_string(), val);
    Ok(())
}

fn sub_variables(
    variable_dict: &HashMap<String, String>,
    words: &mut Vec<String>) -> Option<()> {
    // If Some(), words is list of nonexistent variables.
    let mut all_vars_exist: bool = true;
    for word in words.iter_mut() {
        if word.starts_with('@') {
            let key: &str = match word.chars().next() {
                Some(c) => &word[c.len_utf8()..].trim_end(),
                None => "",
            };
            if let Some(value) = variable_dict.get(key) {
                *word = value.to_string();
            } else {
                all_vars_exist = false;
            }
        }
    }

    if all_vars_exist {
        return None;
    }

    words.retain(|s| s.starts_with('@'));
    Some(())
}

fn valid_assignment(expression: &Vec<&str>) -> Result<()> {
    match expression.get(1) {
        Some(&"=") => Ok(()),
        Some(other) => Err(
            Error::ItemNotFound(String::from("No assignment"))
        ),
        None => Err(
            Error::ItemNotFound(String::from("No assignment"))
        ),
    }
}

// Accept two vectors from the user and do an operation with them.
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
