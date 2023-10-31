use rusqlite::{types::Value as RusqliteValue, Error, Params, Statement};
use serde_json::{json, Value as JsonValue};

#[allow(dead_code)]
pub fn query_to_json<P: Params>(mut stmt: Statement, params: P) -> Result<JsonValue, Error> {
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
