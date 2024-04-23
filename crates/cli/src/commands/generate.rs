use std::{
    collections::HashMap,
    fs::{self, create_dir_all},
    path::{Path, PathBuf},
    process::exit,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::Result;
use regex::{Captures, Regex};
use serde::Deserialize;
use walkdir::WalkDir;

use crate::config::CONFIG;

use super::commands::GenerateArgs;

pub async fn command_generate(command: &GenerateArgs) -> Result<()> {
    let GenerateFiles { up, down } = generate_migration(command)?;

    let up_path = up.path;
    let down_path = down.path;

    if let Some(parent_dir) = up_path.parent() {
        create_dir_all(parent_dir)?;
    }

    fs::write(
        &up_path,
        up.content.trim().replace("            ", "").as_bytes(),
    )?;
    fs::write(&down_path, down.content.as_bytes())?;

    tracing::info!("{}", &up_path.display());
    tracing::info!("{}", &down_path.display());

    let list_of_templates = generate_files_from_templates(command)?;

    for template in list_of_templates {
        let path = template.path;

        if let Some(parent_dir) = path.parent() {
            create_dir_all(parent_dir)?;
        }

        fs::write(&path, template.content.as_bytes())?;
        tracing::info!("{}", &path.display());
    }

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
            let uuid_value = &format!("TEXT UNIQUE CHECK ({column_name} != '') DEFAULT (uuid())",);
            let column_type = match column_type {
                "blob" => "BLOB",
                "boolean" => "BOOLEAN",
                "timestamp" => "INTEGER DEFAULT (strftime('%s', 'now'))",
                "float" | "real" => "REAL",
                "integer" => "INTEGER",
                "number" => "INTEGER",
                "text" | "string" => "TEXT",
                "uuid" => uuid_value,
                _ => "TEXT",
            };

            let extra_space = if index < command.columns.len() - 1 {
                "\n                "
            } else {
                ""
            };

            acc.push_str(&format!(
                "{} {} NOT NULL,{}",
                snake_case(column_name),
                column_type,
                extra_space
            ));

            acc
        },
    );

    let database = &command.database;
    let table = &command.table;
    let table_snake_case = snake_case(table);
    let prefix = now();

    let up_content = format!(
        r#"
            CREATE TABLE IF NOT EXISTS {table_snake_case}(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                uuid TEXT UNIQUE CHECK (uuid != '') DEFAULT (uuid()) NOT NULL,
                {columns}
                created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
                updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
            );

            CREATE TRIGGER IF NOT EXISTS trigger_{table_snake_case}_update 
                AFTER UPDATE ON {table_snake_case}
                BEGIN
                    UPDATE {table_snake_case}
                    SET updated_at=(strftime('%s', 'now'))
                    WHERE id=OLD.id;
                END;
        "#
    );

    let migrations_folder = &CONFIG.structure.migrations_folder.clone();
    let up_file_path = Path::new(migrations_folder)
        .join(database)
        .join(format!("{prefix}-{table}-up.sql"));

    let down_content = format!(r#"DROP TABLE {table_snake_case};"#);

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
struct Template {
    content: String,
    path: PathBuf,
}

fn generate_files_from_templates(command: &GenerateArgs) -> Result<Vec<Template>> {
    let templates_folder = &CONFIG.structure.templates_folder.clone();
    let table = &command.table;
    let mut list_of_templates: Vec<Template> = Vec::new();

    for file in WalkDir::new(templates_folder)
        .into_iter()
        .filter_map(|file| file.ok())
    {
        let file_path = file
            .path()
            .display()
            .to_string()
            .replace(templates_folder, &CONFIG.structure.functions_folder.clone())
            .replace("/**/", &format!("/{table}/"))
            .replace("/**", &format!("/{table}"))
            .replace(".**.", &format!(".{table}."))
            .replace("**.", &format!("{table}."));

        let is_file = file.file_type().is_file();

        if is_file {
            let template = fs::read_to_string(file.path())?;
            let variables = variables_generator(command, table);

            list_of_templates.push(Template {
                content: template_engine(&template, &variables)?,
                path: PathBuf::from(file_path),
            });
        }
    }

    Ok(list_of_templates)
}

fn variables_generator(command: &GenerateArgs, table: &String) -> HashMap<String, Data> {
    let mut map = HashMap::new();
    map.insert(
        "database".to_string(),
        Data::String(command.database.to_string()),
    );

    map.insert("table".to_string(), Data::String(table.to_string()));
    map.insert(
        "tableCamelCase".to_string(),
        Data::String(camel_case(table)),
    );
    map.insert(
        "tableHyphenCase".to_string(),
        Data::String(hyphen_case(table)),
    );
    map.insert(
        "tableSnakeCase".to_string(),
        Data::String(snake_case(table)),
    );
    map.insert("tableDotCase".to_string(), Data::String(dot_case(table)));
    map.insert("tablePathCase".to_string(), Data::String(path_case(table)));
    map.insert(
        "tableConstantCase".to_string(),
        Data::String(constant_case(table)),
    );
    map.insert(
        "tablePascalCase".to_string(),
        Data::String(pascal_case(table)),
    );
    map.insert(
        "tableCapitalCase".to_string(),
        Data::String(capital_case(table)),
    );
    map.insert(
        "tableLowerCase".to_string(),
        Data::String(lower_case(table)),
    );
    map.insert(
        "tableSentenceCase".to_string(),
        Data::String(sentence_case(table)),
    );
    map.insert(
        "tableUpperCase".to_string(),
        Data::String(upper_case(table)),
    );
    map.insert(
        "tableUpperCaseFirst".to_string(),
        Data::String(upper_case_first(table)),
    );
    map.insert(
        "tableLowerCaseFirst".to_string(),
        Data::String(lower_case_first(table)),
    );

    // Length
    map.insert(
        "columnsLength".to_string(),
        Data::Number(command.columns.len() as i32),
    );

    let columns =
        command
            .columns
            .clone()
            .into_iter()
            .enumerate()
            .fold(vec![], |mut acc, (index, column)| {
                let parts: Vec<&str> = column.split(':').collect();
                let column_name = parts[0].trim();
                let column_type = parts[1].trim();

                let mut map_column = HashMap::new();

                map_column.insert("columnIndex".to_string(), Data::Number(index as i32));
                map_column.insert("columnFirst".to_string(), Data::Boolean(0 == index));
                map_column.insert(
                    "columnLast".to_string(),
                    Data::Boolean((command.columns.len() - 1) == index),
                );

                // Column name
                map_column.insert(
                    "columnName".to_string(),
                    Data::String(column_name.to_string()),
                );
                map_column.insert(
                    "columnNameCamelCase".to_string(),
                    Data::String(camel_case(column_name)),
                );
                map_column.insert(
                    "columnNameHyphenCase".to_string(),
                    Data::String(hyphen_case(column_name)),
                );
                map_column.insert(
                    "columnNameSnakeCase".to_string(),
                    Data::String(snake_case(column_name)),
                );
                map_column.insert(
                    "columnNameDotCase".to_string(),
                    Data::String(dot_case(column_name)),
                );
                map_column.insert(
                    "columnNamePathCase".to_string(),
                    Data::String(path_case(column_name)),
                );
                map_column.insert(
                    "columnNameConstantCase".to_string(),
                    Data::String(constant_case(column_name)),
                );
                map_column.insert(
                    "columnNamePascalCase".to_string(),
                    Data::String(pascal_case(column_name)),
                );
                map_column.insert(
                    "columnNameCapitalCase".to_string(),
                    Data::String(capital_case(column_name)),
                );
                map_column.insert(
                    "columnNameLowerCase".to_string(),
                    Data::String(lower_case(column_name)),
                );
                map_column.insert(
                    "columnNameSentenceCase".to_string(),
                    Data::String(sentence_case(column_name)),
                );
                map_column.insert(
                    "columnNameUpperCase".to_string(),
                    Data::String(upper_case(column_name)),
                );
                map_column.insert(
                    "columnNameUpperCaseFirst".to_string(),
                    Data::String(upper_case_first(column_name)),
                );
                map_column.insert(
                    "columnNameLowerCaseFirst".to_string(),
                    Data::String(lower_case_first(column_name)),
                );

                // Column type

                map_column.insert(
                    "columnType".to_string(),
                    Data::String(column_type.to_string()),
                );
                map_column.insert(
                    "columnTypeMatchTS".to_string(),
                    match column_type {
                        "blob" => Data::String("Blob".to_string()),
                        "boolean" => Data::String("boolean".to_string()),
                        "number" => Data::String("number".to_string()),
                        "integer" => Data::String("number".to_string()),
                        "float" => Data::String("number".to_string()),
                        "real" => Data::String("number".to_string()),
                        "timestamp" => Data::String("string".to_string()),
                        "text" => Data::String("string".to_string()),
                        "string" => Data::String("string".to_string()),
                        "uuid" => Data::String("string".to_string()),
                        _ => Data::String("unknown".to_string()),
                    },
                );
                map_column.insert(
                    "columnTypeCamelCase".to_string(),
                    Data::String(camel_case(column_type)),
                );
                map_column.insert(
                    "columnTypeHyphenCase".to_string(),
                    Data::String(hyphen_case(column_type)),
                );
                map_column.insert(
                    "columnTypeSnakeCase".to_string(),
                    Data::String(snake_case(column_type)),
                );
                map_column.insert(
                    "columnTypeDotCase".to_string(),
                    Data::String(dot_case(column_type)),
                );
                map_column.insert(
                    "columnTypePathCase".to_string(),
                    Data::String(path_case(column_type)),
                );
                map_column.insert(
                    "columnTypeConstantCase".to_string(),
                    Data::String(constant_case(column_type)),
                );
                map_column.insert(
                    "columnTypePascalCase".to_string(),
                    Data::String(pascal_case(column_type)),
                );
                map_column.insert(
                    "columnTypeCapitalCase".to_string(),
                    Data::String(capital_case(column_type)),
                );
                map_column.insert(
                    "columnTypeLowerCase".to_string(),
                    Data::String(lower_case(column_type)),
                );
                map_column.insert(
                    "columnTypeSentenceCase".to_string(),
                    Data::String(sentence_case(column_type)),
                );
                map_column.insert(
                    "columnTypeUpperCase".to_string(),
                    Data::String(upper_case(column_type)),
                );
                map_column.insert(
                    "columnTypeUpperCaseFirst".to_string(),
                    Data::String(upper_case_first(column_type)),
                );
                map_column.insert(
                    "columnTypeLowerCaseFirst".to_string(),
                    Data::String(lower_case_first(column_type)),
                );

                acc.push(map_column);

                acc
            });

    let unique_column_types: Vec<String> = columns.iter().fold(vec![], |mut acc, column| {
        let column_type_match_ts = column.get("columnTypeMatchTS").unwrap().to_string();

        if !acc.contains(&column_type_match_ts) {
            acc.push(column_type_match_ts.to_lowercase());
        }

        acc
    });
    map.insert(
        "columnsListOfUniqueTSTypes".to_string(),
        Data::String(unique_column_types.join(", ")),
    );
    map.insert("columns".to_string(), Data::Array(columns));

    map
}

#[derive(Debug, Deserialize)]
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
            Data::Array(a) => write!(f, "{:?}", a),
        }
    }
}

fn template_engine(template: &str, variables: &HashMap<String, Data>) -> Result<String> {
    let for_regex =
        Regex::new(r"\{%\s*for\s*(\w*?)\s*in\s*(\w*?)\s*%\}(\s*)((.|\n)*?)\{%\s*endfor\s*%\}")
            .unwrap();
    let template = for_regex
        .replace_all(template, |caps: &Captures| {
            let variable_name = caps.get(1).unwrap().as_str().trim();
            let object_name = caps.get(2).unwrap().as_str().trim();
            let pre_space = caps.get(3).unwrap().as_str();
            let code = caps.get(4).unwrap().as_str().trim().to_string();

            let mut repeated_code: Vec<String> = Vec::new();

            if let Data::Array(list) = &variables[object_name] {
                for item in list.iter() {
                    let mut replaced_code = code.to_string();

                    for (key, value) in item.iter() {
                        let regex_if = Regex::new(r"\{%\s*if\s*(.*?)\s*%}").unwrap();
                        replaced_code = regex_if
                            .replace_all(&replaced_code, |caps: &Captures| {
                                let capture = caps.get(1).unwrap().as_str().trim();
                                let replaced = format!(" {capture} ")
                                    .replace(
                                        &format!(r" {variable_name}.{key} "),
                                        &format!(" {value} "),
                                    )
                                    .to_string();

                                format!("{{% if {replaced} %}}")
                            })
                            .to_string();

                        let regex_variable =
                            Regex::new(&format!(r"\{{\{{\s*{variable_name}\.{key}\s*\}}\}}",))
                                .unwrap();
                        replaced_code = regex_variable
                            .replace_all(&replaced_code, |_: &regex::Captures| value.to_string())
                            .to_string();
                    }

                    repeated_code.push(replaced_code);
                }
            }

            repeated_code.iter().fold(String::new(), |acc, s| {
                let pre_space = if acc.is_empty() { "" } else { pre_space };
                format!("{}{}{}", acc, pre_space, s)
            })
        })
        .to_string();

    let if_else_regex = Regex::new(
        r"\{%\s*if\s*(.*?)\s*%\}((.|\n)*?)(\{%\s*else\s*%\}((.|\n)*?)\{%\s*endif\s*%\}|\{%\s*endif\s*%\})",
    )
    .unwrap();
    let template = if_else_regex
        .replace_all(&template, |caps: &Captures| {
            let expression = caps.get(1).unwrap().as_str().trim();
            let tokens: Vec<&str> = expression.split_whitespace().collect();

            let key = if tokens.len() >= 3 {
                evaluate(expression, variables).unwrap().to_string()
            } else {
                expression.to_string()
            };
            let if_code = caps.get(2).unwrap().as_str().trim();
            let else_code = caps.get(5).map_or("", |m| m.as_str()).trim();

            if key == "true" {
                if_code.to_string()
            } else if key == "false" {
                else_code.to_string()
            } else if let Data::Boolean(exp) = variables[&key] {
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

    let print_regex = Regex::new(r"\{\{(.*?)\}\}").unwrap();
    let template = print_regex
        .replace_all(&template, |caps: &Captures| {
            let key = caps.get(1).unwrap().as_str().trim();

            if let Some(value) = variables.get(key) {
                value.to_string()
            } else {
                String::new()
            }
        })
        .to_string();

    Ok(template)
}

fn evaluate(expression: &str, variables: &HashMap<String, Data>) -> Result<bool, &'static str> {
    let tokens: Vec<&str> = expression.split_whitespace().collect();

    if expression.contains("&&") {
        let parts = expression.split("&&");

        let values: Vec<bool> = parts
            .map(|part| {
                if let Ok(bool_value) = evaluate(part, variables) {
                    bool_value
                } else {
                    false
                }
            })
            .collect();

        return Ok(values.iter().all(|&x| x));
    } else if expression.contains("||") {
        let parts = expression.split("||");
        let values: Vec<bool> = parts
            .map(|part| {
                if let Ok(bool_value) = evaluate(part, variables) {
                    bool_value
                } else {
                    false
                }
            })
            .collect();

        return Ok(values.iter().any(|&x| x));
    }

    if tokens.len() != 3 {
        return Err("Expression must be in the form 'a operator b'");
    }

    let a = tokens[0];
    let a = if let Some(value) = variables.get(a) {
        value.to_string()
    } else {
        a.to_string()
    };
    let b = tokens[2];
    let b = if let Some(value) = variables.get(b) {
        value.to_string()
    } else {
        b.to_string()
    };

    if let Ok(a_num) = a.parse::<i32>() {
        if let Ok(b_num) = b.parse::<i32>() {
            match tokens[1] {
                "==" => Ok(a_num == b_num),
                "!=" => Ok(a_num != b_num),
                "<" => Ok(a_num < b_num),
                "<=" => Ok(a_num <= b_num),
                ">" => Ok(a_num > b_num),
                ">=" => Ok(a_num >= b_num),
                _ => Err("Unsupported operator"),
            }
        } else {
            Err("b is not a valid i32")
        }
    } else {
        match tokens[1] {
            "==" => Ok(a == b),
            "!=" => Ok(a != b),
            _ => Err("Unsupported operator"),
        }
    }
}

fn camel_case(input: &str) -> String {
    let input = hyphen_case(input);
    let words: Vec<&str> = input.split(|c: char| !c.is_alphanumeric()).collect();
    let camel_case = words
        .iter()
        .enumerate()
        .map(|(i, word)| {
            if i == 0 {
                word.to_lowercase()
            } else {
                format!("{}{}", word[..1].to_uppercase(), word[1..].to_lowercase())
            }
        })
        .collect();
    camel_case
}

fn snake_case(input: &str) -> String {
    input
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>()
        .to_lowercase()
}

fn constant_case(input: &str) -> String {
    input
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>()
        .to_uppercase()
}

fn dot_case(input: &str) -> String {
    input
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '.' })
        .collect::<String>()
        .to_lowercase()
}

fn hyphen_case(input: &str) -> String {
    input
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .to_lowercase()
}

fn path_case(input: &str) -> String {
    input
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '/' })
        .collect::<String>()
        .to_lowercase()
}

fn pascal_case(input: &str) -> String {
    let words: Vec<&str> = input.split(|c: char| !c.is_alphanumeric()).collect();
    let mut pascal_case = String::new();
    for word in words {
        pascal_case.push_str(&word[..1].to_uppercase());
        pascal_case.push_str(&word[1..].to_lowercase());
    }
    pascal_case
}

fn capital_case(input: &str) -> String {
    let input = hyphen_case(input);
    let words: Vec<&str> = input.split(|c: char| !c.is_alphanumeric()).collect();
    let mut capital_case = String::new();
    for word in words {
        capital_case.push_str(&format!(
            "{}{} ",
            word[..1].to_uppercase(),
            word[1..].to_lowercase()
        ));
    }
    capital_case.trim().to_string()
}

fn lower_case(input: &str) -> String {
    input.to_lowercase()
}

fn sentence_case(input: &str) -> String {
    let input = hyphen_case(input);
    let words: Vec<&str> = input.split(|c: char| !c.is_alphanumeric()).collect();
    let sentence_case = words
        .iter()
        .enumerate()
        .map(|(i, word)| {
            if i == 0 {
                format!("{}{} ", word[..1].to_uppercase(), word[1..].to_lowercase())
            } else {
                word.to_lowercase()
            }
        })
        .collect::<String>()
        .trim()
        .to_string();
    sentence_case
}

fn upper_case(input: &str) -> String {
    input.to_uppercase()
}

fn upper_case_first(input: &str) -> String {
    let mut chars = input.chars();
    if let Some(first_char) = chars.next() {
        format!("{}{}", first_char.to_uppercase(), chars.as_str())
    } else {
        String::new()
    }
}

fn lower_case_first(input: &str) -> String {
    let mut chars = input.chars();
    if let Some(first_char) = chars.next() {
        format!("{}{}", first_char.to_lowercase(), chars.as_str())
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::config::CONFIG;

    use super::*;

    #[test]
    fn test_command_generate() -> Result<()> {
        let command = &GenerateArgs {
            table: "test-table".to_string(),
            columns: vec![
                "blob:blob".to_string(),
                "boolean:boolean".to_string(),
                "number:number".to_string(),
                "integer:integer".to_string(),
                "float:float".to_string(),
                "real:real".to_string(),
                "timestamp:timestamp".to_string(),
                "string:string".to_string(),
                "text:text".to_string(),
                "uuid:uuid".to_string(),
            ],
            database: "main".to_string(),
        };

        let expected_up_content = r#"
            CREATE TABLE IF NOT EXISTS test_table(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                uuid TEXT UNIQUE CHECK (uuid != '') DEFAULT (uuid()) NOT NULL,
                blob BLOB NOT NULL,
                boolean BOOLEAN NOT NULL,
                number INTEGER NOT NULL,
                integer INTEGER NOT NULL,
                float REAL NOT NULL,
                real REAL NOT NULL,
                timestamp INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
                string TEXT NOT NULL,
                text TEXT NOT NULL,
                uuid TEXT UNIQUE CHECK (uuid != '') DEFAULT (uuid()) NOT NULL,
                created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
                updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
            );

            CREATE TRIGGER IF NOT EXISTS trigger_test_table_update 
                AFTER UPDATE ON test_table
                BEGIN
                    UPDATE test_table
                    SET updated_at=(strftime('%s', 'now'))
                    WHERE id=OLD.id;
                END;
        "#;
        let expected_down_content = r#"DROP TABLE test_table;"#;

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
    fn test_camel_case() {
        assert_eq!(camel_case("t es-t_na.me"), "tEsTNaMe");
        assert_eq!(camel_case("He_llo wOr$ld"), "heLloWorLd");
    }

    #[test]
    fn test_hyphen_case() {
        assert_eq!(hyphen_case("t es-t_na.me"), "t-es-t-na-me");
        assert_eq!(hyphen_case("He_llo wOr$ld"), "he-llo-wor-ld");
    }

    #[test]
    fn test_snake_case() {
        assert_eq!(snake_case("t es-t_na.me"), "t_es_t_na_me");
        assert_eq!(snake_case("He_llo wOr$ld"), "he_llo_wor_ld");
    }

    #[test]
    fn test_dot_case() {
        assert_eq!(dot_case("t es-t_na.me"), "t.es.t.na.me");
        assert_eq!(dot_case("He_llo wOr$ld"), "he.llo.wor.ld");
    }

    #[test]
    fn test_path_case() {
        assert_eq!(path_case("t es-t_na.me"), "t/es/t/na/me");
        assert_eq!(path_case("He_llo wOr$ld"), "he/llo/wor/ld");
    }

    #[test]
    fn test_constant_case() {
        assert_eq!(constant_case("t es-t_na.me"), "T_ES_T_NA_ME");
        assert_eq!(constant_case("He_llo wOr$ld"), "HE_LLO_WOR_LD");
    }

    #[test]
    fn test_pascal_case() {
        assert_eq!(pascal_case("t es-t_na.me"), "TEsTNaMe");
        assert_eq!(pascal_case("He_llo wOr$ld"), "HeLloWorLd");
    }

    #[test]
    fn test_capital_case() {
        assert_eq!(capital_case("t es-t_na.me"), "T Es T Na Me");
        assert_eq!(capital_case("He_llo wOr$ld"), "He Llo Wor Ld");
    }

    #[test]
    fn test_lower_case() {
        assert_eq!(lower_case("t es-t_na.me"), "t es-t_na.me");
        assert_eq!(lower_case("He_llo wOr$ld"), "he_llo wor$ld");
    }

    #[test]
    fn test_sentence_case() {
        assert_eq!(sentence_case("tesT_Name"), "Test name");
    }

    #[test]
    fn test_upper_case() {
        assert_eq!(upper_case("test name"), "TEST NAME");
    }

    #[test]
    fn test_upper_case_first() {
        assert_eq!(upper_case_first("test name"), "Test name");
    }

    #[test]
    fn test_lower_case_first() {
        assert_eq!(lower_case_first("TEST NAME"), "tEST NAME");
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::commands::generate::GenerateArgs;

        #[test]
        fn test_variables_generator() {
            let command = GenerateArgs {
                database: "test_db.sql".to_string(),
                table: "test_table".to_string(),
                columns: vec![
                    "blob:blob".to_string(),
                    "boolean:boolean".to_string(),
                    "number:number".to_string(),
                    "integer:integer".to_string(),
                    "float:float".to_string(),
                    "real:real".to_string(),
                    "timestamp:timestamp".to_string(),
                    "string:string".to_string(),
                    "text:text".to_string(),
                    "uuid:uuid".to_string(),
                ],
            };
            let table = "test_table".to_string();

            let result = variables_generator(&command, &table);

            assert_eq!(result.get("database").unwrap().to_string(), "test_db.sql");
            assert_eq!(result.get("table").unwrap().to_string(), "test_table");

            let keys = vec![
                "tableCamelCase",
                "tableHyphenCase",
                "tableSnakeCase",
                "tableDotCase",
                "tablePathCase",
                "tableConstantCase",
                "tablePascalCase",
                "tableCapitalCase",
                "tableLowerCase",
                "tableSentenceCase",
                "tableUpperCase",
                "tableUpperCaseFirst",
                "tableLowerCaseFirst",
            ];

            for key in keys {
                assert!(result.get(key).is_some());
            }

            assert_eq!(
                result.get("columnsLength").unwrap().to_string(),
                "10".to_string()
            );
            assert_eq!(
                result
                    .get("columnsListOfUniqueTSTypes")
                    .unwrap()
                    .to_string(),
                "blob, boolean, number, string"
            );

            let columns = match result.get("columns").unwrap() {
                Data::Array(columns) => columns,
                _ => panic!("Expected columns to be an array"),
            };

            assert_eq!(columns.len(), 10);

            columns.iter().enumerate().for_each(|(index, column)| {
                if index == 0 {
                    assert_eq!(column.get("columnFirst").unwrap().to_string(), "true");
                } else {
                    assert_eq!(column.get("columnFirst").unwrap().to_string(), "false");
                }

                if index == 9 {
                    assert_eq!(column.get("columnLast").unwrap().to_string(), "true");
                } else {
                    assert_eq!(column.get("columnLast").unwrap().to_string(), "false");
                }

                assert_eq!(
                    column.get("columnIndex").unwrap().to_string(),
                    index.to_string()
                );

                assert_eq!(
                    column.get("columnName").unwrap().to_string(),
                    command.columns[index].split(':').collect::<Vec<&str>>()[0]
                );

                assert_eq!(
                    column.get("columnType").unwrap().to_string(),
                    command.columns[index].split(':').collect::<Vec<&str>>()[1]
                );

                if index == 0 {
                    assert_eq!(column.get("columnTypeMatchTS").unwrap().to_string(), "Blob");
                    assert_eq!(column.get("columnType").unwrap().to_string(), "blob");
                } else if index == 1 {
                    assert_eq!(
                        column.get("columnTypeMatchTS").unwrap().to_string(),
                        "boolean"
                    );
                    assert_eq!(column.get("columnType").unwrap().to_string(), "boolean");
                } else if index == 2 {
                    assert_eq!(
                        column.get("columnTypeMatchTS").unwrap().to_string(),
                        "number"
                    );
                    assert_eq!(column.get("columnType").unwrap().to_string(), "number");
                } else if index == 3 {
                    assert_eq!(
                        column.get("columnTypeMatchTS").unwrap().to_string(),
                        "number"
                    );
                    assert_eq!(column.get("columnType").unwrap().to_string(), "integer");
                } else if index == 4 {
                    assert_eq!(
                        column.get("columnTypeMatchTS").unwrap().to_string(),
                        "number"
                    );
                    assert_eq!(column.get("columnType").unwrap().to_string(), "float");
                } else if index == 5 {
                    assert_eq!(
                        column.get("columnTypeMatchTS").unwrap().to_string(),
                        "number"
                    );
                    assert_eq!(column.get("columnType").unwrap().to_string(), "real");
                } else if index == 6 {
                    assert_eq!(
                        column.get("columnTypeMatchTS").unwrap().to_string(),
                        "string"
                    );
                    assert_eq!(column.get("columnType").unwrap().to_string(), "timestamp");
                } else if index == 7 {
                    assert_eq!(
                        column.get("columnTypeMatchTS").unwrap().to_string(),
                        "string"
                    );
                    assert_eq!(column.get("columnType").unwrap().to_string(), "string");
                } else if index == 8 {
                    assert_eq!(
                        column.get("columnTypeMatchTS").unwrap().to_string(),
                        "string"
                    );
                    assert_eq!(column.get("columnType").unwrap().to_string(), "text");
                } else if index == 9 {
                    assert_eq!(
                        column.get("columnTypeMatchTS").unwrap().to_string(),
                        "string"
                    );
                    assert_eq!(column.get("columnType").unwrap().to_string(), "uuid");
                }

                let keys = vec![
                    "columnNameCamelCase",
                    "columnNameHyphenCase",
                    "columnNameSnakeCase",
                    "columnNameDotCase",
                    "columnNamePathCase",
                    "columnNameConstantCase",
                    "columnNamePascalCase",
                    "columnNameCapitalCase",
                    "columnNameLowerCase",
                    "columnNameSentenceCase",
                    "columnNameUpperCase",
                    "columnNameUpperCaseFirst",
                    "columnNameLowerCaseFirst",
                    "columnTypeCamelCase",
                    "columnTypeHyphenCase",
                    "columnTypeSnakeCase",
                    "columnTypeDotCase",
                    "columnTypePathCase",
                    "columnTypeConstantCase",
                    "columnTypePascalCase",
                    "columnTypeCapitalCase",
                    "columnTypeLowerCase",
                    "columnTypeSentenceCase",
                    "columnTypeUpperCase",
                    "columnTypeUpperCaseFirst",
                    "columnTypeLowerCaseFirst",
                ];

                for key in keys {
                    assert!(column.get(key).is_some());
                }
            });
        }
    }

    #[test]
    fn test_template_engine() {
        let data = {
            let mut map = HashMap::new();
            map.insert("name".to_string(), Data::String("John".to_string()));
            map.insert("age".to_string(), Data::Number(25));
            map.insert("is_adult".to_string(), Data::Boolean(true));
            let mut sibling1 = HashMap::new();
            sibling1.insert("name".to_string(), Data::String("Jane".to_string()));
            sibling1.insert("age".to_string(), Data::Number(10));
            sibling1.insert("is_adult".to_string(), Data::Boolean(false));
            sibling1.insert("age1".to_string(), Data::Number(25));
            sibling1.insert("age2".to_string(), Data::Number(25));
            sibling1.insert("age3".to_string(), Data::Number(25));
            sibling1.insert("age4".to_string(), Data::Number(25));
            let mut sibling2 = HashMap::new();
            sibling2.insert("name".to_string(), Data::String("Mike".to_string()));
            sibling2.insert("age".to_string(), Data::Number(30));
            sibling2.insert("is_adult".to_string(), Data::Boolean(true));
            let mut sibling3 = HashMap::new();
            sibling3.insert("name".to_string(), Data::String("Emily".to_string()));
            sibling3.insert("age".to_string(), Data::Number(20));
            sibling3.insert("is_adult".to_string(), Data::Boolean(true));
            map.insert(
                "siblings".to_string(),
                Data::Array(vec![sibling1, sibling2, sibling3]),
            );
            map
        };

        let template = r#"
            <h1>Hello, {{ name }}!</h1>
            <p>Age: {{ age }}</p>
            {% if is_adult == true %}
            <p>You are an adult.</p>
            {% else   %}
            <p>You are not an adult.</p>
            {% endif %}
            <h2>Your siblings are:</h2>
            <ul>
                {% for   sibling in siblings %}
                <li>{{ sibling.name }} is {{ sibling.age }} years old.</li>
                {% endfor %}
                {% for sibling in siblings %}<li>{{sibling.name}} is {{ sibling.age }} years old.</li>{% endfor %}
                {% for sibling in siblings %}
                {% if sibling.is_adult == false %}
                <li>Is: {{ sibling.name }}</li>
                {% else %}
                <li>Else: {{ sibling.name }}</li>
                {% endif %}
                {% if sibling.name == Mike || sibling.name == Emily && sibling.is_adult == true %}
                <li>Is Multiple: {{ sibling.name }}</li>
                {% endif %}
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
                <li>Jane is 10 years old.</li><li>Mike is 30 years old.</li><li>Emily is 20 years old.</li>
                <li>Is: Jane</li>
                
                <li>Else: Mike</li>
                <li>Is Multiple: Mike</li>
                <li>Else: Emily</li>
                <li>Is Multiple: Emily</li>
            </ul>
        "#;

        assert_eq!(template_engine(template, &data).unwrap(), expected_output);
    }

    #[test]
    fn test_template_engine_if_expressions() {
        let data = {
            let mut map = HashMap::new();
            map.insert("name".to_string(), Data::String("John".to_string()));
            map.insert("age".to_string(), Data::Number(25));
            map.insert("is_adult".to_string(), Data::Boolean(true));
            map
        };

        let template = r#"
            {% if 1 == 1 %}
            <p>1 is == 1</p>
            {% endif %}
            {% if is_adult == true %}
            <p>You are an adult</p>
            {% endif %}
            {% if is_adult != false %}
            <p>You are not a child</p>
            {% endif %}
            {% if age == 25 %}
            <p>You have 25 years</p>
            {% endif %}
            {% if age >= 25 %}
            <p>You have >= 25 years</p>
            {% endif %}
            {% if age <= 25 %}
            <p>You have <= 25 years</p>
            {% endif %}
            {% if age > 18 %}
            <p>You have more than 18 years</p>
            {% endif %}
            {% if age < 30 %}
            <p>You have less than 30 years</p>
            {% endif %}
            {% if name == John %}
            <p>Your name is John</p>
            {% endif %}
            {% if name != Jane %}
            <p>Your name is not Jane</p>
            {% endif %}
        "#;

        let expected_output = r#"
            <p>1 is == 1</p>
            <p>You are an adult</p>
            <p>You are not a child</p>
            <p>You have 25 years</p>
            <p>You have >= 25 years</p>
            <p>You have <= 25 years</p>
            <p>You have more than 18 years</p>
            <p>You have less than 30 years</p>
            <p>Your name is John</p>
            <p>Your name is not Jane</p>
        "#;

        assert_eq!(template_engine(template, &data).unwrap(), expected_output);
    }

    #[test]
    fn test_evaluate_and() {
        let mut data = HashMap::new();
        data.insert("a".to_string(), Data::Boolean(true));

        let result = evaluate("a == a && a == a", &data);
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_evaluate_and_false() {
        let mut data = HashMap::new();
        data.insert("a".to_string(), Data::Boolean(true));

        let result = evaluate("a == a && a != a", &data);
        assert_eq!(result.unwrap(), false);
    }

    #[test]
    fn test_evaluate_or() {
        let mut data = HashMap::new();
        data.insert("a".to_string(), Data::Boolean(false));

        let result = evaluate("a != a || a == a", &data);
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_evaluate_or_false() {
        let mut data = HashMap::new();
        data.insert("a".to_string(), Data::Boolean(false));

        let result = evaluate("a != a || a != a", &data);
        assert_eq!(result.unwrap(), false);
    }
}
