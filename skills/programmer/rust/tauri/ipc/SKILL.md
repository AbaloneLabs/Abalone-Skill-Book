---
name: tauri-ipc
description: Tauri IPC (invoke commands and events) rules and pitfalls when building desktop apps with Tauri. Use when defining #[tauri::command], emitting or listening to events, passing data between the Rust backend and the web frontend, or debugging command invocation failures.
---

# Tauri IPC

Tauri bridges a Rust backend with a web frontend via commands and events. The boundary is where most bugs hide. Keep these in mind.

## Commands (`#[tauri::command]`)

### Return types must serialize
Commands return `Result<T, E>` where both `T` and `E` are `Serialize`. The frontend receives JSON. A non-serializable error type causes a confusing runtime failure.

### Errors need `serde::Serialize`
Custom error types must implement `Serialize`. The idiomatic pattern:
```rust
#[derive(Debug, thiserror::Error, serde::Serialize)]
enum AppError {
    #[error("io: {0}")]
    Io(String),
}
// map io::Error -> AppError::Io(e.to_string())
```
Don't return raw `io::Error` - it doesn't serialize cleanly.

### Accessing managed state
State, windows, and app handles are injected as arguments:
```rust
#[tauri::command]
fn cmd(state: State<AppState>, app: AppHandle) -> Result<(), AppError> { ... }
```
Order and types matter. Missing the right argument = command fails to register.

### Async commands
Long-running commands should be `async`. Otherwise the IPC call blocks the main thread and freezes the UI.

## Events

### Event name typos
Frontend `listen("update")` must exactly match backend `app.emit("update", ...)`. A typo = silent no-op. Keep event names in a shared constant or enum.

### Payload serialization
Event payloads must be `Serialize`/`Clone`. Mismatched field names between Rust struct and TS type = undefined fields on the frontend.

## Self-Check

- [ ] Do all command return types and error types implement `Serialize`?
- [ ] Are long-running commands `async` to avoid UI freezes?
- [ ] Do event names match exactly between frontend and backend?
- [ ] Are injected arguments (State, AppHandle) in the correct form?
- [ ] Are frontend TS types kept in sync with Rust structs?
