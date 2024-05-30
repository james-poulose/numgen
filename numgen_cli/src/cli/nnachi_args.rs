use super::common_args::CommonArgs;
use clap::Args;

#[derive(Args)]
pub struct NnacciArgs {
    #[arg(
        short = 'd',
        long = "dimension",
        default_value_t = 2,
        help = "The dimension of the Nnachi series (Fibonacci and Tribonacci have a dimension of 2 and 3 respectively)."
    )]
    pub dimension: i8,

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
