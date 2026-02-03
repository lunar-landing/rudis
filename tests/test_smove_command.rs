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
    fn test_smove_basic() {
        let mut con = setup();
        
        // 使用唯一的键名避免测试间干扰
        let source_key = "test_smove_source";
        let dest_key = "test_smove_dest";
        
        // 清理测试数据
        let _: () = con.del(source_key).unwrap();
        let _: () = con.del(dest_key).unwrap();
        
        // 准备测试数据
        let _: () = con.sadd(source_key, "a").unwrap();
        let _: () = con.sadd(source_key, "b").unwrap();
        let _: () = con.sadd(source_key, "c").unwrap();
        
        let _: () = con.sadd(dest_key, "x").unwrap();
        let _: () = con.sadd(dest_key, "y").unwrap();
        
        // 测试 SMOVE 命令
        let result: i32 = redis::cmd("SMOVE")
            .arg(source_key)
            .arg(dest_key)
            .arg("a")
            .query(&mut con)
            .unwrap();
        
        // 验证返回结果
        assert_eq!(result, 1);
        
        // 验证源集合中已移除该成员
        let source_members: Vec<String> = con.smembers(source_key).unwrap();
        assert_eq!(source_members.len(), 2);
        assert!(!source_members.contains(&"a".to_string()));
        assert!(source_members.contains(&"b".to_string()));
        assert!(source_members.contains(&"c".to_string()));
        
        // 验证目标集合中已添加该成员
        let dest_members: Vec<String> = con.smembers(dest_key).unwrap();
        assert_eq!(dest_members.len(), 3);
        assert!(dest_members.contains(&"a".to_string()));
        assert!(dest_members.contains(&"x".to_string()));
        assert!(dest_members.contains(&"y".to_string()));
        
        // 清理测试数据
        let _: () = con.del(source_key).unwrap();
        let _: () = con.del(dest_key).unwrap();
    }

    #[test]
    fn test_smove_member_not_in_source() {
        let mut con = setup();
        
        // 使用唯一的键名避免测试间干扰
        let source_key = "test_smove_not_in_source";
        let dest_key = "test_smove_not_in_dest";
        
        // 清理测试数据
        let _: () = con.del(source_key).unwrap();
        let _: () = con.del(dest_key).unwrap();
        
        // 准备测试数据
        let _: () = con.sadd(source_key, "a").unwrap();
        let _: () = con.sadd(source_key, "b").unwrap();
        
        let _: () = con.sadd(dest_key, "x").unwrap();
        
        // 测试 SMOVE 命令，成员不在源集合中
        let result: i32 = redis::cmd("SMOVE")
            .arg(source_key)
            .arg(dest_key)
            .arg("c")
            .query(&mut con)
            .unwrap();
        
        // 验证返回结果（应该返回 0）
        assert_eq!(result, 0);
        
        // 验证源集合没有变化
        let source_members: Vec<String> = con.smembers(source_key).unwrap();
        assert_eq!(source_members.len(), 2);
        
        // 验证目标集合没有变化
        let dest_members: Vec<String> = con.smembers(dest_key).unwrap();
        assert_eq!(dest_members.len(), 1);
        assert!(!dest_members.contains(&"c".to_string()));
        
        // 清理测试数据
        let _: () = con.del(source_key).unwrap();
        let _: () = con.del(dest_key).unwrap();
    }

    #[test]
    fn test_smove_to_nonexistent_destination() {
        let mut con = setup();
        
        // 使用唯一的键名避免测试间干扰
        let source_key = "test_smove_to_nonex_source";
        let dest_key = "test_smove_to_nonex_dest";
        
        // 清理测试数据
        let _: () = con.del(source_key).unwrap();
        let _: () = con.del(dest_key).unwrap();
        
        // 准备测试数据
        let _: () = con.sadd(source_key, "a").unwrap();
        let _: () = con.sadd(source_key, "b").unwrap();
        
        // 测试 SMOVE 命令，目标集合不存在（应该创建）
        let result: i32 = redis::cmd("SMOVE")
            .arg(source_key)
            .arg(dest_key)
            .arg("a")
            .query(&mut con)
            .unwrap();
        
        // 验证返回结果
        assert_eq!(result, 1);
        
        // 验证源集合中已移除该成员
        let source_members: Vec<String> = con.smembers(source_key).unwrap();
        assert_eq!(source_members.len(), 1);
        assert!(!source_members.contains(&"a".to_string()));
        
        // 验证目标集合已创建并包含该成员
        let dest_members: Vec<String> = con.smembers(dest_key).unwrap();
        assert_eq!(dest_members.len(), 1);
        assert!(dest_members.contains(&"a".to_string()));
        
        // 清理测试数据
        let _: () = con.del(source_key).unwrap();
        let _: () = con.del(dest_key).unwrap();
    }

    #[test]
    fn test_smove_member_already_in_destination() {
        let mut con = setup();
        
        // 使用唯一的键名避免测试间干扰
        let source_key = "test_smove_already_dest_source";
        let dest_key = "test_smove_already_dest_dest";
        
        // 清理测试数据
        let _: () = con.del(source_key).unwrap();
        let _: () = con.del(dest_key).unwrap();
        
        // 准备测试数据（成员在两个集合中都存在）
        let _: () = con.sadd(source_key, "a").unwrap();
        let _: () = con.sadd(source_key, "b").unwrap();
        
        let _: () = con.sadd(dest_key, "a").unwrap();
        let _: () = con.sadd(dest_key, "x").unwrap();
        
        // 测试 SMOVE 命令，成员已在目标集合中
        let result: i32 = redis::cmd("SMOVE")
            .arg(source_key)
            .arg(dest_key)
            .arg("a")
            .query(&mut con)
            .unwrap();
        
        // 验证返回结果（应该返回 1，因为移动成功）
        assert_eq!(result, 1);
        
        // 验证源集合中已移除该成员
        let source_members: Vec<String> = con.smembers(source_key).unwrap();
        assert_eq!(source_members.len(), 1);
        assert!(!source_members.contains(&"a".to_string()));
        
        // 验证目标集合中仍然包含该成员（不重复）
        let dest_members: Vec<String> = con.smembers(dest_key).unwrap();
        assert_eq!(dest_members.len(), 2);
        assert!(dest_members.contains(&"a".to_string()));
        assert!(dest_members.contains(&"x".to_string()));
        
        // 清理测试数据
        let _: () = con.del(source_key).unwrap();
        let _: () = con.del(dest_key).unwrap();
    }

    #[test]
    fn test_smove_source_does_not_exist() {
        let mut con = setup();
        
        // 使用唯一的键名避免测试间干扰
        let source_key = "test_smove_source_not_exist";
        let dest_key = "test_smove_source_not_exist_dest";
        
        // 清理测试数据
        let _: () = con.del(source_key).unwrap();
        let _: () = con.del(dest_key).unwrap();
        
        // 准备目标集合
        let _: () = con.sadd(dest_key, "x").unwrap();
        
        // 测试 SMOVE 命令，源集合不存在
        let result: i32 = redis::cmd("SMOVE")
            .arg(source_key)
            .arg(dest_key)
            .arg("a")
            .query(&mut con)
            .unwrap();
        
        // 验证返回结果（应该返回 0）
        assert_eq!(result, 0);
        
        // 验证目标集合没有变化
        let dest_members: Vec<String> = con.smembers(dest_key).unwrap();
        assert_eq!(dest_members.len(), 1);
        assert!(!dest_members.contains(&"a".to_string()));
        
        // 清理测试数据
        let _: () = con.del(source_key).unwrap();
        let _: () = con.del(dest_key).unwrap();
    }

    #[test]
    fn test_smove_wrong_type() {
        let mut con = setup();
        
        // 使用唯一的键名避免测试间干扰
        let source_key = "test_smove_wrong_type_source";
        let dest_key = "test_smove_wrong_type_dest";
        
        // 清理测试数据
        let _: () = con.del(source_key).unwrap();
        let _: () = con.del(dest_key).unwrap();
        
        // 创建一个字符串类型的键（不是集合）
        let _: () = con.set(source_key, "string_value").unwrap();
        
        // 准备目标集合
        let _: () = con.sadd(dest_key, "x").unwrap();
        
        // 测试 SMOVE 命令，源键类型错误
        let result: Result<i32, _> = redis::cmd("SMOVE")
            .arg(source_key)
            .arg(dest_key)
            .arg("a")
            .query(&mut con);
        
        // 验证返回错误
        assert!(result.is_err() || result.unwrap() == 0);
        
        // 清理测试数据
        let _: () = con.del(source_key).unwrap();
        let _: () = con.del(dest_key).unwrap();
    }
}
