mod cli;
mod math;

// Make custom errors with enum!
// use anyhow::{
//     Result, anyhow, bail
// };

fn main() -> anyhow::Result<()> {
    let args: cli::Params = cli::get_args()?;
    println!("res = {:?}", args.res);
    Ok(())
}
