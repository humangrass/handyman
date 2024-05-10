use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt;

pub enum LogLevel {
    Info,
    Trace,
    Debug,
    Warn,
    Error,
}


pub fn new_tracer_logger(log_level: LogLevel) {
    let log_level_filter = match log_level {
        LogLevel::Trace => LevelFilter::TRACE,
        LogLevel::Debug => LevelFilter::DEBUG,
        LogLevel::Info => LevelFilter::INFO,
        LogLevel::Warn => LevelFilter::WARN,
        LogLevel::Error => LevelFilter::ERROR,
    };

    let format = fmt::format()
        .with_level(true) // include levels in formatted output
        .with_target(true) // include targets
        .with_thread_ids(true) // include the thread ID of the current thread
        .with_thread_names(true) // include the name of the current thread
        .compact(); // use the `Compact` formatting style.

    tracing_subscriber::fmt()
        .event_format(format)
        .with_max_level(log_level_filter)
        .init();
}
