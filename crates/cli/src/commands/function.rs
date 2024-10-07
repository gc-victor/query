#[allow(unused_imports)]
use std::path::Path;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
#[allow(unused_imports)]
use std::{
    env, fs,
    process::{exit, Command},
};

#[allow(unused_imports)]
use anyhow::anyhow;

use anyhow::Result;
use colored::Colorize;
use regex::Regex;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
use walkdir::WalkDir;

#[allow(unused_imports)]
use crate::{
    cache::{Cache, CacheItem},
    config::CONFIG,
    utils::{detect_package_manager, has_node_modules_binary, http_client, which},
};

use super::commands::FunctionArgs;

pub async fn command_function(command: &FunctionArgs) -> Result<()> {
    let is_delete = command.delete;
    let path = command
        .path
        .clone()
        .unwrap_or(CONFIG.structure.functions_folder.to_owned());
    let metadata = fs::metadata(&path)?;
    let is_file = metadata.is_file();

    if is_file {
        let FunctionBuilder {
            active,
            function,
            method,
            path,
        } = function_builder(&path)?;

        if is_delete {
            let body = json!({
                "method": method,
                "path": path,
            })
            .to_string();

            match http_client("function-builder", Some(&body), Method::DELETE).await {
                Ok(_) => {
                    eprintln!(
                        "{} Successfully function deleted!!!!",
                        String::from('●').green()
                    );
                }
                Err(e) => eprintln!("{} {}", String::from('●').red(), e),
            };

            return Ok(());
        };

        let body = json!({
            "active": active,
            "function": function,
            "method": method,
            "path": path,
        })
        .to_string();

        match http_client("function-builder", Some(&body), Method::POST).await {
            Ok(_) => {
                eprintln!(
                    "{} Successfully function updated!!!!",
                    String::from('●').green()
                );
            }
            Err(e) => eprintln!("{} {}", String::from('●').red(), e),
        };
    } else {
        let functions_folder = env::current_dir()?.join(path).to_str().unwrap().to_string();

        for entry in WalkDir::new(&functions_folder) {
            let entry = entry?;

            if entry.file_type().is_file() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.starts_with("connect.index")
                        || file_name.starts_with("connect.[slug]")
                        || file_name.starts_with("delete.index")
                        || file_name.starts_with("delete.[slug]")
                        || file_name.starts_with("get.index")
                        || file_name.starts_with("get.[slug]")
                        || file_name.starts_with("head.index")
                        || file_name.starts_with("head.[slug]")
                        || file_name.starts_with("options.index")
                        || file_name.starts_with("options.[slug]")
                        || file_name.starts_with("patch.index")
                        || file_name.starts_with("patch.[slug]")
                        || file_name.starts_with("post.index")
                        || file_name.starts_with("post.[slug]")
                        || file_name.starts_with("put.index")
                        || file_name.starts_with("put.[slug]")
                        || file_name.starts_with("trace.index")
                        || file_name.starts_with("trace.[slug]")
                    {
                        let file_path = entry.path().display().to_string();

                        let FunctionBuilder {
                            active,
                            function,
                            method,
                            path,
                        } = function_builder(&file_path)?;

                        let mut hasher = DefaultHasher::new();
                        Hash::hash_slice(&function, &mut hasher);
                        let value = hasher.finish().to_string();

                        let cache_key = file_path.replace(&(functions_folder.clone() + "/"), "");
                        let mut cache = Cache::new();
                        let is_cached = match cache.get(&cache_key) {
                            Some(cache_item) => cache_item.value == value,
                            None => false,
                        };

                        if !is_cached {
                            let body_path = path.replace(
                                &env::current_dir()?
                                    .join(&CONFIG.structure.functions_folder)
                                    .to_str()
                                    .unwrap()
                                    .to_string(),
                                "",
                            );
                            let body_path = if body_path.is_empty() {
                                "/".to_string()
                            } else {
                                body_path
                            };
                            let body = json!({
                                "active": active,
                                "function": function,
                                "method": method,
                                "path": body_path,
                            })
                            .to_string();

                            match http_client("function-builder", Some(&body), Method::POST).await {
                                Ok(_) => {
                                    println!(
                                        "{} Function updated: {cache_key}",
                                        String::from('●').green()
                                    );
                                    cache.set(CacheItem {
                                        key: cache_key,
                                        value,
                                    })?;
                                }
                                Err(e) => eprintln!("{} {}", String::from('●').red(), e),
                            };
                        } else {
                            println!("{} Function cached: {cache_key}", String::from('●').green());
                        }
                    }
                } else {
                    continue;
                }
            }
        }
    };

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FunctionBuilder {
    active: bool,
    function: Vec<u8>,
    method: String,
    path: String,
}

fn function_builder(file_path: &str) -> Result<FunctionBuilder> {
    match std::fs::read(file_path) {
        Ok(_) => (),
        Err(_) => {
            panic!(r#"The file "{}" doesn't exists"#, file_path);
        }
    };

    let reg =
        Regex::new(r#"^(.*)\/(get|head|post|put|delete|connect|options|trace|patch)\.(index|\[slug\])\.(js|jsx|ts|tsx)$"#)
            .unwrap();
    let name: String = reg.replace(file_path, "$3").to_string();
    if name != "index" && name != "[slug]" {
        panic!(
            r#"The file "{}" doesn't have a valid name. It should be "index" or "[slug]". Ex. "get.index.(js|jsx|ts|tsx)" or "post.[slug].(js|jsx|ts|tsx)""#,
            file_path
        );
    }

    let functions_folder = &CONFIG.structure.functions_folder;
    let path: String = reg.replace(file_path, "$1$3").to_string();
    let path = path
        .split(&format!("/{functions_folder}"))
        .last()
        .unwrap()
        .to_string();
    let path = path
        .replace(functions_folder, "")
        .replace("index", "")
        .replacen("/[slug]", "/:slug", 10)
        .replacen("[slug]", "/:slug", 10);
    let path = if path.is_empty() {
        "/".to_string()
    } else {
        path
    };

    let method = reg.replace(file_path, "$2").to_uppercase();
    let re = Regex::new(r"^(GET|HEAD|POST|PUT|DELETE|CONNECT|OPTIONS|TRACE|PATCH)$").unwrap();
    if !re.is_match(&method) {
        panic!(
            r#"The file "{}" doesn't have a method as a prefix. Ex. "get.*""#,
            file_path
        );
    }

    let function = esbuild(file_path)?;

    Ok(FunctionBuilder {
        active: true,
        function,
        method,
        path,
    })
}

#[cfg(test)]
pub fn esbuild(_: &str) -> Result<Vec<u8>> {
    Ok(vec![60, 112, 62, 72, 111, 108, 97, 33, 60, 47, 112, 62])
}

#[cfg(not(test))]
pub fn esbuild(function_path: &str) -> Result<Vec<u8>> {
    let out_dir = &format!("{}/dist/functions", CONFIG.current_exe);
    let out_dir_flag = &format!("--outdir={}", out_dir);

    let path = function_path;

    let mut args = vec![
        path.to_string(),
        "--bundle".to_string(),
        "--format=esm".to_string(),
        "--legal-comments=none".to_string(),
        "--minify=true".to_string(),
        "--target=esnext".to_string(),
        "--platform=browser".to_string(),
        out_dir_flag.to_string(),
    ];

    let config_esbuild = &CONFIG.esbuild;

    for (key, value) in config_esbuild {
        let flag = if value.is_empty() {
            format!("--{}", key)
        } else {
            format!("--{}={}", key, value)
        };

        args.push(flag);
    }

    let esbuild_global = which("esbuild").unwrap_or_default();
    let hash_esbuild_global = !esbuild_global.is_empty();
    let hash_esbuild_local_binary = has_node_modules_binary("esbuild");
    let hash_esbuild = hash_esbuild_local_binary || hash_esbuild_global;

    let pm = detect_package_manager();
    if !hash_esbuild {
        eprintln!(
            "{} The esbuild binary does not exist. Please, run `{} install esbuild` first",
            String::from('●').red(),
            pm.npm
        );
        exit(1);
    }

    if hash_esbuild_local_binary {
        let package = Path::new("node_modules").join(".bin").join("esbuild");
        let package = package.to_str().unwrap().to_string();

        match Command::new(package).args(&args).output() {
            Ok(output) => {
                let output = std::str::from_utf8(&output.stderr)?;

                if output.contains("[ERROR]") {
                    eprintln!("{} {:?}", String::from('●').red(), output);
                    exit(1);
                }
            }
            Err(e) => {
                eprintln!("{:?}", e);
                exit(1);
            }
        };
    } else {
        match Command::new(esbuild_global).args(&args).output() {
            Ok(output) => {
                let output = std::str::from_utf8(&output.stderr)?;

                if output.contains("[ERROR]") {
                    eprintln!("{} {:?}", String::from('●').red(), output);
                    exit(1);
                }
            }
            Err(e) => {
                eprintln!("{} {}", String::from('●').red(), e);
                exit(1);
            }
        };
    };

    let bundle_path = format!("{}/{}", out_dir, function_path.split('/').last().unwrap());
    let re = Regex::new(r"(\.tsx|\.ts)$").unwrap();
    let bundle_path = re.replace(&bundle_path, ".js").to_string();

    let function = fs::read_to_string(&bundle_path)?;

    if function.is_empty() {
        eprintln!(
            "{} The function {function_path} is empty",
            String::from('●').red()
        );
        exit(1)
    }

    let function = function.replace("var handleRequest", "globalThis.___handleRequest");
    let re_export = Regex::new(r"export\{([\S]+) as handleRequest\};\n$").unwrap();
    let function = re_export.replace(&function, "globalThis.___handleRequest = $1;".to_string());

    fs::remove_file(bundle_path)?;

    Ok(function.as_bytes().to_vec())
}

#[cfg(test)]
mod tests {
    use std::fs::create_dir_all;

    use super::*;

    #[test]
    #[should_panic(
        expected = r#"The file "../../.tests/src/functions/post.not_exist.js" doesn't exists"#
    )]
    fn test_function_builder_file_not_exist() {
        let dir = "../../.tests/src/functions".to_string();
        let path = format!("{dir}/post.not_exist.js");

        function_builder(&path).unwrap();
    }

    struct TestFunctionBuilderFileNameNoValidName;

    impl Drop for TestFunctionBuilderFileNameNoValidName {
        fn drop(&mut self) {
            std::fs::remove_file("../../.tests/src/functions/post.no_index.js").unwrap();
        }
    }

    #[test]
    #[should_panic(
        expected = r#"The file "../../.tests/src/functions/post.no_index.js" doesn't have a valid name. It should be "index" or "[slug]". Ex. "get.index.(js|jsx|ts|tsx)" or "post.[slug].(js|jsx|ts|tsx)""#
    )]
    fn test_function_builder_file_name_no_valid_name() {
        let _after = TestFunctionBuilderFileNameNoValidName;

        let dir = "../../.tests/src/functions".to_string();
        let path = format!("{dir}/post.no_index.js");

        create_dir_all(dir).unwrap();

        std::fs::write(&path, "<p>Hola!</p>").unwrap();

        function_builder(&path).unwrap();
    }

    struct TestFunctionBuilderFileNameInvalidPrefix;

    impl Drop for TestFunctionBuilderFileNameInvalidPrefix {
        fn drop(&mut self) {
            std::fs::remove_file("../../.tests/src/functions/invalid_prefix.js").unwrap();
        }
    }

    #[test]
    #[should_panic(
        expected = r#"The file "../../.tests/src/functions/invalid_prefix.js" doesn't have a valid name. It should be "index" or "[slug]". Ex. "get.index.(js|jsx|ts|tsx)" or "post.[slug].(js|jsx|ts|tsx)""#
    )]
    fn test_function_builder_invalid_prefix() {
        let _after = TestFunctionBuilderFileNameInvalidPrefix;

        let dir = "../../.tests/src/functions".to_string();
        let path = format!("{dir}/invalid_prefix.js");

        create_dir_all(dir).unwrap();

        std::fs::write(&path, "<p>Hola!</p>").unwrap();

        function_builder(&path).unwrap();
    }

    #[test]
    fn test_function_builder() {
        let dir = "../../.tests/src/functions".to_string();
        let path = format!("{dir}/post.index.js");

        create_dir_all(dir).unwrap();

        std::fs::write(&path, "<p>Hola!</p>").unwrap();

        let function_builder = function_builder(&path).unwrap();

        assert!(function_builder.active);
        assert_eq!(
            function_builder.function,
            [60, 112, 62, 72, 111, 108, 97, 33, 60, 47, 112, 62]
        );
        assert_eq!(function_builder.method, "POST");
        assert_eq!(function_builder.path, "/");

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_function_builder_with_slug() {
        let dir = "../../.tests/src/functions/test".to_string();
        let path = format!("{dir}/get.[slug].ts");

        create_dir_all(dir).unwrap();

        std::fs::write(&path, "<p>Hola!</p>").unwrap();

        let function_builder = function_builder(&path).unwrap();

        assert!(function_builder.active);
        assert_eq!(
            function_builder.function,
            [60, 112, 62, 72, 111, 108, 97, 33, 60, 47, 112, 62]
        );
        assert_eq!(function_builder.method, "GET");
        assert_eq!(function_builder.path, "/test/:slug");

        fs::remove_file(path).unwrap();
    }
}
