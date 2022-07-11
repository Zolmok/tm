use std::fmt::{self, Display, Formatter};
use std::io;
use std::io::Write;
use std::process::{Command, ExitStatus, Output};

pub struct Args(pub Vec<String>);

impl Display for Args {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0.join(" "))
    }
}

#[derive(Debug)]
pub struct App {
    pub command: String,
    pub args: Vec<String>,
}

/// Run an app and display its output
///
/// This is useful when you just want to run an app and don't care about parsing its output
///
/// # Arguments
///
/// * `app` - An app of type `App`
fn run_app<'a>(app: &App) -> Result<ExitStatus, std::io::Error> {
    let child_result = Command::new(app.command.clone())
        .args(app.args.clone())
        .spawn();
    let mut child = match child_result {
        Ok(result) => result,
        Err(error) => return Err(error),
    };

    match child.try_wait() {
        Ok(Some(status)) => Ok(status),
        Ok(None) => match child.wait() {
            Ok(result) => Ok(result),
            Err(error) => return Err(error),
        },
        Err(error) => Err(error),
    }
}

/// Run an app and return its output
///
/// This is usefull when you need to parse the return of an app
///
/// # Arguments
///
/// * `app` - An app of type `App`
fn run_app_output<'a>(app: &App) -> Result<Output, std::io::Error> {
    Command::new(app.command.clone())
        .args(app.args.clone())
        .output()
}

/// Parse the output of `tmux list-sessions` and create a menu from which to choose one
///
/// # Arguments
///
/// * `app` - An app of type `App`
fn run_with_output(app: App) {
    match run_app_output(&app) {
        Ok(output) => {
            match std::str::from_utf8(&output.stdout) {
                Ok(result) => {
                    // lines will be the list of tmux sessions
                    let count = result.lines().count();
                    let lines: Vec<&str> = result.lines().collect();

                    if count > 0 {
                        // print the sessions with an index from which to choose (1 based)
                        result.lines().enumerate().for_each(|(index, line)| {
                            println!("{}) {}", index + 1, line);
                        });

                        print!("$ ");
                        // `print!` doesn't output until we do this
                        match io::stdout().flush() {
                            Ok(_result) => (),
                            Err(error) => panic!("error: {}", error)
                        };

                        let mut choice = String::new();
                        let stdin = io::stdin();

                        match stdin.read_line(&mut choice) {
                            Ok(_result) => (),
                            Err(error) => panic!("error: {}", error),
                        };

                        let choice_index = match choice.trim().parse::<usize>() {
                            Ok(result) => result,
                            Err(error) => {
                                println!("error: {}", error);
                                // return something out of bounds so the `if` below fails
                                count + 1
                            }
                        };

                        if choice_index > count || choice_index < count {
                            println!("You didn't select an appropriate choice");
                        } else {
                            // we need the actual session name associated with the choice the user made
                            let session = lines[choice_index - 1].to_string();
                            // attach to the session that was chosen
                            // tmux attach -t <session>
                            let tmux_attach = App {
                                command: String::from("tmux"),
                                args: vec!["attach".to_string(), "-t".to_string(), session],
                            };

                            match run_app(&tmux_attach) {
                                Ok(_status) => (),
                                Err(error) => panic!("error: {}", error),
                            };
                        }
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

fn main() {
    // list the available tmux sessions
    // tmux ls -F "#S"
    let tmux_list_sessions = App {
        command: String::from("tmux"),
        args: vec!["ls".to_string(), "-F".to_string(), "#S".to_string()],
    };

    run_with_output(tmux_list_sessions);
}
