use serde_json::{json, Value};
use sled::{Config, Db};

// 定义一个插入JSON的函数
fn insert_json(db: &Db, key: &str, json: Value) {
    let json_str = json.to_string();

    db.insert(key.as_bytes(), json_str.as_bytes());
}
// 查询函数
fn get_json(db: &Db, key: &str) -> Result<Value, sled::Error> {
    match db.get(key.as_bytes()) {
        Ok(Some(v)) => {
            // 解析JSON
            let value: Value = serde_json::from_slice(&v).unwrap();
            Ok(value)
        }
        Ok(None) => Err(sled::Error::ReportableBug("error".to_string())),
        Err(e) => Err(e),
    }
}

#[test]
fn test() {
    let db: sled::Db = sled::open("my_db").unwrap();
    let key = "key2";

    // 构造一个JSON对象
    let json = json!({
        "state": "pass",
        "case-name": "zzy"
    });

    // 调用函数插入
    insert_json(&db, key, json);

    // 查询
    let result = get_json(&db, "key2");

    match result {
        Ok(json) => {
            println!("Got JSON: {}", json);
        }
        Err(e) => println!("Error: {}", e),
    }
}
