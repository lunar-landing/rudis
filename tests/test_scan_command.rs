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
    fn test_scan_basic() {
        let mut con = setup();
        
        // 清理数据库
        let _: () = redis::cmd("FLUSHDB").exec(&mut con).unwrap();
        
        // 添加一些测试数据
        let _: () = con.set("scan-key1", "value1").unwrap();
        let _: () = con.set("scan-key2", "value2").unwrap();
        let _: () = con.set("scan-key3", "value3").unwrap();
        
        // 测试基本的SCAN命令
        let result: (i32, Vec<String>) = redis::cmd("SCAN").arg("0").query(&mut con).unwrap();
        let (cursor, keys) = result;

        // 验证返回的结果
        assert!(cursor >= 0);  // 游标应该大于等于0
        assert!(!keys.is_empty());
        // 检查至少包含我们设置的一个键
        assert!(keys.contains(&"scan-key1".to_string()) || keys.contains(&"scan-key2".to_string()) || keys.contains(&"scan-key3".to_string()));
    }

    #[test]
    fn test_scan_with_match() {
        let mut con = setup();
        
        // 清理数据库
        let _: () = redis::cmd("FLUSHDB").exec(&mut con).unwrap();
        
        // 添加一些测试数据
        let _: () = con.set("user:1", "value1").unwrap();
        let _: () = con.set("user:2", "value2").unwrap();
        let _: () = con.set("admin:1", "value3").unwrap();
        
        // 测试带MATCH参数的SCAN命令
        let result: (i32, Vec<String>) = redis::cmd("SCAN").arg("0").arg("MATCH").arg("user:*").query(&mut con).unwrap();
        let (cursor, keys) = result;
        
        // 验证返回的结果
        assert!(cursor >= 0);
        // 所有返回的键都应该匹配模式"user:*"
        for key in &keys {
            assert!(key.starts_with("user:"));
        }
    }

    #[test]
    fn test_scan_with_count() {
        let mut con = setup();
        
        // 清理数据库
        let _: () = redis::cmd("FLUSHDB").exec(&mut con).unwrap();
        
        // 添加一些测试数据
        for i in 0..20 {
            let _: () = con.set(format!("test_key_{}", i), format!("value_{}", i)).unwrap();
        }
        
        // 测试带COUNT参数的SCAN命令
        let result: (i32, Vec<String>) = redis::cmd("SCAN").arg("0").arg("COUNT").arg("5").query(&mut con).unwrap();
        let (cursor, keys) = result;
        
        // 验证返回的结果
        assert!(cursor >= 0);
        // 应该返回大约5个键（具体数量可能因实现而异）
        assert!(!keys.is_empty());
        assert!(keys.len() <= 10); // 给一些余地
    }

    #[test]
    fn test_scan_complete_iteration() {
        let mut con = setup();
        
        // 清理数据库
        let _: () = redis::cmd("FLUSHDB").exec(&mut con).unwrap();
        
        // 添加一些测试数据
        let test_keys: Vec<String> = (0..10).map(|i| format!("scan_test_key_{}", i)).collect();
        for key in &test_keys {
            let _: () = con.set(key, "test_value").unwrap();
        }
        
        // 完整迭代所有键
        let mut cursor = 0;
        let mut all_found_keys = Vec::new();
        
        loop {
            let result: (i32, Vec<String>) = redis::cmd("SCAN").arg(cursor).arg("MATCH").arg("scan_test_key_*").query(&mut con).unwrap();
            let (next_cursor, keys) = result;
            
            all_found_keys.extend(keys);
            
            cursor = next_cursor;
            if cursor == 0 {
                break;
            }
        }
        
        // 验证找到了所有键
        assert_eq!(all_found_keys.len(), test_keys.len());
        for key in &test_keys {
            assert!(all_found_keys.contains(key));
        }
    }
}