use std::collections::HashMap;

use rusqlite::{types::Value as RusqliteValue, Error, Params, Statement};

use super::value::Value;

pub fn statement_to_vec<P: Params>(
    mut stmt: Statement,
    params: P,
) -> Result<Vec<HashMap<String, Value>>, Error> {
    let names = stmt
        .column_names()
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>();
    let mut rows = stmt.query(params)?;
    let mut values = Vec::new();

    while let Some(each_row) = rows.next()? {
        let mut hash_map = HashMap::new();

        for name in names.iter() {
            let value = match each_row.get::<_, _>(name.as_ref())? {
                RusqliteValue::Null => Value::Null,
                RusqliteValue::Integer(v) => Value::Integer(v),
                RusqliteValue::Real(v) => Value::Float(v),
                RusqliteValue::Text(v) => Value::Text(v),
                RusqliteValue::Blob(v) => Value::Blob(v),
            };

            hash_map.insert(name.clone(), value);
        }

        values.push(hash_map);
    }

    Ok(values)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rusqlite::Connection;

    use crate::controllers::utils::{statement_to_vec::statement_to_vec, value::Value};

    #[test]
    fn test_return_a_vec() {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute(
            r"
                CREATE TABLE test (
                    text TEXT,
                    int INTEGER NOT NULL DEFAULT (0),
                    float REAL NOT NULL DEFAULT (0.1),
                    blob BLOB,
                    nll TEXT
                )
            ",
            (),
        )
        .unwrap();

        conn.execute(
            "INSERT INTO test (text, blob) VALUES ('test 0', ?1)",
            [vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]],
        )
        .unwrap();

        conn.execute(
            "INSERT INTO test (text, int, float, blob, nll) VALUES ('test 1', 1, 0.1, ?1, NULL)",
            [vec![]],
        )
        .unwrap();

        let stmt = conn.prepare("SELECT * from test");
        let values = statement_to_vec(stmt.unwrap(), []).unwrap();

        let mut hash_map_0 = HashMap::new();
        let mut hash_map_1 = HashMap::new();

        hash_map_0.insert("text".to_string(), Value::Text("test 0".to_string()));
        hash_map_0.insert("int".to_string(), Value::Integer(0));
        hash_map_0.insert("float".to_string(), Value::Float(0.1));
        hash_map_0.insert(
            "blob".to_string(),
            Value::Blob(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
        );
        hash_map_0.insert("nll".to_string(), Value::Null);

        hash_map_1.insert("text".to_string(), Value::Text("test 1".to_string()));
        hash_map_1.insert("int".to_string(), Value::Integer(1));
        hash_map_1.insert("float".to_string(), Value::Float(0.1));
        hash_map_1.insert("blob".to_string(), Value::Blob(vec![]));
        hash_map_1.insert("nll".to_string(), Value::Null);

        assert_eq!(vec![hash_map_0, hash_map_1], values);
    }
}
