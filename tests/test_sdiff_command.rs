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
    fn test_sdiff_basic() {
        let mut con = setup();
        
        // 使用唯一的键名避免测试间干扰
        let set1_key = "set1_basic";
        let set2_key = "set2_basic";
        
        // 准备测试数据
        let _: () = con.sadd(set1_key, "a").unwrap();
        let _: () = con.sadd(set1_key, "b").unwrap();
        let _: () = con.sadd(set1_key, "c").unwrap();
        
        let _: () = con.sadd(set2_key, "c").unwrap();
        let _: () = con.sadd(set2_key, "d").unwrap();
        let _: () = con.sadd(set2_key, "e").unwrap();
        
        // 测试SDIFF命令
        let result: Vec<String> = con.sdiff(&[set1_key, set2_key]).unwrap();
        
        // 验证结果
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"a".to_string()));
        assert!(result.contains(&"b".to_string()));
        
        // 清理测试数据
        let _: () = con.del(set1_key).unwrap();
        let _: () = con.del(set2_key).unwrap();
    }
    
    #[test]
    fn test_sdiff_with_nonexistent_key() {
        let mut con = setup();
        
        // 使用唯一的键名避免测试间干扰
        let set1_key = "set1_nonexistent";
        
        // 准备测试数据
        let _: () = con.sadd(set1_key, "a").unwrap();
        let _: () = con.sadd(set1_key, "b").unwrap();
        
        // 测试SDIFF命令，其中一个键不存在
        let result: Vec<String> = con.sdiff(&[set1_key, "nonexistent"]).unwrap();
        
        // 验证结果
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"a".to_string()));
        assert!(result.contains(&"b".to_string()));
        
        // 清理测试数据
        let _: () = con.del(set1_key).unwrap();
    }
    
    #[test]
    fn test_sdiff_empty_result() {
        let mut con = setup();
        
        // 使用唯一的键名避免测试间干扰
        let set1_key = "set1_empty";
        let set2_key = "set2_empty";
        
        // 准备测试数据
        let _: () = con.sadd(set1_key, "a").unwrap();
        let _: () = con.sadd(set1_key, "b").unwrap();
        
        let _: () = con.sadd(set2_key, "a").unwrap();
        let _: () = con.sadd(set2_key, "b").unwrap();
        
        // 测试SDIFF命令，结果应该为空
        let result: Vec<String> = con.sdiff(&[set1_key, set2_key]).unwrap();
        
        // 验证结果
        assert_eq!(result.len(), 0);
        
        // 清理测试数据
        let _: () = con.del(set1_key).unwrap();
        let _: () = con.del(set2_key).unwrap();
    }
    
    #[test]
    fn test_sdiff_multiple_sets() {
        let mut con = setup();
        
        // 使用唯一的键名避免测试间干扰
        let set1_key = "set1_multiple";
        let set2_key = "set2_multiple";
        let set3_key = "set3_multiple";
        
        // 准备测试数据
        let _: () = con.sadd(set1_key, "a").unwrap();
        let _: () = con.sadd(set1_key, "b").unwrap();
        let _: () = con.sadd(set1_key, "c").unwrap();
        let _: () = con.sadd(set1_key, "d").unwrap();
        
        let _: () = con.sadd(set2_key, "b").unwrap();
        let _: () = con.sadd(set3_key, "c").unwrap();
        
        // 测试SDIFF命令，与多个集合做差集
        let result: Vec<String> = con.sdiff(&[set1_key, set2_key, set3_key]).unwrap();
        
        // 验证结果
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"a".to_string()));
        assert!(result.contains(&"d".to_string()));
        
        // 清理测试数据
        let _: () = con.del(set1_key).unwrap();
        let _: () = con.del(set2_key).unwrap();
        let _: () = con.del(set3_key).unwrap();
    }
}