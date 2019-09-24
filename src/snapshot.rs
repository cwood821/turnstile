
#[derive(Debug)]
pub struct Snapshot {
  pub key: String,
  pub value: usize,  
  pub environment: String
}

impl Snapshot { 
  pub fn new(key: &str, value: usize, environment: &str) -> Self {
    Snapshot {
      key: key.to_string(),
      value, 
      environment: environment.to_string() 
    }
  }

  pub fn key(&self) -> String {
    self.key.to_string()
  }
}