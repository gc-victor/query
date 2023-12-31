#[allow(unused_imports)]
use std::{
    env, fs,
    process::{exit, Command},
};

use anyhow::Result;
use liquid::ValueView;
use regex::Regex;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;
use walkdir::WalkDir;

use crate::{
    cache::{Cache, CacheItem},
    config::CONFIG,
    utils::{http_client, line_break},
};

use super::commands::FunctionArgs;

#[cfg(windows)]
const ESBUILD: &str = "esbuild.cmd";

#[allow(dead_code)]
#[cfg(not(windows))]
const ESBUILD: &str = "esbuild";

pub async fn command_function(command: &FunctionArgs) -> Result<()> {
    let is_delete = command.delete;
    let path = command
        .path
        .clone()
        .unwrap_or(CONFIG.structure.functions_folder.clone());
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
                    line_break();
                    info!("Successfully function deleted!!!!");
                    line_break();
                }
                Err(err) => panic!("{}", err),
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
                line_break();
                info!("Successfully function updated!!!!");
                line_break();
            }
            Err(err) => panic!("{}", err),
        };
    } else {
        let functions_folder = env::current_dir()?.join(path).to_str().unwrap().to_string();
        for entry in WalkDir::new(functions_folder) {
            let entry = entry?;

            if entry.file_type().is_file() {
                let file_path = entry.path().display().to_string();

                let FunctionBuilder {
                    active,
                    function,
                    method,
                    path,
                } = function_builder(&file_path)?;

                let body_path = path.replace(
                    &env::current_dir()?
                        .join(CONFIG.structure.functions_folder.clone())
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

                let mut cache = Cache::new();
                let value = function.to_vec().to_kstr().to_string();
                let is_cached = match cache.get(&path) {
                    Some(cache_item) => cache_item.value == value,
                    None => false,
                };

                if !is_cached {
                    match http_client("function-builder", Some(&body), Method::POST).await {
                        Ok(_) => {
                            info!("Function updated: {}", file_path);
                            cache.set(CacheItem { key: path, value })?;
                        }
                        Err(err) => panic!("{}", err),
                    };
                } else {
                    info!("Function cached: {file_path}");
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
            panic!(r#"The function file "{}" doesn't exists"#, file_path);
        }
    };
    let reg =
        Regex::new(r#"^(.*)\/(delete|get|patch|put|post)\.(index|\[slug\])\.(js|ts)$"#).unwrap();
    let name: String = reg.replace(file_path, "$3").to_string();
    if name != "index" && name != "[slug]" {
        panic!(
            r#"The file "{}" doesn't have a valid name. It should be "index" or "[slug]". Ex. "get.index.js" or "post.[slug].js""#,
            file_path
        );
    }
    let path: String = reg.replace(file_path, "$1$3").to_string();
    let path = path.split("/functions").last().unwrap().to_string();
    let path = path
        .replace("functions", "")
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

    Command::new(ESBUILD)
        .args([
            &path,
            "--bundle",
            "--format=esm",
            "--legal-comments=none",
            "--minify=true",
            "--target=esnext",
            "--platform=browser",
            out_dir_flag,
        ])
        .output()
        .expect("Failed to execute esbuild");

    let bundle_path = format!("{}/{}", out_dir, function_path.split('/').last().unwrap());
    let function = fs::read_to_string(&bundle_path)?;
    let function = function.replace("var handleRequest", "globalThis.___handleRequest");
    let re = Regex::new(r"export\{(\w) as handleRequest\};\n$").unwrap();
    let var: &str = re.captures(&function).unwrap().get(1).unwrap().as_str();
    let function = re.replace(
        &function,
        &format!("globalThis.___handleRequest = {};", var),
    );

    fs::remove_file(bundle_path)?;

    Ok(function.as_bytes().to_vec())
}

#[cfg(test)]
mod tests {
    use std::fs::create_dir_all;

    use super::*;

    #[test]
    #[should_panic(
        expected = r#"The function file "../../.tests/path/to/functions/post.not_exist.js" doesn't exists"#
    )]
    fn test_function_builder_file_not_exist() {
        let dir = "../../.tests/path/to/functions".to_string();
        let path = format!("{dir}/post.not_exist.js");

        function_builder(&path).unwrap();
    }

    struct TestFunctionBuilderFileNameNoValidName;

    impl Drop for TestFunctionBuilderFileNameNoValidName {
        fn drop(&mut self) {
            std::fs::remove_file("../../.tests/path/to/functions/post.no_index.js").unwrap();
        }
    }

    #[test]
    #[should_panic(
        expected = r#"The file "../../.tests/path/to/functions/post.no_index.js" doesn't have a valid name. It should be "index" or "[slug]". Ex. "get.index.js" or "post.[slug].js""#
    )]
    fn test_function_builder_file_name_no_valid_name() {
        let _after = TestFunctionBuilderFileNameNoValidName;

        let dir = "../../.tests/path/to/functions".to_string();
        let path = format!("{dir}/post.no_index.js");

        create_dir_all(dir).unwrap();

        std::fs::write(&path, "<p>Hola!</p>").unwrap();

        function_builder(&path).unwrap();
    }

    struct TestFunctionBuilderFileNameInvalidPrefix;

    impl Drop for TestFunctionBuilderFileNameInvalidPrefix {
        fn drop(&mut self) {
            std::fs::remove_file("../../.tests/path/to/functions/invalid_prefix.js").unwrap();
        }
    }

    #[test]
    #[should_panic(
        expected = r#"The file "../../.tests/path/to/functions/invalid_prefix.js" doesn't have a valid name. It should be "index" or "[slug]". Ex. "get.index.js" or "post.[slug].js""#
    )]
    fn test_function_builder_invalid_prefix() {
        let _after = TestFunctionBuilderFileNameInvalidPrefix;

        let dir = "../../.tests/path/to/functions".to_string();
        let path = format!("{dir}/invalid_prefix.js");

        create_dir_all(dir).unwrap();

        std::fs::write(&path, "<p>Hola!</p>").unwrap();

        function_builder(&path).unwrap();
    }

    #[test]
    fn test_function_builder() {
        let dir = "../../.tests/path/to/functions".to_string();
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
        let dir = "../../.tests/path/to/functions/test".to_string();
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
