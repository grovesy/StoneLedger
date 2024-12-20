use std::fmt;
use mime::Mime;
use crate::{MetaIndex, Temporality, DataOsUri, Signature, hashing};


#[derive(Clone)]
pub struct MetaRecord {
    id: DataOsUri,
    object_uri: DataOsUri,
    schema_uri: DataOsUri,
    content_uri: DataOsUri,
    source_app_uri: DataOsUri,
    publisher_app_uri: DataOsUri,
    content_signature: Signature,
    system_valid: Temporality,
    content_type: Mime,
    object_trace: Vec<Signature>,
    index: MetaIndex,
}

impl MetaRecord {
    pub fn new(
            object_uri: DataOsUri,
            schema_uri: DataOsUri,
            content_uri: DataOsUri,
            source_app_uri: DataOsUri,
            publisher_app_uri: DataOsUri,
            content_signature: Signature,
            content_type: Mime,
            object_trace: Vec<Signature>,
            index: MetaIndex) -> Self {
        
        // Create the hash of the meta records data
        let mut to_hash = String::new();

        // Push the data to the string to be hashed
        to_hash.push_str(&object_uri.to_owned().to_string());
        to_hash.push_str(";");
        to_hash.push_str(&content_signature.signature());
        to_hash.push_str(";");

        // Create the hash
        let meta_hash = hashing::hash_string(to_hash);

        // Create the meta URI in the formation of meta://<schema_domain>/<schema_model>/<hash>
        let resource = format!("{}/{}", object_uri.resource(), meta_hash);
        let id = DataOsUri::new(
            crate::Namespace::Meta,
            schema_uri.domain().to_string(),
            resource);

        MetaRecord {
            id,
            object_uri,
            schema_uri,
            content_uri,
            system_valid: Temporality::new(),
            content_type,
            index,
            source_app_uri,
            publisher_app_uri,
            content_signature,
            object_trace,
        }
    }

    pub fn id(&self) -> &DataOsUri {
        &self.id
    }

    pub fn object_uri(&self) -> &DataOsUri {
        &self.object_uri
    }

    pub fn schema_uri(&self) -> &DataOsUri {
        &self.schema_uri
    }

    pub fn content(&self) -> &DataOsUri {
        &self.content_uri
    }

    pub fn system_valid(&self) -> &Temporality {
        &self.system_valid
    }

    pub fn content_type(&self) -> &str {
        self.content_type.as_ref()
    }

    pub fn index(&self) -> &MetaIndex {
        &self.index
    }

    pub fn source_app_uri(&self) -> &DataOsUri {
        &self.source_app_uri
    }

    pub fn publisher_app_uri(&self) -> &DataOsUri {
        &self.publisher_app_uri
    }

    pub fn content_signature(&self) -> &Signature {
        &self.content_signature
    }

    pub fn object_trace(&self) -> &[Signature] {
        self.object_trace.as_ref()
    }

}

impl fmt::Display for MetaRecord {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&(
            format!(
                "[id:{}, object_uri: {}, schema_uri: {}, content_uri: {}, source_app_uri: {}, index: {}", 
                self.id(), 
                self.object_uri(),
                self.schema_uri(),
                self.content(),
                self.source_app_uri(),
                self.index)
        ))
    }
}