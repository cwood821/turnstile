use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt)]
#[structopt(name = "turnstile", about = "get, set, and compare data", rename_all="kebab-case")]
pub enum Turnstile {
    /// Get the most recent value of a key 
    Get {
        key: String,
    },
    /// Record a value for a key. When no date is provided, system time is used. 
    Record {
        key: String,
        #[structopt(long)]
        date: bool,
        // #[structopt(long)]
        // all: bool,
        // repository: Option<String>
    },
    // Commit {
    //     #[structopt(short)]
    //     message: Option<String>,
    //     #[structopt(short)]
    //     all: bool
    // }
}