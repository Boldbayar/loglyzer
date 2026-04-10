use clap::Parser;

#[derive(Parser)]
pub struct Args {
    pub folder: String,
    #[arg(long, default_value = "error")]
    pub level: String, // error, info, all
}
