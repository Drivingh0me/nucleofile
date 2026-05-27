mod cli;
mod erro;
// mod math;
// fn main() -> anyhow::Result<()> {

fn main() -> anyhow::Result<()> {
    let args: cli::Params = cli::get_args()?;
    println!("res = {:?}", args.res);

    // Testing errors
    let test: i32 = dbg!(erro::test_err()?);

    Ok(())
}
