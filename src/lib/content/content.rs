use data_encoding::HEXLOWER;
use sha2::{Sha256, Digest};
use crate::{DataOsUri, Namespace};

pub fn new_content_uri(object_uri: DataOsUri, bytes: Vec<u8>) -> DataOsUri {                        
    // create a Sha256 object
    let mut hasher = Sha256::new();
    
    // write input message
    hasher.update(bytes);
    
    // read hash digest and consume hasher
    let result = hasher.finalize();
    let h = HEXLOWER.encode(result.as_ref());
                 
    let resource = format!("{}/{}", object_uri.resource(), h);
    let domain = object_uri.domain().to_string();
    
    return DataOsUri::new(Namespace::Content, domain, resource);
}

#[cfg(test)]
mod tests {
    use super::{Namespace, DataOsUri};
    use super::new_content_uri;

    #[test]
    fn test_creating_a_new_content_uri() {        
        
        // Create the Schema URI
        let schema_uri = DataOsUri::new(
            Namespace::Schema,
            String::from("trade.biz"),
            String::from("ir-swap")
        );

        assert_eq!(schema_uri.to_string(), "schema://trade.biz/ir-swap");        

        // Create the object URI        
        let object_uri = crate::new_object_uri(schema_uri, String::from("1234"));

        assert_eq!(object_uri.to_string(), "object://trade.biz/ir-swap/1234");

        // Create the content URI
        let trade_payload = b"{\"trade_id\": 1234}".to_vec();
        let content_uri = new_content_uri(object_uri, trade_payload);

        assert_eq!(content_uri.to_string(), "content://trade.biz/ir-swap/1234/acf4581ee1fccf4303df459c8581aa2474d2b6249a867d0cc9d7ce9ce33ee32f");
    }
}
