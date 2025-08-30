use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(version, about="Finds the name of a file in your device")]
pub struct Args {
    #[clap(short, long, default_value="\0", help="Sequence of characters to find")]
    pub content: String,

    #[clap(short, long, default_value="\0", help="Pattern to find")]
    pub regex: String,

    #[clap(short, long, help="Root Paths")]
    pub paths: Vec<PathBuf>,

    #[clap(short, long, default_value_t = 1, help="threads to use")]
    pub threads: usize,
}