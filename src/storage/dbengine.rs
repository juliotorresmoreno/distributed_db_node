use std::collections::HashMap;
use std::sync::{ Arc, Mutex };
use crate::protocol::statement::ColumnDefinition;

#[derive(Debug, Clone)]
pub struct DBEngine {
    databases: HashMap<String, Database>,
}

#[derive(Debug, Clone)]
pub struct Database {
    tables: HashMap<String, Table>,
}

#[derive(Debug, Clone)]
pub struct Table {
    columns: Vec<ColumnDefinition>,
    storage: String,
    rows: Vec<HashMap<String, String>>,
}

impl DBEngine {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(
            Mutex::new(Self {
                databases: HashMap::new(),
            })
        )
    }

    // =====================
    // Database Operations
    // =====================

    /// Creates a new database
    pub fn create_database(&mut self, database_name: &str) {
        println!("create_database: {}", database_name);
        // To be implemented later
    }

    /// Drops an existing database
    pub fn drop_database(&mut self, database_name: &str) {
        println!("drop_database: {}", database_name);
        // To be implemented later
    }

    /// Shows all databases
    pub fn show_databases(&self) -> Vec<String> {
        println!("show_databases");
        // To be implemented later
        Vec::new()
    }

    // =====================
    // Table Operations
    // =====================

    /// Creates a new table within a database
    pub fn create_table(
        &mut self,
        database_name: &str,
        table_name: &str,
        columns: &[ColumnDefinition],
        storage: &str
    ) {
        println!("create_table: {} in {}", table_name, database_name);
        println!("Columns: {:?}", columns);
        println!("Storage: {}", storage);
        // To be implemented later
    }

    /// Drops an existing table within a database
    pub fn drop_table(&mut self, database_name: &str, table_name: &str) {
        println!("drop_table: {} from {}", table_name, database_name);
        // To be implemented later
    }

    /// Alters an existing table within a database
    pub fn alter_table(&mut self, database_name: &str, table_name: &str) {
        println!("alter_table: {} in {}", table_name, database_name);
        // To be implemented later
    }

    /// Renames an existing table within a database
    pub fn rename_table(&mut self, database_name: &str, old_name: &str, new_name: &str) {
        println!("rename_table: {} to {} in {}", old_name, new_name, database_name);
        // To be implemented later
    }

    /// Describes the schema of a table
    pub fn describe_table(
        &self,
        database_name: &str,
        table_name: &str
    ) -> Option<Vec<ColumnDefinition>> {
        println!("describe_table: {} in {}", table_name, database_name);
        // To be implemented later
        None
    }

    // =====================
    // Data Operations
    // =====================

    /// Inserts a new row into a table
    pub fn insert(&mut self, database_name: &str, table_name: &str, row: HashMap<String, String>) {
        println!("insert into {} in {}", table_name, database_name);
        // To be implemented later
    }

    /// Selects all rows from a table
    pub fn select(&self, database_name: &str, table_name: &str) -> Vec<HashMap<String, String>> {
        println!("select from {} in {}", table_name, database_name);
        // To be implemented later
        Vec::new()
    }

    /// Updates rows in a table
    pub fn update(&mut self, database_name: &str, table_name: &str, row: HashMap<String, String>) {
        println!("update in {} in {}", table_name, database_name);
        // To be implemented later
    }

    /// Deletes rows from a table
    pub fn delete(&mut self, database_name: &str, table_name: &str) {
        println!("delete from {} in {}", table_name, database_name);
        // To be implemented later
    }
}
