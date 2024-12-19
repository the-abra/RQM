use clap::Parser;

/// Args for command line arguments
#[derive(Parser, Debug)]
#[command(name = "RQM", about = "An application for managing VMs")]
pub struct Args {
    /// Language for localization
    #[arg(short, long, default_value = "en")]
    pub lang: String,
}