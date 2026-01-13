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

    fn msetnx(con: &mut Connection, key_values: &[(&str, &str)]) -> RedisResult<i32> {
        let mut cmd = redis::cmd("MSETNX");
        for &(key, value) in key_values {
            cmd.arg(key).arg(value);
        }
        cmd.query(con)
    }

    #[test]
    fn test_msetnx_success() {
        let mut con = setup();
        
        // 确保测试键不存在
        let _: () = con.del(&["msetnx-key1", "msetnx-key2"]).unwrap();
        
        // 测试所有键都不存在时的成功情况
        let result: i32 = msetnx(&mut con, &[("msetnx-key1", "value1"), ("msetnx-key2", "value2")]).unwrap();
        assert_eq!(result, 1);
        
        // 验证键值是否正确设置
        let value1: String = con.get("msetnx-key1").unwrap();
        let value2: String = con.get("msetnx-key2").unwrap();
        assert_eq!(value1, "value1");
        assert_eq!(value2, "value2");
    }

    #[test]
    fn test_msetnx_fail_when_one_key_exists() {
        let mut con = setup();
        
        // 确保第一个键存在，第二个键不存在
        let _: () = con.set("msetnx-exist-key", "existing-value").unwrap();
        let _: () = con.del("msetnx-not-exist-key").unwrap();
        
        // 测试当至少一个键存在时，应该返回0且不设置任何键
        let result: i32 = msetnx(&mut con, &[("msetnx-exist-key", "new-value1"), ("msetnx-not-exist-key", "new-value2")]).unwrap();
        assert_eq!(result, 0);
        
        // 验证现有键的值没有被更改
        let existing_value: String = con.get("msetnx-exist-key").unwrap();
        assert_eq!(existing_value, "existing-value");
        
        // 验证新键也没有被设置
        let new_key_exists: bool = con.exists("msetnx-not-exist-key").unwrap();
        assert_eq!(new_key_exists, false);
    }

    #[test]
    fn test_msetnx_fail_when_all_keys_exist() {
        let mut con = setup();
        
        // 确保所有键都存在
        let _: () = con.set("msetnx-all-exist-key1", "existing-value1").unwrap();
        let _: () = con.set("msetnx-all-exist-key2", "existing-value2").unwrap();
        
        // 测试当所有键都存在时，应该返回0
        let result: i32 = msetnx(&mut con, &[("msetnx-all-exist-key1", "new-value1"), ("msetnx-all-exist-key2", "new-value2")]).unwrap();
        assert_eq!(result, 0);
        
        // 验证现有键的值没有被更改
        let existing_value1: String = con.get("msetnx-all-exist-key1").unwrap();
        let existing_value2: String = con.get("msetnx-all-exist-key2").unwrap();
        assert_eq!(existing_value1, "existing-value1");
        assert_eq!(existing_value2, "existing-value2");
    }

    #[test]
    fn test_msetnx_with_single_key() {
        let mut con = setup();
        
        // 确保键不存在
        let _: () = con.del("msetnx-single-key").unwrap();
        
        // 测试单个键的情况
        let result: i32 = msetnx(&mut con, &[("msetnx-single-key", "single-value")]).unwrap();
        assert_eq!(result, 1);
        
        // 验证键值是否正确设置
        let value: String = con.get("msetnx-single-key").unwrap();
        assert_eq!(value, "single-value");
    }

    #[test]
    fn test_msetnx_atomicity() {
        let mut con = setup();
        
        // 确保第一个键存在，其余键不存在
        let _: () = con.set("msetnx-atomic-key1", "existing-value").unwrap();
        let _: () = con.del(&["msetnx-atomic-key2", "msetnx-atomic-key3"]).unwrap();
        
        // 测试原子性：即使部分键不存在，只要有一个键存在，就不应该设置任何键
        let result: i32 = msetnx(&mut con, &[
            ("msetnx-atomic-key1", "new-value1"),
            ("msetnx-atomic-key2", "new-value2"),
            ("msetnx-atomic-key3", "new-value3")
        ]).unwrap();
        assert_eq!(result, 0);
        
        // 验证没有任何键被设置或修改
        let value1: String = con.get("msetnx-atomic-key1").unwrap();
        assert_eq!(value1, "existing-value");
        
        let key2_exists: bool = con.exists("msetnx-atomic-key2").unwrap();
        assert_eq!(key2_exists, false);
        
        let key3_exists: bool = con.exists("msetnx-atomic-key3").unwrap();
        assert_eq!(key3_exists, false);
    }
}