use crate::snapshot::Snapshot;
use std::fs::File;
use std::io::Write;

use reqwest::Client;

pub enum StorageError {
  FAILED_TO_STORE
}


pub trait Repository {
    fn store(snapshot: Snapshot) -> Result<(), StorageError>;
    fn get_count(project: &str, branch: &str) -> usize;
}

pub struct Storage {
  // Some storage system that implements a storage trait
}

impl Storage {

  pub fn store(&self, snapshot: Snapshot) -> Result<(), StorageError> {
    let mut file = File::create("foo.txt").map_err(|e| StorageError::FAILED_TO_STORE)?;
    file.write_all(b"Hello, world!").map_err(|e| StorageError::FAILED_TO_STORE)?;

    // Make storage request to API for some key
    let url = format!("localhost:3000/{}", snapshot.key());
    let body = reqwest::get(&url).map_err(|e| StorageError::FAILED_TO_STORE)?;

    // println!("{}", );
    Ok(())
  }
}