use std::{
    fs,
    io::BufRead,
    process::{exit, Command, Stdio},
    thread,
};

use anyhow::Result;
use colored::Colorize;
use inquire::Confirm;
use toml::{map::Map, Table, Value};

use crate::config::CLI;

use super::commands::TaskArgs;

#[derive(Debug, serde::Deserialize)]
pub struct TaskTable {
    #[serde(flatten)]
    pub table: Option<Table>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Task {
    pub task: Option<TaskTable>,
}

pub fn command_task(command: &TaskArgs) -> Result<()> {
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
    let config: Task = match toml::from_str(&contents) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{} {}", String::from('●').red(), e);
            exit(1);
        }
    };

    let table = config.task.and_then(|task| task.table);

    if command.list && command.task.is_empty() && table.is_some() {
        if let Some(table) = table {
            for (task, command) in &table {
                if !command.is_table() {
                    let command = command.as_str().unwrap().trim();
                    eprintln!(r#"{} [{task}]: `{command}`"#, String::from('●').green(),);
                }
            }

            for (task, command) in table {
                if command.is_table() {
                    eprintln!(r#"{} {}:"#, String::from('●').green(), task.green());
                    let command = command.as_table().unwrap();
                    for (task, command) in command {
                        let command = command.as_str().unwrap().trim();
                        eprintln!(r#"{} [{task}]: `{command}`"#, String::from('●').green(),);
                    }
                }
            }
        }

        return Ok(());
    }

    if let Some(table) = table {
        let command_task = &command.task;
        let tasks_group = command_task.first().unwrap();

        if !table.get(tasks_group).unwrap().is_table() {
            execute_command(&table, &String::new(), tasks_group)?;
        };

        let commands = table
            .get(tasks_group)
            .map(|c| c.as_table().unwrap())
            .unwrap_or_else(|| {
                eprintln!(
                    "{}",
                    format!(
                        "{} Task group `{}` not found",
                        String::from('●').red(),
                        tasks_group
                    )
                    .red()
                );
                exit(1);
            });

        if command.list {
            for (task, command) in commands {
                let command = command.as_str().unwrap();
                eprintln!(r#"{} [{task}]: `{command}`"#, String::from('●').green());
            }

            return Ok(());
        }

        if command_task.len() == 1 {
            let yes = command.yes;
            let task = command_task.first().unwrap();

            if !yes
                && !Confirm::new(&format!("Do you want to execute all the `{task}` tasks?"))
                    .with_default(true)
                    .prompt()?
            {
                return Ok(());
            }

            for task in commands.keys() {
                execute_command(&table, tasks_group, task)?;
            }
        }

        if command_task.len() > 1 {
            let task = command_task.get(1).unwrap();

            execute_command(&table, tasks_group, task)?;
        }
    }

    Ok(())
}

fn execute_command(
    extra_command: &Map<String, Value>,
    tasks_group: &String,
    task: &String,
) -> Result<()> {
    let command = get_command(extra_command, tasks_group, task)?;

    let mut cmd = if cfg!(target_os = "windows") {
        let mut cmd = Command::new("cmd");
        cmd.arg("/C").arg(&command);
        cmd
    } else {
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg(&command);
        cmd
    };

    let mut child = match cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn() {
        Ok(child) => child,
        Err(e) => {
            eprintln!("{} {}", String::from('●').red(), e);
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

                    if message.is_empty() {
                        continue;
                    }

                    eprintln!("{} {}", String::from('●').red(), message);
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

    Ok(())
}

fn get_command(
    extra_command: &Map<String, Value>,
    tasks_group: &String,
    task: &String,
) -> Result<String, anyhow::Error> {
    let command = if tasks_group.is_empty() {
        extra_command.get(task).and_then(|c| c.as_str())
    } else {
        extra_command
            .get(tasks_group)
            .and_then(|task_command| task_command.get(task))
            .and_then(|c| c.as_str())
    };

    match command {
        Some(command) => Ok(command.to_string()),
        None => {
            let error_message = if tasks_group.is_empty() {
                format!("{} Task `{}` not found", String::from('●').red(), task)
            } else {
                format!(
                    "{} Task `{}` not found in the group `{}`",
                    String::from('●').red(),
                    task,
                    tasks_group
                )
            };

            eprintln!("{} {}", String::from('●').red(), error_message);
            exit(1);
        }
    }
}
