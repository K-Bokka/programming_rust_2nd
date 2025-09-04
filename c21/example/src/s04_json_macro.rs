use std::collections::HashMap;

macro_rules! impl_from_num_for_json {
    ( $( $t:ident )* ) => {
        $(
            impl From<$t> for Json {
                fn from(n: $t) -> Json {
                Json::Number(n as f64)
                }
            }
        )*
    };
}

impl_from_num_for_json!(u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 usize isize f32 f64);

macro_rules! json {
    (null) => {
        Json::Null
    };
    ( [ $( $element:tt ),* ] ) => {
        Json::Array(vec![ $( json!($element) ),* ])
    };
    ( { $( $key:tt : $value:tt ),* } ) => {
        {
            let mut fiealds = Box::new(HashMap::new());
            $( fiealds.insert($key.to_string(), json!($value)); )*
            Json::Object(fiealds)
        }
    };
    ( $other:tt ) => {
        Json::from($other)
    }
}

#[test]
fn json_null() {
    assert_eq!(json!(null), Json::Null);
}

#[test]
fn json_array() {
    assert_eq!(json!([]), Json::Array(vec![]));
    assert_eq!(
        json!([1, 2, 3]),
        Json::Array(vec![1.into(), 2.into(), 3.into()])
    );
}

#[test]
fn json_object() {
    assert_eq!(json!({}), Json::Object(Box::new(HashMap::new())));
    assert_eq!(
        json!({ "name": null, "age": 10 }),
        Json::Object(Box::new(
            vec![
                ("name".to_string(), Json::Null),
                ("age".to_string(), Json::Number(10.into()))
            ]
            .into_iter()
            .collect(),
        ))
    );
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

    println!("21.4.2 Recursion in macro");

    println!("21.4.3 Using trait in macro");

    let width = 4.0;
    let desc = json!({
        "width": width,
        "height": (width * 9.0 / 4.0)
    });
    println!("{:?}", desc);

    println!("21.4.4 Scope & Hygienic macro");

    let fields = "Fields, W.C.";
    let role = json!({
        "name": "Larson E. Whipsnade",
        "actor": fields
    });
    println!("{:?}", role);

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

impl From<bool> for Json {
    fn from(b: bool) -> Json {
        Json::Bool(b)
    }
}

impl From<String> for Json {
    fn from(s: String) -> Json {
        Json::String(s)
    }
}

impl<'a> From<&'a str> for Json {
    fn from(s: &'a str) -> Json {
        Json::String(s.to_string())
    }
}
