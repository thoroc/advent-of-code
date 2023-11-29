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
                if args.all {
                    Ok(CommandTypes::SolveAll(args.all))
                } else if args.last {
                    Ok(CommandTypes::SolveLast(args.last))
                } else {
                    let year = args.year.unwrap();
                    let day = args.day.unwrap_or(0);

                    if day < 1 {
                        Ok(CommandTypes::SolveYear(year))
                    } else {
                        Ok(CommandTypes::SolveYearDay((year, day)))
                    }
                }
            }
            Commands::List(args) => {
                if args.all {
                    Ok(CommandTypes::ListAll(args.all))
                } else if args.last.is_some() {
                    Ok(CommandTypes::ListLast(args.last.unwrap()))
                } else {
                    Ok(CommandTypes::ListYear(args.year.unwrap()))
                }
            }
        }
    }
}
