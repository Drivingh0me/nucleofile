mod cli;
mod erro;
// mod math;

fn main() -> erro::Result<()> {
    let args: cli::Params = cli::get_args()?;
    println!("res = {:?}", args.res);

    // Testing errors
    let _test: i32 = erro::test_err(1)?;

    let _test2: u32 = erro::test_err2(1)?;

    print!("yay!\n");

    Ok(())
}
