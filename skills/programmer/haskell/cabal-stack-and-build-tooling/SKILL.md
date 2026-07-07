---
name: haskell_cabal_stack_and_build_tooling.md
description: Use when the agent is setting up or configuring a Haskell build (cabal cabal.project, stack stack.yaml, Cabal .cabal file, HLS/ghcid for IDE, Nix for reproducible builds), managing dependencies and bounds (upper bounds, allow-newer, PVP bounds), building executables/libraries/tests/benchmarks, working with multiple GHC versions, profiling builds, dealing with dependency-conflict / module cycles / Cabal hell, or is diagnosing "dependency version conflict / cannot install", "cabal repl vs stack ghci", "upper bounds too tight / too loose", "build not reproducible", or build-tooling confusion. Covers cabal vs stack vs Nix, the Cabal file structure, dependency bounds and PVP, multi-package projects, HLS/ghcid, profiling builds, and the traps of bound conflicts, reproducibility, and tooling choice paralysis.
---

# Cabal, Stack, And Build Tooling In Haskell

Haskell build tooling has three main options — `cabal` (the official, with `cabal.project` and the `v2-` commands), `stack` (curated snapshot-based, reproducible), and Nix (fully reproducible, steep learning curve) — plus the IDE tooling (HLS, ghcid). Agents confuse cabal and stack (different dependency resolution: cabal resolves from Hackage with bounds; stack uses curated snapshots), set dependency bounds wrong (PVP upper bounds too tight block installation, too loose allow build-breaking changes), hit "Cabal hell" (conflicting transitive dependencies), or fail to make builds reproducible (no pinned versions/snapshots). The judgment problem is to choose the tool by project needs, to set dependency bounds deliberately per the PVP, to manage multi-package projects, and to ensure reproducibility.

Agents hit dependency conflicts, set wrong bounds, or mix cabal and stack confusingly. The remedy is to pick one tool, set PVP-correct bounds, pin for reproducibility, and use the IDE tooling.

## Core Rules

### Choose cabal, stack, Or Nix By Project Needs

- **cabal** (with `cabal.project`): the official tool, resolves dependencies from Hackage against bounds. Best for libraries published to Hackage, projects wanting the latest GHC/packages, and those comfortable managing bounds. Use `cabal build`/`cabal run`/`cabal repl`/`cabal test` (the `v2-` commands are the default now).
- **stack**: uses curated *snapshots* (Stackage, a tested set of compatible package versions), giving reproducible builds out of the box. Best for applications, teams wanting stability, and reproducibility without Nix. Use `stack build`/`stack exec`/`stack ghci`/`stack test`.
- **Nix**: fully reproducible (pinned nixpkgs), composes with cabal/stack (`haskell.nix`, `cabal2nix`). Best for reproducible CI/deployment and polyglot repos, at the cost of a learning curve.

For a new library, cabal is idiomatic (Hackage-bound). For an application wanting stability, stack. For reproducible ops, Nix. Do not mix cabal and stack resolution in one project (pick one).

- cabal: official, Hackage resolution, for libraries/latest packages.
- stack: curated snapshots, reproducible, for applications/stability.
- Nix: full reproducibility, for CI/ops, steep curve. Pick one tool per project.

### Set Dependency Bounds Deliberately Per The PVP

The Haskell Package Versioning Policy (PVP) uses `major.major.minor.patch` where the first *two* components are the "major version" (changes can break). Bounds should be `>= lower` (the version you tested) and `< upper` (the next potential-breaking major). Common errors: no upper bound (a dependency release breaks your build with no warning) or an overly-tight upper bound (blocks installation with a newer, compatible version). Set bounds as `>= A.B.C && < A.(B+1)` for the major version you support; relax with `allow-newer` (cabal) / `allow-newer-deps` only when you have tested the newer version. For libraries, *do* set upper bounds (protect users from breakage); for applications, you may omit them (you control the build). Keep bounds honest (tested, not aspirational).

- PVP: first two components are "major" (may break); bound `>= tested` and `< next-major`.
- Libraries: set upper bounds to protect users; applications: may omit (you control the build).
- Relax with `allow-newer` only after testing; do not leave bounds aspirational.

### Manage Multi-Package Projects With cabal.project / stack

A multi-package project has several `.cabal` files under a root. In cabal, a `cabal.project` lists the packages (`packages: foo bar baz`) and shared constraints (flags, bounds, source-repository-package for unreleased deps); `cabal build all` builds everything. In stack, the packages are in `stack.yaml`'s `packages` list. Keep the dependency graph acyclic (a package should not depend on one that depends back). For internal libraries shared across packages, either publish to Hackage or use `cabal.project`'s `source-repository-package`/sublib support. Shared build flags and bounds go in `cabal.project`/`stack.yaml`, not duplicated per package.

- `cabal.project` lists packages and shared constraints; `stack.yaml`'s `packages` for stack.
- Keep the package graph acyclic; shared flags/bounds centralized, not duplicated.
- Use `source-repository-package` for unreleased deps; publish or sublib for shared internal code.

### Ensure Reproducibility (Pin Versions / Snapshots / Nix)

A reproducible build gives the same result over time and across machines. With cabal, pin via a `cabal.project.freeze` (freezes resolved versions) or `cabal.project` constraints; with stack, the `stack.yaml` resolver (snapshot) pins everything; with Nix, the nixpkgs revision pins the world. For CI, use the freeze/snapshot/Nix pin to avoid "works on my machine" / "build broke because a dep released a new version." Commit the freeze/snapshot/lockfile. For libraries, do not commit a freeze (users resolve their own); for applications, do.

- cabal: `cabal.project.freeze` or constraints; stack: the `stack.yaml` resolver; Nix: nixpkgs revision.
- Commit the lockfile for applications (reproducibility); do not for libraries (users resolve).
- CI uses the pin to avoid drift.

### Use HLS/ghcid For Fast Feedback; Profile Builds For Performance

For development, HLS (Haskell Language Server) powers IDE features (completion, type info, go-to-definition) in VS Code/other editors; `ghcid` (a lightweight terminal REPL) gives fast recompile-on-save with errors/warnings. Both rely on fast incremental builds — keep the project structured for it (split modules by change frequency, avoid huge modules). For performance work, build with profiling (`cabal build --enable-profiling` / `stack build --profile`) and run with `+RTS -p` (cost-centre profile) / `-hc` (heap profile) / the eventlog for time/space analysis. Profiling builds are slower; keep a non-profiling build for normal dev.

- HLS for IDE features; `ghcid` for fast terminal feedback; both rely on incremental builds.
- Profile builds (`--enable-profiling`/`--profile`, `+RTS -p/-hc`) for performance; keep a non-profiling build for dev.
- Structure modules for incremental compilation (split by change frequency).

## Common Traps

### Mixing cabal And stack Resolution

Different resolvers produce different builds. Pick one tool per project.

### No Upper Bounds (Library)

A dependency release breaks the build with no warning. Set PVP upper bounds for libraries.

### Overly-Tight Upper Bounds

Blocks compatible newer versions. Bound by major version, not patch.

### Cabal Hell (Conflicting Transitive Deps)

Two packages requiring incompatible versions of a third. Pin/relax deliberately; use snapshots (stack) or constraints.

### No Reproducibility (Unpinned Builds)

A dep release breaks CI. Pin via freeze/snapshot/Nix; commit the lockfile for applications.

### Aspirational Bounds

Bounds claiming support you haven't tested. Test before setting/relaxing bounds.

### Cyclic Package Dependencies

A package depending on one that depends back. Keep the graph acyclic; extract shared code.

### Profiling Build Used For Normal Dev

Profiling builds are slow. Keep a separate non-profiling build for everyday work.

## Self-Check

- [ ] One build tool (cabal/stack/Nix) is chosen per project and used consistently; cabal vs stack resolution is not mixed.
- [ ] Dependency bounds follow the PVP (`>= tested`, `< next-major`); libraries set upper bounds to protect users, applications may omit; bounds are tested, not aspirational.
- [ ] Multi-package projects use `cabal.project`/`stack.yaml` with an acyclic package graph and centralized shared flags/bounds.
- [ ] Builds are reproducible (freeze/snapshot/Nix pin committed for applications; not for libraries); CI uses the pin to avoid drift.
- [ ] IDE tooling (HLS) and fast feedback (`ghcid`) are set up, with modules structured for incremental compilation.
- [ ] Profiling builds (`--enable-profiling`/`--profile`, `+RTS -p/-hc`) are available for performance work, separate from the normal dev build.
- [ ] Dependency conflicts (Cabal hell) are resolved by pinning/snapshots/constraints, not by ad hoc `allow-newer` cascades.
- [ ] The build setup has been considered under reproducibility, bound conflicts, multi-package structure, and CI, and remains correct and maintainable.
