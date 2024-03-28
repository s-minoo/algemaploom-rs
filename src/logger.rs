use std::env;

use log::{debug, LevelFilter};
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
use log4rs::Config;

pub fn init_logger() -> anyhow::Result<()> {
    let mut log_config_file = env::current_dir()?;
    log_config_file.push("log4rs.yaml");

    if log_config_file.exists() {
        log4rs::init_file(log_config_file, Default::default())?;
    } else {
        log4rs::init_config(build_log_config())?;
        debug!("Using fallback default logger config");
    }

    Ok(())
}

pub fn build_log_config() -> Config {
    let stderr = ConsoleAppender::builder()
        .target(Target::Stderr)
        .encoder(Box::new(PatternEncoder::new(
            "{h({d(%Y-%m-%d %H:%M:%S %Z)(utc)} - {([{l}]:):<8} {m})}{n}",
        )))
        .build();

    let file_appender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S %Z)(utc)} - {M}:{L} - [{l}]: {m}{n}",
        )))
        .append(false)
        .build(format!(
            "{}/log/execution.log",
            env::current_dir().unwrap().to_string_lossy()
        ))
        .unwrap();

    let trace_threshold = ThresholdFilter::new(LevelFilter::Trace);
    let debug_threshold = ThresholdFilter::new(LevelFilter::Debug);

    Config::builder()
        .appender(
            Appender::builder()
                .filter(Box::new(debug_threshold))
                .build("stderr", Box::new(stderr)),
        )
        .appender(
            Appender::builder()
                .filter(Box::new(trace_threshold))
                .build("file", Box::new(file_appender)),
        )
        .build(
            Root::builder()
                .appender("stderr")
                .appender("file")
                .build(LevelFilter::Trace),
        )
        .unwrap()
}
