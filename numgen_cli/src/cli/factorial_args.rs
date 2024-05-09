use super::common_args::CommonArgs;
use clap::Args;

#[derive(Args)]
pub struct FactorialArgs {
    #[arg(
        short = 'n',
        long = "number",
        default_value_t = 0,
        help = "The start value (instead of 0)."
    )]
    pub number: i32,

    // CommonArgs contains properties that are common to multiple structs.
    #[command(flatten)]
    pub common: CommonArgs,
}
