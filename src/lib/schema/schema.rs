use crate::{DataOsUri, Namespace};

pub fn new_schema_uri(domain: String, model: String) -> DataOsUri {                
    return DataOsUri::new(Namespace::Schema, domain, model);
}

#[cfg(test)]
mod tests {
    use crate::Namespace;
    
    #[test]
    fn test_creating_a_new_schema_uri() {        
        let result = crate::new_schema_uri(String::from("trade.biz"), String::from("ir-swap"));

        assert_eq!(result.namespace(), &Namespace::Schema);
        assert_eq!(result.domain(), "trade.biz");
        assert_eq!(result.resource(), "ir-swap");
        assert_eq!(result.to_string(), "schema://trade.biz/ir-swap");
    }
}