use anyhow::Result;
use regex::{Captures, Regex};
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

pub fn bind_object_to_params(
    values: Value,
    query: String,
) -> Result<(ParamsFromIter<Vec<rusqlite::types::Value>>, String)> {
    let mut updated_query = query;
    let mut params: Vec<rusqlite::types::Value> = Vec::new();
    if let Value::Object(o) = values {
        for (index, (key, value)) in o.iter().enumerate() {
            // https://www.sqlite.org/lang_expr.html#varparam
            if key.starts_with(':') || key.starts_with('@') || key.starts_with('$') {
                let re = Regex::new(&format!(r#"(?m)('[^']*')|{key}"#)).unwrap();

                updated_query = re
                    .replace_all(&updated_query, |caps: &Captures| match caps.get(1) {
                        Some(m) => m.as_str().to_string(),
                        None => format!("?{}", index + 1),
                    })
                    .to_string();
            } else {
                anyhow::bail!(
                    "Invalid key: {}. A named parameter should start with : or @ or $",
                    key
                )
            }

            params.push(get_value(value));
        }
    }

    Ok((rusqlite::params_from_iter(params), updated_query))
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
    fn test_bind_object_to_params_empty() {
        let values = json!({});
        let query = "SELECT * FROM users".to_owned();
        let result = bind_object_to_params(values, query).unwrap();

        assert_eq!(format!("{:?}", result.0), "ParamsFromIter([])".to_string());
        assert_eq!(result.1, "SELECT * FROM users");
    }

    #[test]
    fn test_bind_object_to_params_single() {
        let values = json!({":name": "Alice"});
        let query = "SELECT * FROM users WHERE name = :name".to_owned();
        let result = bind_object_to_params(values, query).unwrap();

        assert_eq!(
            format!("{:?}", result.0),
            "ParamsFromIter([Text(\"Alice\")])".to_string()
        );
        assert_eq!(result.1, "SELECT * FROM users WHERE name = ?1");
    }

    #[test]
    fn test_bind_object_to_params_multiple() {
        let values = json!({":name": "Alice", ":age": 30});
        let query = "SELECT * FROM users WHERE name = :name AND age = :age".to_owned();
        let result = bind_object_to_params(values, query).unwrap();

        assert_eq!(
            format!("{:?}", result.0),
            "ParamsFromIter([Integer(30), Text(\"Alice\")])".to_string()
        );
        assert_eq!(result.1, "SELECT * FROM users WHERE name = ?2 AND age = ?1");
    }

    #[test]
    fn test_bind_object_to_params_same_key_value_as_string() {
        let values = json!({":name": "Alice"});
        let query = "SELECT * FROM users WHERE name = :name AND name = ':name :name '".to_owned();
        let result = bind_object_to_params(values, query).unwrap();

        assert_eq!(
            format!("{:?}", result.0),
            "ParamsFromIter([Text(\"Alice\")])".to_string()
        );
        assert_eq!(
            result.1,
            "SELECT * FROM users WHERE name = ?1 AND name = ':name :name '"
        );
    }

    #[test]
    fn test_bind_object_to_params_blob() {
        let values = json!({":file": [1,2,3]});
        let query = "SELECT * FROM users WHERE file = :file".to_owned();
        let result = bind_object_to_params(values, query).unwrap();

        assert_eq!(
            format!("{:?}", result.0),
            "ParamsFromIter([Blob([1, 2, 3])])".to_string()
        );
        assert_eq!(result.1, "SELECT * FROM users WHERE file = ?1");
    }

    #[test]
    fn test_bind_object_to_params_invalid_key() {
        let values = json!({"foo": "bar"});
        let query = "SELECT * FROM users WHERE name = :name".to_owned();
        let result = bind_object_to_params(values, query);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid key: foo. A named parameter should start with : or @ or $"
        );
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
