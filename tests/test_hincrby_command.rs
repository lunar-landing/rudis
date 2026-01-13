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

    fn hincrby(con: &mut Connection, key: &str, field: &str, increment: i64) -> RedisResult<i64> {
        let mut cmd = redis::cmd("HINCRBY");
        cmd.arg(key).arg(field).arg(increment);
        cmd.query(con)
    }

    #[test]
    fn test_hincrby_basic_operation() {
        let mut con = setup();
        
        // 确保测试键不存在
        let _: () = con.del("myhash").unwrap();
        
        // 测试1: 对不存在的键和字段执行 HINCRBY
        let result: i64 = hincrby(&mut con, "myhash", "field1", 5).unwrap();
        assert_eq!(result, 5);
        
        // 测试2: 对已存在的字段执行 HINCRBY
        let result: i64 = hincrby(&mut con, "myhash", "field1", 1).unwrap();
        assert_eq!(result, 6);
        
        // 测试3: 使用负数增量
        let result: i64 = hincrby(&mut con, "myhash", "field1", -1).unwrap();
        assert_eq!(result, 5);
        
        // 测试4: 对不存在的字段执行 HINCRBY
        let result: i64 = hincrby(&mut con, "myhash", "field2", 10).unwrap();
        assert_eq!(result, 10);
    }

    #[test]
    fn test_hincrby_error_handling() {
        let mut con = setup();
        
        // 测试: 在非哈希键上执行 HINCRBY 应该返回错误
        let _: () = con.set("string_key_error", "value").unwrap();
        
        let result: RedisResult<i64> = hincrby(&mut con, "string_key_error", "field", 1);
        assert!(result.is_err());
    }
}