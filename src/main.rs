use std::collections::HashSet;
use std::io;
use std::io::Write;

mod fs_utils;
mod process_utils;
mod session_utils;

use fs_utils::prompt_valid_path;
use process_utils::{run_output, run_status};
use session_utils::resolve_session_name;

/// Entry point for the tmux session manager CLI.
///
/// This tool lists existing tmux sessions and allows the user to:
/// 1. Attach to an existing session.
/// 2. Create a new session from a specified directory.
fn main() {
    let args = vec!["ls".to_string(), "-F".to_string(), "#S".to_string()];

    match run_output("tmux", &args) {
        Ok(output) => {
            match std::str::from_utf8(&output.stdout) {
                Ok(result) => {
                    let lines: Vec<&str> = result.lines().collect();
                    let count = lines.len();
                    let existing_sessions: HashSet<&str> = lines.iter().copied().collect();

                    if count > 0 {
                        lines.iter().enumerate().for_each(|(index, line)| {
                            println!("{}) {}", index + 1, line);
                        });
                    } else {
                        println!("No existing tmux sessions found.");
                    }

                    print!("Select a session number or type 'n' to create a new session: ");
                    io::stdout().flush().expect("Failed to flush stdout");

                    let mut choice = String::new();

                    io::stdin()
                        .read_line(&mut choice)
                        .expect("Failed to read input");

                    let trimmed_choice = choice.trim();

                    if trimmed_choice.eq_ignore_ascii_case("n") {
                        let full_path = prompt_valid_path();

                        let suggested_name = match full_path.file_name().and_then(|n| {
                            let name_str = n.to_string_lossy();

                            if name_str == "." || name_str == ".." || name_str.is_empty() {
                                None
                            } else {
                                Some(name_str.to_string())
                            }
                        }) {
                            Some(name) => name,
                            None => {
                                print!("Enter a name for the new tmux session: ");

                                io::stdout().flush().expect("Failed to flush stdout");

                                let mut input_name = String::new();

                                io::stdin()
                                    .read_line(&mut input_name)
                                    .expect("Failed to read input");

                                let trimmed = input_name.trim();

                                if trimmed.is_empty() {
                                    println!("Session name cannot be empty.");
                                    return;
                                }
                                trimmed.to_string()
                            }
                        };

                        match resolve_session_name(&suggested_name, &existing_sessions) {
                            Some(session_name) => {
                                let args = vec![
                                    "new-session".to_string(),
                                    "-s".to_string(),
                                    session_name,
                                    "-c".to_string(),
                                    full_path.display().to_string(),
                                ];

                                match run_status("tmux", &args) {
                                    Ok(_status) => (),
                                    Err(e) => panic!("Failed to start session: {}", e),
                                }
                            }
                            None => {
                                let attach_args =
                                    vec!["attach".to_string(), "-t".to_string(), suggested_name];

                                match run_status("tmux", &attach_args) {
                                    Ok(_status) => (),
                                    Err(e) => panic!("Failed to attach: {}", e),
                                }
                            }
                        }

                        return;
                    }

                    let choice_index: usize = match trimmed_choice.parse::<usize>() {
                        Ok(result) => result,
                        Err(error) => {
                            println!("error: {}", error);
                            count + 1
                        }
                    };

                    if choice_index > count || choice_index < 1 {
                        println!("You didn't select an appropriate choice");
                    } else {
                        let session = lines[choice_index - 1].to_string();
                        let attach_args = vec!["attach".to_string(), "-t".to_string(), session];

                        match run_status("tmux", &attach_args) {
                            Ok(_status) => (),
                            Err(error) => panic!("error: {}", error),
                        };
                    }
                }
                Err(error) => panic!("error: {}", error),
            }

            match std::str::from_utf8(&output.stderr) {
                Ok(result) => println!("{}", result),
                Err(error) => println!("{}", error),
            }
        }
        Err(error) => panic!("error: {}", error),
    };
}
