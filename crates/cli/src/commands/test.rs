use std::{
    collections::hash_map::DefaultHasher,
    error::Error,
    fmt::{Display, Formatter},
    fs,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    process::{exit, Command},
    sync::{Arc, Mutex, Once},
    time::Instant,
};

use crate::cache::{Cache, CacheItem};

use colored::Colorize;
use jsx_parser::jsx_precompile::jsx_precompile;
use num_cpus;
use query_runtime::{timers::TimerPoller, Runtime};
use rayon::prelude::*;
use regex::Regex;
use rquickjs::{async_with, Array, Function, Module, Object, Promise};
use walkdir::WalkDir;
use watchexec::Watchexec;

use crate::{
    config::CONFIG,
    utils::{detect_package_manager, has_node_modules_binary, which},
};

use super::commands::TestArgs;
use constants::*;

mod constants {
    // File and Extension Constants
    pub const TEST_FILE_EXTENSIONS: [&str; 4] = ["js", "jsx", "ts", "tsx"];
    pub const TEST_FILE_PATTERNS: [&str; 2] = [".test.ext", ".spec.ext"];

    // ESBuild Constants
    pub const ESBUILD_CMD: &str = "esbuild";
    pub const ESBUILD_FLAGS: [&str; 6] = [
        "--bundle",
        "--format=esm",
        "--minify-whitespace",
        "--legal-comments=none",
        "--platform=browser",
        "--jsx=preserve",
    ];

    // External Module Constants
    pub const EXTERNAL_MODULES: [&str; 4] = [
        "--external:query:email",
        "--external:query:database",
        "--external:query:plugin",
        "--external:query:test",
    ];

    // Message Constants
    pub const WATCHING_MSG: &str = "Watching...";
    pub const CHANGES_DETECTED_MSG: &str = "Changes detected, running tests...";
    pub const SKIPPING_UNCHANGED_MSG: &str = "Skipping unchanged test bundle";
    pub const ERROR_MSG: &str = "[ERROR]";

    // Test Report Constants
    pub const TEST_FAILURES_HEADER: &str = "Test Failures:";
    pub const TESTS_LABEL: &str = "Tests:";
    pub const PASSED_LABEL: &str = "Passed:";
    pub const FAILED_LABEL: &str = "Failed:";
    pub const TIME_LABEL: &str = "Time:";
    pub const TOTAL_TIME_LABEL: &str = "Total time:";

    // Polyfill Imports
    pub const IMPORTS: [&str; 11] = [
        "polyfill/blob",
        "polyfill/console",
        "polyfill/fetch",
        "polyfill/file",
        "polyfill/form-data",
        "polyfill/request",
        "polyfill/response",
        "polyfill/web-streams",
        "js/database",
        "js/handle-response",
        "js/jsx-helpers",
    ];
}

static ESBUILD_CHECK: Once = Once::new();
static THREAD_POOL_INIT: Once = Once::new();

#[derive(Debug)]
pub enum TestRunnerError {
    NoTestFiles,
    NoMatchingFiles,
    FileReadError {
        path: PathBuf,
        error: std::io::Error,
    },
    ESBuildNotFound {
        package_manager: String,
    },
    ESBuildError(String),
    JSXCompileError(String),
    RuntimeError(String),
    WatchError(String),
    IoError(std::io::Error),
    QuickJsError(rquickjs::Error),
    ThreadError(String),
    TokioRuntimeError(String),
    RegexError(regex::Error),
}

impl Display for TestRunnerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoTestFiles => {
                write!(f, "{} No test files specified", String::from('●').yellow())
            }
            Self::NoMatchingFiles => write!(
                f,
                "{} No matching test files found in specified paths",
                String::from('●').yellow()
            ),
            Self::FileReadError { path, error } => {
                write!(
                    f,
                    "{} Failed to read test file {}: {}",
                    String::from('●').red(),
                    path.display(),
                    error
                )
            }
            Self::ESBuildNotFound { package_manager } => {
                write!(
                    f,
                    "{} esbuild not found. Please run `{} install esbuild` first",
                    String::from('●').red(),
                    package_manager
                )
            }
            Self::ESBuildError(msg) => write!(
                f,
                "{} esbuild execution failed: {}",
                String::from('●').red(),
                msg
            ),
            Self::JSXCompileError(msg) => {
                write!(f, "{} JSX compile error: {}", String::from('●').red(), msg)
            }
            Self::RuntimeError(msg) => {
                write!(f, "{} Runtime error: {}", String::from('●').red(), msg)
            }
            Self::WatchError(msg) => write!(f, "{} Watch error: {}", String::from('●').red(), msg),
            Self::IoError(err) => write!(f, "{} IO error: {}", String::from('●').red(), err),
            Self::QuickJsError(err) => {
                write!(f, "{} QuickJS error: {}", String::from('●').red(), err)
            }
            Self::ThreadError(msg) => {
                write!(f, "{} Thread error: {}", String::from('●').red(), msg)
            }
            Self::TokioRuntimeError(msg) => write!(
                f,
                "{} Tokio runtime error: {}",
                String::from('●').red(),
                msg
            ),
            Self::RegexError(err) => write!(f, "{} Regex error: {}", String::from('●').red(), err),
        }
    }
}

impl Error for TestRunnerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::FileReadError { error, .. } => Some(error),
            Self::IoError(err) => Some(err),
            Self::QuickJsError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for TestRunnerError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<rquickjs::Error> for TestRunnerError {
    fn from(error: rquickjs::Error) -> Self {
        Self::QuickJsError(error)
    }
}

impl From<regex::Error> for TestRunnerError {
    fn from(error: regex::Error) -> Self {
        Self::RegexError(error)
    }
}

type Result<T> = std::result::Result<T, TestRunnerError>;

#[derive(Debug, Clone)]
struct TestResult {
    filename: String,
    current_suite: String,
    name: String,
    start: u64,
    end: u64,
    error: Option<String>,
}

#[derive(Debug)]
pub struct TestRunner {
    test_files: Vec<PathBuf>,
    test_name_pattern: String,
    enable_spy: bool,
    filters: Vec<PathBuf>,
    cache: Cache,
}

#[derive(Debug)]
struct TestStats {
    total_tests: usize,
    total_passed: usize,
    total_failed: usize,
    first_test_start: u64,
    last_test_end: u64,
}

impl TestStats {
    fn new() -> Self {
        Self {
            total_tests: 0,
            total_passed: 0,
            total_failed: 0,
            first_test_start: 0,
            last_test_end: 0,
        }
    }

    fn update(&mut self, has_error: bool, start_time: u64, end_time: u64) {
        self.total_tests += 1;
        if has_error {
            self.total_failed += 1;
        } else {
            self.total_passed += 1;
        }

        if self.first_test_start == 0 || start_time < self.first_test_start {
            self.first_test_start = start_time;
        }

        if self.last_test_end == 0 || end_time > self.last_test_end {
            self.last_test_end = end_time;
        }
    }

    fn print_summary(&self) {
        eprintln!("\n{} {}", TESTS_LABEL, self.total_tests);
        eprintln!(
            "{}",
            format!("{} {}", PASSED_LABEL, self.total_passed).green()
        );
        eprintln!(
            "{}",
            format!("{} {}", FAILED_LABEL, self.total_failed).red()
        );

        let total_time = (self.last_test_end - self.first_test_start) as f64 / 1_000_000.0;
        eprintln!(
            "{}",
            format!("{} {:.2}ms", TIME_LABEL, total_time).bright_black()
        );
    }
}

pub async fn command_test(command: &TestArgs) -> Result<()> {
    let mut runner = TestRunner::new(command)?;
    runner.run(command.watch).await
}

impl TestRunner {
    pub fn new(args: &TestArgs) -> Result<Self> {
        let test_name_pattern = args
            .test_name_pattern
            .as_deref()
            .unwrap_or_default()
            .to_owned();
        let filters: Vec<PathBuf> = args.filters.iter().map(PathBuf::from).collect();
        let test_files = Self::get_test_files(&filters)?;

        Ok(Self {
            test_files,
            test_name_pattern,
            enable_spy: args.spy,
            filters,
            cache: Cache::new(),
        })
    }

    pub async fn run(&mut self, watch: bool) -> Result<()> {
        if watch {
            eprintln!("{} {}", String::from('●').cyan(), WATCHING_MSG);
            self.watch_tests().await?;
        } else {
            let total_failed = self.run_tests()?;
            if total_failed > 0 {
                exit(1);
            }
        }

        Ok(())
    }

    fn get_test_files(filters: &[PathBuf]) -> Result<Vec<PathBuf>> {
        if filters.is_empty() {
            return Err(TestRunnerError::NoTestFiles);
        }

        let files: Vec<PathBuf> = filters
            .iter()
            .flat_map(|filter| {
                WalkDir::new(filter)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().is_file() && Self::is_test_file(e.path()))
                    .map(|e| e.path().to_path_buf())
            })
            .collect();

        if files.is_empty() {
            return Err(TestRunnerError::NoMatchingFiles);
        }

        Ok(files)
    }

    fn is_test_file(path: &Path) -> bool {
        let file_name = path.to_string_lossy();
        TEST_FILE_PATTERNS.iter().any(|pattern| {
            TEST_FILE_EXTENSIONS
                .iter()
                .any(|ext| file_name.ends_with(&pattern.replace("ext", ext)))
        })
    }

    async fn watch_tests(&mut self) -> Result<()> {
        let test_files_clone = self.test_files.clone();
        let pattern_clone = self.test_name_pattern.clone();
        let enable_spy = self.enable_spy;

        self.run_tests()?;

        let wx = Watchexec::new_async(move |mut action| {
            let files = test_files_clone.clone();
            let pattern = pattern_clone.clone();

            Box::new(async move {
                if action.signals().next().is_some() {
                    action.quit();
                } else {
                    eprintln!("\n{} {}", String::from('●').cyan(), CHANGES_DETECTED_MSG);

                    let mut runner = TestRunner {
                        test_files: files,
                        test_name_pattern: pattern,
                        enable_spy,
                        filters: Vec::new(),
                        cache: Cache::new(),
                    };

                    let _ = runner.run_tests();
                }

                action
            })
        })
        .map_err(|e| TestRunnerError::WatchError(e.to_string()))?;

        let filters = self.filters.clone();
        let filters_clone: Vec<_> = filters.iter().map(Path::new).collect();

        wx.config.pathset(filters_clone);

        let _ = wx
            .main()
            .await
            .map_err(|e| TestRunnerError::WatchError(e.to_string()))?;

        Ok(())
    }

    fn run_tests(&mut self) -> Result<u32> {
        let start_time = Some(Instant::now());

        eprintln!(
            "{} Running {} tests files",
            String::from('●').cyan(),
            self.test_files.len()
        );

        ESBUILD_CHECK.call_once(|| {
            if let Err(e) = self.check_esbuild_availability() {
                eprintln!("esbuild check failed: {}", e);
                exit(1);
            }
        });

        THREAD_POOL_INIT.call_once(|| {
            let num_threads = num_cpus::get();
            if let Err(e) = rayon::ThreadPoolBuilder::new()
                .num_threads(num_threads)
                .build_global()
            {
                eprintln!("Warning: Failed to initialize thread pool: {}", e);
            }
        });

        let test_files = self.test_files.clone();
        let test_results: Arc<Mutex<Vec<TestResult>>> = Arc::new(Mutex::new(Vec::new()));
        let test_results_clone = test_results.clone();
        let self_mutex = Arc::new(Mutex::new(self));
        let test_files = if test_files.len() == 1 {
            vec![test_files[0].clone(), PathBuf::new()]
        } else {
            test_files
        };

        test_files.par_iter().try_for_each(|test_file: &PathBuf| {
            if test_file.to_string_lossy().is_empty() {
                return Ok(());
            }

            let test_results = Arc::clone(&test_results_clone);
            let self_mutex = Arc::clone(&self_mutex);

            let runtime = tokio::runtime::Runtime::new()
                .map_err(|e| TestRunnerError::RuntimeError(e.to_string()))?;

            runtime.block_on(async {
                let code = {
                    let mut self_guard = self_mutex.lock().map_err(|e| {
                        TestRunnerError::ThreadError(format!("Failed to lock self: {}", e))
                    })?;
                    let mut generated_code = self_guard.bundle_test_file(test_file)?;

                    let mut hasher = DefaultHasher::new();
                    generated_code.hash(&mut hasher);
                    let cache_value = hasher.finish().to_string();
                    let test_file_str = test_file.to_string_lossy().to_string();

                    if let Some(cached) = self_guard.cache.get(&test_file_str) {
                        if cached.value == cache_value {
                            eprintln!(
                                "{} {}: {}",
                                String::from('✓').green(),
                                SKIPPING_UNCHANGED_MSG,
                                test_file_str.bold()
                            );

                            generated_code = String::new();
                        }
                    }

                    let _ = self_guard.cache.set(CacheItem {
                        key: test_file_str,
                        value: cache_value,
                    });

                    Ok::<String, TestRunnerError>(generated_code)
                }?;

                if code.is_empty() {
                    return Ok(());
                }

                let test_file_str = test_file.to_string_lossy();
                let code = if test_file_str.ends_with(".jsx") || test_file_str.ends_with(".tsx") {
                    jsx_precompile(&code)
                        .map_err(|e| TestRunnerError::JSXCompileError(e.to_string()))?
                } else {
                    code
                };

                let runtime = Runtime::new()
                    .await
                    .map_err(|e| TestRunnerError::RuntimeError(e.to_string()))?;
                let ctx = &runtime.ctx;
                let code_clone = code.clone();
                let results: Vec<TestResult> = async_with!(ctx => |ctx| {
                    let module = Module::declare(
                        ctx.clone(),
                        test_file.to_string_lossy().to_string(),
                        code_clone
                    )?;

                    module.eval().map_err(|e| TestRunnerError::RuntimeError(e.to_string()))?;

                    let global_this: Object = ctx.clone().globals().get("globalThis")?;
                    let handle_tests: Function = global_this.get("___handleTests")?;
                    let promise: Promise = handle_tests.call(())?;

                    while ctx.poll_timers() {}

                    let test_array: Array = promise.into_future().await?;
                    let test_result = test_array.iter::<Object>()
                        .map(|obj| {
                            let obj = obj?;
                            Ok(TestResult {
                                filename: obj.get("filename")?,
                                current_suite: obj.get("currentSuite")?,
                                name: obj.get("name")?,
                                start: obj.get("start")?,
                                end: obj.get("end")?,
                                error: obj.get("error").unwrap_or_default()
                            })
                        })
                        .collect::<Result<Vec<_>>>();

                    test_result
                })
                .await?;

                if results.iter().any(|r| r.error.is_some()) {
                    if let Ok(mut self_guard) = self_mutex.lock() {
                        let _ = self_guard
                            .cache
                            .remove(test_file.to_string_lossy().as_ref());
                    }
                }

                let mut test_results = test_results.lock().map_err(|e| {
                    TestRunnerError::ThreadError(format!("Failed to lock test results: {}", e))
                })?;
                test_results.extend(results);

                Ok::<(), TestRunnerError>(())
            })
        })?;

        let results = test_results.lock().map_err(|e| {
            TestRunnerError::ThreadError(format!("Failed to lock test results: {}", e))
        })?;
        let mut sorted_results = results.clone();
        sorted_results.sort_by(|a, b| a.filename.cmp(&b.filename));

        let mut current_filename = String::new();
        let mut stats = TestStats::new();

        for test_result in sorted_results {
            let TestResult {
                filename,
                current_suite,
                name,
                start,
                end,
                error,
            } = test_result;

            let total_time = (end - start) as f64 / 1_000_000.0;
            let elapsed = format!("[{:.3}ms]", total_time);

            if filename != current_filename {
                eprintln!("\n{}:\n", filename.bold());
                current_filename = filename;
            }

            stats.update(error.is_some(), start, end);

            match error {
                Some(err_msg) => {
                    eprintln!(
                        "{} {} > {} {}\n {} {}",
                        "✗".red(),
                        current_suite,
                        name.bold(),
                        elapsed.bright_black().dimmed(),
                        "-".red(),
                        err_msg.red()
                    );
                }
                None => {
                    eprintln!(
                        "{} {} > {} {}",
                        "✓".green(),
                        current_suite,
                        name.bold(),
                        elapsed.bright_black().dimmed(),
                    );
                }
            }
        }

        let mut current_failure_filename = String::new();
        let mut has_failures = true;

        for result in results.iter() {
            if let Some(error) = &result.error {
                if has_failures {
                    has_failures = false;
                    eprintln!("\n{}", TEST_FAILURES_HEADER.red());
                }
                if result.filename != current_failure_filename {
                    eprintln!("\n{}", result.filename.bold().red());
                    current_failure_filename = result.filename.clone();
                }

                eprintln!(
                    "{}",
                    format!(
                        "  {} {} > {}\n  {}",
                        "✗".red(),
                        result.current_suite,
                        result.name.bold(),
                        error
                    )
                    .red()
                );
            }
        }

        stats.print_summary();

        if let Some(start_time) = start_time {
            let elapsed = start_time.elapsed();
            eprintln!(
                "{}",
                format!("{} {:.2?}", TOTAL_TIME_LABEL, elapsed).bright_black()
            );
        }

        Ok(stats.total_failed as u32)
    }

    fn check_esbuild_availability(&self) -> Result<()> {
        let pm = detect_package_manager();
        let esbuild_global = which(ESBUILD_CMD).unwrap_or_default();
        let has_esbuild_global = !esbuild_global.is_empty();
        let has_esbuild_local_binary = has_node_modules_binary(ESBUILD_CMD);
        let has_esbuild = has_esbuild_local_binary || has_esbuild_global;

        if !has_esbuild {
            return Err(TestRunnerError::ESBuildNotFound {
                package_manager: pm.npm.to_string(),
            });
        }

        Ok(())
    }

    fn bundle_test_file(&mut self, path: &Path) -> Result<String> {
        let outdir = tempfile::tempdir()?;
        let outdir = outdir.path().to_string_lossy();
        let outdir_flag = &format!("--outdir={}", outdir);

        let mut args = vec![path.to_string_lossy().to_string(), outdir_flag.to_string()];

        args.extend(ESBUILD_FLAGS.iter().map(|&s| s.to_string()));
        args.extend(EXTERNAL_MODULES.iter().map(|&s| s.to_string()));

        for (key, value) in &CONFIG.esbuild {
            let flag = if value.is_empty() {
                format!("--{}", key)
            } else {
                format!("--{}={}", key, value)
            };
            args.push(flag);
        }

        let command = if has_node_modules_binary(ESBUILD_CMD) {
            Path::new("node_modules").join(".bin").join(ESBUILD_CMD)
        } else {
            match which(ESBUILD_CMD) {
                Some(esbuild) => Path::new(&esbuild).to_path_buf(),
                None => {
                    return Err(TestRunnerError::ESBuildNotFound {
                        package_manager: "npm".to_string(),
                    });
                }
            }
        };

        match Command::new(command).args(&args).output() {
            Ok(output) => {
                let output = std::str::from_utf8(&output.stderr)
                    .map_err(|e| TestRunnerError::ESBuildError(e.to_string()))?;

                if output.contains(ERROR_MSG) {
                    eprintln!("{} {}", String::from('●').red(), output);
                    return Ok(String::new());
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                return Ok(String::new());
            }
        };

        let path_str = path.to_string_lossy();
        let bundle_path = format!("{}/{}", outdir, path_str.split('/').last().unwrap());
        let re = Regex::new(r"(\.jsx|\.tsx|\.ts)$").map_err(|e| TestRunnerError::RegexError(e))?;
        let bundle_path = re.replace(&bundle_path, ".js").to_string();
        let function = fs::read_to_string(&bundle_path)?;

        let code = if self.enable_spy {
            function.replace(
                    "var __export=(target,all)=>{for(var name in all)__defProp(target,name,{get:all[name],enumerable:true})};",
                    "var __export=(target,all)=>{for(var name in all){Object.defineProperty(target,name,{value:all[name],writable:true});}};"
                )
        } else {
            function.to_string()
        };

        let path_str = path.to_string_lossy().to_string();

        let imports = IMPORTS
            .iter()
            .map(|import| format!("import '{}';", import))
            .collect::<Vec<_>>()
            .join(" ");

        Ok(format!(
            " \
                {} \
                globalThis.___testNamePattern = '{}'; \
                globalThis.___testFilename('{path_str}'); \
                {code} \
                globalThis.___handleTests = async () => {{ return await Promise.resolve(globalThis.___testsResults()); }};
            ",
            imports,
            self.test_name_pattern
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[derive(Default)]
    struct TestArgsBuilder {
        test_name_pattern: Option<String>,
        filters: Vec<String>,
        spy: bool,
    }

    impl TestArgsBuilder {
        fn with_pattern(mut self, pattern: &str) -> Self {
            self.test_name_pattern = Some(pattern.to_string());
            self
        }

        fn with_filters(mut self, filters: Vec<String>) -> Self {
            self.filters = filters;
            self
        }

        fn with_spy(mut self, spy: bool) -> Self {
            self.spy = spy;
            self
        }

        fn build(self) -> TestArgs {
            TestArgs {
                test_name_pattern: self.test_name_pattern,
                filters: self.filters,
                spy: self.spy,
                watch: false,
            }
        }
    }

    fn create_test_file(dir: &Path, name: &str, content: &str) -> PathBuf {
        let path = dir.join(name);
        fs::write(&path, content).unwrap();
        path
    }

    #[test]
    fn test_error_display() {
        let error = TestRunnerError::NoTestFiles;
        assert!(error.to_string().contains("No test files specified"));

        let error = TestRunnerError::NoMatchingFiles;
        assert!(error
            .to_string()
            .contains("No matching test files found in specified paths"));
    }

    #[test]
    fn test_error_source() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let error = TestRunnerError::FileReadError {
            path: PathBuf::from("test.js"),
            error: io_error,
        };
        assert!(error.source().is_some());

        let error = TestRunnerError::NoTestFiles;
        assert!(error.source().is_none());
    }

    #[tokio::test]
    async fn test_no_test_files() {
        let args = TestArgsBuilder::default().with_filters(vec![]).build();

        let result = TestRunner::new(&args);
        assert!(matches!(result.unwrap_err(), TestRunnerError::NoTestFiles));
    }

    #[tokio::test]
    async fn test_no_matching_files() {
        let temp_dir = tempdir().unwrap();
        let args = TestArgsBuilder::default()
            .with_filters(vec![temp_dir.path().to_string_lossy().to_string()])
            .build();

        let result = TestRunner::new(&args);
        assert!(matches!(
            result.unwrap_err(),
            TestRunnerError::NoMatchingFiles
        ));
    }

    #[test]
    fn test_esbuild_not_found_error() {
        let error = TestRunnerError::ESBuildNotFound {
            package_manager: "npm".to_string(),
        };
        assert!(error
            .to_string()
            .contains("esbuild not found. Please run `npm install esbuild` first"));
    }

    #[test]
    fn test_esbuild_execution_error() {
        let error = TestRunnerError::ESBuildError("Build failed".to_string());
        assert!(error
            .to_string()
            .contains("esbuild execution failed: Build failed"));
    }

    #[test]
    fn test_runtime_error() {
        let error = TestRunnerError::RuntimeError("Script error".to_string());
        assert!(error.to_string().contains("Runtime error: Script error"));
    }

    #[test]
    fn test_watch_error() {
        let error = TestRunnerError::WatchError("Watch failed".to_string());
        assert!(error.to_string().contains("Watch error: Watch failed"));
    }

    #[test]
    fn test_io_error() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let error = TestRunnerError::IoError(io_error);
        assert!(error.to_string().contains("IO error: File not found"));
    }

    #[test]
    fn test_quickjs_error() {
        let error = TestRunnerError::QuickJsError(rquickjs::Error::Exception);
        assert!(error.to_string().contains("QuickJS error"));
    }

    #[tokio::test]
    async fn test_new_runner_with_valid_files() {
        let temp_dir = tempdir().unwrap();
        let test_file = create_test_file(
            temp_dir.path(),
            "example.test.js",
            "test('example', () => {});",
        );

        let args = TestArgsBuilder::default()
            .with_filters(vec![test_file.to_string_lossy().to_string()])
            .build();

        let runner = TestRunner::new(&args).unwrap();
        assert_eq!(runner.test_files.len(), 1);
        assert_eq!(runner.test_name_pattern, "");
        assert!(!runner.enable_spy);
    }

    #[tokio::test]
    #[should_panic(expected = "NoTestFiles")]
    async fn test_new_runner_with_no_files() {
        let args = TestArgsBuilder::default().with_filters(vec![]).build();

        TestRunner::new(&args).unwrap();
    }

    #[test]
    fn test_is_test_file() {
        let valid_files = vec![
            "test.test.js",
            "component.spec.tsx",
            "util.test.jsx",
            "service.spec.ts",
        ];

        let invalid_files = vec!["test.js", "spec.tsx", "test.css", "component.ts"];

        for file in valid_files {
            assert!(TestRunner::is_test_file(Path::new(file)));
        }

        for file in invalid_files {
            assert!(!TestRunner::is_test_file(Path::new(file)));
        }
    }

    #[tokio::test]
    async fn test_runner_with_spy_enabled() {
        let temp_dir = tempdir().unwrap();
        let test_file = create_test_file(
            temp_dir.path(),
            "spy.test.js",
            "test('spy test', () => {});",
        );

        let args = TestArgsBuilder::default()
            .with_filters(vec![test_file.to_string_lossy().to_string()])
            .with_spy(true)
            .build();

        let runner = TestRunner::new(&args).unwrap();
        assert!(runner.enable_spy);
    }

    #[test]
    fn test_spy_code_transformation() {
        let temp_dir = tempdir().unwrap();
        let utils_file = create_test_file(
            temp_dir.path(),
            "utils.js",
            "export const utilFn = () => 'util';",
        );

        let test_file_name = "spy_transform.test.js";
        let test_file = create_test_file(
            temp_dir.path(),
            test_file_name,
            r#"
                import * as utils from './utils.js';
                export const testFn = () => {
                    const deps = utils;
                    return deps.utilFn();
                };
                "#,
        );

        let filters = vec![
            test_file.to_string_lossy().to_string(),
            utils_file.to_string_lossy().to_string(),
        ];
        let args = TestArgsBuilder::default()
            .with_filters(filters)
            .with_spy(true)
            .build();

        let mut runner = TestRunner::new(&args).unwrap();
        let path = temp_dir.path().join(test_file_name);
        let code = runner.bundle_test_file(&path).unwrap();

        assert!(!code.contains("var __export=(target,all)=>{for(var name in all)__defProp(target,name,{get:all[name],enumerable:true})};"));
        assert!(code.contains("var __export=(target,all)=>{for(var name in all){Object.defineProperty(target,name,{value:all[name],writable:true});}};"));
    }

    #[tokio::test]
    async fn test_with_multiple_files() {
        let temp_dir = tempdir().unwrap();
        let test_file1 =
            create_test_file(temp_dir.path(), "test1.test.js", "test('test1', () => {});");
        let test_file2 = create_test_file(
            temp_dir.path(),
            "test2.spec.tsx",
            "test('test2', () => {});",
        );
        let non_test_file =
            create_test_file(temp_dir.path(), "normal.js", "console.log('not a test');");

        let filters = vec![temp_dir.path().to_string_lossy().to_string()];
        let files =
            TestRunner::get_test_files(&filters.iter().map(PathBuf::from).collect::<Vec<_>>())
                .unwrap();

        assert_eq!(files.len(), 2);
        assert!(files.contains(&test_file1));
        assert!(files.contains(&test_file2));
        assert!(!files.contains(&non_test_file));
    }

    #[tokio::test]
    async fn test_runner_with_test_pattern() {
        let temp_dir = tempdir().unwrap();
        let test_file = create_test_file(
            temp_dir.path(),
            "pattern.test.js",
            "test('pattern test', () => {});",
        );

        let pattern = "pattern";
        let args = TestArgsBuilder::default()
            .with_filters(vec![test_file.to_string_lossy().to_string()])
            .with_pattern(pattern)
            .build();

        let runner = TestRunner::new(&args).unwrap();
        assert_eq!(runner.test_name_pattern, pattern);
    }
}
