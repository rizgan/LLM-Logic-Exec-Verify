use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    pub cache: HashMap<String, String>,
}

impl Cache {
    pub fn new() -> Cache {
        let mut cache = Cache {
            cache: HashMap::new(),
        };
        cache.restore();
        cache
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.cache.get(key)
    }

    pub fn set(&mut self, key: String, value: String) {
        self.cache.insert(key, value);
        self.save();
    }

    fn save(&mut self) {
        let json = serde_json::to_string(&self.cache).unwrap();
        std::fs::write("cache.json", json).unwrap();
    }

    fn restore(&mut self)  {
        if !std::path::Path::new("cache.json").exists() {
            return;
        }
        let json = std::fs::read_to_string("cache.json").unwrap();
        self.cache = serde_json::from_str(&json).unwrap();
    }
}

