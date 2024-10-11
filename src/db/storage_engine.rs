use super::schema::{Row, Table};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub  struct StorageEngine {
    tables: HashMap<String, Table>,
}


impl StorageEngine {
    pub  fn new()-> Self {
        StorageEngine { tables: HashMap::new() }
    }

    pub  fn create_table(&mut self, name: String, columns: Vec<String>) {
        self.tables.insert(name, Table { columns, rows: Vec::new() });
    }
    pub  fn insert_row(&mut self, table_name: String, row: Row) {
        if let Some(table) =self.tables.get_mut(&table_name)  {
            table.rows.push(row);
        }
    }
    pub  fn serialize() {
        
    }
    pub fn deserialize() {
        
    }
}