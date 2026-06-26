---
name: ownership-and-borrowing
description: Rust ownership, borrowing, and lifetime rules to keep in mind when writing or reviewing Rust code. Use when writing functions, structs, traits, or any code where values are passed, stored, or shared - especially when the borrow checker rejects code or when deciding between references, clones, and ownership transfers.
---

# Ownership and Borrowing

Rust's ownership system prevents memory bugs at compile time. When juggling multiple concerns in complex code, it's easy to violate these rules. Keep the following in mind.

## Core Rules (never violate)

1. **Each value has exactly one owner.** When the owner goes out of scope, the value is dropped.
2. **At any given time, you can have EITHER:**
   - One mutable reference (`&mut T`), OR
   - Any number of immutable references (`&T`)
   - Never both at once.
3. **References must always outlive what they point to.** No dangling references.

## Common Traps

### Cloning to silence the borrow checker
Before reaching for `.clone()`, ask: is this a real ownership need, or am I working around a structural problem? Cloning large data in a hot loop is a silent performance killer. Prefer restructuring borrows, but clone when ownership is genuinely required (e.g., spawning a thread, storing into a collection).

### Mutable borrow held too long
```rust
let mut v = vec![1, 2, 3];
let first = &mut v[0];   // mutable borrow starts
v.push(4);               // ERROR: borrow still alive
*first += 1;
```
Solution: scope borrows tightly. Drop references before reusing the owner.

### Iterating while mutating
Cannot hold an iterator over a collection while mutating it. Collect indices or clone first:
```rust
// Bad: modifying while iterating
// Good: collect what needs changing, then mutate
let to_remove: Vec<_> = items.iter().filter(|x| x.dead).collect();
```

### Lifetime mismatches in structs and function signatures
When a struct holds a reference, it needs a lifetime parameter. This propagates virally and constrains usage. Ask: does this struct really need to borrow, or should it own (`String`, `Vec`, `Box`)?

- **Default to owning** in structs unless there's a clear performance reason to borrow.
- `&str` in function args is idiomatic and flexible; `String` forces callers to give up ownership.

### Closures capturing by reference vs move
- Closures capture by reference by default.
- `move` forces ownership transfer - required for `thread::spawn` and returning closures.
- Forgetting `move` when sending to a thread causes lifetime errors.

## Decision Guide

| Situation | Prefer |
|-----------|--------|
| Short-lived read inside a function | `&T` |
| Need to modify in place | `&mut T` |
| Storing, sending to thread, or returning | Own / `clone()` / `Arc` |
| Shared read-only across threads | `Arc<T>` (T: Sync) |
| Shared mutable across threads | `Arc<Mutex<T>>` or `Arc<RwLock<T>>` |

## Self-Check Before Compiling

- [ ] Does every `&mut` have exclusive access (no overlapping borrows)?
- [ ] Are references guaranteed to outlive the data they point to?
- [ ] Is each `.clone()` justified, not a borrow-checker workaround?
- [ ] Do structs own their data unless borrowing is clearly better?
- [ ] Are borrows scoped as tightly as possible?
