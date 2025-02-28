use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::protocol::createTable::ColumnDefinition;

#[derive(Debug, Clone)]
pub struct KVStore {
    store: HashMap<String, String>,
}

impl KVStore {
    pub fn new() -> Arc<Mutex<Self>> {
        return Arc::new(Mutex::new(Self {
            store: HashMap::new(),
        }));
    }

    pub fn create_database(&mut self, database_name: &str) {
        println!("Creating database: {}", database_name);
    }

    pub fn create_table(&mut self, table_name: &str, columns: &[ColumnDefinition], storage: &str) {
        println!("Creating table: {}", table_name);
        println!("Columns: {:?}", columns);
        println!("Storage: {}", storage);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.store.get(key).cloned()
    }

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    #[allow(dead_code)]
    pub fn delete(&mut self, key: &str) -> bool {
        if !self.store.contains_key(key) {
            return false;
        }
        
        self.store.remove(key);

        return true;
    }
}
