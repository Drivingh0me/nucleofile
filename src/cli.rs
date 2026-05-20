use std::path::PathBuf;
use clap::Parser;
use anyhow::{anyhow};

#[derive(Parser, Debug)]
#[command(arg_required_else_help = false,
    author = "Caleb Griffin",
    version = "0.1.1",
    about = "nucleofile is a tool for chemists to process data",
    long_about = "nucleofile can process scientific data")]
struct Args {
    // Takes any number of args
    #[arg(short)]
    resolution: Option<String>,

    #[arg(short)]
    file: Option<PathBuf>,

    /// Enters debug mode
    #[arg(short, value_name = "debug mode", action = clap::ArgAction::SetTrue)]
    debug: bool,
}

// pub struct Params{
//
// }

pub fn get_args() -> anyhow::Result<()>
{
    let args = Args::parse();
    // let resolution: [u32; 2] = [640, 480];

    let resolution = args.resolution.unwrap_or(String::from("640x480"))
        .split('x')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse().unwrap_or(256))
        .collect::<Vec<u16>>();

    println!("Resolution: {:?}", resolution);

    if let Some(file) = args.file {println!("File: {:?}", file)}

    if args.debug {println!("Phianon is in debug mode!")}

    Ok(())
}
