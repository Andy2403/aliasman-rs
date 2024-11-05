//! The Main process
use clap::Parser;
use ohmyalias::cli::{Cli, Program};

fn main() {
    let cli = Cli::parse();
    let mut prog = Program::new();

    prog.resolve(&cli);
}
