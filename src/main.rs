mod cli;
use clap::Parser;
use cli::*;

fn main() {
    let _config = Config::parse();
}
