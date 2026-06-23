mod cli;
mod interpreter;
pub mod error;
mod math;
mod render;

#[cfg(feature = "gui")]
mod gui;

use error::Result;

fn main() -> Result<()> {
    let args: cli::Params = cli::get_args()?;
    // println!("res = {:?}", args.res);

    #[cfg(feature = "gui")]
    gui::run_gui()?;

    #[cfg(not(feature = "gui"))]
    interpreter::run_interpreter()?;

    Ok(())
}
