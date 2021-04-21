mod conf;

use reqwest::Client;
use std::cmp::Ordering;
use std::process;
use structopt::StructOpt;
use conf::Turnstile;
// use storage::StorageError;

fn main() {
    // let opt = conf::Opt::from_args();

    // if opt.key.is_empty() {
    //     eprintln!("Key must not be empty");
    //     process::exit(1);
    // }

    match Turnstile::from_args() {
        Turnstile::Get { key } => {
            // println!("{:?}", interactive);
            println!("{:?}", key);
            // Get key from request 
            // Make reqwest to store to fetch key
            // handle storage error 
            // return most recent value, date format 
            println!("from get")
        },
        _ => {
            println!("command not supported");
            ()
        },
    }
    // let store = Storage::new("http://localhost:3000");

    ()
 }