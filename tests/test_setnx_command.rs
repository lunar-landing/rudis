use redis::Commands;

#[tokio::test]
async fn test_setnx_command() {
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let mut con = client.get_connection().unwrap();

    // clean up any keys that may be present
    let _: () = redis::cmd("DEL").arg("test_setnx_key").query(&mut con).unwrap_or(());

    // Test SETNX - set when the key does not exist, should return 1
    let result: i32 = redis::cmd("SETNX")
        .arg("test_setnx_key")
        .arg("first_value")
        .query(&mut con)
        .unwrap();
    assert_eq!(result, 1);

    // the verification value is set
    let value: String = con.get("test_setnx_key").unwrap();
    assert_eq!(value, "first_value");

    // Try setting again, the key is already there and should return 0
    let result: i32 = redis::cmd("SETNX")
        .arg("test_setnx_key")
        .arg("second_value")
        .query(&mut con)
        .unwrap();
    assert_eq!(result, 0);

    // the verification value has not been changed
    let value: String = con.get("test_setnx_key").unwrap();
    assert_eq!(value, "first_value");
}

#[tokio::test]
async fn test_setnx_with_different_types() {
    let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    let mut con = client.get_connection().unwrap();

    // clean up any keys that may be present
    let _: () = redis::cmd("DEL").arg("test_setnx_hash").query(&mut con).unwrap_or(());

    // start by creating a key of the hash type
    let _: () = redis::cmd("HSET")
        .arg("test_setnx_hash")
        .arg("field")
        .arg("value")
        .query(&mut con)
        .unwrap();

    // SETNX should return 0 because the key already exists (even if the type is different)
    let result: i32 = redis::cmd("SETNX")
        .arg("test_setnx_hash")
        .arg("string_value")
        .query(&mut con)
        .unwrap();
    assert_eq!(result, 0);

    // verify that the hash value has not been changed
    let value: String = redis::cmd("HGET")
        .arg("test_setnx_hash")
        .arg("field")
        .query(&mut con)
        .unwrap();
    assert_eq!(value, "value");
}

