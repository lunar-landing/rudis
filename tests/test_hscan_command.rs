#[cfg(test)]
mod tests {
    use redis::{Client, Commands, Connection};

    fn setup() -> Connection {
        let client = Client::open("redis://127.0.0.1:6379/").unwrap();
        match client.get_connection() {
            Ok(conn) => conn,
            Err(e) => {
                eprintln!("Failed to get connection: {}", e);
                panic!("Failed to get connection: {}", e);
            }
        }
    }

    #[test]
    fn test_hscan_basic() {
        let mut con = setup();
        
        // clean up the database
        let _: () = con.del("hscan_basic").unwrap();
        
        // add some test data to the hash
        let _: () = con.hset("hscan_basic", "field1", "value1").unwrap();
        let _: () = con.hset("hscan_basic", "field2", "value2").unwrap();
        let _: () = con.hset("hscan_basic", "field3", "value3").unwrap();
        
        // test basic hscan commands
        let result: (i32, Vec<String>) = redis::cmd("HSCAN").arg("hscan_basic").arg("0").query(&mut con).unwrap();
        let (cursor, fields_values) = result;

        // validate the returned results
        assert_eq!(cursor, 0);  // since there are only 3 fields and the default COUNT is 10, all fields should be returned at once, with the cursor set to 0
        assert!(!fields_values.is_empty());
        assert_eq!(fields_values.len(), 6);  // 3 fields - value pairs = 6 elements
        
        // check the returned fields and values
        let mut found_fields = std::collections::HashSet::new();
        for i in (0..fields_values.len()).step_by(2) {
            if i + 1 < fields_values.len() {
                found_fields.insert(fields_values[i].clone());
            }
        }
        assert!(found_fields.contains("field1"));
        assert!(found_fields.contains("field2"));
        assert!(found_fields.contains("field3"));
    }

    #[test]
    fn test_hscan_with_match() {
        let mut con = setup();
        
        // clean up the database
        let _: () = con.del("hscan_match").unwrap();
        
        // add some test data to the hash
        let _: () = con.hset("hscan_match", "user:1", "value1").unwrap();
        let _: () = con.hset("hscan_match", "user:2", "value2").unwrap();
        let _: () = con.hset("hscan_match", "admin:1", "value3").unwrap();
        
        // test hscan commands with match parameters
        let result: (i32, Vec<String>) = redis::cmd("HSCAN").arg("hscan_match").arg("0").arg("MATCH").arg("user:*").query(&mut con).unwrap();
        let (cursor, fields_values) = result;
        
        // validate the returned results
        assert_eq!(cursor, 0);
        assert_eq!(fields_values.len(), 4);  // 2 fields - value pairs = 4 elements
        
        // all returned fields should match the pattern "user:*"
        for i in (0..fields_values.len()).step_by(2) {
            assert!(fields_values[i].starts_with("user:"));
        }
    }

    #[test]
    fn test_hscan_with_count() {
        let mut con = setup();
        
        // clean up the database
        let _: () = con.del("hscan_count").unwrap();
        
        // add some test data to the hash
        for i in 0..20 {
            let _: () = con.hset("hscan_count", format!("field_{}", i), format!("value_{}", i)).unwrap();
        }
        
        // test the hscan command with the count parameter
        let result: (i32, Vec<String>) = redis::cmd("HSCAN").arg("hscan_count").arg("0").arg("COUNT").arg("5").query(&mut con).unwrap();
        let (cursor, fields_values) = result;
        
        // validate the returned results
        assert!(cursor >= 0);
        // should return about 5 field-value pairs (exact number may vary by implementation)
        assert!(!fields_values.is_empty());
        assert!(fields_values.len() <= 20); // give some leeway (5 fields - value pairs = 10 elements, but maybe more)
    }

    #[test]
    fn test_hscan_nonexistent_key() {
        let mut con = setup();
        
        // clean up the database
        let _: () = con.del("hscan_nonexistent").unwrap();
        
        // tests for hscan commands that do not have keys
        let result: (i32, Vec<String>) = redis::cmd("HSCAN").arg("hscan_nonexistent").arg("0").query(&mut con).unwrap();
        let (cursor, fields_values) = result;
        
        // validate the returned results
        assert_eq!(cursor, 0);
        assert!(fields_values.is_empty());
    }

    #[test]
    fn test_hscan_wrong_type() {
        let mut con = setup();
        
        // clean up the database
        let _: () = con.del("hscan_wrong_type").unwrap();
        
        // add a key of type string
        let _: () = con.set("hscan_wrong_type", "value").unwrap();
        
        // test hscan commands for wrong type keys
        let result: Result<(i32, Vec<String>), redis::RedisError> = redis::cmd("HSCAN").arg("hscan_wrong_type").arg("0").query(&mut con);
        
        // verify return errors
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("wrong kind of value"));
    }

    #[test]
    fn test_hscan_complete_iteration() {
        let mut con = setup();
        
        // clean up the database
        let _: () = con.del("hscan_complete").unwrap();
        
        // add some test data to the hash
        let test_fields: Vec<String> = (0..10).map(|i| format!("field_{}", i)).collect();
        for field in &test_fields {
            let _: () = con.hset("hscan_complete", field, "test_value").unwrap();
        }
        
        // iterate on all fields completely
        let mut cursor = 0;
        let mut all_found_fields = std::collections::HashSet::new();
        
        loop {
            let result: (i32, Vec<String>) = redis::cmd("HSCAN").arg("hscan_complete").arg(cursor).arg("MATCH").arg("field_*").query(&mut con).unwrap();
            let (next_cursor, fields_values) = result;
            
            // Extract field name (every other element is the field name)
            for i in (0..fields_values.len()).step_by(2) {
                if i < fields_values.len() {
                    all_found_fields.insert(fields_values[i].clone());
                }
            }
            
            cursor = next_cursor;
            if cursor == 0 {
                break;
            }
        }
        
        // validation found all fields
        assert_eq!(all_found_fields.len(), test_fields.len());
        for field in &test_fields {
            assert!(all_found_fields.contains(field));
        }
    }
}

