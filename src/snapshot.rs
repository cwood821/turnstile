use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Snapshot {
  pub key: String,
  pub value: u64, 
}

impl Snapshot { 
  pub fn new(key: &str, value: u64) -> Self {
    Snapshot {
      key: key.to_string(),
      value, 
    }
  }

  pub fn key(&self) -> String {
    self.key.to_string()
  }
}