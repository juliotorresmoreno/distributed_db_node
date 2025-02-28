use std::collections::HashMap;
use std::sync::{ Arc, Mutex };

use crate::protocol::statement::ColumnDefinition;

#[derive(Debug, Clone)]
pub struct Engine {
    #[allow(dead_code)]
    databases: HashMap<String, Database>,
}

#[derive(Debug, Clone)]
pub struct Database {
    #[allow(dead_code)]
    tables: HashMap<String, Table>,
}

#[derive(Debug, Clone)]
pub struct Table {
    #[allow(dead_code)]
    columns: Vec<ColumnDefinition>,
    
    #[allow(dead_code)]
    storage: String,

    #[allow(dead_code)]
    rows: Vec<HashMap<String, String>>,
}

impl Engine {
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
        return Vec::new();
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

    pub fn truncate_table(&mut self, database_name: &str, table_name: &str) {
        println!("truncate_table: {} in {}", table_name, database_name);
        // To be implemented later
    }

    pub fn show_tables(&self, database_name: &str) -> Vec<String> {
        println!("show_tables in {}", database_name);
        // To be implemented later
        return Vec::new();
    }

    pub fn describe_table(&self, database_name: &str, table_name: &str) -> Vec<ColumnDefinition> {
        println!("describe_table: {} in {}", table_name, database_name);
        // To be implemented later
        return Vec::new();
    }

    // =====================
    // Index Operations
    // =====================
    pub fn create_index(
        &mut self,
        database_name: &str,
        table_name: &str,
        index_name: &str,
        columns: &Vec<String>,
        unique: bool
    ) {
        println!("create_index: {} in {} on {}", index_name, database_name, table_name);
        println!("Columns: {:?}", columns);
        println!("Unique: {}", unique);
        // To be implemented later
    }

    pub fn drop_index(&mut self, database_name: &str, table_name: &str, index_name: &str) {
        println!("drop_index: {} in {} from {}", index_name, database_name, table_name);
        // To be implemented later
    }

    pub fn show_indexes(&self, database_name: &str, table_name: &str) -> Vec<String> {
        println!("show_indexes in {} from {}", database_name, table_name);
        // To be implemented later
        return Vec::new();
    }

    // =====================
    // Data Operations
    // =====================

    /// Inserts a new row into a table
    pub fn insert(
        &mut self,
        database_name: &str,
        table_name: &str,
        row: &Vec<HashMap<String, String>>
    ) {
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
    pub fn update(&mut self, database_name: &str, table_name: &str, row: &HashMap<String, String>) {
        println!("update in {} in {}", table_name, database_name);
        // To be implemented later
    }

    /// Deletes rows from a table
    pub fn delete(&mut self, database_name: &str, table_name: &str) {
        println!("delete from {} in {}", table_name, database_name);
        // To be implemented later
    }

    /// Bulk inserts rows into a table
    pub fn bulk_insert(
        &mut self,
        database_name: &str,
        table_name: &str,
        rows: &Vec<String>,
        values: &Vec<HashMap<String, String>>
    ) {
        println!("bulk_insert into {} in {}", table_name, database_name);
        // To be implemented later
    }

    /// Upserts rows into a table
    pub fn upsert(&mut self, database_name: &str, table_name: &str, row: &HashMap<String, String>) {
        println!("upsert into {} in {}", table_name, database_name);
        // To be implemented later
    }

    // =====================
    // Transaction Management
    // =====================

    /// Begins a new transaction
    pub fn begin_transaction(&mut self, transaction_id: &str, isolation_level: &Option<String>) {
        println!("begin_transaction: {}", transaction_id);
        // To be implemented later
    }
}
