use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "advent-of-code", version = "0.1.0", author = "", about = "")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Solve Puzzles
    Solve(Solve),
    /// List Solved Puzzles
    List(List),
}

#[derive(Args, Debug)]
pub struct Solve {
    /// Speficy the day to run.
    #[clap(short, long, requires_all=["year"])]
    pub day: Option<u8>,

    /// Specify the year to run.
    #[clap(short, long, conflicts_with_all = ["last", "all"])]
    pub year: Option<u16>,

    /// Speficy if we run all puzzle.
    #[clap(short, long, conflicts_with_all = ["last", "year"])]
    pub all: bool,

    /// Specify if we run just the last puzzle.
    #[clap(short, long, conflicts_with_all = ["all", "year"])]
    pub last: bool,
}

#[derive(Args, Debug)]
pub struct List {
    /// Specify the year to run.
    #[clap(short, long, conflicts_with_all = ["last", "all"])]
    pub year: Option<u16>,

    /// Speficy if we run all puzzle.
    #[clap(short, long, conflicts_with_all = ["last", "year"])]
    pub all: bool,

    /// Specify if we list just the last few puzzle.
    #[clap(short, long, conflicts_with_all = ["all", "year"])]
    pub last: Option<u8>,
}

impl Cli {
    pub fn exec(&self) {
        match &self.command {
            Commands::Solve(args) => {
                println!("Solve options={:?}", args);
            }
            Commands::List(args) => {
                println!("List options={:?}", args);
            }
        }
    }
}
