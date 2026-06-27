---
name: rust_safety_boundaries.md
description: Use when the agent is writing or reviewing Rust code involving ownership, borrowing, lifetimes, error propagation, unsafe blocks, cryptography, concurrency primitives, FFI, shared state, or low-level API boundaries.
---

# Rust Safety Boundaries

Rust prevents many classes of bugs, but it does not remove the programmer's responsibility to design boundaries. Ownership, borrowing, error handling, cryptography, concurrency, and unsafe code all become safer only when the code clearly states who owns data, when mutation is allowed, what failure means, and which invariants the compiler cannot check.

Use this skill before changing non-trivial Rust modules, especially around shared state, async servers, FFI, serialization, cryptographic material, unsafe blocks, error conversion, background tasks, or APIs that other modules will depend on.

## Core Rules

### Design Ownership Around Responsibility

Ownership should follow responsibility, not convenience. The owner of a value should be the component responsible for its lifecycle, mutation rules, cleanup, and invariants. Avoid cloning or wrapping in `Arc<Mutex<_>>` before deciding who should own the data.

Ask:

- Which component creates the value?
- Which component is allowed to mutate it?
- Which component must observe a stable snapshot?
- Can the value be borrowed for the duration of a call?
- Does it need shared ownership across tasks or threads?
- Does the type encode its lifecycle state?

Prefer passing references for read-only work, owned values for transfer, and narrow mutable references for controlled mutation. If many functions need mutable access to the same structure, the design may be hiding multiple responsibilities in one type.

### Keep Borrow Lifetimes Small

Long borrows make code harder to change and can force awkward clones. Limit the scope of borrows by extracting values, using small helper functions, or separating read and write phases. In async Rust, never hold a lock guard or mutable borrow across `.await` unless the type and design explicitly support that behavior.

A common pattern is:

1. read and validate input;
2. clone or copy only the small data needed for async work;
3. release locks and borrows;
4. await external work;
5. reacquire state only for the final update.

This keeps the compiler's restrictions aligned with runtime safety.

### Treat `Arc`, `Mutex`, And `RwLock` As Architecture Choices

Shared ownership and locks are not just syntax to satisfy the compiler. They define concurrency behavior, blocking behavior, poisoning behavior, and failure modes.

Before adding a shared wrapper, decide:

- whether state must be shared or can be owned by a task;
- whether mutations need serialization;
- whether a database or channel would be a better boundary;
- whether blocking locks are used inside async code;
- whether lock acquisition order can deadlock;
- whether the protected data is too broad.

Use async-aware locks in async contexts when the lock may be held across await points, but still avoid holding any lock longer than necessary. Prefer message passing, immutable snapshots, or database transactions when they express the workflow more clearly.

### Make Error Types Preserve Meaning

Rust makes it easy to propagate errors with `?`, but a propagated error is only useful if it preserves the right meaning at the boundary. Low-level errors should be enriched with context before crossing module, HTTP, CLI, job, or FFI boundaries.

Design error handling by boundary:

- internal helper errors can be precise and technical;
- module errors should describe domain failure;
- API errors should map to safe client responses;
- logs should retain detailed source context;
- retries should distinguish transient from permanent failures.

Avoid `unwrap`, `expect`, and panic paths in request handlers, workers, parsers, and library code unless panic is truly the correct contract. If a panic would become a user-visible failure or data loss, return an error instead.

### Contain Unsafe Code

`unsafe` does not turn off Rust rules; it moves proof obligations to the programmer. Every unsafe block should be small, local, documented, and wrapped by a safe API that enforces the necessary invariants.

For each unsafe block, state:

- pointer validity and alignment;
- aliasing and mutability rules;
- initialization guarantees;
- lifetime assumptions;
- thread-safety assumptions;
- ownership transfer rules;
- what external code or FFI promises.

Do not scatter unsafe operations through business logic. Build a narrow module with tests and comments explaining the invariants. If a safe crate or standard library API can express the same behavior, prefer it.

### Handle Secrets And Cryptography Conservatively

Rust type safety does not make cryptographic design safe. Do not invent encryption schemes, token formats, password hashing, random number generation, signature verification, or key rotation logic casually.

Use established crates and current algorithms. Keep secret material out of logs, debug output, panic messages, test snapshots, and serialized structs. Prefer constant-time comparisons for secrets. Separate public ids from secret tokens. Plan key loading, rotation, storage, zeroization needs, and failure behavior.

When reviewing crypto code, ask whether the code needs cryptography at all or whether a platform feature, TLS, signed cookie library, password hashing crate, or provider-managed secret would be safer.

### Encode State With Types Where Practical

Use Rust's type system to prevent invalid states. Newtypes can distinguish user ids from organization ids, public ids from database ids, raw passwords from password hashes, unchecked input from validated input, and pending records from committed records.

Enums can encode state transitions better than string flags. Result types can force failure handling. Lifetimes can prevent borrowed data from outliving its owner. Traits can narrow what a caller is allowed to do.

Do not over-abstract simple code, but use types when the cost of mixing values is high.

## Common Traps

### Fighting The Borrow Checker With Clones

Cloning can be correct, but repeated clones to silence the compiler may hide unclear ownership. Check whether a function should borrow, own, or return a transformed value. Clone small stable values deliberately; avoid cloning secrets, large buffers, or mutable state snapshots without reason.

### Holding Locks Across Await

This can cause deadlocks, priority inversions, or throughput collapse. Even when it compiles, it may serialize unrelated work. Release locks before awaiting external calls unless the lock is specifically designed for that pattern.

### Converting Every Error To String

String errors lose structure. They make retry logic, HTTP status mapping, tests, and observability weaker. Preserve typed errors until the boundary where human-readable text is needed.

### Large Unsafe Blocks

Large unsafe blocks make it unclear which operation needs unsafe and which invariants apply. Keep unsafe operations minimal and wrap them in safe functions with documented preconditions and tests.

### Assuming Send And Sync Mean Correct

A type being `Send` or `Sync` means it can cross threads under Rust's rules. It does not prove the business logic is race-free, idempotent, or deadlock-free.

### Logging Debug Structs

`#[derive(Debug)]` can expose secrets, tokens, raw input, or personal data. Review debug output for types that may appear in logs or errors.

## Self-Check

- [ ] Ownership follows lifecycle responsibility, not just compiler appeasement.
- [ ] Borrow scopes are small, and locks or mutable borrows are not held across `.await` without a deliberate reason.
- [ ] Shared state wrappers such as `Arc`, `Mutex`, and `RwLock` reflect a real concurrency design.
- [ ] Errors preserve domain meaning and source context until they reach an external boundary.
- [ ] Panics, `unwrap`, and `expect` are absent from fallible runtime paths unless panic is the intended contract.
- [ ] Unsafe code is small, documented, tested, and wrapped behind a safe API with clear invariants.
- [ ] Secrets and cryptographic operations use established crates and avoid logs, debug output, snapshots, and custom algorithms.
- [ ] Types distinguish values that should not be mixed, such as public ids, database ids, raw input, validated input, secrets, and hashes.
- [ ] Async tasks, channels, locks, and thread boundaries were reviewed for lifetime and shutdown behavior.
- [ ] The code's safety argument covers what the Rust compiler cannot prove.
