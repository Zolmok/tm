# Tmux Session Manager CLI

A simple, interactive CLI tool to manage your [`tmux`](https://github.com/tmux/tmux) sessions with ease.

This tool allows you to:

* View all current `tmux` sessions.
* Attach to any existing session.
* Create a new named session based on a directory path.
* Automatically resolve directory typos by showing the deepest valid path.
* Avoid session name conflicts by detecting existing sessions and prompting intelligently.

---

## 📦 Features

* **Session Listing**: Displays all current `tmux` sessions in a numbered list.
* **Interactive Mode**:

  * Type a number to attach to an existing session.
  * Type `n` to create a new session from a chosen directory.
* **Path Validation**: Helps detect the deepest valid path if a typo is made.
* **Name Collision Handling**: Detects name collisions and allows you to change the name or attach to the existing session.

---

## 🚀 Usage

```bash
cargo run
```

Follow the interactive prompts to attach to or create a session.

---

## 🛠 Dependencies

* [tmux](https://github.com/tmux/tmux) must be installed and available on your `$PATH`.
* Rust (latest stable).

---

## 🧹 Project Structure

```bash
src/
├── main.rs             # CLI entry point and session manager
├── process_utils.rs    # Shell command execution utilities
├── fs_utils.rs         # Directory path validation logic
└── session_utils.rs    # Session name handling and conflict resolution
```

---

## 🔒 License

MIT License © 2025

