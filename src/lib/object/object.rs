use crate::{DataOsUri, Namespace};

pub fn new_object_uri(schema_uri: DataOsUri, id: String) -> DataOsUri {        
    let namespace = Namespace::Object;
    let domain = String::from(schema_uri.domain());
    let resource = format!("{}/{}", schema_uri.resource(), id);    
    return DataOsUri::new(namespace, domain, resource);
}

#[cfg(test)]
mod tests {
    use crate::Namespace;
    use crate::DataOsUri;
    
    #[test]
    fn test_creating_a_new_object_uri() {
        let schema_uri = DataOsUri::new(Namespace::Schema, String::from("trade.biz"), String::from("ir-swap"));
        let id = String::from("1234");
        let result = crate::new_object_uri(schema_uri, id);

        assert_eq!(result.namespace(), &Namespace::Object);
        assert_eq!(result.domain(), "trade.biz");
        assert_eq!(result.resource(), "ir-swap/1234");
        assert_eq!(result.to_string(), "object://trade.biz/ir-swap/1234");
    }
}