use anyhow::Result;
use clap::Args;

use super::common::ValueType;

#[derive(Args, Debug)]
pub struct Cli {
    /// Specify the year to run.
    #[clap(short, long, conflicts_with_all = ["last", "all"])]
    pub year: Option<u16>,

    /// Speficy if we run all puzzle.
    #[clap(short, long, conflicts_with_all = ["last", "year"])]
    pub all: bool,

    /// Specify if we list just the last few puzzle.
    #[clap(short, long, conflicts_with_all = ["all", "year"])]
    pub last: bool,
}

// impl Cli {
//     pub fn exec(&self) -> Result<ValueType<bool, (u16, u8), u16>> {
//         if self.all {
//             println!("List all Puzzles");

//             Ok(ValueType::All(self.all))
//         } else if self.last {
//             println!("List last few Puzzle");

//             Ok(ValueType::Last(self.last))
//         } else if self.year.is_some() {
//             let year = self.year.unwrap();
//             println!("List all Puzzles for: {}", year);

//             Ok(ValueType::AllOf(year))
//         }
//         Ok(ValueType::Default)
//     }
// }
