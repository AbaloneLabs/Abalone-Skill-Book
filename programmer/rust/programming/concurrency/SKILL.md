---
name: concurrency
description: Rust concurrency safety rules and common pitfalls involving threads, async, Mutex, Arc, Send/Sync, and data races. Use when writing multi-threaded code, async code with tokio, sharing state across tasks/threads, or debugging deadlocks and data races.
---

# Concurrency

Rust's type system (`Send`/`Sync`) prevents data races at compile time - but it does NOT prevent deadlocks, logic races, or async pitfalls. Keep these in mind when writing concurrent code.

## Sharing State Across Threads

| Need | Tool |
|------|------|
| Shared read-only | `Arc<T>` where `T: Sync` |
| Shared mutable | `Arc<Mutex<T>>` or `Arc<RwLock<T>>` |
| Message passing | `mpsc` channel / `tokio::sync` |
| Once initialization | `std::sync::OnceLock` / `once_cell` |

Default to **message passing over shared state** ("do not communicate by sharing memory").

## Send and Sync

- `Send`: type can be moved to another thread safely.
- `Sync`: `&T` can be shared between threads safely.
- `Rc<T>` and `RefCell<T>` are **NOT** `Send`/`Sync` - use `Arc` and `Mutex` for thread safety.
- Forgetting this is the most common "cannot be sent between threads" error.

## Common Traps

### Deadlock from lock ordering
If two locks are acquired in different orders on different threads, deadlock is possible. Rule: **always acquire locks in a consistent global order.** Better: hold one lock at a time.

### Holding a lock across an await point
```rust
let data = mutex.lock().unwrap();
db.query().await;  // BAD: lock held while suspended
```
The task may be suspended here while holding the lock, blocking all other tasks waiting on it. **Drop locks before `.await`.** Or use `tokio::sync::Mutex` (but prefer restructuring to avoid holding across await).

### Blocking inside async context
```rust
async fn handler() {
    std::thread::sleep(Duration::from_secs(1));  // BAD: blocks the executor
    // Use tokio::time::sleep().await instead
}
```
Blocking calls (std sleep, blocking IO, CPU loops) starve the async runtime. Use `tokio::task::spawn_blocking` for CPU-heavy or blocking work.

### MutexGuard is not Send
A `std::sync::MutexGuard` cannot be sent across `.await`. This is by design. Restructure so the guard is dropped before the await.

### Spawning without JoinHandle
`thread::spawn` / `tokio::spawn` fire-and-forget. If the task panics, you won't know unless you await the `JoinHandle`. For important tasks, keep the handle and check its result.

### Forgetting to await a spawned task before exit
In async `main`, spawned tasks may be cancelled when `main` returns. Use `tokio::task::JoinSet` or explicitly await handles before exiting.

## Async vs Sync Decision

- **Async**: many I/O-bound tasks, high concurrency, network services.
- **Threads**: few CPU-bound tasks, or when simplicity matters.
Don't mix blocking std IO with tokio - it stalls the runtime.

## Self-Check

- [ ] Are locks acquired in a consistent order (no deadlock risk)?
- [ ] Are no locks held across `.await` points?
- [ ] Is blocking work offloaded to `spawn_blocking`?
- [ ] Are shared types using `Arc`/`Mutex`, not `Rc`/`RefCell`?
- [ ] Are important spawned tasks' results checked (not fire-and-forget)?
