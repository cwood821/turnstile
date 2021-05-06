use::serde::{Serialize, Deserialize};

pub struct Store {
  pub url: String
}

enum StorageError {
  NetworkError,
  ResponseFormatError,
  DoesNotExist
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
  pub key: String,
  pub value: f64,
  pub date: u64 
}

impl Store {

  pub fn get(self: Self, key: String, count: u64) -> Result<Vec<Record>, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let res = client.get(format!("{}/{}?count={}", self.url, key, count))
      .send()?;

    // TODO: map json parse error
    let record: Vec<Record> = res.json()?;

    Ok(record)
  }

  pub fn add(self: Self, record: Record) -> Result<Record, reqwest::Error> {
    let client = reqwest::blocking::Client::new();

    let res = client.post(self.url.to_string())
      .json(&record)
      .send()?;

    let rec: Record = res.json()?;

    Ok(rec)
  }
}


