use std::collections::HashSet;
use std::io::{self, Write};

/// Prompt for session name, handling collisions.
pub fn resolve_session_name(suggested: &str, existing: &HashSet<&str>) -> Option<String> {
    loop {
        println!("Suggested session name: \"{}\"", suggested);
        print!("Press Enter to accept or enter a custom session name: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let candidate = {
            let trimmed = input.trim();

            if trimmed.is_empty() {
                suggested.to_string()
            } else {
                trimmed.to_string()
            }
        };

        if existing.contains(candidate.as_str()) {
            print!(
                "Session \"{}\" already exists. Attach (a) or change name (c)? ",
                candidate
            );
            io::stdout().flush().expect("Failed to flush stdout");

            let mut choice = String::new();

            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read input");

            match choice.trim().to_lowercase().as_str() {
                "a" => return None,
                "c" => continue,
                _ => {
                    println!("Invalid choice.");
                    continue;
                }
            }
        } else {
            return Some(candidate);
        }
    }
}
