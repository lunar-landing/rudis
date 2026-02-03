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
    fn test_sinterstore_basic() {
        let mut con = setup();
        
        // 使用唯一的键名避免测试间干扰
        let set1_key = "test_sinterstore_set1";
        let set2_key = "test_sinterstore_set2";
        let dest_key = "test_sinterstore_dest";
        
        // 清理测试数据
        let _: () = con.del(set1_key).unwrap();
        let _: () = con.del(set2_key).unwrap();
        let _: () = con.del(dest_key).unwrap();
        
        // 准备测试数据
        let _: () = con.sadd(set1_key, "a").unwrap();
        let _: () = con.sadd(set1_key, "b").unwrap();
        let _: () = con.sadd(set1_key, "c").unwrap();
        
        let _: () = con.sadd(set2_key, "b").unwrap();
        let _: () = con.sadd(set2_key, "c").unwrap();
        let _: () = con.sadd(set2_key, "d").unwrap();
        
        // 测试 SINTERSTORE 命令
        let result: i32 = redis::cmd("SINTERSTORE")
            .arg(dest_key)
            .arg(set1_key)
            .arg(set2_key)
            .query(&mut con)
            .unwrap();
        
        // 验证返回的元素数量
        assert_eq!(result, 2);
        
        // 验证目标集合的内容
        let members: Vec<String> = con.smembers(dest_key).unwrap();
        assert_eq!(members.len(), 2);
        assert!(members.contains(&"b".to_string()));
        assert!(members.contains(&"c".to_string()));
        
        // 清理测试数据
        let _: () = con.del(set1_key).unwrap();
        let _: () = con.del(set2_key).unwrap();
        let _: () = con.del(dest_key).unwrap();
    }

    #[test]
    fn test_sinterstore_multiple_sets() {
        let mut con = setup();
        
        // 使用唯一的键名避免测试间干扰
        let set1_key = "test_sinterstore_multi_set1";
        let set2_key = "test_sinterstore_multi_set2";
        let set3_key = "test_sinterstore_multi_set3";
        let dest_key = "test_sinterstore_multi_dest";
        
        // 清理测试数据
        let _: () = con.del(set1_key).unwrap();
        let _: () = con.del(set2_key).unwrap();
        let _: () = con.del(set3_key).unwrap();
        let _: () = con.del(dest_key).unwrap();
        
        // 准备测试数据
        let _: () = con.sadd(set1_key, "a").unwrap();
        let _: () = con.sadd(set1_key, "b").unwrap();
        let _: () = con.sadd(set1_key, "c").unwrap();
        let _: () = con.sadd(set1_key, "d").unwrap();
        
        let _: () = con.sadd(set2_key, "b").unwrap();
        let _: () = con.sadd(set2_key, "c").unwrap();
        let _: () = con.sadd(set2_key, "e").unwrap();
        
        let _: () = con.sadd(set3_key, "c").unwrap();
        let _: () = con.sadd(set3_key, "d").unwrap();
        let _: () = con.sadd(set3_key, "f").unwrap();
        
        // 测试 SINTERSTORE 命令，与多个集合做交集
        let result: i32 = redis::cmd("SINTERSTORE")
            .arg(dest_key)
            .arg(set1_key)
            .arg(set2_key)
            .arg(set3_key)
            .query(&mut con)
            .unwrap();
        
        // 验证返回的元素数量（只有 "c" 在所有三个集合中）
        assert_eq!(result, 1);
        
        // 验证目标集合的内容
        let members: Vec<String> = con.smembers(dest_key).unwrap();
        assert_eq!(members.len(), 1);
        assert!(members.contains(&"c".to_string()));
        
        // 清理测试数据
        let _: () = con.del(set1_key).unwrap();
        let _: () = con.del(set2_key).unwrap();
        let _: () = con.del(set3_key).unwrap();
        let _: () = con.del(dest_key).unwrap();
    }

    #[test]
    fn test_sinterstore_with_nonexistent_key() {
        let mut con = setup();
        
        // 使用唯一的键名避免测试间干扰
        let set1_key = "test_sinterstore_nonex_set1";
        let dest_key = "test_sinterstore_nonex_dest";
        
        // 清理测试数据
        let _: () = con.del(set1_key).unwrap();
        let _: () = con.del(dest_key).unwrap();
        
        // 准备测试数据
        let _: () = con.sadd(set1_key, "a").unwrap();
        let _: () = con.sadd(set1_key, "b").unwrap();
        
        // 测试 SINTERSTORE 命令，其中一个键不存在
        let result: i32 = redis::cmd("SINTERSTORE")
            .arg(dest_key)
            .arg(set1_key)
            .arg("nonexistent")
            .query(&mut con)
            .unwrap();
        
        // 验证返回的元素数量（不存在的键视为空集，交集为空）
        assert_eq!(result, 0);
        
        // 验证目标集合为空
        let members: Vec<String> = con.smembers(dest_key).unwrap();
        assert_eq!(members.len(), 0);
        
        // 清理测试数据
        let _: () = con.del(set1_key).unwrap();
        let _: () = con.del(dest_key).unwrap();
    }

    #[test]
    fn test_sinterstore_empty_result() {
        let mut con = setup();
        
        // 使用唯一的键名避免测试间干扰
        let set1_key = "test_sinterstore_empty_set1";
        let set2_key = "test_sinterstore_empty_set2";
        let dest_key = "test_sinterstore_empty_dest";
        
        // 清理测试数据
        let _: () = con.del(set1_key).unwrap();
        let _: () = con.del(set2_key).unwrap();
        let _: () = con.del(dest_key).unwrap();
        
        // 准备测试数据（没有共同元素）
        let _: () = con.sadd(set1_key, "a").unwrap();
        let _: () = con.sadd(set1_key, "b").unwrap();
        
        let _: () = con.sadd(set2_key, "c").unwrap();
        let _: () = con.sadd(set2_key, "d").unwrap();
        
        // 测试 SINTERSTORE 命令，结果应该为空
        let result: i32 = redis::cmd("SINTERSTORE")
            .arg(dest_key)
            .arg(set1_key)
            .arg(set2_key)
            .query(&mut con)
            .unwrap();
        
        // 验证返回的元素数量
        assert_eq!(result, 0);
        
        // 验证目标集合为空
        let members: Vec<String> = con.smembers(dest_key).unwrap();
        assert_eq!(members.len(), 0);
        
        // 清理测试数据
        let _: () = con.del(set1_key).unwrap();
        let _: () = con.del(set2_key).unwrap();
        let _: () = con.del(dest_key).unwrap();
    }

    #[test]
    fn test_sinterstore_overwrite_destination() {
        let mut con = setup();
        
        // 使用唯一的键名避免测试间干扰
        let set1_key = "test_sinterstore_overwrite_set1";
        let set2_key = "test_sinterstore_overwrite_set2";
        let dest_key = "test_sinterstore_overwrite_dest";
        
        // 清理测试数据
        let _: () = con.del(set1_key).unwrap();
        let _: () = con.del(set2_key).unwrap();
        let _: () = con.del(dest_key).unwrap();
        
        // 先设置目标集合
        let _: () = con.sadd(dest_key, "old1").unwrap();
        let _: () = con.sadd(dest_key, "old2").unwrap();
        
        // 准备源集合
        let _: () = con.sadd(set1_key, "a").unwrap();
        let _: () = con.sadd(set1_key, "b").unwrap();
        let _: () = con.sadd(set2_key, "b").unwrap();
        let _: () = con.sadd(set2_key, "c").unwrap();
        
        // 测试 SINTERSTORE 命令，应该覆盖目标集合
        let result: i32 = redis::cmd("SINTERSTORE")
            .arg(dest_key)
            .arg(set1_key)
            .arg(set2_key)
            .query(&mut con)
            .unwrap();
        
        // 验证返回的元素数量
        assert_eq!(result, 1);
        
        // 验证目标集合已被覆盖
        let members: Vec<String> = con.smembers(dest_key).unwrap();
        assert_eq!(members.len(), 1);
        assert!(members.contains(&"b".to_string()));
        assert!(!members.contains(&"old1".to_string()));
        
        // 清理测试数据
        let _: () = con.del(set1_key).unwrap();
        let _: () = con.del(set2_key).unwrap();
        let _: () = con.del(dest_key).unwrap();
    }
}
