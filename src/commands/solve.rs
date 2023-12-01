use clap::Args;

#[derive(Args, Debug)]
pub struct SolveArgs {
    /// Speficy the day to run.
    #[clap(short, long, requires_all=["year"], value_parser = clap::value_parser!(u8).range(1..25))]
    pub day: Option<u8>,

    /// Specify the year to run.
    #[clap(short, long, conflicts_with_all = ["last", "all"], value_parser = clap::value_parser!(u16).range(2015..2023))]
    pub year: Option<u16>,

    /// Speficy if we run all puzzle.
    #[clap(short, long, conflicts_with_all = ["last", "year"])]
    pub all: bool,

    /// Specify if we run just the last puzzle.
    #[clap(short, long, conflicts_with_all = ["all", "year"])]
    pub last: bool,
}
