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

    fn hincrbyfloat(con: &mut Connection, key: &str, field: &str, increment: f64) -> RedisResult<String> {
        let mut cmd = redis::cmd("HINCRBYFLOAT");
        cmd.arg(key).arg(field).arg(increment);
        cmd.query(con)
    }

    #[test]
    fn test_hincrbyfloat_basic_operation() {
        let mut con = setup();
        
        // 确保测试键不存在
        let _: () = con.del("mykey").unwrap();
        
        // 测试1: 设置初始值并增加0.1
        let _: () = con.hset("mykey", "field", 10.50).unwrap();
        let result: String = hincrbyfloat(&mut con, "mykey", "field", 0.1).unwrap();
        assert_eq!(result, "10.6");
        
        // 测试2: 减少5
        let result: String = hincrbyfloat(&mut con, "mykey", "field", -5.0).unwrap();
        assert_eq!(result, "5.6");
        
        // 测试3: 使用科学计数法
        let _: () = con.hset("mykey", "field", "5.0e3").unwrap();
        let result: String = hincrbyfloat(&mut con, "mykey", "field", 2.0e2).unwrap();
        assert_eq!(result, "5200");
    }

    #[test]
    fn test_hincrbyfloat_field_not_exist() {
        let mut con = setup();
        
        // 确保测试键不存在
        let _: () = con.del("mykey_new").unwrap();
        
        // 测试: 对不存在的字段执行 HINCRBYFLOAT，字段值应初始化为0
        let result: String = hincrbyfloat(&mut con, "mykey_new", "new_field", 3.14).unwrap();
        assert_eq!(result, "3.14");
    }

    #[test]
    fn test_hincrbyfloat_key_not_exist() {
        let mut con = setup();
        
        // 确保测试键不存在
        let _: () = con.del("nonexistent_key").unwrap();
        
        // 测试: 对不存在的键执行 HINCRBYFLOAT，应创建新键和字段
        let result: String = hincrbyfloat(&mut con, "nonexistent_key", "new_field", 1.23).unwrap();
        assert_eq!(result, "1.23");
    }

    #[test]
    fn test_hincrbyfloat_error_handling() {
        let mut con = setup();
        
        // 测试: 在非哈希键上执行 HINCRBYFLOAT 应该返回错误
        let _: () = con.set("string_key_error", "value").unwrap();
        
        let result: RedisResult<String> = hincrbyfloat(&mut con, "string_key_error", "field", 1.0);
        assert!(result.is_err());
    }
}