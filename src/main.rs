mod cli;
mod error;
// mod math;

fn main() -> error::Result<()> {
    let args: cli::Params = cli::get_args()?;
    println!("res = {:?}", args.res);

    // Testing errors
    let _test: u32 = error::test_err(-1)?;

    print!("yay!\n");

    Ok(())
}
