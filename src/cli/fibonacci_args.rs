use clap::Args;

//use super::log_level::LogLevel;

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

    #[arg(short = 'l', long = "log", value_name = "LOG_FILE_PATH")]
    pub log: Option<String>,
    /*
    #[arg(
        short = 'v',
        long = "verbosity",
        default_value_t = LogLevel::Default,
        help = "Verbosity level for logging."
    )]
    pub verbosity: LogLevel,
    */
}
