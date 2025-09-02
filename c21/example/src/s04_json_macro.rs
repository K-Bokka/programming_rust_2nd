use std::collections::HashMap;

macro_rules! json {
    (null) => { Json::Null };
}

#[test]
fn json_null() {
    assert_eq!(json!(null), Json::Null);
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("21.4 JSON Macro");

    let students = Json::Array(vec![
        Json::Object(Box::new(
            vec![
                ("name".to_string(), Json::String("Alice".to_string())),
                ("class_of".to_string(), Json::Number(1926.0)),
                (
                    "major".to_string(),
                    Json::String("Computer Science".to_string()),
                ),
            ]
            .into_iter()
            .collect(),
        )),
        Json::Object(Box::new(
            vec![
                ("name".to_string(), Json::String("Bob".to_string())),
                ("class_of".to_string(), Json::Number(1927.0)),
                (
                    "major".to_string(),
                    Json::String("Computer Science".to_string()),
                ),
            ]
            .into_iter()
            .collect(),
        )),
    ]);

    println!("{:?}", students);

    println!("21.4.1 Fragment type");

    Ok(())
}

#[derive(Clone, PartialEq, Debug)]
enum Json {
    #[allow(dead_code)]
    Null,
    #[allow(dead_code)]
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}
