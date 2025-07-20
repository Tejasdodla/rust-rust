use anyhow::{anyhow, Result};
use std::{
    fs,
    path::PathBuf,
    process::{Command, Stdio},
    time::Duration,
};
use tokio::time::timeout;

#[derive(Debug, Clone)]
pub struct CodeExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub execution_time: Duration,
}

#[derive(Debug)]
pub struct RustCodeExecutor {
    temp_dir: PathBuf,
}

impl RustCodeExecutor {
    pub fn new() -> Result<Self> {
        let temp_dir = std::env::temp_dir().join("rust-learning");
        fs::create_dir_all(&temp_dir)?;
        
        Ok(Self { temp_dir })
    }

    pub async fn execute_code(&self, code: &str) -> Result<CodeExecutionResult> {
        let start_time = std::time::Instant::now();
        
        // Create a unique temporary file
        let file_name = format!("temp_{}.rs", uuid::Uuid::new_v4());
        let source_file = self.temp_dir.join(&file_name);
        let binary_file = self.temp_dir.join(&file_name.replace(".rs", ""));

        // Write code to temporary file
        fs::write(&source_file, code)?;

        // Compile the code
        let compile_result = timeout(
            Duration::from_secs(30),
            self.compile_rust_code(&source_file, &binary_file)
        ).await??;

        if !compile_result.success {
            return Ok(CodeExecutionResult {
                stdout: compile_result.stdout,
                stderr: compile_result.stderr,
                exit_code: compile_result.exit_code,
                execution_time: start_time.elapsed(),
            });
        }

        // Execute the compiled binary
        let execution_result = timeout(
            Duration::from_secs(10),
            self.run_binary(&binary_file)
        ).await??;

        // Clean up temporary files
        let _ = fs::remove_file(&source_file);
        let _ = fs::remove_file(&binary_file);

        Ok(CodeExecutionResult {
            stdout: execution_result.stdout,
            stderr: execution_result.stderr,
            exit_code: execution_result.exit_code,
            execution_time: start_time.elapsed(),
        })
    }

    async fn compile_rust_code(&self, source_file: &PathBuf, binary_file: &PathBuf) -> Result<CompileResult> {
        let output = Command::new("rustc")
            .arg(source_file)
            .arg("-o")
            .arg(binary_file)
            .arg("--edition")
            .arg("2021")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        Ok(CompileResult {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
        })
    }

    async fn run_binary(&self, binary_file: &PathBuf) -> Result<ExecutionResult> {
        let output = Command::new(binary_file)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        Ok(ExecutionResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
        })
    }

    pub fn validate_rust_syntax(&self, code: &str) -> Result<bool> {
        // Basic syntax validation using rustc --parse-only
        let file_name = format!("validate_{}.rs", uuid::Uuid::new_v4());
        let source_file = self.temp_dir.join(&file_name);

        fs::write(&source_file, code)?;

        let output = Command::new("rustc")
            .arg("--parse-only")
            .arg(&source_file)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        let _ = fs::remove_file(&source_file);

        Ok(output.status.success())
    }
}

#[derive(Debug)]
struct CompileResult {
    success: bool,
    stdout: String,
    stderr: String,
    exit_code: i32,
}

#[derive(Debug)]
struct ExecutionResult {
    stdout: String,
    stderr: String,
    exit_code: i32,
}

impl Default for RustCodeExecutor {
    fn default() -> Self {
        Self::new().expect("Failed to create RustCodeExecutor")
    }
}

// Safety wrapper to limit what code can do
pub fn is_safe_code(code: &str) -> bool {
    let dangerous_patterns = [
        "std::process",
        "std::fs",
        "std::net",
        "unsafe",
        "include!",
        "include_str!",
        "include_bytes!",
        "std::thread::spawn",
        "std::env",
        "libc::",
    ];

    for pattern in &dangerous_patterns {
        if code.contains(pattern) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hello_world() {
        let executor = RustCodeExecutor::new().unwrap();
        let code = r#"
fn main() {
    println!("Hello, World!");
}
"#;

        let result = executor.execute_code(code).await.unwrap();
        assert_eq!(result.exit_code, 0);
        assert_eq!(result.stdout.trim(), "Hello, World!");
    }

    #[test]
    fn test_safety_check() {
        let safe_code = "fn main() { println!(\"Hello\"); }";
        let unsafe_code = "fn main() { std::process::Command::new(\"rm\"); }";

        assert!(is_safe_code(safe_code));
        assert!(!is_safe_code(unsafe_code));
    }
}