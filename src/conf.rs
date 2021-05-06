use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt)]
#[structopt(name = "turnstile", about = "record, retrieve, and compare data", rename_all="kebab-case")]
pub enum Turnstile {
    /// Get the most recent value of a key 
    Get {
        key: String,
        #[structopt(long)]
        count: Option<u64>,
    },
    /// Record a value for a key. When no date is provided, system time is used. 
    Record {
        key: String,
        value: f64,
        #[structopt(long)]
        date: Option<u64>,
        // #[structopt(long)]
        // all: bool,
        // repository: Option<String>
    },
    /// List available keys 
    Keys {
        // #[structopt(long)]
        // all: bool,
    }
}