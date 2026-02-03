use redis::Commands;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_psetex_command() {
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let mut con = client.get_connection().unwrap();

    // clean up any keys that may be present
    let _: () = redis::cmd("DEL").arg("test_psetex_key").query(&mut con).unwrap_or(());

    // Test PSETEX - Set key values and specify expiration time (ms)
    let result: String = redis::cmd("PSETEX")
        .arg("test_psetex_key")
        .arg(2000) // 2 seconds = 2000 milliseconds
        .arg("test_value")
        .query(&mut con)
        .unwrap();
    assert_eq!(result, "OK");

    // the verification value is present immediately
    let value: String = con.get("test_psetex_key").unwrap();
    assert_eq!(value, "test_value");

    // wait for expiration
    sleep(Duration::from_millis(2500)).await;

    // the verification key has expired
    let result: Option<String> = con.get("test_psetex_key").ok();
    assert_eq!(result, None);
}

#[tokio::test]
async fn test_psetex_overwrite_existing_key() {
    let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    let mut con = client.get_connection().unwrap();

    // clean up any keys that may be present
    let _: () = redis::cmd("DEL").arg("test_psetex_overwrite").query(&mut con).unwrap_or(());

    // set a key first
    let _: () = redis::cmd("SET").arg("test_psetex_overwrite").arg("old_value").query(&mut con).unwrap();

    // Use PSETEX to override and set a new expiration time
    let result: String = redis::cmd("PSETEX")
        .arg("test_psetex_overwrite")
        .arg(2000)
        .arg("new_value")
        .query(&mut con)
        .unwrap();
    assert_eq!(result, "OK");

    // verify that the new value exists immediately
    let value: String = con.get("test_psetex_overwrite").unwrap();
    assert_eq!(value, "new_value");

    //wait for expiration
    sleep(Duration::from_millis(2500)).await;

    // the verification key has expired
    let result: Option<String> = con.get("test_psetex_overwrite").ok();
    assert_eq!(result, None);
}

