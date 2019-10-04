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
    let opt = conf::Opt::from_args();

    if opt.key.is_empty() {
        eprintln!("Key must not be empty");
        process::exit(1);
    }

    let storage = Storage::new("http://localhost:3000");
    let snapshot = Snapshot::new(&opt.key, opt.value);

    let stored_snapshot = match storage.get(&snapshot) {
        Ok(snap) => snap,
        Err(StorageError::DoesNotExist) => { 
            storage.store(&snapshot).unwrap_or_else(|e| {
                if e == StorageError::FailedToStore {
                   exit("Failed to store value. Check connection to storage provider", 1)
                } else {
                   snapshot.clone()
                }
            })
        },
        Err(_) => exit("Failed to retrieve value for key. Check connection to storage provider.", 1)
    };

    match snapshot.value.cmp(&stored_snapshot.value) { 
        Ordering::Less => {
            if opt.decrease {
                exit("Value increased", 1)
            }
        },
        Ordering::Greater => { 
            if ! opt.decrease {
                exit("Increased", 1);
            }
        },
        Ordering::Equal => println!("Equal")
    }

 }


fn exit(msg: &str, code: i32) -> ! {
    eprintln!("{}", msg);
    process::exit(code);
}