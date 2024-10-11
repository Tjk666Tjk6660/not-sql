use super::storage_engine::StorageEngine;

pub struct ExecutionEngine {
    storage_engine: StorageEngine,
}

impl ExecutionEngine {
    pub fn new(storage_engine: StorageEngine) -> Self {
        ExecutionEngine { storage_engine }
    }

    pub fn execute(&self) {}
}
