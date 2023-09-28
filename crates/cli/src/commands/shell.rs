use reqwest::Method;
use rustyline::{error::ReadlineError, DefaultEditor};
use serde_json::json;

use crate::{config::CONFIG, utils::http_client, utils::json_to_table, utils::line_break};

use super::commands::ShellArgs;

pub async fn command_shell(command: &ShellArgs) -> anyhow::Result<()> {
    let mut rl = DefaultEditor::new()?;
    let load_history = rl.load_history(CONFIG.cli.history_file_path.as_str());

    line_break();
    eprintln!("Welcome to the Query CLI for your SQLite Server!");
    line_break();
    eprintln!(r#"Type ".quit" or CTRL-C to exit the shell"#);
    line_break();

    if load_history.is_err() {
        eprintln!(r#"No "history_file_path" defined in the Query.toml config file."#);
        line_break();
    }

    loop {
        let readline = rl.readline(&format!("{}> ", ""));

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;

                let line = line.trim().to_string();

                if line == ".quit" {
                    line_break();
                    break;
                }

                let line = if line.starts_with(".tables") {
                    let pattern = line.replace(".tables", "").trim().to_owned();

                    if pattern.is_empty() {
                        "SELECT DISTINCT name FROM pragma_table_list WHERE name NOT LIKE 'sqlite%'"
                            .to_string()
                    } else {
                        format!(
                            "SELECT DISTINCT name FROM pragma_table_list WHERE name NOT LIKE 'sqlite%' AND name LIKE '%{}%'",
                            &pattern
                        )
                    }
                } else if line.starts_with(".schema") {
                    let table_name = line.replace(".schema", "").trim().to_owned();

                    if table_name.is_empty() {
                        "SELECT sql FROM sqlite_master WHERE name NOT LIKE 'sqlite_%'".to_string()
                    } else {
                        format!(
                            "SELECT sql FROM sqlite_master WHERE name LIKE '%{}%'",
                            &table_name
                        )
                    }
                } else {
                    line
                };

                let body = json!({
                    "db_name": command.db_name,
                    "query": line
                })
                .to_string();

                let value = match http_client("query", Some(&body), Method::POST).await {
                    Ok(value) => value,
                    Err(err) => {
                        eprintln!("Error: {:?}", err);
                        continue;
                    }
                };

                if value["data"][0].is_null() {
                    line_break();
                    eprintln!("No data returned.");
                    line_break();
                    continue;
                }

                line_break();
                eprintln!("{}", json_to_table(&value["data"])?);
                line_break();
            }
            Err(ReadlineError::Interrupted) => {
                line_break();
                break;
            }
            Err(ReadlineError::Eof) => {}
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    if load_history.is_ok() {
        rl.save_history(CONFIG.cli.history_file_path.as_str())?;
    }

    Ok(())
}
