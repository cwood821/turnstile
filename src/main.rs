mod conf;
mod store;

use std::time::{SystemTime, UNIX_EPOCH};
use std::cmp::Ordering;
use std::env;
use std::process;
use structopt::StructOpt;
use conf::Turnstile;
use store::{Store, Record};
use std::io::{Error, ErrorKind};
use elasticsearch::{
    Elasticsearch, Error as ElasticError,
    http::transport::Transport,
    cat::CatIndicesParts,
    indices::{IndicesCreateParts}
};
use serde_json::json;

enum AppError {
    UnsupportedError = 64,
    IOError = 74,
    ConfigurationError = 78,
    ApplicationError = 70
}

impl From<elasticsearch::Error> for AppError {
    fn from(_: ElasticError) -> Self {
        return AppError::ApplicationError;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::process::exit(match app().await {
        Ok(_) => 0,
        Err(err) => err as i32
    });
}

async fn app() -> Result<i32, AppError> {
    // let base = env::var("API_URL").map_err(|_| AppError::ConfigurationError)?;
    let base = "http://localhost:3000";
    let api = Store {
        url: base.to_string()
    };

    match Turnstile::from_args() {
        Turnstile::Get { key, count } => {
            // TODO: Imple formatter for json/text
            let count = count.unwrap_or(1);

            match api.get(key, count) {
                Ok(records) => {
                    for record in records {
                        println!("{}", record.value);
                    }
                    Ok(0)
                }
                Err(_) => {
                    Err(AppError::IOError)
                }
            }
        },
        Turnstile::Record { key, value, date } => {
            let now = if date.is_some() {
                date.unwrap()
            } else {
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|val| val.as_secs() - 1540)
                    .map_err(|_| AppError::ApplicationError)?
            };

            let record = Record {
                key: key,
                value: value,
                date: now 
            };

            match api.add(record).await {
                Ok(_record) => {
                    Ok(0)
                }
                Err(_) => {
                    Err(AppError::IOError)
                }
            }
        },
        Turnstile::Keys { } => {
            match api.get_keys() {
                Ok(keys) => {
                    for key in keys {
                        println!("{}", key);
                    }
                    Ok(0)
                }
                Err(_) => {
                    Err(AppError::IOError)
                }
            }
        },
        Turnstile::Index { name } => {
            api.create_index(&name).await?;
            Ok(0)
        },
        _ => {
            Err(AppError::UnsupportedError)
        },
    }
 }