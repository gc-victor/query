use std::{
    fmt::Write,
    path::Path,
    process::{exit, Command},
};

use anyhow::Result;
use colored::Colorize;
use query_runtime::Runtime;
use rquickjs::{async_with, Module};
use walkdir::WalkDir;

use crate::{
    config::CONFIG,
    utils::{detect_package_manager, has_node_modules_binary, which},
};

use super::commands::TestArgs;

pub async fn command_test(command: &TestArgs) -> Result<()> {
    let test_patterns = [
        "**/*.test.{js,jsx,ts,tsx}",
        "**/*_test.{js,jsx,ts,tsx}",
        "**/*.spec.{js,jsx,ts,tsx}",
        "**/*_spec.{js,jsx,ts,tsx}",
    ];

    let mut test_files = Vec::new();

    // If specific filters are provided
    if !command.filters.is_empty() {
        for filter in &command.filters {
            // Handle absolute/relative paths differently from filter patterns
            if filter.starts_with("./") || filter.starts_with('/') {
                let path = Path::new(filter);
                if path.exists() {
                    test_files.push(path.to_path_buf());
                } else {
                    eprintln!("{} File {} does not exist", String::from('●').red(), filter);
                    exit(1);
                }
            } else {
                // Walk through directories and find matching test files
                for entry in WalkDir::new(".")
                    .follow_links(true)
                    .into_iter()
                    .filter_map(|e| e.ok())
                {
                    let path = entry.path();
                    if path.is_file() {
                        let file_name = path.to_string_lossy().to_string();

                        // Check if file matches any test pattern
                        let is_test_file = test_patterns.iter().any(|pattern| {
                            let pattern = pattern.replace("**/*.", ".");
                            let pattern = pattern
                                .replace("{js,jsx,ts,tsx}", "js")
                                .replace("{js,jsx,ts,tsx}", "jsx")
                                .replace("{js,jsx,ts,tsx}", "ts")
                                .replace("{js,jsx,ts,tsx}", "tsx");
                            file_name.ends_with(&pattern)
                        });

                        // Check if file matches the filter
                        let matches_filter = if filter.contains('*') {
                            // Simple wildcard matching
                            let filter_parts: Vec<&str> = filter.split('*').collect();
                            let mut matched = true;
                            let name = path.to_string_lossy();
                            for part in filter_parts {
                                if !name.contains(part) {
                                    matched = false;
                                    break;
                                }
                            }
                            matched
                        } else {
                            file_name.contains(filter)
                        };

                        if is_test_file && matches_filter {
                            test_files.push(path.to_path_buf());
                        }
                    }
                }
            }
        }
    } else {
        // No filters - get all test files
        for entry in WalkDir::new(".")
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                let file_name = path.to_string_lossy().to_string();

                // Check if file matches any test pattern
                let is_test_file = test_patterns.iter().any(|pattern| {
                    let pattern = pattern.replace("**/*.", ".");
                    file_name.ends_with(&pattern.replace("{js,jsx,ts,tsx}", "js"))
                });

                if is_test_file {
                    test_files.push(path.to_path_buf());
                }
            }
        }
    }

    // Ensure esbuild is available
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

    if test_files.is_empty() {
        eprintln!("{} No test files found", String::from('●').yellow());
        return Ok(());
    }

    // Generate import statements for all test files
    let mut code = String::new();

    for path in &test_files {
        let mut args = vec![
            path.to_string_lossy().to_string(),
            "--bundle".to_string(),
            "--format=esm".to_string(),
            "--platform=browser".to_string(),
        ];

        for (key, value) in &CONFIG.esbuild {
            let flag = if value.is_empty() {
                format!("--{}", key)
            } else {
                format!("--{}={}", key, value)
            };
            args.push(flag);
        }

        let mut cmd = if has_esbuild_local_binary {
            let package = Path::new("node_modules").join(".bin").join("esbuild");
            Command::new(package)
        } else {
            Command::new(&esbuild_global)
        };

        let output = cmd
            .args(&args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .output()
            .unwrap();

        if !output.status.success() {
            eprintln!(
                "{} Build failed: {}",
                String::from('●').red(),
                String::from_utf8_lossy(&output.stderr)
            );
            exit(1);
        }
        let path_str = path.to_string_lossy();
        code = format!(
            r#"
            {}

            print('\n{}:');
            (() => {{
                {}
                
                __printTestSummary('{}');
            }})();
        "#,
            code,
            path_str,
            String::from_utf8_lossy(&output.stdout),
            path_str
        );
    }

    let code = format!(
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

        {code}

        __printGlobalTestSummary();
        "#
    );

    // Run the test in Query runtime
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

    async_with!(ctx => |ctx| {
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
    })
    .await;

    Ok(())
}
