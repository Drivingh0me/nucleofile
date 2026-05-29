mod cli;
mod error;
mod math;

use error::Result;

fn main() -> Result<()> {
    let args: cli::Params = cli::get_args()?;
    println!("res = {:?}", args.res);

    Ok(())
}
