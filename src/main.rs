use std::io;
use std::io::Write;

extern crate scuttle;

fn main() {
    // list the available tmux sessions
    // tmux ls -F "#S"
    let tmux_list_sessions = scuttle::App {
        command: String::from("tmux"),
        args: vec!["ls".to_string(), "-F".to_string(), "#S".to_string()],
    };

    match scuttle::run_output(&tmux_list_sessions) {
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

                        let choice_index: usize = match choice.trim().parse::<usize>() {
                            Ok(result) => result,
                            Err(error) => {
                                println!("error: {}", error);
                                // return something out of bounds so the `if` below fails
                                count + 1
                            }
                        };

                        if choice_index > count || choice_index < 1 {
                            println!("You didn't select an appropriate choice");
                        } else {
                            // we need the actual session name associated with the choice the user made
                            let session = lines[choice_index - 1].to_string();
                            // attach to the session that was chosen
                            // tmux attach -t <session>
                            let tmux_attach = scuttle::App {
                                command: String::from("tmux"),
                                args: vec!["attach".to_string(), "-t".to_string(), session],
                            };

                            match scuttle::run_status(&tmux_attach) {
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
