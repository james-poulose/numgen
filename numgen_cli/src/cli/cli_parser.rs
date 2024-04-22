use clap::Parser;

use super::commands_type::CommandType;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct CliParser {
    #[command(subcommand)]
    pub command: CommandType,
}
