use std::collections::HashMap;
use crate::{MetaRecord, DataOsUri};
use super::{registry::Registry, RegistryInput, RegistryReadResult, RegistryWriteResult};

pub struct InMemoryRegistry {
    store: HashMap<String, MetaRecord>
}

impl Registry for InMemoryRegistry {

    fn new() -> Self {
        
        InMemoryRegistry {
            store: HashMap::new()
        }
    }

    fn get(&self, uri: DataOsUri) -> Option<RegistryReadResult> {
        let uri_string = uri.to_string();
                
        match self.store.get(&uri_string) {
            Some(v) => {
                let result = RegistryReadResult::new(v.clone());
                return Some(result);
            }
            
            None => return None
        }
    }

    fn add(&mut self, input: RegistryInput) -> RegistryWriteResult {
        self.store.insert(input.meta_record().id().to_string(), input.meta_record());
        RegistryWriteResult::new()
    }
}

impl Default for InMemoryRegistry {
    fn default() -> Self {
        Self::new()
    }
}