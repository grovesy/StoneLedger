use crate::MetaRecord;

pub struct RegistryInput {
    meta_record: MetaRecord
}

impl RegistryInput {
    pub fn new(meta_record: MetaRecord) -> Self {
        Self {
            meta_record,
        } 
    }

    pub fn meta_record(&self) -> MetaRecord {
        return self.meta_record.clone()
    }
}
