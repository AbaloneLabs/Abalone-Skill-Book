---
name: web-async-handlers
description: Rust web server (axum, actix-web) async handler rules and pitfalls. Use when writing route handlers, middleware, or background tasks in Rust web frameworks - especially around blocking calls, state sharing, and request-scoped data.
---

# Web Async Handlers

Rust web frameworks (axum, actix-web) run on async runtimes. Blocking the runtime starves all other requests. Keep these reminders in mind.

## Never Block the Runtime

```rust
// BAD: blocks the executor thread, stalls all requests
async fn handler() {
    std::thread::sleep(Duration::from_secs(1));
}

// GOOD: yields to the runtime
async fn handler() {
    tokio::time::sleep(Duration::from_secs(1)).await;
}
```
Any synchronous blocking call (std sleep, blocking file IO, CPU loops, `std::sync::Mutex` contention) inside an async handler degrades the whole server. Offload with `tokio::task::spawn_blocking`.

## State Sharing

Use framework-provided state extraction, not globals:
```rust
// axum
async fn handler(State(db): State<DbPool>) { ... }
```
- Wrap shared mutable state in `Arc` (the framework clones it per request).
- For connection pools, share an `Arc<Pool>`.

## Common Traps

### Holding a DB connection across an await that may never resolve
Checkout a connection, do the work, return it. Don't hold it while awaiting an external service that might hang - you exhaust the pool.

### Extractor order in axum
Body-consuming extractors (`Json`, `Bytes`) must come last in the handler signature. Putting `State` after `Json` causes confusing "body was already consumed" errors.

### Spawning tasks that outlive the request
`tokio::spawn` inside a handler creates a detached task. If it references request data, it may outlive the request. Use request-scoped task management or `JoinSet`.

## Self-Check

- [ ] Are there no blocking calls inside async handlers?
- [ ] Is shared state passed via framework extractors (Arc-wrapped)?
- [ ] Are DB connections released promptly, not held across risky awaits?
- [ ] In axum, are body-consuming extractors last in the signature?
- [ ] Are spawned background tasks tracked, not fire-and-forget?
