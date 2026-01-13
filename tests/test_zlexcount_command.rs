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

    fn zlexcount(con: &mut Connection, key: &str, min: &str, max: &str) -> RedisResult<i64> {
        redis::cmd("ZLEXCOUNT").arg(key).arg(min).arg(max).query(con)
    }

    #[test]
    fn test_zlexcount_all_members() {
        let mut con = setup();
        
        // 清空数据库
        let _: () = con.del("myzset").unwrap();
        
        // 添加测试数据，所有成员分数为 0
        let _: i64 = con.zadd_multiple("myzset", &[(0.0, "a"), (0.0, "b"), (0.0, "c"), (0.0, "d"), (0.0, "e")]).unwrap();
        
        // 添加更多成员
        let _: i64 = con.zadd_multiple("myzset", &[(0.0, "f"), (0.0, "g")]).unwrap();
        
        // ZLEXCOUNT myzset - +（从负无穷到正无穷，所有成员）
        let result: i64 = zlexcount(&mut con, "myzset", "-", "+").unwrap();
        assert_eq!(result, 7);
        
        // ZLEXCOUNT myzset [b [f（闭区间 b 到 f）
        let result: i64 = zlexcount(&mut con, "myzset", "[b", "[f").unwrap();
        assert_eq!(result, 5); // b, c, d, e, f
    }

    #[test]
    fn test_zlexcount_exclusive_range() {
        let mut con = setup();
        
        // 清空数据库
        let _: () = con.del("myzset2").unwrap();
        
        // 添加测试数据
        let _: i64 = con.zadd_multiple("myzset2", &[(0.0, "a"), (0.0, "b"), (0.0, "c"), (0.0, "d")]).unwrap();
        
        // ZLEXCOUNT myzset2 (a (d（开区间 a 到 d）
        let result: i64 = zlexcount(&mut con, "myzset2", "(a", "(d").unwrap();
        assert_eq!(result, 2); // b, c
    }

    #[test]
    fn test_zlexcount_nonexistent_key() {
        let mut con = setup();
        
        // 清空数据库
        let _: () = con.del("nonexistent").unwrap();
        
        // 对不存在的键执行 ZLEXCOUNT
        let result: i64 = zlexcount(&mut con, "nonexistent", "-", "+").unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_zlexcount_mixed_range() {
        let mut con = setup();
        
        // 清空数据库
        let _: () = con.del("myzset3").unwrap();
        
        // 添加测试数据（按字典序：alpha < beta < delta < gamma）
        let _: i64 = con.zadd_multiple("myzset3", &[(0.0, "alpha"), (0.0, "beta"), (0.0, "delta"), (0.0, "gamma")]).unwrap();
        
        // ZLEXCOUNT myzset3 [beta (gamma（闭开区间：beta <= member < gamma）
        let result: i64 = zlexcount(&mut con, "myzset3", "[beta", "(gamma").unwrap();
        assert_eq!(result, 2); // beta, delta
        
        // ZLEXCOUNT myzset3 - [delta（负无穷到閭区间 delta）
        let result: i64 = zlexcount(&mut con, "myzset3", "-", "[delta").unwrap();
        assert_eq!(result, 3); // alpha, beta, delta
    }

    #[test]
    fn test_zlexcount_wrong_type() {
        let mut con = setup();
        
        // 清空数据库
        let _: () = con.del("mykey").unwrap();
        
        // 设置一个字符串键
        let _: () = con.set("mykey", "value").unwrap();
        
        // 对字符串键执行 ZLEXCOUNT
        let result: RedisResult<i64> = zlexcount(&mut con, "mykey", "-", "+");
        assert!(result.is_err());
    }

    #[test]
    fn test_zlexcount_invalid_range() {
        let mut con = setup();
        
        // 清空数据库
        let _: () = con.del("myzset4").unwrap();
        
        // 添加测试数据
        let _: i64 = con.zadd_multiple("myzset4", &[(0.0, "a"), (0.0, "b"), (0.0, "c"), (0.0, "d")]).unwrap();
        
        // min > max 的无效区间应该返回 0
        let result: i64 = zlexcount(&mut con, "myzset4", "[d", "[a").unwrap();
        assert_eq!(result, 0);
        
        // 开区间也是无效
        let result: i64 = zlexcount(&mut con, "myzset4", "(d", "(a").unwrap();
        assert_eq!(result, 0);
    }
}
