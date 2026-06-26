---
name: security-input-validation
description: Rust input validation and injection prevention rules. Use when handling user input, building SQL queries, processing file paths, deserializing untrusted data, or sanitizing output in Rust applications.
---

# Input Validation

Untrusted input is the root of most vulnerabilities. Rust's type safety helps, but it does not validate semantics. Keep these in mind.

## Validate at the Boundary

Parse and validate input the moment it enters the system (HTTP handler, CLI arg parser, IPC command). Convert it into a validated type. Downstream code can then trust it.

```rust
// Validate, then pass a typed, trusted value downstream
struct Username(String);
impl Username {
    fn parse(s: &str) -> Result<Self, ValidationError> {
        if s.len() > 64 || !s.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(ValidationError::Invalid);
        }
        Ok(Self(s.to_string()))
    }
}
```

## Injection Vectors

### SQL injection
Never format user input into a query string. Always use parameterized queries:
```rust
// BAD
let q = format!("SELECT * FROM users WHERE name = '{}'", input);
// GOOD (sqlx)
sqlx::query("SELECT * FROM users WHERE name = $1").bind(input)
```

### Path traversal
User-provided paths can escape intended directories (`../../etc/passwd`). Canonicalize and verify the path stays within the allowed root:
```rust
let full = base_dir.join(user_path);
let canonical = full.canonicalize()?;
if !canonical.starts_with(&base_dir) { return Err(...); }
```

### Command injection
Never pass user input to `Command::new("sh").arg("-c").arg(user_input)`. Pass arguments as separate args to avoid shell interpretation.

### Deserialization attacks
`serde_json::from_str` of untrusted data is generally safe, but beware:
- Deeply nested structures can cause stack overflows or DoS.
- Formats like `serde_yaml` or `bincode` with untrusted input need size limits.

## Common Traps

### Trusting types instead of values
A `String` typed as "email" in code is not validated unless you actually validate it. The type system won't catch `"; DROP TABLE"`.

### Integer overflow from untrusted input
Use checked arithmetic or saturating ops on sizes/lengths derived from input. A crafted length field can overflow.

### Unbounded resource consumption
No limits on request body size, recursion depth, or allocation size = DoS vector. Set explicit caps.

## Self-Check

- [ ] Is all untrusted input validated at the boundary into trusted types?
- [ ] Are all SQL queries parameterized (no string formatting)?
- [ ] Are file paths canonicalized and confined to an allowed root?
- [ ] Are command arguments passed separately (no shell interpolation)?
- [ ] Are there explicit size/depth limits on untrusted payloads?
