use::serde::Deserialize;

pub struct Store {
  pub url: String
}

enum StorageError {
  NetworkError,
  ResponseFormatError,
  DoesNotExist
}

#[derive(Deserialize, Debug)]
pub struct Record {
  pub key: String,
  pub value: f64,
  pub date: i32 
}

impl Store {
  pub fn get(self: Self, key: String) -> Result<Record, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    println!("pre req");
    let res = client.get(format!("{}/{}", self.url, key))
      .send()?;

    // TODO: map json parse error
    let record: Record = res.json()?;

    Ok(record)
  }
}


