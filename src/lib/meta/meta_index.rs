pub(crate) use std::fmt;

#[derive(Hash, Debug, Clone)]
pub struct MetaIndex{
    items: Vec<MetaIndexValue>
}

impl MetaIndex {
    pub fn new(items: Vec<MetaIndexValue>) -> Self { Self { items } }

    pub fn items(&self) -> &[MetaIndexValue] {
        self.items.as_ref()
    }
}

#[derive(Hash, Debug, Clone)]
pub struct MetaIndexValue{
    key: String,
    value: String,
}

impl MetaIndexValue {
    pub fn new(key: String, value: String) -> Self { Self { key, value } }
    
    pub fn key(&self) -> &str {
        self.key.as_ref()
    }

    pub fn value(&self) -> &str {
        self.value.as_ref()
    }
}

impl fmt::Display for MetaIndexValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&(format!("{}:{}", self.key(), self.value())))
    }
}

impl fmt::Display for MetaIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        _ = f.write_str("[");
        for x in &self.items {
            _ =f.write_str(&(format!("[{}:{}];", x.key(), x.value())));
        }
        f.write_str("]")
    }
}