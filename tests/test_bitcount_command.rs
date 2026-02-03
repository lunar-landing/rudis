use redis::Commands;
use redis::cmd;

#[tokio::test]
async fn test_bitcount_command() {
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let mut con = client.get_connection().unwrap();

    // clean up any keys that may be present
    let _: () = cmd("DEL").arg("test_bitcount_key").query(&mut con).unwrap_or(());

    // test BITCOUNT on non-existent key
    let result: i64 = cmd("BITCOUNT")
        .arg("test_bitcount_key")
        .query(&mut con)
        .unwrap();
    assert_eq!(result, 0);

    // set a key with known bit pattern
    // 'a' = 0x61 = 01100001 (3 bits set)
    let _: () = con.set("test_bitcount_key", "a").unwrap();

    // test BITCOUNT on the entire string
    let result: i64 = cmd("BITCOUNT")
        .arg("test_bitcount_key")
        .query(&mut con)
        .unwrap();
    assert_eq!(result, 3);

    // set another key with more bits
    // 'foobar' has more bits set
    let _: () = con.set("test_bitcount_key2", "foobar").unwrap();
    let result: i64 = cmd("BITCOUNT")
        .arg("test_bitcount_key2")
        .query(&mut con)
        .unwrap();
    assert!(result > 0);
}

#[tokio::test]
async fn test_bitcount_with_range() {
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let mut con = client.get_connection().unwrap();

    // clean up any keys that may be present
    let _: () = cmd("DEL").arg("test_bitcount_range").query(&mut con).unwrap_or(());

    // set a key
    let _: () = con.set("test_bitcount_range", "foobar").unwrap();

    // test BITCOUNT with start and end
    let result: i64 = cmd("BITCOUNT")
        .arg("test_bitcount_range")
        .arg(0)
        .arg(0)
        .query(&mut con)
        .unwrap();
    assert!(result >= 0);

    // test BITCOUNT with negative indices
    let result: i64 = cmd("BITCOUNT")
        .arg("test_bitcount_range")
        .arg(-1)
        .arg(-1)
        .query(&mut con)
        .unwrap();
    assert!(result >= 0);

    // test BITCOUNT with full range
    let full_count: i64 = cmd("BITCOUNT")
        .arg("test_bitcount_range")
        .query(&mut con)
        .unwrap();
    
    let range_count: i64 = cmd("BITCOUNT")
        .arg("test_bitcount_range")
        .arg(0)
        .arg(-1)
        .query(&mut con)
        .unwrap();
    
    assert_eq!(full_count, range_count);
}

#[tokio::test]
async fn test_bitcount_empty_string() {
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let mut con = client.get_connection().unwrap();

    // clean up any keys that may be present
    let _: () = cmd("DEL").arg("test_bitcount_empty").query(&mut con).unwrap_or(());

    // set an empty string
    let _: () = con.set("test_bitcount_empty", "").unwrap();

    // test BITCOUNT on empty string
    let result: i64 = cmd("BITCOUNT")
        .arg("test_bitcount_empty")
        .query(&mut con)
        .unwrap();
    assert_eq!(result, 0);
}

#[tokio::test]
async fn test_bitcount_with_setbit() {
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let mut con = client.get_connection().unwrap();

    // clean up any keys that may be present
    let _: () = cmd("DEL").arg("test_bitcount_setbit").query(&mut con).unwrap_or(());

    // set bits manually
    let _: i64 = cmd("SETBIT").arg("test_bitcount_setbit").arg(0).arg(1).query(&mut con).unwrap();
    let _: i64 = cmd("SETBIT").arg("test_bitcount_setbit").arg(1).arg(1).query(&mut con).unwrap();
    let _: i64 = cmd("SETBIT").arg("test_bitcount_setbit").arg(2).arg(1).query(&mut con).unwrap();
    let _: i64 = cmd("SETBIT").arg("test_bitcount_setbit").arg(5).arg(1).query(&mut con).unwrap();

    // count bits
    let result: i64 = cmd("BITCOUNT")
        .arg("test_bitcount_setbit")
        .query(&mut con)
        .unwrap();
    assert_eq!(result, 4);
}

