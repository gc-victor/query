use std::path::Path;
use std::process::Command;
use std::{env, fs};

use serde_json;

fn main() {
    if !Path::new("esbuild.tgz").exists() {
        let mut esbuild_version = "0.19.4".to_string();
        let is_release = env::var("PROFILE").unwrap() == "release";

        if is_release {
            let release = Command::new("sh")
                .arg("-c")
                .arg("curl --silent \"https://api.github.com/repos/evanw/esbuild/releases/latest\"")
                .output()
                .expect("Failed to execute command")
                .stdout;
            let release = String::from_utf8(release).unwrap();

            esbuild_version = serde_json::from_str::<serde_json::Value>(&release).unwrap()
                ["tag_name"]
                .to_string();
        }

        fs::create_dir_all("../../esbuild").unwrap();

        let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
        let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

        match target_os.as_str() {
            "linux" => match target_arch.as_str() {
                "x86_64" => {
                    download_esbuild("linux-x64", &esbuild_version);
                }
                _ => panic!("Unsupported target architecture: {}", target_arch),
            },
            "macos" => match target_arch.as_str() {
                "x86_64" => {
                    download_esbuild("darwin-x64", &esbuild_version);
                }
                "arm64" => {
                    download_esbuild("darwin-arm64", &esbuild_version);
                }
                _ => panic!("Unsupported target architecture: {}", target_arch),
            },
            "windows" => match target_arch.as_str() {
                "x86_64" => {
                    download_esbuild("win32-x64", &esbuild_version);
                }
                _ => panic!("Unsupported target architecture: {}", target_arch),
            },
            _ => panic!("Unsupported target OS: {}", target_os),
        }
    }
}

fn download_esbuild(package: &str, version: &str) {
    let output = Command::new("curl")
        .arg("-o")
        .arg("esbuild.tgz")
        .arg(format!(
            "https://registry.npmjs.org/@esbuild/{}/-/{}-{}.tgz",
            package, package, version
        ))
        .output()
        .expect("Failed to execute command");
    assert!(output.status.success());

    let output = Command::new("tar")
        .arg("xzf")
        .arg("esbuild.tgz")
        .arg("--directory")
        .arg("../../esbuild")
        .output()
        .expect("Failed to execute command");
    assert!(output.status.success());
}
