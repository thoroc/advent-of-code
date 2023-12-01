use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "advent-of-code", version = "0.1.0", author = "", about = "")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Solve Puzzles
    #[command(arg_required_else_help = true)]
    Solve(super::solve::SolveArgs),
    /// List Solved Puzzles
    #[command(arg_required_else_help = true)]
    List(super::list::ListArgs),
}
