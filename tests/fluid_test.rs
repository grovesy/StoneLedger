use stone_ledger::{InMemoryRegistry, MetaIndex, MetaIndexValue, Registry, RegistryInput};


#[test]
fn test_creating_a_new_object_uri() {
    // 0. Declare my application = we are a trade booking application in the swap-blotter feature
    let new_app_uri = stone_ledger::new_app_uri(String::from("booking.trade"), String::from("swap-blotter"));
    let app_uri = new_app_uri;
    
    // 1. Build a Schema URI
    let schema_uri = stone_ledger::new_schema_uri(String::from("trade.biz"), String::from("ir-swap"));        
    let object_uri = stone_ledger::new_object_uri(schema_uri.clone(), String::from("1234"));
   
    // 3. Build a content URI
    let trade_payload = b"{\"trade_id\": 1234}".to_vec();
    let content_uri = stone_ledger::new_content_uri(object_uri.clone(), trade_payload);
   
    // 4. Build a Meta record - this is our external index values.
    let index = MetaIndex::new(vec![
        MetaIndexValue::new(
            String::from("trade_id"),
            String::from("1234")
        )]);

    // 5, now for the Meta data record
    let meta_record = stone_ledger::MetaRecord::new(
        object_uri.clone(),
        schema_uri.clone(),
        content_uri,
        app_uri.clone(),
        app_uri.clone(),
        stone_ledger::Signature::new(
            app_uri, String::from("my-sig"), 
            String::from("my-apps-fingerprint")),
        mime::APPLICATION_JSON,
        vec![],
        index);

    assert_eq!(meta_record.id().to_string(), "meta://trade.biz/ir-swap/1234/7ad2cb9e3a2becf203d6662c83f0311fa709a92a9b48fcdff7d70c175627ec10");

    let input = RegistryInput::new(meta_record.clone());
    let mut registry = InMemoryRegistry::new();
    
    registry.add(input);
    
    match registry.get(meta_record.id().clone()) {
        Some(v) => {
            assert_eq!(v.meta_record().id().to_string(), "meta://trade.biz/ir-swap/1234/7ad2cb9e3a2becf203d6662c83f0311fa709a92a9b48fcdff7d70c175627ec10");
        },
        None => {
            assert!(false);
        }
    }


    println!("{}", meta_record);

}
