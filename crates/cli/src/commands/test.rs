use std::{
    fmt::Write,
    path::{Path, PathBuf},
    process::{exit, Command},
    time::Instant,
};

use anyhow::Result;
use colored::Colorize;
use query_runtime::Runtime;
use rquickjs::{async_with, Module, Object};
use walkdir::WalkDir;
use watchexec::Watchexec;

use crate::{
    config::CONFIG,
    utils::{detect_package_manager, has_node_modules_binary, which},
};

use super::commands::TestArgs;

pub async fn command_test(command: &TestArgs) -> Result<()> {
    let test_name_pattern = command.test_name_pattern.as_deref().unwrap_or_default();
    let test_files = get_test_files(&command.filters)?;
    let filters: Vec<&Path> = command.filters.iter().map(Path::new).collect();

    if command.watch {
        eprintln!(
            "{} Watching {} test files...",
            String::from('●').cyan(),
            test_files.len()
        );
        watch_and_run_tests(filters, test_files, test_name_pattern.to_owned()).await?;
    } else {
        let total_failed = run_tests(&test_files, test_name_pattern).await?;

        if total_failed > 0 {
            exit(1);
        }
    }

    Ok(())
}

fn get_test_files(filters: &[String]) -> Result<Vec<PathBuf>> {
    if filters.is_empty() {
        eprintln!("{} No test files specified", String::from('●').yellow());
        exit(1);
    }

    let files: Vec<PathBuf> = filters
        .iter()
        .flat_map(|filter| {
            WalkDir::new(filter)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_file() && is_test_file(e.path()))
                .map(|e| e.path().to_path_buf())
        })
        .collect();

    if files.is_empty() {
        eprintln!(
            "{} No matching test files found in specified paths",
            String::from('●').yellow()
        );
        exit(1);
    }

    Ok(files)
}

fn is_test_file(path: &Path) -> bool {
    let file_name = path.to_string_lossy();
    [".test.ext", ".spec.ext"].iter().any(|pattern| {
        ["js", "jsx", "ts", "tsx"]
            .iter()
            .any(|ext| file_name.ends_with(&pattern.replace("ext", ext)))
    })
}

async fn watch_and_run_tests(
    filters: Vec<&Path>,
    test_files: Vec<PathBuf>,
    test_name_pattern: String,
) -> Result<()> {
    // Run tests initially
    run_tests(&test_files, &test_name_pattern).await?;

    let test_files_clone = test_files.clone();
    let pattern_clone = test_name_pattern.clone();

    let wx = Watchexec::new_async(move |mut action| {
        let files = test_files_clone.clone();
        let pattern = pattern_clone.clone();

        // Use std::thread::spawn to run tests in a separate thread
        Box::new(async move {
            if action.signals().next().is_some() {
                action.quit();
            } else {
                eprintln!(
                    "\n{} Changes detected, running tests...",
                    String::from('●').cyan()
                );

                let (tx, rx) = std::sync::mpsc::channel();
                std::thread::spawn(move || {
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    let result = rt.block_on(run_tests(&files, &pattern));
                    tx.send(result).unwrap();
                });

                if let Ok(Err(e)) = rx.recv() {
                    eprintln!("{} Error running tests: {}", String::from('●').red(), e);
                }
            }
            action
        })
    })?;

    wx.config.pathset(filters);

    // Start watching
    let _ = wx.main().await?;

    Ok(())
}

async fn run_tests(test_files: &[PathBuf], test_name_pattern: &str) -> Result<u32> {
    let start_time = Instant::now();

    check_esbuild_availability()?;

    let mut code = {
        format!(
            r#"
                import 'polyfill/blob';
                import 'polyfill/console';
                import 'polyfill/fetch';
                import 'polyfill/file';
                import 'polyfill/form-data';
                import 'polyfill/request';
                import 'polyfill/response';
                import 'polyfill/web-streams';

                import 'js/database';
                import 'js/handle-response';
                import 'js/jsx-helpers';

                globalThis.___testNamePattern = '{test_name_pattern}';
            "#
        )
    };
    let failed_builds = bundle_test_files(test_files, &mut code)?;
    if failed_builds > 0 {
        return Ok(1);
    }

    eprintln!(
        "{} Running {} tests...",
        String::from('●').cyan(),
        test_files.len()
    );

    code = format!(
        r#"
        {code}

        ___printGlobalTestSummary();
        "#
    );

    // Run the tests
    let runtime = match Runtime::new().await {
        Ok(runtime) => runtime,
        Err(e) => {
            eprintln!("{} Runtime Error: {}", String::from('●').red(), e);
            exit(1);
        }
    };

    let ctx = runtime.ctx;
    let test_bundle_name = "test-bundle.js";
    let imports = test_files.iter().fold(String::new(), |mut output, path| {
        let _ = writeln!(output, "{}", path.to_string_lossy());
        output
    });

    let total_failed: u32 = async_with!(ctx => |ctx| {
        let module = match Module::declare(ctx.clone(), test_bundle_name.to_string(), &*code) {
            Ok(m) => m,
            Err(e) => {
                eprintln!(
                    "{} Test failed: {} - {}",
                    String::from('✗').red(),
                    imports,
                    e
                );
                exit(1);
            }
        };

        if let Err(e) = module.eval() {
            eprintln!(
                "{} Test failed: {} - {}",
                String::from('✗').red(),
                imports,
                e
            );
            exit(1);
        }

        let global_this: Object = match ctx.clone().globals().get("globalThis") {
            Ok(o) => o,
            Err(e) => {
                tracing::error!("Error: {}", e);
                exit(1);
            },
        };
        let total_failed: u32 = match global_this.get("___totalFailed") {
            Ok(f) => f,
            Err(e) => {
                tracing::error!("Error: {}", e);
                exit(1);
            },
        };

        let elapsed = start_time.elapsed();
        eprintln!("\nTotal execution time: {:.2?}", elapsed);

        total_failed
    })
    .await;

    Ok(total_failed)
}

fn check_esbuild_availability() -> Result<()> {
    let pm = detect_package_manager();
    let esbuild_global = which("esbuild").unwrap_or_default();
    let has_esbuild_global = !esbuild_global.is_empty();
    let has_esbuild_local_binary = has_node_modules_binary("esbuild");
    let has_esbuild = has_esbuild_local_binary || has_esbuild_global;

    if !has_esbuild {
        eprintln!(
            "{} The esbuild binary does not exist. Please run `{} install esbuild` first",
            String::from('●').red(),
            pm.npm
        );
        exit(1);
    }

    Ok(())
}

fn bundle_test_files(test_files: &[PathBuf], code: &mut String) -> Result<usize> {
    let mut failed_builds = Vec::new();

    for path in test_files {
        let mut args = vec![
            path.to_string_lossy().to_string(),
            "--bundle".to_string(),
            "--format=esm".to_string(),
            "--platform=browser".to_string(),
            "--external:polyfill/blob".to_string(),
            "--external:polyfill/console".to_string(),
            "--external:polyfill/fetch".to_string(),
            "--external:polyfill/file".to_string(),
            "--external:polyfill/form-data".to_string(),
            "--external:polyfill/request".to_string(),
            "--external:polyfill/response".to_string(),
            "--external:polyfill/web-streams".to_string(),
            "--external:js/database".to_string(),
            "--external:js/handle-response".to_string(),
            "--external:js/jsx-helpers".to_string(),
        ];

        for (key, value) in &CONFIG.esbuild {
            let flag = if value.is_empty() {
                format!("--{}", key)
            } else {
                format!("--{}={}", key, value)
            };
            args.push(flag);
        }

        let mut cmd = if has_node_modules_binary("esbuild") {
            let package = Path::new("node_modules").join(".bin").join("esbuild");
            Command::new(package)
        } else {
            Command::new(which("esbuild").unwrap())
        };

        let output = match cmd
            .args(&args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .output()
        {
            Ok(o) => o,
            Err(e) => {
                failed_builds.push((
                    path.to_string_lossy().into_owned(),
                    format!("Failed to bundle test file: {}", e),
                ));
                continue;
            }
        };

        if !output.status.success() {
            failed_builds.push((
                path.to_string_lossy().into_owned(),
                String::from_utf8_lossy(&output.stderr).into_owned(),
            ));
            continue;
        }

        let new_code = String::from_utf8_lossy(&output.stdout);
        let path_str = path.to_string_lossy();
        *code = format!(
            r#"
            {code}

            print('\n{path_str}:');
            (() => {{
                {new_code}

                ___printTestSummary('{path_str}');
            }})();
        "#
        );
    }

    for (path, error) in &failed_builds {
        eprintln!("\n{}:", path);
        eprintln!("{}", error);
    }

    if !failed_builds.is_empty() {
        return Ok(1);
    }

    Ok(0)
}
