pub use std::boxed::Box;
pub use std::collections::HashMap;
pub use std::string::ToString;

#[macro_export]
macro_rules! json {
    (null) => {
        $crate::Json::Null
    };
    ( [ $( $element:tt ),* ] ) => {
        $crate::Json::Array(vec![ $( json!($element) ),* ])
    };
    ( { $( $key:tt : $value:tt ),* } ) => {
        {
            let mut fiealds = $crate::macros::Box::new($crate::macros::HashMap::new());
            $( fiealds.insert($crate::macros::ToString::to_string($key), json!($value)); )*
            $crate::Json::Object(fiealds)
        }
    };
    ( $other:tt ) => {
        $crate::Json::from($other)
    }
}

#[cfg(test)]
mod test {
    use crate::Json;
    use std::collections::HashMap;

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
}
