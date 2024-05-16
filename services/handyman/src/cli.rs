use std::path::PathBuf;
use clap::Parser;
use logger::tracer_logger::LogLevel;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Sets a custom path to config file
    #[arg(short, long, value_name = "CONFIG_FILE", default_value = "config.yaml")]
    pub config: PathBuf,

    /// Sets a custom path to tasks file
    #[arg(short, long, value_name = "TASKS_FILE", default_value = "tasks.yaml")]
    pub tasks: PathBuf,

    /// Sets a custom log level
    #[arg(
        short,
        long,
        value_name = "LOG_LEVEL",
        default_value = "info",
        value_enum
    )]
    pub log_level: LogLevel,
}

impl Cli {
    pub fn new() -> Cli {
        Cli::parse()
    }
}
