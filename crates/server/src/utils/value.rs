use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Value {
    Blob(Vec<u8>),
    Float(f64),
    Integer(i64),
    Null,
    Text(String),
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_values() {
        assert_eq!(1, serde_json::to_value(Value::Integer(1)).unwrap());
        assert_eq!(Value::Integer(1), Value::Integer(1));

        assert_eq!(1.0, serde_json::to_value(Value::Float(1.0)).unwrap());
        assert_eq!(Value::Float(1.0), Value::Float(1.0));

        assert_eq!(
            String::from(""),
            serde_json::to_value(Value::Text("".to_owned())).unwrap()
        );
        assert_eq!(Value::Text("".to_owned()), Value::Text("".to_owned()));

        assert_eq!(
            json!([0]),
            serde_json::to_value(Value::Blob(vec![0])).unwrap()
        );
        assert_eq!(Value::Blob(vec![0]), Value::Blob(vec![0]));
    }
}
