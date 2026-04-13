use clap::{Parser, ValueEnum};

#[derive(Clone, ValueEnum)]
pub enum OutputType {
    Console,
    File,
    Grafana,
}

#[derive(Clone, ValueEnum)]
pub enum LogLevel {
    Error,
    Info,
    All,
}

#[derive(Parser)]
pub struct Args {
    pub folder: String,
    #[arg(long, default_value = "error")]
    pub level: LogLevel,

    #[arg(long, value_enum, default_value_t = OutputType::Console)]
    pub output: OutputType,
}
