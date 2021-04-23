
pub struct Store {
  pub url: String
}

enum StorageError {
  NetworkError,
  DoesNotExist
}

impl Store {
  pub fn get(self: Self, key: String) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let res = client.get(format!("{}/{}", self.url, key))
      .send()?;

    let text = res.text()?;
    Ok(text)
    // res.json()?
  }
}


