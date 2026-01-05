use std::process::Command;

fn cargo_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_tm"))
}

#[test]
fn help_flag_shows_usage() {
    let output = cargo_bin().arg("--help").output().unwrap();

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("tm"));
    assert!(stdout.contains("tmux session manager"));
    assert!(stdout.contains("USAGE"));
    assert!(stdout.contains("OPTIONS"));
    assert!(stdout.contains("--help"));
    assert!(stdout.contains("--version"));
}

#[test]
fn short_help_flag_shows_usage() {
    let output = cargo_bin().arg("-h").output().unwrap();

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("USAGE"));
}

#[test]
fn version_flag_shows_version() {
    let output = cargo_bin().arg("--version").output().unwrap();

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("tm"));
    assert!(stdout.contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn short_version_flag_shows_version() {
    let output = cargo_bin().arg("-V").output().unwrap();

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn unknown_argument_shows_error() {
    let output = cargo_bin().arg("--invalid").output().unwrap();

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown argument"));
    assert!(stderr.contains("--invalid"));
}
