use crate::MetaRecord;


#[derive()]
pub struct RegistryReadResult {
    meta_record: MetaRecord
}

impl RegistryReadResult {
    pub fn new(meta_record: MetaRecord) -> Self { 
        Self { 
            meta_record
        } 
    }

    pub fn meta_record(&self) -> &MetaRecord {
        &self.meta_record
    }
}
