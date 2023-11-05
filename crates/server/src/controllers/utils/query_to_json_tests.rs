#[cfg(test)]
mod tests {
    use rusqlite::Connection;
    use serde_json::json;

    use crate::sqlite::query_to_json::query_to_json;

    #[test]
    fn should_return_a_json_without_being_filtered_by_params() {
        let db = Connection::open_in_memory().unwrap();

        db.execute(
            r"
                CREATE TABLE test (
                    text TEXT,
                    int INTEGER NOT NULL DEFAULT (0)
                )
            ",
            (),
        )
        .unwrap();
        db.execute("INSERT INTO test (text) VALUES ('test 0')", ())
            .unwrap();
        db.execute("INSERT INTO test (text, int) VALUES ('test 1', 1)", ())
            .unwrap();

        let stmt = db.prepare("SELECT * from test");
        let json = query_to_json(stmt.unwrap(), []).unwrap();

        assert_eq!(
            "[{\"int\":0,\"text\":\"test 0\"},{\"int\":1,\"text\":\"test 1\"}]",
            json.to_string()
        );
    }

    #[test]
    fn should_return_a_json_filtered_by_params_using_types() {
        let db = Connection::open_in_memory().unwrap();

        db.execute(
            r"
                CREATE TABLE test (
                    text TEXT,
                    int INTEGER NOT NULL DEFAULT (0),
                    real REAL,
                    blob BLOB,
                    nil NULL
                )
            ",
            (),
        )
        .unwrap();
        db.execute("INSERT INTO test (text) VALUES ('test 0')", ())
            .unwrap();
        db.execute(
            "INSERT INTO test (text, int, real, blob, nil) VALUES ('test 1', 1, 0.1, ?, null)",
            [b"0102030405060708090a0b0c0d0e0f"],
        )
        .unwrap();

        let stmt = db.prepare("SELECT * from test WHERE text = ? AND int = ? AND real = ?");
        let _json = query_to_json(stmt.unwrap(), ["test 1", "1", "0.1"]).unwrap();

        assert_eq!(
            json!([
            {
                "blob": b"0102030405060708090a0b0c0d0e0f",
                "int":1,
                "nil": null,
                "real": 0.1,
                "text":"test 1"
            }
            ])
            .to_string(),
            _json.to_string()
        );
    }

    #[test]
    fn should_return_an_invalid_param_error() {
        let db = Connection::open_in_memory().unwrap();

        db.execute(
            r"
                CREATE TABLE test (
                    text TEXT,
                    int INTEGER NOT NULL DEFAULT (0)
                )
            ",
            (),
        )
        .unwrap();
        db.execute("INSERT INTO test (text) VALUES ('test 0')", ())
            .unwrap();
        db.execute("INSERT INTO test (text, int) VALUES ('test 1', 1)", ())
            .unwrap();

        let stmt = db.prepare("SELECT * from test");

        assert_eq!(
            r#"Invalid parameter name: :no-no"#,
            query_to_json(stmt.unwrap(), &[(":no-no", "1")])
                .unwrap_err()
                .to_string()
        );
    }
}
