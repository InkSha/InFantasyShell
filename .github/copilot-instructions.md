# Copilot instructions

## Build, test, and lint commands

- `cargo build`
- `cargo run`
- `cargo test`
- `cargo test <test_name_or_pattern>`
  - Examples: `cargo test splits_command_and_arguments`, `cargo test echo_redirection_writes_into_vfs`
- `cargo fmt`
- `cargo fmt -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`

## High-level architecture

- `src/main.rs` is just the Tokio entrypoint; it boots `cmd::Cmd` and hands off to the interactive shell loop.
- `src/cmd.rs` is the shell front end: it enables raw terminal mode, reads keystrokes through `cmd::terminal`, parses input, dispatches registered commands, prints `CommandOutput`, and refreshes the prompt after every command.
- `src/cmd/runtime.rs` builds `ShellState`, which is the session object shared by every command. It owns the current actor, `cwd`, `home`, and the full `System`.
- `src/system.rs` groups CPU, memory, storage, and network, but the implemented game state lives in `system::storage`. That storage layer is the in-memory virtual filesystem plus permissions, path resolution, node metadata, and recursive size accounting.
- `src/system/storage.rs` seeds the playable world in `Storage::new_mvp_world()`. The default shell session starts at `/home/player` and the initial world includes player files plus gameplay nodes like `/player/memory`, `/monster/slime/hp`, and `/monster/slime/ai.sh`.
- `src/cmd/inbuilt/*.rs` contains the built-in shell commands. These handlers are intentionally thin and mostly delegate to storage APIs such as `change_dir`, `list_dir`, `read_file`, `write_file`, `chmod`, and `remove`.
- `TODO.md` is the implementation roadmap. It shows which shell/game systems are already in scope and which shell features are explicitly deferred, such as full piping, full redirection, and background jobs.

## Key conventions

- Keep command modules thin. If behavior changes the world state, the canonical implementation point is usually `system::storage`, not the terminal layer.
- The parser is intentionally minimal: `cmd::parser::parse()` only splits on whitespace. Shell syntax is not modeled centrally yet, so command-specific behavior can live in handlers; for example, `echo` interprets `>` itself and performs file writes directly through storage.
- Startup state is part of product behavior. `ShellState::new("player")` always seeds a fresh world and places the user in `/home/player`, so changes to onboarding content or default world layout belong in `Storage::new_mvp_world()`.
- Permissions are enforced inside storage, not in command handlers. Directory traversal/list/write and file read/write checks all happen in the storage layer, and `chmod` is limited to the node owner.
- Paths are resolved relative to `cwd` with support for `/`, `.`, and `..`. Prompt rendering also depends on `storage.absolute_path()`, so path-resolution changes affect both command behavior and the visible prompt.
- The current tests live next to the modules they cover, mainly under `src/cmd/**`. When adding command behavior, follow that pattern instead of introducing a separate test layout unless the change truly crosses modules.
- Also follow `.github/instructions/rust.instructions.md` for Rust edits: this repository is on Rust 2024, expects `cargo fmt`, and treats `cargo clippy --all-targets --all-features -- -D warnings` as the lint baseline.
