use std::path::PathBuf;
//use image::ImageReader;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(arg_required_else_help = false,
    author = "Caleb Griffin",
    version = "0.1.1",
    about = "nucleofile is a tool for chemists to process data",
    long_about = "nucleofile can process scientific data")]
struct Args {
    // Takes any number of args
    #[arg(short, num_args = 1..)]
    resolution: Option<String>,

    #[arg(short)]
    file: Option<PathBuf>,

    /// Enters debug mode
    #[arg(short, value_name = "debug mode", action = clap::ArgAction::SetTrue)]
    debug: bool,
}

// pub struct Arguments {
//
// }

pub fn get_args()
{
    let args = Args::parse();
    let resolution: [u32; 2] = [640, 480];

    match args.resolution {
        Some(resolution) => println!("Resolution: {:?}", resolution),
        None => (),
    }

    if let Some(file) = args.file {println!("File: {:?}", file)}

    if args.debug {println!("Phianon is in debug mode!")}
}
