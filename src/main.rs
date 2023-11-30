mod commands;

use clap::Parser;
use commands::cli::Cli;

fn main() {
    let cli = Cli::parse();

    println!("Advent of Code - Cli");

    match cli.command {
        commands::cli::Commands::Solve(cli) => {
            println!("{:?}", cli)
        }
        commands::cli::Commands::List(cli) => {
            println!("{:?}", cli)
        }
    }
}
