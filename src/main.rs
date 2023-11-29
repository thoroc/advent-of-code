mod cli;

use clap::Parser;

use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    println!("Advent of Code - Cli");

    // println!("{:?}", value);

    match &cli.command {
        Commands::Solve(args) => {
            if args.all {
                println!("Solving all Puzzles");
            } else if args.last {
                println!("Solving last Puzzle");
            } else if args.year.is_some() {
                let year = args.year.unwrap();
                if args.day.is_some() {
                    let day = args.day.unwrap();
                    println!("Solving Puzzle for {}/12/{}", year, day);
                } else {
                    println!("Solving all Puzzles for {}", year);
                }
            }
        }
        Commands::List(args) => {
            if args.all {
                println!("List all Puzzles");
            } else if args.last.is_some() {
                let days = args.last.unwrap();
                println!("List Puzzles for the last {} days", days);
            } else if args.year.is_some() {
                let year = args.year.unwrap();
                println!("List all Puzzles for: {}", year);
            }
        }
    }
}
