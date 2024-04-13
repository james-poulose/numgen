use clap::Args;

//use super::output_level::OutputLevel;

#[derive(Args)]
pub struct FibonacciArgs {
    //pub dest: String,
    #[arg(short, long, default_value_t = 0)]
    pub start: u8,

    #[arg(short, long)]
    pub end: u8,
    //#[arg(short, long, value_enum, default_value_t=OutputLevel::Default)]
    //pub verbosity: OutputLevel,
}
