use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use regex::{Captures, Regex};

use crate::config::CONFIG;

use super::commands::GenerateArgs;

use std::fs::create_dir_all;

pub async fn command_generate(command: &GenerateArgs) -> Result<()> {
    let GenerateFiles { up, down } = generate_migration(command)?;

    if let Some(parent_dir) = up.path.parent() {
        create_dir_all(parent_dir)?;
    }

    fs::write(
        up.path,
        up.content.trim().replace("            ", "").as_bytes(),
    )?;
    fs::write(down.path, down.content.as_bytes())?;

    // TODO: generate files from template
    // TODO: info files created

    Ok(())
}

struct File {
    content: String,
    path: PathBuf,
}

struct GenerateFiles {
    up: File,
    down: File,
}

fn generate_migration(command: &GenerateArgs) -> Result<GenerateFiles> {
    let columns = command.columns.clone().into_iter().enumerate().fold(
        String::new(),
        |mut acc, (index, column)| {
            let parts: Vec<&str> = column.split(':').collect();
            let column_name = parts[0].trim();
            let column_type = parts[1].trim();
            let column_type = match column_type {
                "blob" => "BLOB",
                "bool" | "boolean" => "BOOLEAN",
                "date" | "timestamp" => "INTEGER DEFAULT (strftime('%s', 'now'))",
                "float" | "real" => "REAL",
                "integer" => "INTEGER",
                "text" | "tinytext" => "TEXT",
                _ => "TEXT",
            };

            let extra_space = if index < command.columns.len() - 1 {
                "\n                "
            } else {
                ""
            };

            acc.push_str(&format!(
                "{} {} NOT NULL,{}",
                column_name, column_type, extra_space
            ));

            acc
        },
    );

    let database = &command.database;
    let table = &command.table;
    let prefix = now();

    let up_content = format!(
        r#"
            CREATE TABLE IF NOT EXISTS {table}(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                {columns}
                created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
                updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
            );

            CREATE TRIGGER IF NOT EXISTS trigger_{table}_update 
                AFTER UPDATE ON {table}
                BEGIN
                    UPDATE {table}
                    SET updated_at=(strftime('%s', 'now'))
                    WHERE id=OLD.id;
                END;
        "#
    );

    let migrations_folder = &CONFIG.structure.migrations_folder.clone();
    let up_file_path = Path::new(migrations_folder)
        .join(database)
        .join(format!("{prefix}-{table}-up.sql"));

    let down_content = format!(r#"DELETE TABLE {table};"#);
    let down_file_path = Path::new(migrations_folder)
        .join(database)
        .join(format!("{prefix}-{table}-down.sql"));

    let up = File {
        content: up_content,
        path: up_file_path,
    };
    let down = File {
        content: down_content,
        path: down_file_path,
    };

    Ok(GenerateFiles { up, down })
}

fn now() -> std::string::String {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).unwrap();
    let in_seconds = since_the_epoch.as_secs();
    let (year, month, day) = {
        let days = in_seconds / 86400;
        let years = (days - days / 146097 * 3 / 4 + 1) * 400 / 146097;
        let days_of_year = days - (years * 365 + years / 4 - years / 100 + years / 400);
        let months = (days_of_year * 12 + 6) / 367;
        let day = days_of_year - (months * 367 / 12);
        let month = months + 1;
        let year = years + 1970;
        (year as u32, month as u32, day as u32 + 1)
    };

    let hours = (in_seconds / 3600) % 24;
    let minutes = (in_seconds / 60) % 60;
    let seconds = in_seconds % 60;

    format!(
        "{:04}{:02}{:02}{:02}{:02}{:02}",
        year, month, day, hours, minutes, seconds
    )
}

#[derive(Debug)]
pub enum Data {
    Number(i32),
    Boolean(bool),
    String(String),
    Array(Vec<HashMap<String, Data>>),
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Number(num) => write!(f, "{}", num),
            Data::Boolean(b) => write!(f, "{}", b),
            Data::String(s) => write!(f, "{}", s),
            Data::Array(_) => write!(f, "Array"),
        }
    }
}

fn template_engine(template: &str, data: &HashMap<&str, Data>) -> Result<String> {
    let if_else_regex = Regex::new(
        r"\{%\s*if\s*(.*?)\s*%\}((.|\n)*?)(\{%\s*else\s*%\}((.|\n)*?)\{%\s*endif\s*%\}|\{%\s*endif\s*%\})",
    )
    .unwrap();
    let template = if_else_regex
        .replace_all(template, |caps: &Captures| {
            let key = caps.get(1).unwrap().as_str().trim();
            let if_code = caps.get(2).unwrap().as_str().trim();
            let else_code = caps.get(5).map_or("", |m| m.as_str()).trim();

            if let Data::Boolean(exp) = data[key] {
                if exp {
                    if_code.to_string()
                } else {
                    else_code.to_string()
                }
            } else {
                tracing::error!("Parsing key {} as boolean", key);
                exit(1)
            }
        })
        .to_string();

    let for_regex =
        Regex::new(r"\{%\s*for\s*(\w*?)\s*in\s*(\w*?)\s*%\}(\s*)((.|\n)*?)\{%\s*endfor\s*%\}")
            .unwrap();
    let template = for_regex
        .replace_all(&template, |caps: &Captures| {
            let variable_name = caps.get(1).unwrap().as_str().trim();
            let object_name = caps.get(2).unwrap().as_str().trim();
            let pre_space = caps.get(3).unwrap().as_str();
            let code = caps.get(4).unwrap().as_str().trim().to_string();

            let mut repeated_code: Vec<String> = Vec::new();

            if let Data::Array(list) = &data[object_name] {
                for item in list.iter() {
                    for (index, (key, value)) in item.iter().enumerate() {
                        let code = if index == 0 {
                            code.to_string()
                        } else {
                            match repeated_code.last() {
                                Some(value) => value.to_owned(),
                                None => String::new(),
                            }
                        };

                        let replaced_code = code.replace(
                            &format!("{{{{ {}.{} }}}}", variable_name, key),
                            &value.to_string(),
                        );

                        if index == 1 {
                            repeated_code.pop();
                        }

                        repeated_code.push(replaced_code);
                    }
                }
            }

            repeated_code.iter().fold(String::new(), |acc, s| {
                let pre_space = if acc.is_empty() { "" } else { pre_space };
                format!("{}{}{}", acc, pre_space, s)
            })
        })
        .to_string();

    let print_regex = Regex::new(r"\{\{(.*?)\}\}").unwrap();
    let template = print_regex
        .replace_all(&template, |caps: &Captures| {
            let key = caps.get(1).unwrap().as_str().trim();

            if let Some(value) = data.get(key) {
                value.to_string()
            } else {
                String::new()
            }
        })
        .to_string();

    let template = template.replace("{#", "<!--").replace("#}", "-->");

    Ok(template)
}

#[cfg(test)]
mod tests {
    use crate::config::CONFIG;

    use super::*;

    #[test]
    fn test_command_generate() -> Result<()> {
        let command = &GenerateArgs {
            table: "users".to_string(),
            columns: vec![
                "blob:blob".to_string(),
                "boolean:boolean".to_string(),
                "bool:bool".to_string(),
                "date:date".to_string(),
                "integer:integer".to_string(),
                "float:float".to_string(),
                "real:real".to_string(),
                "timestamp:timestamp".to_string(),
                "tinytext:tinytext".to_string(),
                "text:text".to_string(),
            ],
            database: "main".to_string(),
        };

        let expected_up_content = r#"
            CREATE TABLE IF NOT EXISTS users(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                blob BLOB NOT NULL,
                boolean BOOLEAN NOT NULL,
                bool BOOLEAN NOT NULL,
                date INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
                integer INTEGER NOT NULL,
                float REAL NOT NULL,
                real REAL NOT NULL,
                timestamp INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
                tinytext TEXT NOT NULL,
                text TEXT NOT NULL,
                created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
                updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
            );

            CREATE TRIGGER IF NOT EXISTS trigger_users_update 
                AFTER UPDATE ON users
                BEGIN
                    UPDATE users
                    SET updated_at=(strftime('%s', 'now'))
                    WHERE id=OLD.id;
                END;
        "#;
        let expected_down_content = r#"DELETE TABLE users;"#;

        let database = &command.database;
        let migrations_folder = &CONFIG.structure.migrations_folder.clone();
        let expected_up_path = Path::new(migrations_folder).join(database).join(format!(
            "{}-{}-up.sql",
            now(),
            command.table
        ));
        let expected_down_path = Path::new(migrations_folder).join(database).join(format!(
            "{}-{}-down.sql",
            now(),
            command.table
        ));

        let GenerateFiles { up, down } = generate_migration(command)?;

        assert_eq!(up.content.trim(), expected_up_content.trim());
        assert_eq!(down.content.trim(), expected_down_content.trim());
        assert_eq!(up.path, expected_up_path);
        assert_eq!(down.path, expected_down_path);

        Ok(())
    }

    #[test]
    fn test_template_engine() {
        let data = {
            let mut map = HashMap::new();
            map.insert("name", Data::String("John".to_string()));
            map.insert("age", Data::Number(25));
            map.insert("is_adult", Data::Boolean(true));
            let mut sibling1 = HashMap::new();
            sibling1.insert("name".to_string(), Data::String("Jane".to_string()));
            sibling1.insert("age".to_string(), Data::Number(10));
            let mut sibling2 = HashMap::new();
            sibling2.insert("name".to_string(), Data::String("Mike".to_string()));
            sibling2.insert("age".to_string(), Data::Number(30));
            let mut sibling3 = HashMap::new();
            sibling3.insert("name".to_string(), Data::String("Emily".to_string()));
            sibling3.insert("age".to_string(), Data::Number(20));
            map.insert("siblings", Data::Array(vec![sibling1, sibling2, sibling3]));
            map
        };

        let template = r#"
            <h1>Hello, {{ name }}!</h1>
            <p>Age: {{ age }}</p>
            {%     
                if is_adult  %}
            <p>You are an adult.</p>
            {% else   %}
            <p>You are not an adult.</p>
            {% endif %}
            <h2>Your siblings are:</h2>
            <ul>
                {% for   sibling in siblings %}
                <li>{{ sibling.name }} is {{ sibling.age }} years old.</li>
                {% endfor %}
            </ul>
        "#;

        let expected_output = r#"
            <h1>Hello, John!</h1>
            <p>Age: 25</p>
            <p>You are an adult.</p>
            <h2>Your siblings are:</h2>
            <ul>
                <li>Jane is 10 years old.</li>
                <li>Mike is 30 years old.</li>
                <li>Emily is 20 years old.</li>
            </ul>
        "#;

        assert_eq!(template_engine(template, &data).unwrap(), expected_output);
    }
}
