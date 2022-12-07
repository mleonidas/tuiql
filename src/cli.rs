use crate::config::CliConfig;
use structopt::StructOpt;

/// A cross-platform TUI database management tool written in Rust
#[derive(StructOpt, Debug)]
#[structopt(name = "tuiql")]
pub struct Cli {
    #[structopt(flatten)]
    pub config: CliConfig,
}

pub fn parse() -> Cli {
    Cli::from_args()
}
