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

enum AppError {
    UnsupportedError = 64,
    IOError = 74,
    ConfigurationError = 78
}

fn main() {
    // TODO: make enum that maps errors types to codes
    std::process::exit(match app() {
        Ok(_) => 0,
        Err(err) => {
            // TODO: print to stderr message
            err as i32
        }
    });
}

fn app() -> Result<i32, AppError> {
    match Turnstile::from_args() {
        Turnstile::Get { key } => {
            // TODO: make url config
            let api = Store {
                url: "http://localhost:3000".to_string()
            };

            // TODO: Imple formatter for json/text
            match api.get(key) {
                Ok(record) => {
                    println!("{:?}", record);
                    println!("{}", record.value);
                    Ok(0)
                }
                Err(_) => {
                    Err(AppError::IOError)
                }
            }
        },
        _ => {
            Err(AppError::UnsupportedError)
        },
    }

 }