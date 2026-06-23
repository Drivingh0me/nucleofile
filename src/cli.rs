use std::path::PathBuf;
use clap::Parser;
use crate::render;

use crate::error::Result;

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

// Make Resolution the struct from render.rs
pub struct Params{
    pub res: render::Resolution,
    pub file: Option<PathBuf>,
    pub debug: bool,
}

pub fn get_args() -> Result<Params>
{
    let args = Args::parse();

    let res = args.resolution.unwrap_or(String::from("640x480"))
        .split('x')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse().unwrap_or(256))
        .collect::<Vec<u32>>();

    let res: render::Resolution = match res.len() {
        0 => render::Resolution {
            w: 256,
            h: 256,
        },
        1 => render::Resolution {
            w: res[0],
            h: 256,
        },
        _ => render::Resolution {
            w: res[0],
            h: res[1],
        },
    };

    Ok(Params {
        res,
        file: args.file,
        debug: args.debug,
    })
}
