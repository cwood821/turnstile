mod storage;
mod snapshot;
mod conf;

use reqwest::Client;
use crate::storage::Storage;
use crate::snapshot::Snapshot;
use structopt::StructOpt;

fn main() {
    // Parse CLI args
    let opt = conf::Opt::from_args();

    println!("{:?}", opt);

    // Load up environmental configuration
    let storage = Storage {};

    // TODO: Check that required values are sensible
    let snapshot = Snapshot::new(&opt.key, opt.value, &opt.env);

    println!("{:?}", snapshot);

    storage.store(snapshot).unwrap_or(()) 

    // TODO: Exit 1 on falure
 }
