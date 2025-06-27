use std::path::PathBuf;

/// Prompt repeatedly until a valid path is provided.
/// If invalid, informs user how far the path resolves.
pub fn prompt_valid_path() -> PathBuf {
    use std::io::{self, Write};

    loop {
        print!("Enter directory path for new session: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut dir_path = String::new();

        io::stdin()
            .read_line(&mut dir_path)
            .expect("Failed to read input");

        let full_path = PathBuf::from(dir_path.trim());

        if full_path.exists() {
            return full_path;
        }

        let mut current = PathBuf::from("/");
        let mut last_good = current.clone();

        for part in full_path.components().skip(1) {
            current = current.join(part);

            if current.exists() {
                last_good = current.clone();
            } else {
                println!("Invalid path. Valid up to: {}", last_good.display());
                break;
            }
        }
    }
}
