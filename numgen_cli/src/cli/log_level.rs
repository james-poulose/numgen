use std::fmt;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum LogLevel {
    All,
    Default,
    Info,
    Warning,
    Error,
    Debug,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
