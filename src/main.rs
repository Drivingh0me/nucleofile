mod cli;
mod error;
mod math;

use error::Result;

fn main() -> Result<()> {
    let args: cli::Params = cli::get_args()?;
    println!("res = {:?}", args.res);

    #[cfg(feature = "gui")]
    println!("GUI not implemented yet");

    #[cfg(not(feature = "gui"))]
    println!("TUI compiled");

    Ok(())
}
