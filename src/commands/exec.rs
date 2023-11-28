use anyhow::Result;
use clap::Parser;

use super::cli::Commands;

#[derive(Parser, Debug)]
#[clap(name = "advent-of-code", version = "0.1.0", author = "", about = "")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug)]
pub enum CommandTypes {
    SolveAll(bool),
    SolveLast(bool),
    SolveYearDay((u16, u8)),
    SolveYear(u16),
    ListAll(bool),
    ListLast(u16),
    ListYear(u16),
}

impl Cli {
    pub fn exec(&self) -> Result<CommandTypes> {
        match &self.command {
            Commands::Solve(args) => {
                println!("Solve options={:?}", args);

                if args.all {
                    println!("Solving for all");

                    Ok(CommandTypes::SolveAll(args.all))
                } else if args.last {
                    println!("Solving for last");

                    Ok(CommandTypes::SolveLast(args.last))
                } else {
                    let year = args.year.unwrap();
                    let day = args.day.unwrap_or(0);

                    if day < 1 {
                        println!("Solving for {:?}", year);

                        Ok(CommandTypes::SolveYear(year))
                    } else {
                        println!("Solving for {:?}/12/{:?}", year, day);

                        Ok(CommandTypes::SolveYearDay((year, day)))
                    }
                }
            }
            Commands::List(args) => {
                println!("List options={:?}", args);

                if args.all {
                    println!("Listing for all");

                    Ok(CommandTypes::ListAll(args.all))
                } else if args.last.is_some() {
                    println!("Listing for last {} days", args.last.unwrap());

                    Ok(CommandTypes::ListLast(args.last.unwrap()))
                } else {
                    println!("Listing for {:?}", args.year.unwrap());

                    Ok(CommandTypes::ListYear(args.year.unwrap()))
                }
            }
        }
    }
}
