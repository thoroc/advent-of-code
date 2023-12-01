mod commands;
mod puzzles;

use clap::Parser;

use crate::commands::exec::{Cli, CommandTypes};

fn main() {
    let cli = Cli::parse();

    println!("Advent of Code - Cli");

    let result = cli.exec();
    let value = result.unwrap();

    // println!("{:?}", value);

    match value {
        CommandTypes::ListAll(_) => {
            println!("List all Puzzles")
        }
        CommandTypes::ListLast(days) => {
            println!("List Puzzles for the last {} days", days)
        }
        CommandTypes::ListYear(year) => {
            println!("List all Puzzles for: {}", year)
        }
        CommandTypes::SolveAll(_) => {
            println!("Solving all Puzzles")
        }
        CommandTypes::SolveLast(_) => {
            println!("Solving last Puzzle")
        }
        CommandTypes::SolveYear(year) => {
            println!("Solving all Puzzles for {}", year)
        }
        CommandTypes::SolveYearDay((year, day)) => {
            println!("Solving Puzzle for {}/12/{}", year, day)
        }
    }
}
