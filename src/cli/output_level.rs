use std::fmt;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum OutputLevel {
    All,
    Default,
    Info,
    Warning,
    Error,
    Debug,
}

impl fmt::Display for OutputLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
