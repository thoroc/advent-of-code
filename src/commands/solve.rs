use anyhow::Result;
use clap::Args;

use super::common::ValueType;

#[derive(Args, Debug)]
pub struct Cli {
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

// impl Cli {
//     pub fn exec(&self) -> Result<ValueType<bool, (u16, u8), u16>> {
//         if self.all {
//             println!("Solving all Puzzles");

//             Ok(ValueType::All(self.all))
//         } else if self.last {
//             println!("Solving last Puzzle");

//             Ok(ValueType::Last(self.last))
//         } else if self.year.is_some() {
//             let year = self.year.unwrap();
//             if self.day.is_some() {
//                 let day = self.day.unwrap();
//                 println!("Solving Puzzle for {}/12/{}", year, day);

//                 Ok(ValueType::OneOf((year, day)))
//             } else {
//                 println!("Solving all Puzzles for {}", year);

//                 Ok(ValueType::AllOf(year))
//             }
//         }
//         Ok(ValueType::Default)
//     }
// }
