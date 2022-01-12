use::serde::{Serialize, Deserialize};
use elasticsearch::{
  Elasticsearch, 
  Error as ElasticError,
  http::transport::Transport,
  cat::CatIndicesParts,
  indices::{IndicesCreateParts},
  SearchParts,
  IndexParts
};
use serde_json::json;
use serde_json::Value;
use std::io::{Error, ErrorKind};

pub struct Store {
  pub url: String
}

pub enum StorageError {
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

type Key = String;

#[derive(Serialize, Deserialize, Debug)]
struct RecordsResult {
  pub data: Vec<Record>
}

#[derive(Serialize, Deserialize, Debug)]
struct KeysResult {
  pub data: Vec<Key>
}

impl From<elasticsearch::Error> for StorageError {
  fn from(error: elasticsearch::Error) -> Self {
    StorageError::NetworkError
  }
}

impl Store {

  pub async fn get(self: Self, key: String, count: u64) -> Result<Record, StorageError> {
    let transport = Transport::single_node("http://localhost:9200")?;
    let client = Elasticsearch::new(transport);

    // Search for key, then sort by date and pop most recent
    let response = client
      .search(SearchParts::Index(&["test"]))
      .from(0)
      .size(1)
      .body(json!({
          "size": "5",
          "sort" : [
            { 
              "date" : {
                "format": "epoch_millis",
                "order": "desc"
              }
            },
          ],
          "query": {
              "match": {
                  "key": key 
              }
          }
      }))
      .send()
      .await?;

    if (response.status_code().is_success()) {
      let response_body = response.json::<Value>().await?;
      let resp =  &response_body["hits"]["hits"].as_array().unwrap()[0]["_source"]; 

      let record = Record {
        value: resp.get("value").unwrap().as_f64().unwrap(),
        date: resp.get("date").unwrap().as_u64().unwrap(),
        key: resp.get("key").unwrap().to_string()
      };

      Ok(record)
    } else {
      return Err(StorageError::NetworkError) 
    }

  }

  pub async fn add(self: Self, record: Record) -> Result<Record, elasticsearch::Error> {
    // let client = reqwest::blocking::Client::new();

    // let res = client.post(self.url.to_string())
    //   .json(&record)
    //   .send()?;

      let transport = Transport::single_node("http://localhost:9200")?;
      let client = Elasticsearch::new(transport);
  
      let response = client
        .index(IndexParts::Index("test"))
        .body(json!({
            "key": record.key,
            "value": record.value,
            "date": record.date
        }))
        .send()
        .await?;

      println!("Added record");
  
      let successful = response.status_code().is_success();

      // let rec: Record = response.json()?;
      let rec = Record {
        value: 1.0,
        key: "test".to_string(),
        date: 123
      };

      Ok(rec)
  }

  pub fn get_keys(self: Self) -> Result<Vec<String>, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let res = client.get(format!("{}/keys", self.url))
      .send()?;

    // TODO: map json parse error
    let result: KeysResult = res.json()?;

    Ok(result.data)
  }

  pub async fn create_index(self: Self, name: &str) -> Result<(), elasticsearch::Error> {
    // TODO: this will need to become cloud-id based for auth and paramaterized by env values
    let transport = Transport::single_node("http://localhost:9200")?;
    let client = Elasticsearch::new(transport);

    // TODO: check if this index already exists and signal that it does to the user
    let response = client
        .indices()
        .create(IndicesCreateParts::Index(name))
        .body(json!({
            "mappings" : {
                "properties" : {
                    "key" : { 
                      "type" : "text",
                      "fields": {
                        "raw": { 
                          "type":  "keyword"
                        }
                      }
                    },
                    "value" : { 
                      "type" : "double" 
                    },
                    "date": {
                      "type":   "date",
                      "format": "yyyy-MM-dd HH:mm:ss||yyyy-MM-dd||epoch_millis"
                    }
                }
            }
    }))
    .send()
    .await?;

    println!("Created index {}", name);
    Ok(())
  }

}


