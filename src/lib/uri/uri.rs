use std::fmt;
use strum_macros::EnumString;
use strum_macros::Display;

#[derive(Hash, Debug, Eq, PartialEq, Display, EnumString, Clone, Copy)]
pub enum Namespace {
    #[strum(serialize = "schema")]
    Schema,
    
    #[strum(serialize = "object")]
    Object,
    
    #[strum(serialize = "content")]
    Content,
    
    #[strum(serialize = "meta")]
    Meta,

    #[strum(serialize = "app")]
    App    
}

#[derive(Hash, Debug, Clone)]
pub struct DataOsUri {
    namespace: Namespace,
    domain: String,
    resource: String
}

impl DataOsUri {

    /// Returns a reference to the namespace of this [`DataOsUri`].
    pub fn namespace(&self) -> &Namespace {
        &self.namespace
    }

    /// Returns a reference to the domain of this [`DataOsUri`].
    pub fn domain(&self) -> &str {
        self.domain.as_ref()
    }

    /// Returns a reference to the resource of this [`DataOsUri`].
    pub fn resource(&self) -> &str {
        self.resource.as_ref()
    }

    /// Creates a new [`DataOsUri`].
    pub fn new(namespace: Namespace, domain: String, resource: String) -> Self {
        DataOsUri{
            namespace,
            domain,
            resource,
        }
    }
}

impl fmt::Display for DataOsUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&(format!("{}://{}/{}", self.namespace(), self.domain(), self.resource())))
    }
}

#[cfg(test)]
mod tests {
    use crate::Namespace;
    use crate::DataOsUri;
        
    #[test]
    fn test_namespace_to_string() {        
        assert_eq!(Namespace::Schema.to_string(), "schema");
        assert_eq!(Namespace::Object.to_string(), "object");
        assert_eq!(Namespace::Content.to_string(), "content");
        assert_eq!(Namespace::Meta.to_string(), "meta");
        assert_eq!(Namespace::App.to_string(), "app");
    }

    #[test]
    fn test_creating_a_new_uri() {
        let uri = DataOsUri {
            namespace: Namespace::Schema,
            domain: String::from("trade.biz"),
            resource: String::from("ir-swap"),
        };

        assert_eq!(uri.namespace(), &Namespace::Schema);
        assert_eq!(uri.domain(), "trade.biz");
        assert_eq!(uri.resource(), "ir-swap");
        assert_eq!(uri.to_string(), "schema://trade.biz/ir-swap");
    }
}