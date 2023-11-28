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
pub enum CommandTypes<B, T, N> {
    Boolean(B),
    Tuple(T),
    Number(N),
}

impl Cli {
    pub fn exec(&self) -> Result<CommandTypes<bool, (u16, u8), u16>> {
        match &self.command {
            Commands::Solve(args) => {
                println!("Solve options={:?}", args);

                if args.all {
                    println!("Solving for all");

                    Ok(CommandTypes::Boolean(args.all))
                } else if args.last {
                    println!("Solving for last");

                    Ok(CommandTypes::Boolean(args.last))
                } else {
                    println!(
                        "Solving for {:?}/12/{:?}",
                        args.year.unwrap(),
                        args.day.unwrap()
                    );

                    Ok(CommandTypes::Tuple((args.year.unwrap(), args.day.unwrap())))
                }
            }
            Commands::List(args) => {
                println!("List options={:?}", args);

                if args.all {
                    println!("Solving for all");

                    Ok(CommandTypes::Boolean(args.all))
                } else if args.last.is_some() {
                    println!("Solving for last {} days", args.last.unwrap());

                    Ok(CommandTypes::Number(args.last.unwrap()))
                } else {
                    println!("Solving for {:?}", args.year.unwrap());

                    Ok(CommandTypes::Number(args.year.unwrap()))
                }
            }
        }
    }
}
