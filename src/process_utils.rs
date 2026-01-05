use anyhow::{bail, Result};
use std::process::{Command, Output};

/// Executes a command and captures its standard output and error.
pub fn run_output(command: &str, args: &[String]) -> Result<Output> {
    Ok(Command::new(command).args(args).output()?)
}

/// Executes a command and waits for it to complete.
/// Returns an error if the command fails or exits with non-zero status.
pub fn run_status(command: &str, args: &[String]) -> Result<()> {
    let mut child = Command::new(command).args(args).spawn()?;
    let status = child.wait()?;

    if !status.success() {
        if let Some(code) = status.code() {
            bail!("Command '{}' exited with status {}", command, code);
        } else {
            bail!("Command '{}' was terminated by signal", command);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_output_captures_stdout() {
        let args = vec!["hello".to_string()];
        let output = run_output("echo", &args).unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.trim() == "hello");
    }

    #[test]
    fn run_output_with_multiple_args() {
        let args = vec!["hello".to_string(), "world".to_string()];
        let output = run_output("echo", &args).unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.trim() == "hello world");
    }

    #[test]
    fn run_status_succeeds_for_true_command() {
        let args: Vec<String> = vec![];
        let result = run_status("true", &args);
        assert!(result.is_ok());
    }

    #[test]
    fn run_status_fails_for_false_command() {
        let args: Vec<String> = vec![];
        let result = run_status("false", &args);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("exited with status"));
    }
}
