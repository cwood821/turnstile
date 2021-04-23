mod conf;
mod store;

// use reqwest::blocking::Client;
use std::cmp::Ordering;
use std::process;
use structopt::StructOpt;
use conf::Turnstile;
use store::Store;
use std::io::{Error, ErrorKind};
// use std::box::Box;

// use storage::StorageError;

fn main() {
    // TODO: make enum that maps errors types to codes
    std::process::exit(match app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}

fn app() -> Result<(), Box<dyn std::error::Error>> {
    // let opt = conf::Opt::from_args();

    // if opt.key.is_empty() {
    //     eprintln!("Key must not be empty");
    //     process::exit(1);
    // }

    let client = reqwest::blocking::Client::new();

    match Turnstile::from_args() {
        Turnstile::Get { key } => {
            // TODO: make url config
            let api = Store {
                url: "http://localhost:3000".to_string()
            };

            // TODO: Imple formatter for json/text
            let res = api.get(key);
            // TODO: Handle if key is an error and return exit with relevant status code from
            // https://www.freebsd.org/cgi/man.cgi?query=sysexits&apropos=0&sektion=0&manpath=FreeBSD+4.3-RELEASE&format=html

            match res {
                Ok(value) => {
                    println!("{}", value);
                    Ok(())
                }
                Err(_) => {
                    let err = Error::new(ErrorKind::Other, "oh no!"); 
                    Err(Box::new(err))
                }
            }
        },
        _ => {
            println!("command not supported");
            let err = Error::new(ErrorKind::Other, "oh no!");
            Err(Box::new(err))
        },
    }

 }