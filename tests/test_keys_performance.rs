#[cfg(test)]
mod tests {
    use std::time::Instant;
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
    fn test_keys_performance_with_10k_keys() {
        let mut con = setup();
        
        // 清空数据库以确保测试的一致性
        let _: () = con.flushdb().unwrap();
        
        // 插入10000个键值对
        println!("插入10000个键值对...");
        let start_insert = Instant::now();
        for i in 0..10000 {
            let key = format!("perf_test_key_{}", i);
            let value = format!("perf_test_value_{}", i);
            let _: () = con.set(key, value).unwrap();
        }
        let insert_duration = start_insert.elapsed();
        println!("插入10000个键耗时: {:?}", insert_duration);
        
        // 测试匹配所有键的性能
        println!("测试KEYS * 命令性能...");
        let start_keys_all = Instant::now();
        let all_keys: Vec<String> = con.keys("*").unwrap();
        let keys_all_duration = start_keys_all.elapsed();
        println!("KEYS * 命令返回 {} 个键，耗时: {:?}", all_keys.len(), keys_all_duration);
        
        // 测试匹配特定模式的性能
        println!("测试KEYS perf_test_key_* 命令性能...");
        let start_keys_pattern = Instant::now();
        let pattern_keys: Vec<String> = con.keys("perf_test_key_*").unwrap();
        let keys_pattern_duration = start_keys_pattern.elapsed();
        println!("KEYS perf_test_key_* 命令返回 {} 个键，耗时: {:?}", pattern_keys.len(), keys_pattern_duration);
        
        // 测试匹配前缀的性能
        println!("测试KEYS perf_test_key_1* 命令性能...");
        let start_keys_prefix = Instant::now();
        let prefix_keys: Vec<String> = con.keys("perf_test_key_1*").unwrap();
        let keys_prefix_duration = start_keys_prefix.elapsed();
        println!("KEYS perf_test_key_1* 命令返回 {} 个键，耗时: {:?}", prefix_keys.len(), keys_prefix_duration);
        
        // 测试匹配单个键的性能
        println!("测试KEYS perf_test_key_5000 命令性能...");
        let start_keys_single = Instant::now();
        let single_key: Vec<String> = con.keys("perf_test_key_5000").unwrap();
        let keys_single_duration = start_keys_single.elapsed();
        println!("KEYS perf_test_key_5000 命令返回 {} 个键，耗时: {:?}", single_key.len(), keys_single_duration);
        
        // 断言验证
        assert_eq!(pattern_keys.len(), 10000);
        assert_eq!(single_key.len(), 1);
        
        // 清理数据
        let _: () = con.flushdb().unwrap();
    }
}