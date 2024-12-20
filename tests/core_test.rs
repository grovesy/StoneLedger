use stone_ledger::{MetaIndex, MetaIndexValue};


#[test]
fn test_creating_a_new_object_uri() {

    // 0. Declare my application = we are a trade booking application in the swap-blotter feature
    let app_uri = stone_ledger::new_app_uri(String::from("booking.trade"), String::from("swap-blotter"));
    
    // 1. Build a Schema URI
    let schema_uri = stone_ledger::new_schema_uri(String::from("trade.biz"), String::from("ir-swap"));
    assert_eq!(schema_uri.to_string(), "schema://trade.biz/ir-swap");        

    // 2. Build an object URI        
    let object_uri = stone_ledger::new_object_uri(schema_uri.clone(), String::from("1234"));
    assert_eq!(object_uri.to_string(), "object://trade.biz/ir-swap/1234");

    // 3. Build a content URI
    let trade_payload = b"{\"trade_id\": 1234}".to_vec();
    let content_uri = stone_ledger::new_content_uri(object_uri.clone(), trade_payload);
    assert_eq!(content_uri.to_string(), "content://trade.biz/ir-swap/1234/acf4581ee1fccf4303df459c8581aa2474d2b6249a867d0cc9d7ce9ce33ee32f");

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

    println!("{}", meta_record);

}
