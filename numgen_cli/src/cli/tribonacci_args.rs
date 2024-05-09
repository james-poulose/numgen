use super::common_args::CommonArgs;
use clap::Args;

#[derive(Args)]
pub struct TribonacciArgs {
    #[arg(
        short = 'c',
        long = "count",
        default_value_t = 3,
        help = "Total numbers that will be generated."
    )]
    pub count: u32,

    // CommonArgs contains properties that are common to multiple structs.
    #[command(flatten)]
    pub common: CommonArgs,
}
