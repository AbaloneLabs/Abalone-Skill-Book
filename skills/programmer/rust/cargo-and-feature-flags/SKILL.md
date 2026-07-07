---
name: rust_cargo_and_feature_flags.md
description: Use when the agent is configuring a Rust project's Cargo.toml, designing feature flags and conditional compilation (cfg attributes), writing or reviewing build.rs build scripts, choosing dependency versions and optional dependencies, reasoning about feature unification across a dependency graph, enabling features per-target or per-platform, debugging "feature X is required but not enabled" or cyclic dependency errors, or deciding how to expose optional functionality without bloating compile times. Covers feature design, cfg gating, the feature unification problem, build script obligations, edition selection, and the tradeoff between compile-time configurability and combinatorial explosion.
---

# Cargo And Feature Flags

Cargo is Rust's build system and package manager, and feature flags are its mechanism for conditional compilation. Features let one crate serve many configurations: optional dependencies, platform-specific code, heavy vs light builds, and std vs no-std. The judgment problem is designing features that compose correctly across a dependency graph, do not explode into untestable combinations, and do not silently change behavior in ways consumers do not expect.

Agents tend to add features reactively, gate code with `cfg` without testing both branches, and assume features are additive when they are actually subtractive or behavior-changing. The harm appears as "works on my machine" bugs where a downstream dependency's feature flag enables code that the author never compiled, feature unification that pulls in heavy dependencies a user tried to avoid, and build scripts with side effects that make builds non-reproducible. The real work is treating features as a public API, keeping them additive, testing the matrix, and making build scripts deterministic.

## Core Rules

### Treat Features As Additive And Composable

The single most important convention in the Rust ecosystem is that features should be **additive**: enabling a feature adds capability; it should not remove or fundamentally change existing behavior. This matters because of feature unification — when multiple crates in a graph depend on your crate with different features, Cargo enables the **union** of all requested features for the single compiled copy.

- Additive features compose safely under unification. If crate A enables `json` and crate B enables `yaml`, your crate compiles with both, and both work.
- Subtractive or mutually exclusive features break under unification. If `backend-a` and `backend-b` are features that conflict, and two dependents each pick one, the unified build has both enabled and may fail to compile or behave wrongly.
- If you truly need mutually exclusive options, you cannot express that with features alone. Use separate crates, an enum selected at runtime, or a build-time constant the user sets explicitly.

Design every feature as "enabling this adds something." If you find yourself wanting a feature that switches behavior, reconsider the design.

### Gate Code With `cfg`, And Test Both Branches

Conditional compilation via `#[cfg(feature = "...")]` and `#[cfg(target_os = "...")]` removes code at compile time. Every gated branch is a separate program that must compile and pass tests.

- A feature-gated module that has not been compiled is not verified. CI must build with the feature on and off (and with `--no-default-features`) to catch rot.
- Platform-gated code (`#[cfg(windows)]`) must be tested on the target platform or at least compiled via cross-checks. Code that only exists on one OS silently rots on the others.
- Use `cfg!` macros and `#[cfg]` attributes consistently; do not mix runtime checks for things that are compile-time constants.

The trap is gating a file behind a feature, never building without it, and shipping a release where the no-feature path does not compile. Test the matrix you claim to support.

### Design Default Features Carefully

`default = [...]` lists features enabled when the consumer does nothing special. Defaults shape the out-of-the-box experience and the compile-time footprint.

- Default features should give a sensible, working experience for the majority of users.
- Heavy or niche features should be opt-in, not default, so a user who only needs the core does not pay for them.
- Be cautious changing defaults in a minor release: a user who relied on `--no-default-features` semantics or assumed a feature was always on may break. Document default changes as breaking-adjacent.

### Manage Dependencies: Versions, Optional, And Dev

Cargo distinguishes regular dependencies, optional dependencies (tied to features), and dev-dependencies (only for tests/benches/examples).

- **Optional dependencies** (`[dependencies] foo = { version = "...", optional = true }`) create an implicit feature named `foo`. Gate the code that uses them with `#[cfg(feature = "foo")]`.
- **Dev-dependencies** (`[dev-dependencies]`) are only available to tests, benchmarks, and examples, never to the library or binary itself. Putting a test-only crate in `[dependencies]` bloats every consumer's build.
- **Version specifiers** use SemVer ranges. For libraries, default to a caret range (`^1.2`) that allows compatible patch/minor updates. For applications with a lockfile, the range matters less but should still be honest about compatibility.
- Avoid `*` (any version) and overly broad ranges; they invite surprising upgrades and conflicts.

### Keep Build Scripts Deterministic And Minimal

`build.rs` runs before compilation and can generate code, set `cfg` values, and locate system libraries. Build scripts are powerful and dangerous because they execute arbitrary code during the build.

- A build script's output must be deterministic for its inputs. If it prints different `cargo:rerun-if-changed` or generated code based on time, environment, or network state, builds become non-reproducible and caching breaks.
- Emit `cargo:rerun-if-changed=PATH` for any input the script reads, so Cargo knows when to rerun it. Otherwise Cargo uses a conservative heuristic.
- Avoid network access and side effects in build scripts. They make offline builds and reproducible packaging impossible.
- Prefer `build.rs` only when you genuinely need to generate code or probe the system; many needs are better met with a feature flag or a proc-macro.

### Pin The Edition Deliberately

The edition (`edition = "2021"`) controls which language features and default behaviors are active. Editions are opt-in and backward compatible at the language level, but they change defaults (e.g., prelude contents, closure capture semantics). Choose the latest stable edition for new projects, and treat an edition bump as a coordinated migration, not a silent change.

## Common Traps

### Mutually Exclusive Features That Unify Badly

Two features that select different backends will both be enabled when two dependents request different ones, leading to compile errors or wrong runtime behavior. Use runtime selection or separate crates instead.

### Feature-Gated Code That Never Compiles

A branch gated behind a feature that CI never builds will rot. Always test `--no-default-features` and each documented feature combination.

### Optional Dependency Without The Matching `cfg`

Adding `optional = true` to a dependency creates a feature, but the code using that dependency must be gated with `#[cfg(feature = "...")]`. Forgetting the gate produces "unresolved import" errors when the feature is off.

### Dev-Dependency Leaking Into The Library

A crate only needed for tests placed in `[dependencies]` forces every consumer to compile and download it. Move test-only crates to `[dev-dependencies]`.

### Non-Deterministic Build Script

A `build.rs` that reads the clock, fetches from the network, or depends on untracked files breaks reproducible builds and CI caching. Make outputs depend only on declared inputs.

### Over-Broad Version Range

`foo = "*"` or a very wide range invites a future incompatible release into the build. Use caret ranges that match the API stability you rely on.

### Feature That Changes Public API Silently

A feature that adds or removes public items changes downstream code depending on whether it is enabled. Document such features prominently and prefer additive additions.

## Self-Check

- [ ] Every feature is additive; no two features are mutually exclusive in a way that breaks under unification.
- [ ] CI builds and tests the crate with default features, with `--no-default-features`, and with each documented feature combination.
- [ ] Optional dependencies are paired with `#[cfg(feature = "...")]` gates on the code that uses them.
- [ ] Test-only crates live in `[dev-dependencies]`, not `[dependencies]`.
- [ ] Default features give a sensible out-of-the-box experience without pulling in heavy optional code.
- [ ] The build script (if any) is deterministic, declares its inputs via `rerun-if-changed`, and performs no network or side-effecting operations.
- [ ] Version specifiers are honest about compatibility and avoid `*`.
- [ ] Platform-conditional code (`cfg(target_os = ...)`) is compiled or tested on each target it claims to support.
- [ ] Feature-gated public API changes are documented as part of the crate's compatibility story.
