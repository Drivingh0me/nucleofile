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

    #[cfg(feature = "gui")]
    gui::run_gui()?;

    #[cfg(not(feature = "gui"))]
    interpreter::run_interpreter()?;

    Ok(())
}
