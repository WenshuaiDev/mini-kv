use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Store {
    map: HashMap<String, String>,
}

#[derive(Debug)]
#[allow(unused)]
pub enum StoreError {
    IOError(std::io::Error),
    SerdeError(serde_json::Error),
    KeyNotFound,
}

impl From<std::io::Error> for StoreError {
    fn from(err: std::io::Error) -> Self {
        StoreError::IOError(err)
    }
}

impl From<serde_json::Error> for StoreError {
    fn from (err: serde_json::Error) -> Self {
        StoreError::SerdeError(err)
    }
}

impl Store {
    pub fn new() -> Self {
        Store {
            map: HashMap::new(),
        }
    }

    pub fn load(path: &str) -> Result<Store, StoreError> {
        if !Path::new(path).exists() {
            return Ok(Store::new());
        }

        let data = std::fs::read_to_string(path)?;
        let store: Store = serde_json::from_str(&data)?;
        Ok(store)
    }

    pub fn save(&self, path: &str) -> Result<(), StoreError> {
        let data = serde_json::to_string_pretty(&self)?;
        let mut file = std::fs::File::create(path)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Result<String, StoreError> {
        self.map.get(key).cloned().ok_or(StoreError::KeyNotFound)
    }

    pub fn delete(&mut self, key: &str) -> Result<(), StoreError> {
        if self.map.remove(key).is_some() {
            Ok(())
        } else {
            Err(StoreError::KeyNotFound)
        }
    }
}