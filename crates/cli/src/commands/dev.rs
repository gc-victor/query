use std::{
    env, fs,
    io::BufRead,
    path::Path,
    process::{exit, Command, Stdio},
    thread,
    time::Duration,
};

use anyhow::Result;
use colored::Colorize;
use watchexec::Watchexec;

use crate::{
    run_server::run_query_server,
    utils::{
        block_until_server_is_ready, check_port_usage, detect_package_manager, has_module,
        stop_query_server, which,
    },
};

use super::commands::DevArgs;

pub async fn command_dev(command: &DevArgs) -> Result<()> {
    check_config_file_exist();

    if command.no_port_check {
        match check_port_usage() {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
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
        // Push the tasks before starting the watcher
        push_tasks();

        let mut last_event_time = tokio::time::Instant::now();

        let wx = Watchexec::new_async(move |mut action| {
            Box::new(async move {
                for event in action.events.iter() {
                    let tags = &event.tags;
                    let has_close_write = match tags.get(1) {
                        Some(tag) => format!("{:?}", tag) == "FileEventKind(Access(Close(Write)))",
                        None => false,
                    };
                    let delay = Duration::from_millis(250);

                    if has_close_write && tokio::time::Instant::now() - last_event_time > delay {
                        last_event_time = tokio::time::Instant::now();

                        push_tasks();
                    }

                    tokio::time::sleep(delay).await;
                }

                if action.signals().next().is_some() {
                    action.quit();
                }

                action
            })
        })
        .unwrap();

        wx.main();

        let paths = vec!["src", "public"];

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

fn push_tasks() {
    query_command(vec!["task", "dev", "-y"]);
    query_command(vec!["asset", "dist"]);
    query_command(vec!["asset", "public"]);
    // NOTE: the function should be executed after the assets
    query_command(vec!["function"]);
}

fn query_command(args: Vec<&str>) {
    let binary = "query";
    let module = "@qery/query";

    let pm = detect_package_manager();

    let package_global = match which(binary) {
        Some(package_global) => package_global,
        None => String::new(),
    };
    let hash_package_global = !package_global.is_empty();
    let hash_package_local_module = has_module(binary);
    let hash_package = hash_package_local_module || hash_package_global;

    if !hash_package {
        eprintln!("The {} binary isn't installed.", binary);
        eprintln!(
            "Please, run `{} install --save-dev {}` first.",
            pm.npm, module
        );
        exit(1);
    }

    let mut child: std::process::Child = if hash_package_local_module {
        let npx = pm.npx.to_string();
        let npx = npx.split(' ').collect::<Vec<&str>>();
        let npx = npx[0];

        let current_dir = env::current_dir().unwrap();
        let package = current_dir.join("node_modules").join(".bin").join(binary);
        let package = package.to_str().unwrap().to_string();

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
