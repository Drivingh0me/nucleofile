mod cli;
// use anyhow::{
//     Result, anyhow, bail
// };

fn main() -> anyhow::Result<()> {
    cli::get_args()?;
    Ok(())
}
