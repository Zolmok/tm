use anyhow::Result;
use std::collections::HashSet;
use std::io::{self, Write};

/// Result of session name resolution.
pub enum SessionAction {
    /// Create a new session with this name
    Create(String),
    /// Attach to an existing session with this name
    Attach(String),
}

/// Validates a tmux session name.
/// Returns an error message if invalid, None if valid.
pub fn validate_session_name(name: &str) -> Option<&'static str> {
    if name.is_empty() {
        return Some("Session name cannot be empty.");
    }

    if name.contains(':') {
        return Some("Session name cannot contain colons (:).");
    }

    if name.contains('.') {
        return Some("Session name cannot contain periods (.).");
    }

    if name.starts_with(' ') || name.ends_with(' ') {
        return Some("Session name cannot start or end with spaces.");
    }

    None
}

/// Prompt for session name, handling collisions and validation.
/// Returns the action to take (create new or attach to existing), or None if user quits.
pub fn resolve_session_name(suggested: &str, existing: &HashSet<&str>) -> Result<Option<SessionAction>> {
    loop {
        println!("Suggested session name: \"{}\"", suggested);
        print!("Press Enter to accept, enter a custom name, or 'q' to quit: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let trimmed = input.trim();

        if trimmed.eq_ignore_ascii_case("q") {
            return Ok(None);
        }

        let candidate = if trimmed.is_empty() {
            suggested.to_string()
        } else {
            trimmed.to_string()
        };

        // Validate the session name
        if let Some(error_msg) = validate_session_name(&candidate) {
            println!("{}", error_msg);
            continue;
        }

        if existing.contains(candidate.as_str()) {
            print!(
                "Session \"{}\" already exists. Attach (a), change name (c), or quit (q)? ",
                candidate
            );
            io::stdout().flush()?;

            let mut choice = String::new();
            io::stdin().read_line(&mut choice)?;

            match choice.trim().to_lowercase().as_str() {
                "a" => return Ok(Some(SessionAction::Attach(candidate))),
                "c" => continue,
                "q" => return Ok(None),
                _ => {
                    println!("Invalid choice.");
                    continue;
                }
            }
        } else {
            return Ok(Some(SessionAction::Create(candidate)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_session_name_accepts_valid_names() {
        assert!(validate_session_name("myproject").is_none());
        assert!(validate_session_name("my-project").is_none());
        assert!(validate_session_name("my_project").is_none());
        assert!(validate_session_name("MyProject123").is_none());
        assert!(validate_session_name("a").is_none());
        assert!(validate_session_name("my project").is_none()); // spaces in middle OK
    }

    #[test]
    fn validate_session_name_rejects_empty() {
        assert_eq!(
            validate_session_name(""),
            Some("Session name cannot be empty.")
        );
    }

    #[test]
    fn validate_session_name_rejects_colons() {
        assert_eq!(
            validate_session_name("my:project"),
            Some("Session name cannot contain colons (:).")
        );
        assert_eq!(
            validate_session_name(":start"),
            Some("Session name cannot contain colons (:).")
        );
        assert_eq!(
            validate_session_name("end:"),
            Some("Session name cannot contain colons (:).")
        );
    }

    #[test]
    fn validate_session_name_rejects_periods() {
        assert_eq!(
            validate_session_name("my.project"),
            Some("Session name cannot contain periods (.).")
        );
        assert_eq!(
            validate_session_name(".hidden"),
            Some("Session name cannot contain periods (.).")
        );
        assert_eq!(
            validate_session_name("file.txt"),
            Some("Session name cannot contain periods (.).")
        );
    }

    #[test]
    fn validate_session_name_rejects_leading_spaces() {
        assert_eq!(
            validate_session_name(" myproject"),
            Some("Session name cannot start or end with spaces.")
        );
        assert_eq!(
            validate_session_name("  myproject"),
            Some("Session name cannot start or end with spaces.")
        );
    }

    #[test]
    fn validate_session_name_rejects_trailing_spaces() {
        assert_eq!(
            validate_session_name("myproject "),
            Some("Session name cannot start or end with spaces.")
        );
        assert_eq!(
            validate_session_name("myproject  "),
            Some("Session name cannot start or end with spaces.")
        );
    }

    #[test]
    fn validate_session_name_rejects_both_leading_and_trailing_spaces() {
        assert_eq!(
            validate_session_name(" myproject "),
            Some("Session name cannot start or end with spaces.")
        );
    }
}
