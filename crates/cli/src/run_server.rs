use std::{
    env,
    io::BufRead,
    process::{exit, Command, Stdio},
    thread,
};

use colored::Colorize;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::ser::PrettyFormatter;
use serde_json::Serializer;

use crate::utils::{detect_package_manager, has_node_modules_binary, which};

const QUERY_SERVER_MODULE: &str = "@qery/query-server";
const QUERY_SERVER_BINARY: &str = "query-server";
const ESBUILD_MODULE: &str = "esbuild";
const ESBUILD_BINARY: &str = "esbuild";

pub fn run_query_server(verbose: bool, silent: bool) {
    let pm = detect_package_manager();

    let query_server_global = which(QUERY_SERVER_BINARY).unwrap_or_default();
    let hash_query_server_global = !query_server_global.is_empty();
    let hash_query_server_local_binary = has_node_modules_binary(QUERY_SERVER_BINARY);
    let hash_query_server = hash_query_server_local_binary || hash_query_server_global;

    let esbuild_global = which(ESBUILD_BINARY).unwrap_or_default();
    let hash_esbuild_global = !esbuild_global.is_empty();
    let hash_esbuild_local_binary = has_node_modules_binary(ESBUILD_BINARY);
    let hash_esbuild = hash_esbuild_local_binary || hash_esbuild_global;

    if !hash_query_server && !hash_esbuild {
        eprintln!(
            "The {} and {} modules aren't installed.",
            QUERY_SERVER_BINARY, ESBUILD_BINARY
        );
        eprintln!(
            "Please, run `{} install --save-dev {} {}` first.",
            pm.npm, QUERY_SERVER_MODULE, ESBUILD_MODULE
        );
        exit(1);
    }

    if !hash_query_server {
        eprintln!("The {} module isn't installed.", QUERY_SERVER_MODULE);
        eprintln!(
            "Please, run `{} install --save-dev {}` first.",
            pm.npm, QUERY_SERVER_MODULE
        );
        exit(1);
    }

    if !hash_esbuild {
        eprintln!("The {} module isn't installed.", ESBUILD_MODULE);
        eprintln!(
            "Please, run `{} install --save-dev {}` first.",
            pm.npm, ESBUILD_MODULE
        );
        exit(1);
    }

    let mut child: std::process::Child = if hash_query_server_local_binary {
        let current_dir = env::current_dir().unwrap();
        let package = current_dir
            .join("node_modules")
            .join(".bin")
            .join(QUERY_SERVER_BINARY);

        let package = if env::var("QUERY_CLI_DEV").is_ok() {
            let exe_path = env::current_exe().unwrap();
            let exe_dir = exe_path.parent().unwrap();

            match env::var("QUERY_CLI_DEV") {
                Ok(_) => &format!("{}/query-server", exe_dir.display()),
                Err(_) => {
                    eprintln!("Failed get the query-server path");
                    exit(1);
                }
            }
        } else {
            match package.to_str() {
                Some(package) => package,
                None => {
                    eprintln!("Failed to convert path to string");
                    exit(1);
                }
            }
        };

        let stdout = if silent {
            Stdio::null()
        } else {
            Stdio::piped()
        };

        let stderr = if silent {
            Stdio::null()
        } else {
            Stdio::piped()
        };

        match Command::new(package).stdout(stdout).stderr(stderr).spawn() {
            Ok(child) => child,
            Err(e) => {
                let pm = detect_package_manager();
                let npx = pm.npx.to_string();
                let npx: Vec<&str> = npx.split(' ').collect::<Vec<&str>>();
                let npx = npx[0];

                eprintln!(
                    "Failed to execute command `{} {}`",
                    npx, QUERY_SERVER_BINARY
                );
                eprintln!("Error: {}", e);
                exit(1);
            }
        }
    } else {
        let stdout = if silent {
            Stdio::null()
        } else {
            Stdio::piped()
        };

        let stderr = if silent {
            Stdio::null()
        } else {
            Stdio::piped()
        };

        let query_server_global = if env::var("QUERY_CLI_DEV").is_ok() {
            match env::var("QUERY_CLI_DEV") {
                Ok(dev_path) => &format!("{dev_path}/query-server"),
                Err(_) => &query_server_global,
            }
        } else {
            &query_server_global
        };

        match Command::new(query_server_global)
            .stdout(stdout)
            .stderr(stderr)
            .spawn()
        {
            Ok(child) => child,
            Err(e) => {
                eprintln!("Failed to execute command `{}`", QUERY_SERVER_BINARY);
                eprintln!("Error: {}", e);
                exit(1);
            }
        }
    };

    let stdout_thread = thread::spawn(move || {
        let stdout = child.stdout.take().expect("Failed to open stdout");
        let reader = std::io::BufReader::new(stdout);
        server_logs(reader, verbose);
    });

    let stderr_thread = thread::spawn(move || {
        let stderr = child.stderr.take().expect("Failed to open stderr");
        let reader = std::io::BufReader::new(stderr);
        server_logs(reader, verbose);
    });

    let _ = stdout_thread.join();
    let _ = stderr_thread.join();
}

fn server_logs<T>(mut reader: std::io::BufReader<T>, verbose: bool)
where
    T: std::io::Read,
{
    let mut line = String::new();
    loop {
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                let trimmed_line = line.trim();
                let formatted_log = if verbose {
                    parse_and_format::<LogRecord>(trimmed_line)
                } else {
                    parse_and_format::<LogAdd>(trimmed_line)
                };

                if let Some(log) = formatted_log {
                    eprintln!("{}", log);
                }
            }
            Err(e) => {
                eprintln!("Error reading output: {}", e);
                break;
            }
        }
        line.clear();
    }
}

fn parse_and_format<T: serde::de::DeserializeOwned + LogFormat>(line: &str) -> Option<String> {
    match serde_json::from_str::<T>(line) {
        Ok(log) => {
            let formatted_log = log.format();
            if !formatted_log.is_empty() {
                Some(formatted_log)
            } else {
                None
            }
        }
        Err(e) => {
            eprintln!("Error: {}", line);
            eprintln!("Failed to parse log: {}", e);
            None
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LogAdd {
    pub code: Option<u16>,
    pub console: Option<bool>,
    #[serde(flatten)]
    pub extras: serde_json::Map<String, serde_json::Value>,
    pub init: Option<bool>,
    pub level: u8,
    #[serde(rename = "msg")]
    pub message: String,
}

trait LogFormat {
    fn format(&self) -> String;
}

impl LogFormat for LogAdd {
    fn format(&self) -> String {
        if self.init.is_some() && self.init.unwrap() {
            return self.message.to_string();
        }

        let message = self
            .message
            .trim_end()
            .replace(" - EVENT", "")
            .replace("QuickJS", "Query JS Runtime");

        let re = Regex::new(r"\s*query_server::[:\w]+\s*").unwrap();
        let message = re.replace(&message, "");

        if self.console.is_some() && self.console.unwrap() {
            let dot = String::from('●');
            let dot = if self.level == 50 {
                dot.red()
            } else if self.level == 40 {
                dot.yellow()
            } else {
                dot.green()
            };

            let message = if self.level == 50 {
                message.red()
            } else if self.level == 40 {
                message.yellow()
            } else {
                message.normal()
            };

            let console = "[CONSOLE]";
            let console = if self.level == 50 {
                console.red()
            } else if self.level == 40 {
                console.yellow()
            } else {
                console.normal()
            };

            return format!("{} {} {}", dot, console, message,);
        }

        if self.level == 50 {
            let code = match self.code.as_ref() {
                Some(code) => format!(" {}", code),
                None => String::new(),
            };
            let path = match self.extras.get("path") {
                Some(path) => format!(" {}", path),
                None => String::new(),
            };
            let error = match self.extras.get("error") {
                Some(error) => format!(" {}", error),
                None => String::new(),
            };
            return format!(
                "{} {}{}{}{}",
                String::from('●').red(),
                message.red(),
                code.red(),
                path.red(),
                error.red()
            );
        }

        if self.level == 40 {
            return format!("{} {}", String::from('●').yellow(), message.yellow(),);
        }

        if message == "[ADD_ASSET - END]" {
            let asset_name = self.extras.get("asset_name").unwrap();
            let asset_name = asset_name.as_str().unwrap();
            let mime_type = self.extras.get("mime_type").unwrap();
            let mime_type = mime_type.as_str().unwrap();

            return format!(
                "{} [{}] {}::{}",
                String::from('●').green(),
                "PUSH - ASSET",
                asset_name.trim_start_matches('/'),
                mime_type,
            );
        }

        if message == "[ADD_FUNCTION - END]" {
            let path = self.extras.get("path").unwrap();
            let path = path.as_str().unwrap();
            let method = self.extras.get("method").unwrap();
            let method = method.as_str().unwrap();

            return format!(
                "{} [{}] {}::{}",
                String::from('●').green(),
                "PUSH - FUNCTION",
                path.trim_start_matches('/'),
                method,
            );
        }

        if message == "[ADD_PLUGIN - END]" {
            let plugin_name = self.extras.get("plugin_name").unwrap();

            return format!(
                "{} [{}] {}",
                String::from('●').green(),
                "PUSH - PLUGIN",
                plugin_name.as_str().unwrap().trim_start_matches('/'),
            );
        }

        if message == "[MIGRATION_CONTROLLER - END]" {
            let db_name = self.extras.get("db_name").unwrap();

            return format!(
                "{} [{}] {}",
                String::from('●').green(),
                "PUSH - MIGRATION",
                db_name.as_str().unwrap().trim_start_matches('/'),
            );
        }

        String::new()
    }
}

// CREDIT: https://github.com/LukeMathWalker/bunyan/blob/e3362cb045e207f8333eb7cd4c725a78566da331/src/record.rs
#[derive(serde::Deserialize)]
pub struct LogRecord {
    pub console: Option<bool>,
    #[serde(flatten)]
    pub extras: serde_json::Map<String, serde_json::Value>,
    pub file: Option<String>,
    pub hostname: Option<String>,
    pub init: Option<bool>,
    pub level: u8,
    pub line: Option<u32>,
    #[serde(rename = "msg")]
    pub message: String,
    pub name: Option<String>,
    #[serde(rename = "pid")]
    pub process_identifier: Option<u32>,
    pub req: Option<String>,
    pub target: Option<String>,
    pub time: String,
    #[serde(rename = "v")]
    pub version: Option<u8>,
}

impl LogFormat for LogRecord {
    fn format(&self) -> String {
        if self.init.is_some() && self.init.unwrap() {
            return self.message.to_string();
        }

        let level = match self.level {
            10 => "TRACE",
            20 => "DEBUG",
            30 => "INFO",
            40 => "WARN",
            50 => "ERROR",
            _ => "UNKNOWN",
        };
        let dot: colored::ColoredString = if level == "ERROR" {
            String::from('●').red()
        } else if level == "WARN" {
            String::from('●').yellow()
        } else {
            String::from('●').green()
        };
        let message = self
            .message
            .trim_end()
            .replace("QuickJS", "Query JS Runtime");

        let re = Regex::new(r"\s*query_server::[:\w]+\s*").unwrap();
        let message = re.replace(&message, "");

        let message = if level == "ERROR" {
            message.red()
        } else if level == "WARN" {
            message.yellow()
        } else {
            message.normal()
        };
        let extras = format_extras(&self.extras);
        let extras = if level == "ERROR" {
            extras.red()
        } else if level == "WARN" {
            extras.yellow()
        } else {
            extras.normal()
        };

        let formatted = format!("{dot} {message}{extras}");
        formatted
    }
}

fn format_extras(extra_fields: &serde_json::Map<String, serde_json::Value>) -> String {
    let mut details = Vec::new();
    let mut extras = Vec::new();
    for (key, value) in extra_fields {
        let stringified = if let serde_json::Value::String(s) = value {
            // Preserve strings unless they contain whitespaces/are empty
            // In that case, we want surrounding quotes.
            if s.contains(' ') || s.is_empty() {
                format!("\"{}\"", s)
            } else {
                s.to_owned()
            }
        } else {
            json_to_indented_string(value, "  ")
        };

        if stringified.contains('\n') || stringified.len() > 50 {
            if let serde_json::Value::String(s) = value {
                details.push(indent(&format!("{}: {}", key.bold(), s)));
            } else {
                details.push(indent(&format!("{}: {}", key.bold(), stringified)));
            }
        } else {
            extras.push(format!("{}={}", key.bold(), stringified));
        }
    }
    let formatted_details = if !details.is_empty() {
        details
            .into_iter()
            .collect::<Vec<String>>()
            .join("    --\n")
            .to_string()
    } else {
        "".into()
    };
    let formatted_extras = if !extras.is_empty() {
        format!(
            " ({})",
            extras.into_iter().collect::<Vec<String>>().join(",")
        )
    } else {
        "".into()
    };

    if formatted_details.is_empty() {
        return formatted_extras.to_string();
    }

    format!("{}\n{}", formatted_extras, formatted_details)
}

fn json_to_indented_string(value: &serde_json::Value, indent: &str) -> String {
    let mut writer = Vec::with_capacity(128);
    let formatter = PrettyFormatter::with_indent(indent.as_bytes());
    let mut serializer = Serializer::with_formatter(&mut writer, formatter);
    value.serialize(&mut serializer).unwrap();
    unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(writer)
    }
}

fn indent(s: &str) -> String {
    format!("    {}", s.lines().collect::<Vec<&str>>().join("    "))
}
