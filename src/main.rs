mod commands;

use clap::Parser;

use crate::commands::cli::Cli;

fn main() {
    println!("Hello World");

    let cli = Cli::parse();

    cli.exec()
}
