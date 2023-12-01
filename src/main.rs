mod commands;
mod puzzles;

use clap::Parser;
use commands::cli::Cli;

fn main() {
    let cli = Cli::parse();

    println!("Advent of Code - Cli");

    match cli.command {
        commands::cli::Commands::Solve(args) => {
            println!("{:?}", args)
        }
        commands::cli::Commands::List(args) => {
            println!("{:?}", args)
        }
    }
}
