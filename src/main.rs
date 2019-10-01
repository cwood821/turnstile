mod storage;
mod snapshot;
mod conf;

use reqwest::Client;
use std::cmp::Ordering;
use std::process;
use crate::storage::Storage;
use crate::snapshot::Snapshot;
use structopt::StructOpt;
use storage::StorageError;

fn main() {
    // Parse CLI args
    let opt = conf::Opt::from_args();

    if opt.key.is_empty() {
        eprintln!("Key must not be empty");
        process::exit(1);
    }

    // Load up environmental configuration
    let storage = Storage::new("http://localhost:3000");
    let snapshot = Snapshot::new(&opt.key, opt.value);


    // Change this to if let
    // TODO: Cleanup error handling and conditionality
    // TODO: Add flags for increase/decrease
    // TODO: Add passive flag

    let stored_snapshot = match storage.get(&snapshot) {
        Ok(snap) => snap,
        Err(StorageError::DoesNotExist) => { 
            println!("Attempting to store");
            storage.store(&snapshot).unwrap()
        }
        _ => return ()
    };

    match snapshot.value.cmp(&stored_snapshot.value) { 
        Ordering::Less => println!("Less"),
        Ordering::Greater => { 
            exit("Increased", 1);
        },
        Ordering::Equal => println!("Equal")
    }

    // At this point, we should update it 
 }


fn exit(msg: &str, code: i32) {
    eprintln!("{}", msg);
    process::exit(code);
}

fn run() {
    // 
}