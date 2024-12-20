use crate::{DataOsUri, Namespace};

pub fn new_app_uri(app_domain: String , app_component: String) -> DataOsUri {            
    return DataOsUri::new(Namespace::App, app_domain, app_component);
}

#[cfg(test)]
mod tests {
    use crate::Namespace;
    
    #[test]
    fn test_creating_a_new_object_uri() {        
        let app_domain = String::from("booking.trade.net");
        let component = String::from("trade-booking-blotter");
        let result = crate::new_app_uri(app_domain, component);

        assert_eq!(result.namespace(), &Namespace::App);
        assert_eq!(result.domain(), "booking.trade.net");
        assert_eq!(result.resource(), "trade-booking-blotter");
        assert_eq!(result.to_string(), "app://booking.trade.net/trade-booking-blotter");
    }
}