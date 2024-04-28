use clap::Args;
/* This struct contains all common properties that needs to go on all *Args types.
 */
#[derive(Args)]
pub struct CommonArgs {
    #[arg(
        short = 'l',
        long = "log",
        value_name = "LOG_FILE_PATH",
        help = "File name to log the output to."
    )]
    pub log: Option<String>,

    #[arg(
        short = 'v',
        long = "verbosity",
        default_value = Some("Info"),
        help = "Verbosity level for logging."
    )]
    pub verbosity: Option<String>,
}
