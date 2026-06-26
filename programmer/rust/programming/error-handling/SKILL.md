---
name: error-handling
description: Rust error handling conventions and pitfalls to watch for when writing or reviewing Rust code. Use when designing error types, propagating failures with the ? operator, deciding between panic and Result, or choosing error libraries (thiserror, anyhow).
---

# Error Handling

Rust forces you to handle errors explicitly. Under deadline pressure or in deep call stacks, it's tempting to cut corners. Keep these reminders in mind.

## Core Principle

**`Result` is for expected, recoverable failures. `panic!` is for impossible states and programmer bugs.** Never use `panic!`/`unwrap()`/`expect()` for errors a caller could reasonably handle.

## When panic is acceptable

- Prototyping / throwaway code
- A true invariant violation that means the program is in an unrecoverable state
- `unwrap()` on a value you just proved is `Some`/`Ok` (document why with `expect("reason")`)

If there's any realistic path where the value could be `None`/`Err`, return a `Result` instead.

## Error Type Design

### Library code: use `thiserror`
Define a dedicated error enum. Callers can match on variants.
```rust
#[derive(Debug, thiserror::Error)]
enum DbError {
    #[error("connection failed: {0}")]
    Connection(#[from] io::Error),
    #[error("not found: {id}")]
    NotFound { id: u64 },
}
```

### Application code: use `anyhow`
For code where you just propagate errors up and don't need callers to match variants.
```rust
use anyhow::{Context, Result};
fn load_config() -> Result<Config> {
    let s = fs::read_to_string(path).context("failed to read config file")?;
    // ...
}
```

## Common Traps

### Swallowing errors silently
```rust
let _ = file.write_all(&data);  // BAD: ignoring a real failure
let _ = result;                 // BAD: why even compute it?
```
If you truly intend to ignore, use `let _ = ...` with a comment explaining why. Otherwise propagate with `?` or handle it.

### Missing context on `?`
Bare `?` loses the "where" and "what":
```rust
let f = File::open(path)?;                    // error says nothing useful
let f = File::open(path).context("open cfg")?; // error is debuggable
```
Always add `.context()` in application code so errors are traceable.

### Mixing `Option` and `Result` carelessly
- `Option`: a value is present or absent (not an error condition).
- `Result`: an operation succeeded or failed.
Don't use `Option::unwrap` where the absence is actually a failure - return `Result`.

### Forgetting to handle the `Err` arm
Exhaustive `match` forces this, but `if let` and `let Ok(_) = ...` can silently drop errors. When using `if let Ok(x) = op()`, decide deliberately what happens on `Err`.

### Box<dyn Error> in public APIs
This erases type info so callers can't match. Use a concrete error type (thiserror) in library public APIs.

## Self-Check

- [ ] Are all fallible operations returning `Result`, not panicking?
- [ ] Does every `?` in app code have `.context()`?
- [ ] Is the error type appropriate (thiserror for libs, anyhow for apps)?
- [ ] Are no errors silently dropped (`let _ =`)?
- [ ] Does the error message tell the user what failed and why?
