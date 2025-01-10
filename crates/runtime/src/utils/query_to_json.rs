use rusqlite::{types::Value as RusqliteValue, Error, Params, Statement};
use serde_json::{json, Value as JsonValue};

pub fn query_to_json<P: Params>(stmt: &mut Statement, params: P) -> Result<JsonValue, Error> {
    let names = stmt
        .column_names()
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>();
    let mut rows = stmt.query(params)?;
    let mut values = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let mut prev = json!({});
        for name in names.iter() {
            let value = match row.get::<_, _>(name.as_ref())? {
                RusqliteValue::Null => JsonValue::Null,
                RusqliteValue::Integer(v) => JsonValue::from(v),
                RusqliteValue::Real(v) => JsonValue::from(v),
                RusqliteValue::Text(v) => JsonValue::from(v),
                RusqliteValue::Blob(v) => JsonValue::from(v),
            };
            let next = json!({ name.clone(): value });
            _merge(&mut prev, &next);
        }
        values.push(prev)
    }

    Ok(serde_json::Value::Array(values))
}

fn _merge(prev: &mut JsonValue, next: &JsonValue) {
    match (prev, next) {
        (&mut JsonValue::Object(ref mut prev), JsonValue::Object(next)) => {
            for (k, v) in next {
                _merge(prev.entry(k.clone()).or_insert(JsonValue::Null), v);
            }
        }
        (prev, next) => {
            *prev = next.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use rusqlite::Connection;

    use crate::utils::bind_to_params::{bind_array_to_params, bind_named_params};

    use super::*;

    #[test]
    fn test_query_to_json_array_params() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, age INTEGER)",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO users (name, age) VALUES ('Alice', 25), ('Bob', 30)",
            [],
        )
        .unwrap();

        let query = "SELECT * FROM users WHERE age > ?";
        let values: JsonValue = serde_json::from_str("[24]").unwrap();

        let params = bind_array_to_params(values);
        let mut statement = conn.prepare(query).unwrap();
        let result = query_to_json(&mut statement, params).unwrap();

        assert_eq!(result.as_array().unwrap().len(), 2);
        assert_eq!(result[0]["name"], "Alice");
        assert_eq!(result[0]["age"], 25);
        assert_eq!(result[1]["name"], "Bob");
        assert_eq!(result[1]["age"], 30);
    }

    #[test]
    fn test_query_to_json_named_params() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, age INTEGER)",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO users (name, age) VALUES ('Alice', 25), ('Bob', 30)",
            [],
        )
        .unwrap();

        let query = "SELECT * FROM users WHERE age > :age";
        let values: JsonValue = serde_json::from_str(r#"{":age": 24}"#).unwrap();

        let params_bound = bind_named_params(values);
        let params: &[(&str, &dyn rusqlite::ToSql)] = &params_bound
            .iter()
            .map(|(name, val)| (name.as_str(), val as &dyn rusqlite::ToSql))
            .collect::<Vec<_>>();
        let mut statement = conn.prepare(query).unwrap();

        let result = query_to_json(&mut statement, params).unwrap();

        assert_eq!(result.as_array().unwrap().len(), 2);
        assert_eq!(result[0]["name"], "Alice");
        assert_eq!(result[0]["age"], 25);
        assert_eq!(result[1]["name"], "Bob");
        assert_eq!(result[1]["age"], 30);
    }
}
