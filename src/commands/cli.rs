use anyhow::Result;
use clap::{Parser, Subcommand};

use super::common::ValueType;

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
    Solve(super::solve::Cli),
    /// List Solved Puzzles
    #[command(arg_required_else_help = true)]
    List(super::list::Cli),
}

// impl Cli {
//     pub fn exec(&self) -> Result<ValueType<bool, (u16, u8), u16>> {
//         match &self.command {
//             Commands::Solve(cli) => cli.exec(),
//             Commands::List(cli) => cli.exec(),
//         }
//     }
// }
