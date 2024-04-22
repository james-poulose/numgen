use std::str::FromStr;

use log::LevelFilter;
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

pub fn init_log(args: Vec<String>) {
    // Choose the desired date format (for both file & console). e.g. "d(%Y-%m-%d %H:%M:%S)" will result in "2024-04-17 00:23:19".
    // More patterns can be found here: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
    const LOG_PATTERN: &str = "{d(%Y-%m-%d %H:%M:%S)}\t{l} - {m}\n";

    // User may/may not supply a file path. All file related configuration need to be made conditional.
    let mut file_path: String = String::new();
    let mut file_logging_enabled = false;
    let file_path_result = get_log_path_from_args(&args);
    match file_path_result {
        Ok(path) => {
            file_logging_enabled = true;
            file_path = path;
        }
        _ => { /* Ignoring other cases. We care only if there is a valid file path. */ }
    }

    let level = get_verbosity_level_from_args(&args);

    // Build a stderr logger.
    let console_builder = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOG_PATTERN)))
        .target(Target::Stderr)
        .build();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let mut root_builder = Root::builder();

    if file_logging_enabled {
        root_builder = root_builder.appender("logfile");
    }

    root_builder = root_builder.appender("stderr");
    let root = root_builder.build(LevelFilter::Trace);

    let mut config_builder = Config::builder();

    if file_logging_enabled {
        let log_file_builder = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new(LOG_PATTERN)))
            .build(file_path)
            .unwrap();
        config_builder = config_builder
            .appender(Appender::builder().build("logfile", Box::new(log_file_builder)));
    }

    let config = config_builder
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(console_builder)),
        )
        .build(root)
        .unwrap();

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let _handle = log4rs::init_config(config);
}

fn get_log_path_from_args(args: &Vec<String>) -> Result<String, bool> {
    // We do not have clap yet. Even if we have clap, we have to go through all commands in the 'match' and get the "-l" from its args.
    // There is no way in clap just to see if "-l" is present or not. So we are resorting to the basic way of reading CLI args.
    for i in 0..args.len() {
        if (args[i] == "-l" || args[i] == "--log") && i + 1 < args.len() {
            let maybe_path = args[i + 1].clone();
            return Ok(maybe_path);
        }
    }
    return Err(false);
}

fn get_verbosity_level_from_args(args: &Vec<String>) -> LevelFilter {
    // We do not have clap yet. Even if we have clap, we have to go through all commands in the 'match' and get the options from its args.
    // There is no way in clap just to see if a particular option is present or not. So we are resorting to the basic way of reading CLI args.
    for i in 0..args.len() {
        if (args[i] == "-v" || args[i] == "--verbosity") && i + 1 < args.len() {
            let maybe_level_filter_str = args[i + 1].clone();

            let level_filter_parse_result = LevelFilter::from_str(&maybe_level_filter_str);
            match level_filter_parse_result {
                Ok(filter) => {
                    return filter;
                }
                _ => {
                    println!("Invalid LevelFilter");
                }
            }

            // No need to continue the loop, exit fast.
            return LevelFilter::Info;
        }
    }

    // If none supplied, use this as the default.
    return LevelFilter::Info;
}
