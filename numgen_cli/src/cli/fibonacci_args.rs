use super::common_args::CommonArgs;
use clap::Args;

#[derive(Args)]
pub struct FibonacciArgs {
    //pub dest: String,
    #[arg(
        short = 's',
        long = "start",
        default_value_t = 0,
        help = "The start value (instead of 0)."
    )]
    pub start: u64,

    #[arg(
        short = 'c',
        long = "count",
        help = "Total numbers that will be generated."
    )]
    pub count: Option<u32>,

    // CommonArgs contains properties that are common to multiple structs.
    #[command(flatten)]
    pub common: CommonArgs,
}
