use rusqlite::ParamsFromIter;
use serde_json::Value;

pub fn bind_array_to_params(values: Value) -> ParamsFromIter<Vec<rusqlite::types::Value>> {
    let mut params: Vec<rusqlite::types::Value> = Vec::new();
    if let Value::Array(a) = values {
        for value in a {
            params.push(get_value(&value));
        }
    }
    rusqlite::params_from_iter(params)
}

pub fn bind_named_params(json: Value) -> Vec<(String, rusqlite::types::Value)> {
    let mut params = Vec::new();

    if let Value::Object(map) = json {
        for (key, value) in map {
            let rusqlite_value = get_value(&value);
            params.push((key, rusqlite_value));
        }
    }

    params
}

fn get_value(values: &Value) -> rusqlite::types::Value {
    match values {
        Value::Null => rusqlite::types::Value::Null,
        Value::Bool(b) => rusqlite::types::Value::from(b.to_owned()),
        Value::Number(n) => {
            if n.is_i64() {
                rusqlite::types::Value::Integer(n.as_i64().unwrap())
            } else {
                rusqlite::types::Value::Real(n.as_f64().unwrap())
            }
        }
        Value::String(s) => rusqlite::types::Value::from(s.to_string()),
        Value::Array(v) => {
            let vec: Vec<u8> = v
                .iter()
                .map(|v| v.to_string().parse::<u8>().unwrap())
                .collect::<Vec<_>>();

            rusqlite::types::Value::Blob(vec)
        }
        Value::Object(_) => rusqlite::types::Value::Null,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_bind_array_to_params_empty() {
        let values = json!([]);
        let result = bind_array_to_params(values);

        assert_eq!(format!("{:?}", result), "ParamsFromIter([])".to_string());
    }

    #[test]
    fn test_bind_array_to_params_single() {
        let values = json!([42]);
        let result = bind_array_to_params(values);

        assert_eq!(
            format!("{:?}", result),
            "ParamsFromIter([Integer(42)])".to_string()
        );
    }

    #[test]
    fn test_bind_array_to_params_multiple() {
        let values = json!([1, 2, 3]);
        let result = bind_array_to_params(values);

        assert_eq!(
            format!("{:?}", result),
            "ParamsFromIter([Integer(1), Integer(2), Integer(3)])".to_string()
        );
    }

    #[test]
    fn test_bind_array_to_params_mixed() {
        let values = json!([1, "two", 3.0]);
        let result = bind_array_to_params(values);

        assert_eq!(
            format!("{:?}", result),
            "ParamsFromIter([Integer(1), Text(\"two\"), Real(3.0)])".to_string()
        );
    }

    #[test]
    fn test_bind_named_params_empty() {
        let json = json!({});
        let result = bind_named_params(json);

        assert_eq!(result, Vec::new());
    }

    #[test] 
    fn test_bind_named_params_single() {
        let json = json!({
            "id": 42
        });
        let result = bind_named_params(json);

        assert_eq!(
            result,
            vec![("id".to_string(), rusqlite::types::Value::Integer(42))]
        );
    }

    #[test]
    fn test_bind_named_params_multiple() {
        let json = json!({
            "id": 1,
            "name": "test",
            "active": true,
            "score": 3.14,
            "data": null
        });
        let mut result = bind_named_params(json);

        // Sort result by key for consistent comparison
        result.sort_by(|a, b| a.0.cmp(&b.0));

        let mut expected = vec![
            ("active".to_string(), rusqlite::types::Value::Integer(1)),
            ("data".to_string(), rusqlite::types::Value::Null),
            ("id".to_string(), rusqlite::types::Value::Integer(1)),
            ("name".to_string(), rusqlite::types::Value::Text("test".to_string())),
            ("score".to_string(), rusqlite::types::Value::Real(3.14))
        ];
        expected.sort_by(|a, b| a.0.cmp(&b.0));

        assert_eq!(result, expected);
    }


    #[test]
    fn test_get_value_null() {
        let value = json!(null);
        let result = get_value(&value);

        assert_eq!(result, rusqlite::types::Value::Null);
    }

    #[test]
    fn test_get_value_bool() {
        let value = json!(true);
        let result = get_value(&value);

        assert_eq!(result, rusqlite::types::Value::from(true));
    }

    #[test]
    fn test_get_value_integer() {
        let value = json!(42);
        let result = get_value(&value);

        assert_eq!(result, rusqlite::types::Value::Integer(42));
    }

    #[test]
    fn test_get_value_float() {
        let value = json!(3.04);
        let result = get_value(&value);

        assert_eq!(result, rusqlite::types::Value::Real(3.04));
    }

    #[test]
    fn test_get_value_string() {
        let value = json!("hello");
        let result = get_value(&value);

        assert_eq!(result, rusqlite::types::Value::from("hello".to_string()));
    }

    #[test]
    fn test_get_value_array() {
        let value = json!([1, 2, 3]);
        let result = get_value(&value);

        assert_eq!(result, rusqlite::types::Value::Blob(vec![1, 2, 3]));
    }

    #[test]
    fn test_get_value_object() {
        let value = json!({"foo": "bar"});
        let result = get_value(&value);

        assert_eq!(result, rusqlite::types::Value::Null);
    }
}
