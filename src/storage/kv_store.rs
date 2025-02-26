use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct KVStore {
    store: HashMap<String, String>,
}

impl KVStore {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            store: HashMap::new(),
        }))
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.store.get(key).cloned()
    }

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn delete(&mut self, key: &str) -> bool {
        if !self.store.contains_key(key) {
            return false;
        }
        
        self.store.remove(key);

        return true;
    }
}
