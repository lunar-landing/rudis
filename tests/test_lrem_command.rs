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

    /// 测试用例来自 Redis 官方文档
    /// redis> RPUSH mylist "hello"
    /// redis> RPUSH mylist "hello"
    /// redis> RPUSH mylist "foo"
    /// redis> RPUSH mylist "hello"
    /// redis> LREM mylist -2 "hello"
    /// (integer) 2
    /// redis> LRANGE mylist 0 -1
    /// 1) "hello"
    /// 2) "foo"
    #[test]
    fn test_lrem_official_example() {
        let mut con = setup();

        // 清理测试数据
        let _: () = con.del("mylist-lrem-test").unwrap();

        // 准备测试数据
        let _: i32 = con.rpush("mylist-lrem-test", "hello").unwrap();
        let _: i32 = con.rpush("mylist-lrem-test", "hello").unwrap();
        let _: i32 = con.rpush("mylist-lrem-test", "foo").unwrap();
        let _: i32 = con.rpush("mylist-lrem-test", "hello").unwrap();

        // 从表尾开始移除2个"hello"
        let removed: i32 = con.lrem("mylist-lrem-test", -2, "hello").unwrap();
        assert_eq!(removed, 2);

        // 验证结果
        let range: Vec<String> = con.lrange("mylist-lrem-test", 0, -1).unwrap();
        assert_eq!(range.len(), 2);
        assert_eq!(range[0], "hello");
        assert_eq!(range[1], "foo");
    }

    /// 测试 count > 0: 从表头开始向表尾搜索
    #[test]
    fn test_lrem_positive_count() {
        let mut con = setup();

        let _: () = con.del("lrem-positive-test").unwrap();

        let _: i32 = con.rpush("lrem-positive-test", "a").unwrap();
        let _: i32 = con.rpush("lrem-positive-test", "b").unwrap();
        let _: i32 = con.rpush("lrem-positive-test", "a").unwrap();
        let _: i32 = con.rpush("lrem-positive-test", "c").unwrap();
        let _: i32 = con.rpush("lrem-positive-test", "a").unwrap();

        // 从表头开始移除1个"a"
        let removed: i32 = con.lrem("lrem-positive-test", 1, "a").unwrap();
        assert_eq!(removed, 1);

        // 验证结果 - 第一个"a"应该被移除
        let range: Vec<String> = con.lrange("lrem-positive-test", 0, -1).unwrap();
        assert_eq!(range.len(), 4);
        assert_eq!(range[0], "b");
        assert_eq!(range[1], "a");
        assert_eq!(range[2], "c");
        assert_eq!(range[3], "a");
    }

    /// 测试 count < 0: 从表尾开始向表头搜索
    #[test]
    fn test_lrem_negative_count() {
        let mut con = setup();

        let _: () = con.del("lrem-negative-test").unwrap();

        let _: i32 = con.rpush("lrem-negative-test", "a").unwrap();
        let _: i32 = con.rpush("lrem-negative-test", "b").unwrap();
        let _: i32 = con.rpush("lrem-negative-test", "a").unwrap();
        let _: i32 = con.rpush("lrem-negative-test", "c").unwrap();
        let _: i32 = con.rpush("lrem-negative-test", "a").unwrap();

        // 从表尾开始移除1个"a"
        let removed: i32 = con.lrem("lrem-negative-test", -1, "a").unwrap();
        assert_eq!(removed, 1);

        // 验证结果 - 最后一个"a"应该被移除
        let range: Vec<String> = con.lrange("lrem-negative-test", 0, -1).unwrap();
        assert_eq!(range.len(), 4);
        assert_eq!(range[0], "a");
        assert_eq!(range[1], "b");
        assert_eq!(range[2], "a");
        assert_eq!(range[3], "c");
    }

    /// 测试 count = 0: 移除所有与 value 相等的元素
    #[test]
    fn test_lrem_zero_count() {
        let mut con = setup();

        let _: () = con.del("lrem-zero-test").unwrap();

        let _: i32 = con.rpush("lrem-zero-test", "a").unwrap();
        let _: i32 = con.rpush("lrem-zero-test", "b").unwrap();
        let _: i32 = con.rpush("lrem-zero-test", "a").unwrap();
        let _: i32 = con.rpush("lrem-zero-test", "c").unwrap();
        let _: i32 = con.rpush("lrem-zero-test", "a").unwrap();

        // 移除所有"a"
        let removed: i32 = con.lrem("lrem-zero-test", 0, "a").unwrap();
        assert_eq!(removed, 3);

        // 验证结果 - 所有"a"都应该被移除
        let range: Vec<String> = con.lrange("lrem-zero-test", 0, -1).unwrap();
        assert_eq!(range.len(), 2);
        assert_eq!(range[0], "b");
        assert_eq!(range[1], "c");
    }

    /// 测试列表不存在的情况
    #[test]
    fn test_lrem_nonexistent_key() {
        let mut con = setup();

        let _: () = con.del("lrem-nonexistent-test").unwrap();

        // 在不存在的键上执行 lrem 应该返回 0
        let removed: i32 = con.lrem("lrem-nonexistent-test", 1, "hello").unwrap();
        assert_eq!(removed, 0);
    }

    /// 测试值不存在于列表中的情况
    #[test]
    fn test_lrem_value_not_found() {
        let mut con = setup();

        let _: () = con.del("lrem-not-found-test").unwrap();

        let _: i32 = con.rpush("lrem-not-found-test", "a").unwrap();
        let _: i32 = con.rpush("lrem-not-found-test", "b").unwrap();
        let _: i32 = con.rpush("lrem-not-found-test", "c").unwrap();

        // 移除不存在的值
        let removed: i32 = con.lrem("lrem-not-found-test", 1, "z").unwrap();
        assert_eq!(removed, 0);

        // 验证列表没有变化
        let range: Vec<String> = con.lrange("lrem-not-found-test", 0, -1).unwrap();
        assert_eq!(range.len(), 3);
    }

    /// 测试 count 大于实际匹配数量的情况
    #[test]
    fn test_lrem_count_exceeds_matches() {
        let mut con = setup();

        let _: () = con.del("lrem-exceed-test").unwrap();

        let _: i32 = con.rpush("lrem-exceed-test", "a").unwrap();
        let _: i32 = con.rpush("lrem-exceed-test", "b").unwrap();
        let _: i32 = con.rpush("lrem-exceed-test", "a").unwrap();

        // 尝试移除5个"a"，但实际只有2个
        let removed: i32 = con.lrem("lrem-exceed-test", 5, "a").unwrap();
        assert_eq!(removed, 2);

        // 验证结果
        let range: Vec<String> = con.lrange("lrem-exceed-test", 0, -1).unwrap();
        assert_eq!(range.len(), 1);
        assert_eq!(range[0], "b");
    }

    /// 测试移除后列表为空的情况
    #[test]
    fn test_lrem_empty_after_remove() {
        let mut con = setup();

        let _: () = con.del("lrem-empty-test").unwrap();

        let _: i32 = con.rpush("lrem-empty-test", "a").unwrap();
        let _: i32 = con.rpush("lrem-empty-test", "a").unwrap();
        let _: i32 = con.rpush("lrem-empty-test", "a").unwrap();

        // 移除所有"a"
        let removed: i32 = con.lrem("lrem-empty-test", 0, "a").unwrap();
        assert_eq!(removed, 3);

        // 验证结果 - 列表应该为空
        let range: Vec<String> = con.lrange("lrem-empty-test", 0, -1).unwrap();
        assert_eq!(range.len(), 0);
    }
}
