mod conf;
mod store;

// use reqwest::blocking::Client;
use std::time::{SystemTime, UNIX_EPOCH};
use std::cmp::Ordering;
use std::env;
use std::process;
use structopt::StructOpt;
use conf::Turnstile;
use store::{Store, Record};
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

            if let Ok(base) = env::var("API_URL") {
                let api = Store {
                    url: base
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
            }
            else {
                Err(AppError::ConfigurationError)
            }


        },
        Turnstile::Record { key, value } => {
            let api = Store {
                url: "http://localhost:3000".to_string()
            };

            let start = SystemTime::now();
            let now = match start.duration_since(UNIX_EPOCH) {
                Ok(val) => val.as_secs(),
                Err(_) => 0
            };

            let record = Record {
                key: key,
                value: value,
                // TODO: make this app error somehow
                date: now 
            };

            // api.add()
            println!("Record {:?}", record);
            Ok(0)
        }
        _ => {
            Err(AppError::UnsupportedError)
        },
    }

 }