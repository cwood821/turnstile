use crate::snapshot::Snapshot;
use std::fs::File;
use std::vec::Vec;
use reqwest::StatusCode;

use reqwest::Client;

#[derive(Debug)]
pub enum StorageError {
  FailedToStore,
  FailedToGet,
  FailedToParse,
  DoesNotExist
}

pub struct Storage {
  pub api_url: String,
  snapshots: Box<Vec<Snapshot>>
}

impl Storage {

  pub fn new (api_url: &str) -> Self {
    Storage {
      api_url: api_url.to_string(),
      snapshots: Box::new(Vec::new())
    }
  }

  pub fn has(&self, snapshot: &Snapshot) -> bool {
    match self.get(&snapshot) {
      Ok(_) => true,
      Err(StorageError::FailedToParse) => {
        false
      }
      Err(e) => {
        println!("{:?}", e);
        false
      }
    }
  } 

  pub fn get(&self, snapshot: &Snapshot) -> Result<Snapshot, StorageError>  {
    let url = format!("{}/{}", self.api_url, snapshot.key());

    let mut resp = reqwest::get(&url)
      .map_err(|e| StorageError::FailedToGet)?;
      
    match resp.status() {
      StatusCode::OK => {
        let json_val: Snapshot = resp.json().map_err(|e| StorageError::FailedToParse)?;
        Ok(json_val) 
      } 
      StatusCode::NOT_FOUND => {
        Err(StorageError::DoesNotExist)
      },
      // Could be any other status 500, 301, etc.
      _ => Err(StorageError::FailedToGet)
    }
  }

  pub fn store(&self, snapshot: &Snapshot) -> Result<Snapshot, StorageError> {
    let client = reqwest::Client::new();
    let url = format!("http://localhost:3000/{}", snapshot.key());

    let result: Snapshot = client.post(&url)
      .json(&snapshot)
      .send()
      .map_err(|e| StorageError::FailedToStore)?
      .json()
      .map_err(|e| StorageError::FailedToParse)?;

    Ok(result)
  }
}