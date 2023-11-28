use clap::Parser;

use super::cli::Commands;

#[derive(Parser, Debug)]
#[clap(name = "advent-of-code", version = "0.1.0", author = "", about = "")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn exec(&self) {
        match &self.command {
            Commands::Solve(args) => {
                println!("Solve options={:?}", args);

                if args.all {
                    println!("Solving for all")
                } else if args.last {
                    println!("Solving for last")
                } else {
                    println!(
                        "Solving for {:?}/12/{:?}",
                        args.year.unwrap(),
                        args.day.unwrap()
                    )
                }
            }
            Commands::List(args) => {
                println!("List options={:?}", args);

                if args.all {
                    println!("Solving for all")
                } else if args.last.is_some() {
                    println!("Solving for last {} days", args.last.unwrap())
                } else {
                    println!("Solving for {:?}", args.year.unwrap(),)
                }
            }
        }
    }
}
