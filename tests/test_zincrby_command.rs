#[cfg(test)]
mod tests {
    use redis::{Client, Commands, Connection, RedisResult};

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

    fn zincrby(con: &mut Connection, key: &str, increment: &str, member: &str) -> RedisResult<String> {
        redis::cmd("ZINCRBY").arg(key).arg(increment).arg(member).query(con)
    }

    #[test]
    fn test_zincrby_basic_operation() {
        let mut con = setup();
        
        // 确保测试键不存在
        let _: () = con.del("myzset_basic").unwrap();
        
        // 测试1: 对不存在的键和成员执行 ZINCRBY
        let result: String = zincrby(&mut con, "myzset_basic", "1.5", "member1").unwrap();
        assert_eq!(result, "1.5");
        
        // 测试2: 对已存在的成员执行 ZINCRBY
        let result: String = zincrby(&mut con, "myzset_basic", "2.5", "member1").unwrap();
        assert_eq!(result, "4"); // 1.5 + 2.5 = 4
        
        // 测试3: 使用负数增量
        let result: String = zincrby(&mut con, "myzset_basic", "-1", "member1").unwrap();
        assert_eq!(result, "3"); // 4 + (-1) = 3
        
        // 测试4: 对不存在的成员执行 ZINCRBY
        let result: String = zincrby(&mut con, "myzset_basic", "5", "member2").unwrap();
        assert_eq!(result, "5");
    }

    #[test]
    fn test_zincrby_verify_scores() {
        let mut con = setup();
        
        // 先设置测试数据
        let _: () = con.del("myzset_scores").unwrap();
        
        // 执行 ZINCRBY 操作
        let _: String = zincrby(&mut con, "myzset_scores", "1.5", "member1").unwrap();
        let _: String = zincrby(&mut con, "myzset_scores", "2.5", "member1").unwrap(); // 1.5 + 2.5 = 4
        let _: String = zincrby(&mut con, "myzset_scores", "-1", "member1").unwrap(); // 4 + (-1) = 3
        let _: String = zincrby(&mut con, "myzset_scores", "5", "member2").unwrap();
        
        // 验证成员的分数是否正确
        let score1: String = con.zscore("myzset_scores", "member1").unwrap();
        assert_eq!(score1, "3");
        
        let score2: String = con.zscore("myzset_scores", "member2").unwrap();
        assert_eq!(score2, "5");
    }

    #[test]
    fn test_zincrby_error_handling() {
        let mut con = setup();
        
        // 测试: 在非有序集合键上执行 ZINCRBY 应该返回错误
        let _: () = con.set("string_key_error", "value").unwrap();
        
        let result: RedisResult<String> = zincrby(&mut con, "string_key_error", "1", "member");
        assert!(result.is_err());
    }
}