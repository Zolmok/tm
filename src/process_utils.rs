use std::process::{Command, ExitStatus, Output};

/// Executes a command and captures its standard output and error.
///
/// # Arguments
///
/// * `command` - The name of the command to run.
/// * `args` - Arguments to pass to the command.
///
/// # Returns
///
/// * `Ok(Output)` if the command runs successfully.
/// * `Err(std::io::Error)` if the command fails to start or run.
pub fn run_output(command: &str, args: &[String]) -> Result<Output, std::io::Error> {
    let output_result = Command::new(command).args(args).output();

    match output_result {
        Ok(output) => Ok(output),
        Err(error) => Err(error),
    }
}

/// Executes a command and waits for its exit status.
///
/// # Arguments
///
/// * `command` - The name of the command to run.
/// * `args` - Arguments to pass to the command.
///
/// # Returns
///
/// * `Ok(ExitStatus)` if the command completes.
/// * `Err(std::io::Error)` if the command fails to spawn or wait.
pub fn run_status(command: &str, args: &[String]) -> Result<ExitStatus, std::io::Error> {
    let child_result = Command::new(command).args(args).spawn();

    let mut child = match child_result {
        Ok(result) => result,
        Err(error) => return Err(error),
    };

    match child.try_wait() {
        Ok(Some(status)) => Ok(status),
        Ok(None) => match child.wait() {
            Ok(result) => Ok(result),
            Err(error) => Err(error),
        },
        Err(error) => Err(error),
    }
}
