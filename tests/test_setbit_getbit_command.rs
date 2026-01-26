use redis::Commands;
use redis::cmd;

#[tokio::test]
async fn test_setbit_command() {
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let mut con = client.get_connection().unwrap();

    // clean up any keys that may be present
    let _: () = cmd("DEL").arg("test_setbit_key").query(&mut con).unwrap_or(());

    // test SETBIT - Set bit at offset 0 to 1
    let result: i64 = cmd("SETBIT")
        .arg("test_setbit_key")
        .arg(0)
        .arg(1)
        .query(&mut con)
        .unwrap();
    assert_eq!(result, 0); // didn t exist before returns 0

    // test SETBIT - Set bit at offset 1 to 1
    let result: i64 = cmd("SETBIT")
        .arg("test_setbit_key")
        .arg(1)
        .arg(1)
        .query(&mut con)
        .unwrap();
    assert_eq!(result, 0);

    // test SETBIT - Set bit at offset 0 to 0
    let result: i64 = cmd("SETBIT")
        .arg("test_setbit_key")
        .arg(0)
        .arg(0)
        .query(&mut con)
        .unwrap();
    assert_eq!(result, 1);

    // verify the value
    let value: String = con.get("test_setbit_key").unwrap();
    // The string should contain the bits we set
    assert!(!value.is_empty());
}

#[tokio::test]
async fn test_getbit_command() {
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let mut con = client.get_connection().unwrap();

    // clean up any keys that may be present
    let _: () = cmd("DEL").arg("test_getbit_key").query(&mut con).unwrap_or(());

    // test GETBIT on non-existent key
    let result: i64 = cmd("GETBIT")
        .arg("test_getbit_key")
        .arg(0)
        .query(&mut con)
        .unwrap();
    assert_eq!(result, 0);

    // set a key first
    let _: () = con.set("test_getbit_key", "a").unwrap(); // 'a' = 0x61 = 01100001

    // test GETBIT at offset 0 (should be 0, first bit of 'a')
    let result: i64 = cmd("GETBIT")
        .arg("test_getbit_key")
        .arg(0)
        .query(&mut con)
        .unwrap();
    assert_eq!(result, 0);

    // test GETBIT at offset 1 (should be 1, second bit of 'a')
    let result: i64 = cmd("GETBIT")
        .arg("test_getbit_key")
        .arg(1)
        .query(&mut con)
        .unwrap();
    assert_eq!(result, 1);

    // test GETBIT at offset 2 (should be 1, third bit of 'a')
    let result: i64 = cmd("GETBIT")
        .arg("test_getbit_key")
        .arg(2)
        .query(&mut con)
        .unwrap();
    assert_eq!(result, 1);

    // test GETBIT at offset 100 (should be 0, out of range)
    let result: i64 = cmd("GETBIT")
        .arg("test_getbit_key")
        .arg(100)
        .query(&mut con)
        .unwrap();
    assert_eq!(result, 0);
}

#[tokio::test]
async fn test_setbit_getbit_combined() {
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let mut con = client.get_connection().unwrap();

    // clean up any keys that may be present
    let _: () = cmd("DEL").arg("test_bit_combined").query(&mut con).unwrap_or(());

    // set bits at various offsets
    let _: i64 = cmd("SETBIT").arg("test_bit_combined").arg(0).arg(1).query(&mut con).unwrap();
    let _: i64 = cmd("SETBIT").arg("test_bit_combined").arg(1).arg(0).query(&mut con).unwrap();
    let _: i64 = cmd("SETBIT").arg("test_bit_combined").arg(2).arg(1).query(&mut con).unwrap();
    let _: i64 = cmd("SETBIT").arg("test_bit_combined").arg(7).arg(1).query(&mut con).unwrap();

    // verify bits
    let bit0: i64 = cmd("GETBIT").arg("test_bit_combined").arg(0).query(&mut con).unwrap();
    assert_eq!(bit0, 1);

    let bit1: i64 = cmd("GETBIT").arg("test_bit_combined").arg(1).query(&mut con).unwrap();
    assert_eq!(bit1, 0);

    let bit2: i64 = cmd("GETBIT").arg("test_bit_combined").arg(2).query(&mut con).unwrap();
    assert_eq!(bit2, 1);

    let bit7: i64 = cmd("GETBIT").arg("test_bit_combined").arg(7).query(&mut con).unwrap();
    assert_eq!(bit7, 1);
}

#[tokio::test]
async fn test_setbit_large_offset() {
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let mut con = client.get_connection().unwrap();

    // clean up any keys that may be present
    let _: () = cmd("DEL").arg("test_setbit_large").query(&mut con).unwrap_or(());

    // set bit at a large offset
    let result: i64 = cmd("SETBIT")
        .arg("test_setbit_large")
        .arg(100)
        .arg(1)
        .query(&mut con)
        .unwrap();
    assert_eq!(result, 0);

    // verify the bit
    let bit: i64 = cmd("GETBIT")
        .arg("test_setbit_large")
        .arg(100)
        .query(&mut con)
        .unwrap();
    assert_eq!(bit, 1);

    // verify bits before the set one are 0
    let bit99: i64 = cmd("GETBIT")
        .arg("test_setbit_large")
        .arg(99)
        .query(&mut con)
        .unwrap();
    assert_eq!(bit99, 0);
}

