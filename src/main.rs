use anyhow::{Context, Result};
use std::collections::HashSet;
use std::env;
use std::io::{self, Write};
use std::process::ExitCode;

mod fs_utils;
mod process_utils;
mod session_utils;

use fs_utils::prompt_valid_path;
use process_utils::{run_output, run_status};
use session_utils::{resolve_session_name, validate_session_name, SessionAction};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> ExitCode {
    // Handle --help and --version flags
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "-h" | "--help" => {
                print_help();
                return ExitCode::SUCCESS;
            }
            "-V" | "--version" => {
                println!("tm {}", VERSION);
                return ExitCode::SUCCESS;
            }
            arg => {
                eprintln!("Unknown argument: {}", arg);
                eprintln!("Usage: tm [--help | --version]");
                return ExitCode::FAILURE;
            }
        }
    }

    if let Err(e) = run() {
        eprintln!("Error: {:#}", e);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn print_help() {
    println!("tm {} - tmux session manager", VERSION);
    println!();
    println!("USAGE:");
    println!("    tm [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help       Print help information");
    println!("    -V, --version    Print version information");
    println!();
    println!("DESCRIPTION:");
    println!("    Interactive tool for managing tmux sessions.");
    println!("    Lists existing sessions and allows attaching or creating new ones.");
}

/// Main application logic.
fn run() -> Result<()> {
    let sessions = list_sessions()?;
    let existing_sessions: HashSet<&str> = sessions.iter().map(|s| s.as_str()).collect();

    display_sessions(&sessions);

    let choice = prompt_choice()?;

    if choice.eq_ignore_ascii_case("n") {
        handle_new_session(&existing_sessions)?;
    } else {
        handle_session_selection(&choice, &sessions)?;
    }

    Ok(())
}

/// Lists all existing tmux sessions.
fn list_sessions() -> Result<Vec<String>> {
    let args = vec!["ls".to_string(), "-F".to_string(), "#S".to_string()];
    let output = run_output("tmux", &args).context("Failed to run tmux ls")?;

    // Print any stderr output (e.g., "no server running")
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !stderr.trim().is_empty() {
        eprint!("{}", stderr);
    }

    let stdout = std::str::from_utf8(&output.stdout).context("Invalid UTF-8 in tmux output")?;

    Ok(stdout.lines().map(|s| s.to_string()).collect())
}

/// Displays the list of sessions to the user.
fn display_sessions(sessions: &[String]) {
    if sessions.is_empty() {
        println!("No existing tmux sessions found.");
    } else {
        for (index, session) in sessions.iter().enumerate() {
            println!("{}) {}", index + 1, session);
        }
    }
}

/// Prompts the user to select a session or create a new one.
fn prompt_choice() -> Result<String> {
    print!("Select a session number or type 'n' to create a new session: ");
    io::stdout().flush()?;

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;

    Ok(choice.trim().to_string())
}

/// Handles creating a new session.
fn handle_new_session(existing_sessions: &HashSet<&str>) -> Result<()> {
    let full_path = match prompt_valid_path()? {
        Some(path) => path,
        None => {
            println!("Cancelled.");
            return Ok(());
        }
    };

    let suggested_name = match get_suggested_name(&full_path)? {
        Some(name) => name,
        None => {
            println!("Cancelled.");
            return Ok(());
        }
    };

    match resolve_session_name(&suggested_name, existing_sessions)? {
        Some(SessionAction::Create(session_name)) => {
            let args = vec![
                "new-session".to_string(),
                "-s".to_string(),
                session_name,
                "-c".to_string(),
                full_path.display().to_string(),
            ];
            run_status("tmux", &args).context("Failed to create new session")?;
        }
        Some(SessionAction::Attach(session_name)) => {
            attach_to_session(&session_name)?;
        }
        None => {
            println!("Cancelled.");
        }
    }

    Ok(())
}

/// Gets the suggested session name from a path.
/// If the path basename is invalid for tmux, prompts user for a name.
/// Returns None if user quits.
fn get_suggested_name(path: &std::path::Path) -> Result<Option<String>> {
    if let Some(name) = path.file_name().and_then(|n| {
        let name_str = n.to_string_lossy();
        if name_str == "." || name_str == ".." || name_str.is_empty() {
            None
        } else {
            let name = name_str.to_string();
            // Only use basename if it's a valid session name
            if validate_session_name(&name).is_none() {
                Some(name)
            } else {
                None
            }
        }
    }) {
        return Ok(Some(name));
    }

    // Path has no usable basename or basename is invalid, prompt for a valid name
    loop {
        print!("Enter a name for the new tmux session (or 'q' to quit): ");
        io::stdout().flush()?;

        let mut input_name = String::new();
        io::stdin().read_line(&mut input_name)?;

        let trimmed = input_name.trim();

        if trimmed.eq_ignore_ascii_case("q") {
            return Ok(None);
        }

        if let Some(error_msg) = validate_session_name(trimmed) {
            println!("{}", error_msg);
            continue;
        }

        return Ok(Some(trimmed.to_string()));
    }
}

/// Handles selecting an existing session by number.
fn handle_session_selection(choice: &str, sessions: &[String]) -> Result<()> {
    if sessions.is_empty() {
        println!("No sessions available. Use 'n' to create a new session.");
        return Ok(());
    }

    let choice_index: usize = match choice.parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input: '{}'. Please enter a number or 'n'.", choice);
            return Ok(());
        }
    };

    if choice_index < 1 || choice_index > sessions.len() {
        println!(
            "Invalid selection. Please choose a number between 1 and {}.",
            sessions.len()
        );
        return Ok(());
    }

    let session = &sessions[choice_index - 1];
    attach_to_session(session)
}

/// Attaches to an existing tmux session.
fn attach_to_session(session_name: &str) -> Result<()> {
    let args = vec![
        "attach".to_string(),
        "-t".to_string(),
        session_name.to_string(),
    ];
    run_status("tmux", &args).context(format!("Failed to attach to session '{}'", session_name))?;
    Ok(())
}
