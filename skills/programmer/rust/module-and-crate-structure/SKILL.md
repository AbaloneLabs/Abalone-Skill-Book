---
name: rust_module_and_crate_structure.md
description: Use when the agent is organizing a Rust codebase into modules and crates, deciding visibility (pub, pub(crate), pub(super)), splitting a binary into a library + binary crate, designing the mod tree with mod.rs vs file-based modules, re-exporting items with pub use, structuring a Cargo workspace, choosing crate boundaries, or debugging "cannot find type/value in this scope", "X is private", or circular module dependency errors. Covers module hierarchy design, the visibility decision ladder, re-export strategy, the lib/bin split, workspace layout, and the tradeoff between one large crate and many small ones.
---

# Module And Crate Structure

Rust's module and crate system is how you organize code, control what is public, and divide a project into independently compiled units. Unlike languages where files automatically become namespaces, Rust requires you to explicitly declare the module tree and choose visibility for every item. The judgment problem is designing a structure that makes the right things reachable, hides implementation details, keeps compile times and dependency graphs sane, and does not force every consumer through a maze of paths.

Agents tend to either flatten everything into one file (losing encapsulation), or over-split into many tiny crates (exploding compile times and forcing `pub` on internals to satisfy cross-crate access). The harm appears as public APIs that accidentally expose internal types, `pub use` walls that hide everything, workspaces where a one-line change rebuilds everything, and crates that exist only to satisfy an aesthetic preference for smallness. The real work is deciding visibility deliberately, splitting at boundaries that match compilation and ownership, and re-exporting so consumers get a clean path.

## Core Rules

### Decide Visibility Deliberately, Defaulting To The Narrowest That Works

Rust visibility is a ladder, and the default (`pub` omitted entirely) is private. Move up the ladder only when you have a reason.

- **Private (default)**: visible only within the defining module and its descendants. This is where almost everything should live.
- **`pub(crate)`**: visible anywhere within the current crate, but not to external consumers. This is the right choice for internal helpers that sibling modules need.
- **`pub(super)`**: visible to the parent module only. Useful for tightly coupled parent/child pairs.
- **`pub(in path)`**: visible within a specific ancestor path. Rare, but precise when needed.
- **`pub`**: visible to everyone, including external crates. This is part of your public API and carries compatibility obligations.

The trap is reaching for `pub` to silence a "private" error. When the compiler says `field is private`, the first question is not "how do I make it public" but "should the caller really reach into this struct". Often the answer is to add a method, not to widen visibility. A crate that marks everything `pub` has no encapsulation; a crate that marks everything `pub(crate)` cannot be consumed externally. Choose per item, with the narrowest level that satisfies the actual access need.

### Build The Module Tree Explicitly, Do Not Rely On File Names

In Rust, files do not automatically become modules. You declare the tree with `mod` statements, and the compiler resolves `mod foo;` to either `foo.rs` or `foo/mod.rs`. The parent module must declare its children; orphan files are ignored.

- Prefer the modern `foo.rs` + `foo/bar.rs` layout over `foo/mod.rs` + `foo/bar.rs`. The `mod.rs` style nests and makes it harder to see the tree at a glance; `foo.rs` at the same level as the `foo/` directory is clearer.
- Declare children in the parent: a `lib.rs` or `main.rs` lists `pub mod network; pub mod storage; pub mod config;`. This top-level list is the table of contents of your crate.
- Re-export at the top to give consumers a flat path. `pub mod network;` plus `pub use network::Client;` lets users write `my_crate::Client` instead of `my_crate::network::Client`.

Agents often create a file, expect it to be importable, and are confused when it is missing. The fix is to declare the module in its parent, not to invent import paths.

### Split Binaries Into Library + Binary Crates

A common and valuable pattern is `src/lib.rs` (the library crate) plus `src/main.rs` (the binary crate) in the same package. The binary becomes a thin wrapper that calls into the library.

- The library holds all the real logic, types, and tests. The binary holds argument parsing, configuration loading, and the entry point.
- This makes the logic testable: integration tests in `tests/` can import the library crate by name, which they cannot do with a bare binary.
- It also lets you add multiple binaries (`src/bin/foo.rs`, `src/bin/bar.rs`) that share the library.

Without this split, you end up with a `main.rs` that contains thousands of lines of untestable logic, or you duplicate code across binaries. Default to the lib+bin split for anything beyond a trivial script.

### Choose Crate Boundaries By Compilation, Ownership, And Reuse

A crate is a compilation unit with its own dependency graph and public API. Deciding what is a crate vs a module is a high-stakes decision because crates are expensive to create and change.

Split into separate crates when:

- The code is independently reusable by projects that do not want the rest of your codebase (a parser, a protocol library, a CLI framework).
- The code has a different release cadence, version, or set of dependencies than the core.
- Compile times are dominated by one heavy dependency that only part of the project needs; isolating it in a crate lets the rest compile without it.

Do **not** split into a crate merely to organize files. A crate boundary forces `pub` on everything cross-crate code needs, which erodes encapsulation. If two crates are always changed together and only used together, they should probably be modules in one crate. When in doubt, start with modules and promote to a crate only when a concrete reuse or compile-time reason appears.

### Use Workspaces To Share A Lockfile And Build Context

A Cargo workspace groups multiple crates that are developed together under one `Cargo.toml` with a shared `target/` directory and `Cargo.lock`. Use a workspace when you have multiple related crates (a core library, a CLI, a server, a proc-macro) that you version and test together.

- The shared `target/` means a dependency compiled for one crate is reused by the others, cutting compile time.
- The shared lockfile keeps versions consistent across the workspace.
- Each member crate still has its own `Cargo.toml` and public API.

The trap is making the workspace too granular: dozens of one-file crates each with their own manifest. Prefer fewer, larger member crates over a sprawl of tiny ones.

### Re-Export To Provide A Stable Public Path

Consumers should depend on a stable import path, not on your internal module layout. Use `pub use` to re-export the public API from the crate root or a small number of facade modules. Then you can refactor internal module structure without breaking every `use` statement in your consumers.

- `pub use network::tcp::Client as TcpClient;` gives a stable, documented path.
- Hide internal modules by not re-exporting them; they remain `pub(crate)` or private.
- Document which paths are the supported public surface.

## Common Traps

### Marking Everything `pub` To Silence Errors

When the compiler reports a visibility error, widening to `pub` is usually the wrong fix. It leaks internals into the public API and creates compatibility obligations. Ask whether the caller should reach the item at all, and prefer a method or `pub(crate)`.

### Creating A Crate Per Directory

Splitting the codebase into many tiny crates to mirror the folder structure explodes compile times (each crate compiles separately), forces `pub` on internals, and multiplies manifest maintenance. Use modules within a crate unless there is a concrete reuse or compile reason to split.

### Forgetting To Declare Child Modules

A file `src/network/tcp.rs` is invisible until `src/network.rs` (or `src/network/mod.rs`) contains `mod tcp;`. Orphan files are silently ignored, which is confusing. Always wire children into their parent.

### Leaking Internal Types Through A Public Function

A `pub fn make_thing() -> InternalType` exposes a type you did not intend to make public. If the return type is internal, either make the function `pub(crate)`, return a public wrapper, or reconsider the API. Audit return types and parameter types for accidental exposure.

### Confusing `pub` In A `pub(crate)` Module

An item marked `pub` inside a `pub(crate)` module is still only crate-visible, because the module itself gates access. This is correct but surprising. If you intend the item to be externally public, the enclosing path must be `pub` all the way up.

### Over-Using `mod.rs`

The legacy `foo/mod.rs` layout duplicates filenames and obscures the tree. Prefer `foo.rs` alongside a `foo/` directory. Mixing both styles in one project creates inconsistency.

### Workspace Sprawl

A workspace with thirty single-file crates is harder to maintain than three well-chosen crates. Consolidate crates that always change together.

## Self-Check

- [ ] Visibility is chosen per item, defaulting to private, with `pub(crate)`/`pub(super)` used before `pub`; nothing is marked `pub` merely to silence a compiler error.
- [ ] Every source file is reachable because its parent module declares it with `mod`; there are no orphan files.
- [ ] The project uses the lib+bin split for non-trivial binaries, so logic is testable and reusable.
- [ ] Crate boundaries correspond to real reuse, release-cadence, or compile-time reasons, not to folder aesthetics.
- [ ] Workspaces group genuinely related crates and share a lockfile and target directory; there is no sprawl of tiny single-file crates.
- [ ] The public API is re-exported through stable facade paths so internal refactors do not break consumers.
- [ ] No internal type leaks through a public function signature without intent.
- [ ] The module layout uses a consistent style (preferably `foo.rs` + `foo/`), not a mix of `mod.rs` and file-based modules.
