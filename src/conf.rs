use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt)]
#[structopt(name = "turnstile", about = "record and retrieve data over time", rename_all="kebab-case")]
pub enum Turnstile {
    /// Get the most recent value of a key 
    Get {
        key: String,
        #[structopt(long)]
        count: Option<u64>,
    },
    /// Record a value for a key. Uses system time for default 
    Record {
        key: String,
        value: f64,
        #[structopt(long)]
        date: Option<u64>,
    },
    /// List available keys 
    Keys {
        // #[structopt(long)]
        // all: bool,
    }
}