use std::path::PathBuf;
use clap::Parser;
// use anyhow::{anyhow};

#[derive(Parser, Debug)]
#[command(arg_required_else_help = false,
    author = "Caleb Griffin",
    version = "0.1.1",
    about = "nucleofile is a tool for chemists to process data",
    long_about = "nucleofile can process scientific data, \
        specifally focused on chemistry related data such as \
        NMR, ESI-MS, kinetc plots, UV-Vis and more.")]
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

pub struct Params{
    pub res: [u16; 2],
    pub file: Option<PathBuf>,
    pub debug: bool,
}

pub fn get_args() -> anyhow::Result<Params>
{
    let args = Args::parse();

    let res = args.resolution.unwrap_or(String::from("640x480"))
        .split('x')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse().unwrap_or(256))
        .collect::<Vec<u16>>();

    let res: [u16; 2] = match res.len() {
        0 => [640, 480],
        1 => [res[0], 256],
        _ => [res[0], res[1]],
    };

    // if let Some(file) = args.file {println!("File: {:?}", file)}

    if args.debug {println!("Phianon is in debug mode!")}

    Ok(Params {
        res,
        file: args.file,
        debug: args.debug,
    })
}
