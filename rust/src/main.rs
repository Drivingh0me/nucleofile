mod cli;
mod interpreter;
pub mod error;
mod compute;
mod render;

#[cfg(feature = "gui")]
mod gui;

use error::Result;

fn main() -> Result<()> {
    let args: cli::Params = cli::get_args()?;

    if args.interpreter {
        interpreter::run_interpreter()?;
    } else {
        #[cfg(feature = "gui")]
        gui::run_gui()?;
    }

    Ok(())
}
