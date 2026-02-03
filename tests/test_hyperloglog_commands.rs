#[cfg(test)]
mod tests {
    use redis::{cmd, Client, Connection, RedisResult};
    
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
    fn test_pfadd_basic_and_create_empty() {
        let mut con = setup();

        let key = "hll-test-pfadd-basic";
        let _: () = cmd("DEL").arg(key).query(&mut con).unwrap();

        // PFADD key element -> 1 (new HLL + register changed)
        let r: i64 = cmd("PFADD").arg(key).arg("element1").query(&mut con).unwrap();
        assert_eq!(r, 1);

        // PFADD key new elements -> 1
        let r: i64 = cmd("PFADD")
            .arg(key)
            .arg("element2")
            .arg("element3")
            .arg("element4")
            .query(&mut con)
            .unwrap();
        assert_eq!(r, 1);

        // PFADD key duplicate elements -> 0
        let r: i64 = cmd("PFADD")
            .arg(key)
            .arg("element1")
            .arg("element2")
            .query(&mut con)
            .unwrap();
        assert_eq!(r, 0);

        // PFADD key new element -> 1
        let r: i64 = cmd("PFADD").arg(key).arg("element5").query(&mut con).unwrap();
        assert_eq!(r, 1);

        // PFADD empty key without elements:
        // - if key doesn't exist: should create and return 1
        // - if key exists: should be a no-op and return 0
        let empty_key = "hll-test-pfadd-empty-create";
        let _: () = cmd("DEL").arg(empty_key).query(&mut con).unwrap();

        let r: i64 = cmd("PFADD").arg(empty_key).query(&mut con).unwrap();
        assert_eq!(r, 1);

        let r: i64 = cmd("PFADD").arg(empty_key).query(&mut con).unwrap();
        assert_eq!(r, 0);
    }

    #[test]
    fn test_pfcount_single_and_multi_keys() {
        let mut con = setup();

        let k1 = "hll-test-pfcount-hll1";
        let k2 = "hll-test-pfcount-hll2";
        let k3 = "hll-test-pfcount-hll3";
        let empty = "hll-test-pfcount-empty";
        let _: () = cmd("DEL").arg(k1).arg(k2).arg(k3).arg(empty).query(&mut con).unwrap();

        // hll1: 5 unique
        let _: i64 = cmd("PFADD")
            .arg(k1)
            .arg("element1")
            .arg("element2")
            .arg("element3")
            .arg("element4")
            .arg("element5")
            .query(&mut con)
            .unwrap();

        // hll2: 4 unique
        let _: i64 = cmd("PFADD").arg(k2).arg("a").arg("b").arg("c").arg("d").query(&mut con).unwrap();

        // hll3: 5 unique, overlaps with hll1 on element1/element2
        let _: i64 = cmd("PFADD")
            .arg(k3)
            .arg("element1")
            .arg("element2")
            .arg("x")
            .arg("y")
            .arg("z")
            .query(&mut con)
            .unwrap();

        // empty: create with no elements then add one element
        let _: i64 = cmd("PFADD").arg(empty).query(&mut con).unwrap();
        let _: i64 = cmd("PFADD").arg(empty).arg("new_element").query(&mut con).unwrap();

        let c: i64 = cmd("PFCOUNT").arg(k1).query(&mut con).unwrap();
        assert_eq!(c, 5);

        let c: i64 = cmd("PFCOUNT").arg("hll-test-pfcount-nonexistent").query(&mut con).unwrap();
        assert_eq!(c, 0);

        let c: i64 = cmd("PFCOUNT").arg(empty).query(&mut con).unwrap();
        assert_eq!(c, 1);

        // union(hll1,hll2) = 9
        let c: i64 = cmd("PFCOUNT").arg(k1).arg(k2).query(&mut con).unwrap();
        assert_eq!(c, 9);

        // union(hll1,hll2,nonexistent) = 9
        let c: i64 = cmd("PFCOUNT")
            .arg(k1)
            .arg(k2)
            .arg("hll-test-pfcount-nonexistent")
            .query(&mut con)
            .unwrap();
        assert_eq!(c, 9);

        // union(hll1,hll2,hll3) = 12
        let c: i64 = cmd("PFCOUNT").arg(k1).arg(k2).arg(k3).query(&mut con).unwrap();
        assert_eq!(c, 12);
    }

    #[test]
    fn test_pfmerge_basic_and_missing_source() {
        let mut con = setup();

        let hll1 = "hll-test-pfmerge-hll1";
        let hll2 = "hll-test-pfmerge-hll2";
        let dest = "hll-test-pfmerge-dest";
        let _: () = cmd("DEL").arg(hll1).arg(hll2).arg(dest).query(&mut con).unwrap();

        let _: i64 = cmd("PFADD")
            .arg(hll1)
            .arg("element1")
            .arg("element2")
            .arg("element3")
            .arg("element4")
            .arg("element5")
            .query(&mut con)
            .unwrap();

        let _: i64 = cmd("PFADD").arg(hll2).arg("a").arg("b").arg("c").arg("d").query(&mut con).unwrap();

        let ok: String = cmd("PFMERGE").arg(dest).arg(hll1).arg(hll2).query(&mut con).unwrap();
        assert_eq!(ok, "OK");

        let c: i64 = cmd("PFCOUNT").arg(dest).query(&mut con).unwrap();
        assert_eq!(c, 9);

        // Merge with a missing source key should be OK and ignored
        let _: () = cmd("DEL").arg(dest).query(&mut con).unwrap();
        let ok: String = cmd("PFMERGE")
            .arg(dest)
            .arg(hll1)
            .arg("hll-test-pfmerge-missing")
            .arg(hll2)
            .query(&mut con)
            .unwrap();
        assert_eq!(ok, "OK");

        let c: i64 = cmd("PFCOUNT").arg(dest).query(&mut con).unwrap();
        assert_eq!(c, 9);
    }

    #[test]
    fn test_hyperloglog_wrong_type_errors() {
        let mut con = setup();

        let string_key = "hll-test-wrong-type-string";
        let dest = "hll-test-wrong-type-dest";
        let hll1 = "hll-test-wrong-type-hll1";
        let _: () = cmd("DEL").arg(string_key).arg(dest).arg(hll1).query(&mut con).unwrap();

        let _: () = cmd("SET").arg(string_key).arg("some_value").query(&mut con).unwrap();
        let _: i64 = cmd("PFADD").arg(hll1).arg("x").arg("y").arg("z").query(&mut con).unwrap();

        // PFADD against a non-HLL key -> error
        let r: RedisResult<i64> = cmd("PFADD").arg(string_key).arg("element").query(&mut con);
        assert!(r.is_err());
        let err_msg = format!("{:?}", r.unwrap_err());
        assert!(err_msg.contains("wrong kind of value") || err_msg.contains("WRONGTYPE") || err_msg.contains("ERR"));

        // PFCOUNT against a non-HLL key -> error
        let r: RedisResult<i64> = cmd("PFCOUNT").arg(string_key).query(&mut con);
        assert!(r.is_err());

        // PFMERGE dest is non-HLL key -> error
        let r: RedisResult<String> = cmd("PFMERGE").arg(string_key).arg(hll1).query(&mut con);
        assert!(r.is_err());

        // PFMERGE source includes non-HLL key -> error
        let r: RedisResult<String> = cmd("PFMERGE").arg(dest).arg(string_key).arg(hll1).query(&mut con);
        assert!(r.is_err());
        let err_msg = format!("{:?}", r.unwrap_err());
        assert!(err_msg.contains("wrong kind of value") || err_msg.contains("WRONGTYPE") || err_msg.contains("ERR"));
    }
}

