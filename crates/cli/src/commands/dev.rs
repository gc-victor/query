use std::{
    env, fs,
    io::BufRead,
    path::Path,
    process::{exit, Command, Stdio},
    thread,
    time::Duration,
    vec,
};

use anyhow::Result;
use colored::Colorize;
use lazy_static::lazy_static;
use toml::Table;
use watchexec::Watchexec;

use crate::{
    config::CLI,
    run_server::run_query_server,
    utils::{
        block_until_server_is_ready, check_port_usage, detect_package_manager,
        has_node_modules_binary, stop_query_server, which,
    },
};

use super::commands::DevArgs;

lazy_static! {
    static ref DEV_COMMANDS: Vec<String> = {
        let contents = match fs::read_to_string(CLI::default().config_file_path) {
            Ok(contents) => contents,
            Err(_) => {
                eprintln!(
                    "{}",
                    format!("{} No config file found", String::from('●').red()).red()
                );
                exit(1);
            }
        };
        let config: Table = match toml::from_str(&contents) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("{} {}", String::from('●').red(), e);
                exit(1);
            }
        };

        let mut commands = vec![];

        let task = config
            .get("task")
            .and_then(|task| task.as_table())
            .unwrap_or_else(|| {
                eprintln!(
                    "{} {}",
                    String::from('●').red(),
                    "No task found in the config file".to_string().red()
                );
                exit(1);
            });

        if let Some(dev) = task.get("dev").and_then(|dev| dev.as_table()) {
            for (_, command) in dev {
                commands.push(command.as_str().unwrap().trim().to_string());
            }
        };

        commands
    };
}

pub async fn command_dev(command: &DevArgs) -> Result<()> {
    check_config_file_exist();

    match check_port_usage() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }

    if command.clean {
        clean().unwrap_or(()); // Ignore if there is an error
    }

    let verbose = command.verbose;
    let server = tokio::spawn(async move {
        run_query_server(verbose, false);
    });

    let watcher = tokio::spawn(async move {
        block_until_server_is_ready();

        eprintln!("{}", "Running initial tasks...".to_string().yellow());
        run_tasks();
        eprintln!("{}", "Watching for changes...".to_string().normal());

        let wx = Watchexec::new_async(move |mut action| {
            Box::new(async move {
                for event in action.events.iter() {
                    let tags = &event.tags;
                    let has_close_write = match tags.get(1) {
                        Some(tag) => format!("{:?}", tag) == "FileEventKind(Access(Close(Write)))",
                        None => false,
                    };

                    if has_close_write {
                        run_tasks();
                    }
                }

                if action.signals().next().is_some() {
                    action.quit();
                }

                action
            })
        })
        .unwrap();

        wx.main();

        let paths = vec!["src"];

        wx.config.pathset(paths);
    });

    server.await?;
    watcher.await?;

    Ok(())
}

fn clean() -> Result<()> {
    let dist_dir = Path::new("dist");
    if dist_dir.exists() {
        fs::remove_dir_all(dist_dir)?;
    }
    fs::create_dir(dist_dir)?;

    let cache_file = Path::new(".query/.cache");
    if cache_file.exists() {
        fs::remove_file(cache_file)?;
    }

    let query_server_dbs_path = env::var("QUERY_SERVER_DBS_PATH")
        .expect("QUERY_SERVER_DBS_PATH is not set in the .env file");

    let databases = vec![
        "query_cache_function.sql",
        "query_function.sql",
        "query_asset.sql",
    ];

    for database in &databases {
        let database_path = Path::new(&query_server_dbs_path).join(database);
        fs::remove_file(database_path)?;
    }

    while databases
        .iter()
        .any(|database| Path::new(&query_server_dbs_path).join(database).exists())
    {
        std::thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}

fn check_config_file_exist() {
    let query_toml_path = ".query/Query.toml";

    if !Path::new(query_toml_path).exists() {
        eprintln!(
            "The {} file does not exist. Please, run `query create` first.",
            query_toml_path
        );
        exit(1);
    }
}

fn run_tasks() {
    execute_dev_commands();
    query_command(vec!["asset", "dist"]);
    // NOTE: the function should be executed after the assets
    query_command(vec!["function"]);
}

fn execute_dev_commands() {
    let commands = DEV_COMMANDS.clone();

    for command in commands {
        let mut cmd = if cfg!(target_os = "windows") {
            let mut cmd = Command::new("cmd");
            cmd.arg("/C").arg(command);
            cmd
        } else {
            let mut cmd = Command::new("sh");
            cmd.arg("-c").arg(command);
            cmd
        };

        let mut child = match cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn() {
            Ok(child) => child,
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        };

        let stdout_thread = thread::spawn(move || {
            let stdout = child.stdout.take().expect("Failed to open stdout");
            let mut reader = std::io::BufReader::new(stdout);
            let mut line = String::new();
            loop {
                match reader.read_line(&mut line) {
                    Ok(0) => break,
                    Ok(_) => {
                        let message = line.trim();
                        let message = message.trim_start_matches('"');
                        let message = message
                            .replace('●', "")
                            .replace("\\n\\n", "\n")
                            .replace("\\n", "\n");
                        let message = message.trim_end_matches('"');
                        let message = message.trim();

                        if message.is_empty() {
                            continue;
                        }

                        println!("{} {}", String::from('●').green(), message);
                    }
                    Err(e) => {
                        eprintln!("{}", format!("{} {}", String::from('●'), e).red());
                    }
                }
                line.clear();
            }
        });

        let stderr_thread = thread::spawn(move || {
            let stderr = child.stderr.take().expect("Failed to open stderr");
            let mut reader = std::io::BufReader::new(stderr);
            let mut line = String::new();
            loop {
                match reader.read_line(&mut line) {
                    Ok(0) => break,
                    Ok(_) => {
                        let message = line.trim();
                        let message = message.trim_start_matches('"');
                        let message = message
                            .replace('●', "")
                            .replace("\\n\\n", "\n")
                            .replace("\\n", "\n");
                        let message = message.trim_end_matches('"');
                        let message = message.trim();

                        if message.is_empty()
                            || message.starts_with("Rebuilding...")
                            || message.starts_with("Done in")
                        {
                            continue;
                        }

                        eprintln!("{}", message.red());
                    }
                    Err(e) => {
                        eprintln!("{}", format!("{} {}", String::from('●'), e).red());
                    }
                }
                line.clear();
            }
        });

        let _ = stdout_thread.join();
        let _ = stderr_thread.join();
    }
}

fn query_command(args: Vec<&str>) {
    let binary = "query";
    let module = "@qery/query";

    let pm = detect_package_manager();

    let package_global = which(binary).unwrap_or_default();
    let hash_package_global = !package_global.is_empty();
    let hash_package_local_binary = has_node_modules_binary(binary);
    let hash_package = hash_package_local_binary || hash_package_global;

    if !hash_package {
        eprintln!("The {} binary isn't installed.", binary);
        eprintln!(
            "Please, run `{} install --save-dev {}` first.",
            pm.npm, module
        );
        exit(1);
    }

    let mut child: std::process::Child = if hash_package_local_binary {
        let npx = pm.npx.to_string();
        let npx = npx.split(' ').collect::<Vec<&str>>();
        let npx = npx[0];

        let current_dir = env::current_dir().unwrap();
        let package = current_dir.join("node_modules").join(".bin").join(binary);

        match Command::new(package)
            .args(args)
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(child) => child,
            Err(e) => {
                eprintln!("{}", e);
                eprintln!("Failed to execute command `{} {}`", npx, binary);
                stop_query_server();
                exit(1);
            }
        }
    } else {
        match Command::new(package_global)
            .args(args)
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(child) => child,
            Err(e) => {
                eprintln!("{}", e);
                eprintln!("Failed to execute command `{}`", binary);
                stop_query_server();
                exit(1);
            }
        }
    };

    let stderr_thread = thread::spawn(move || {
        let stderr = child.stderr.take().expect("Failed to open stderr");
        let mut reader = std::io::BufReader::new(stderr);
        let mut line = String::new();
        loop {
            match reader.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => {
                    let message = line.trim();
                    let message = message.trim_start_matches('"');
                    let message = message.replace("\\n\\n", "\n").replace("\\n", "\n");
                    let message = message.trim_end_matches('"');
                    let message = message.trim();

                    if message.is_empty() {
                        break;
                    }

                    eprintln!("{}", message.to_string().red());
                }
                Err(e) => {
                    eprintln!("{}", format!("{}", e).red());
                }
            }

            line.clear();
        }
    });

    let _ = stderr_thread.join();
}
