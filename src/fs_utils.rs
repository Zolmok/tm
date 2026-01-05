use anyhow::Result;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Prompt repeatedly until a valid directory path is provided.
/// Handles both absolute and relative paths.
/// If invalid, informs user how far the path resolves.
/// Enter 'q' to quit.
pub fn prompt_valid_path() -> Result<Option<PathBuf>> {
    loop {
        print!("Enter directory path for new session (or 'q' to quit): ");
        io::stdout().flush()?;

        let mut dir_path = String::new();
        io::stdin().read_line(&mut dir_path)?;

        let trimmed = dir_path.trim();

        if trimmed.eq_ignore_ascii_case("q") {
            return Ok(None);
        }

        let input_path = PathBuf::from(trimmed);

        if input_path.as_os_str().is_empty() {
            println!("Path cannot be empty.");
            continue;
        }

        // Try to canonicalize - this resolves relative paths and symlinks
        match input_path.canonicalize() {
            Ok(canonical) => {
                if canonical.is_dir() {
                    return Ok(Some(canonical));
                } else {
                    println!("Path exists but is not a directory: {}", canonical.display());
                }
            }
            Err(_) => {
                // Path doesn't exist - find the deepest valid ancestor
                show_valid_path_prefix(&input_path);
            }
        }
    }
}

/// Shows the deepest valid ancestor of a path that doesn't exist.
fn show_valid_path_prefix(path: &Path) {
    // Try to resolve what we can
    let resolved = if path.is_absolute() {
        path.to_path_buf()
    } else {
        // For relative paths, start from current directory
        match std::env::current_dir() {
            Ok(cwd) => cwd.join(path),
            Err(_) => {
                println!("Invalid path and unable to determine current directory.");
                return;
            }
        }
    };

    // Walk up the path to find the deepest valid ancestor
    let mut current = resolved.as_path();
    while let Some(parent) = current.parent() {
        if parent.exists() {
            println!("Invalid path. Valid up to: {}", parent.display());
            return;
        }
        current = parent;
    }

    println!("Invalid path: {}", path.display());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn show_valid_path_prefix_finds_valid_ancestor() {
        // Create a temp directory structure
        let temp = tempdir().unwrap();
        let valid_path = temp.path();

        // Test with a non-existent child path
        let invalid_path = valid_path.join("nonexistent").join("deep").join("path");

        // This function prints to stdout, so we just verify it doesn't panic
        show_valid_path_prefix(&invalid_path);
    }

    #[test]
    fn show_valid_path_prefix_handles_absolute_path() {
        let invalid_path = PathBuf::from("/nonexistent/path/to/nowhere");
        // Should not panic
        show_valid_path_prefix(&invalid_path);
    }

    #[test]
    fn show_valid_path_prefix_handles_relative_path() {
        let invalid_path = PathBuf::from("nonexistent/relative/path");
        // Should not panic
        show_valid_path_prefix(&invalid_path);
    }

    #[test]
    fn canonicalize_works_for_existing_directory() {
        let temp = tempdir().unwrap();
        let path = temp.path();

        // Verify the path exists and can be canonicalized
        let canonical = path.canonicalize().unwrap();
        assert!(canonical.is_dir());
    }

    #[test]
    fn canonicalize_fails_for_nonexistent_path() {
        let temp = tempdir().unwrap();
        let nonexistent = temp.path().join("does_not_exist");

        assert!(nonexistent.canonicalize().is_err());
    }

    #[test]
    fn is_dir_distinguishes_files_from_directories() {
        let temp = tempdir().unwrap();

        // Create a file
        let file_path = temp.path().join("testfile.txt");
        fs::write(&file_path, "test content").unwrap();

        // Verify file is not a directory
        assert!(!file_path.is_dir());
        assert!(file_path.is_file());

        // Verify temp directory is a directory
        assert!(temp.path().is_dir());
    }
}
