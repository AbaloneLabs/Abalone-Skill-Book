---
name: modules_and_dependency_management.md
description: Use when the agent is creating or editing a Go module (go.mod, go.sum), adding/upgrading/downgrading dependencies, choosing a module path and version, tagging releases, working with module proxies and vendoring, managing the minimum Go version, dealing with replace/exclude directives, handling workspaces (go.work), diagnosing checksum mismatches, "inconsistent vendoring," "missing go.sum entry," retractions, or dependency conflicts and minimum-version selection. Covers semantic import versioning, module paths, version selection (MVS), proxy and checksum database, vendoring, and release/retract discipline.
---

# Modules And Dependency Management

Go modules are the dependency system, and dependency systems are where "works on my machine" meets "every build for the next five years." A `go.mod` file declares a module path, a Go version, and a set of required dependencies at minimum versions; `go.sum` records cryptographic checksums; the toolchain resolves the final build using Minimum Version Selection (MVS), not a latest-greatest SAT solver. This design is deliberate — MVS makes builds reproducible and predictable at the cost of not always picking the newest compatible version — and it is also the source of nearly every dependency surprise: a `go get` that upgrades a transitive dependency and breaks a build, a version tag that does not match the module path, a proxy that returns a different checksum than a coworker's machine, a retraction that was never published so a known-bad version keeps getting selected. Modules look like a package manager and behave like a versioned contract graph.

The judgment problem is not "how do I run `go get`" but "what version does the build actually resolve to and why, what am I committing to when I bump a dependency, what does the module path imply about versions and imports, and what is the supply-chain and reproducibility story." Agents tend to treat `go get -u` as a harmless update (it can pull in major-version-breaking transitive changes), to ignore the module path's version suffix (`/v2`, `/v3`) and get "module declares its path as X/v2 but was required as X" errors, to commit a `go.mod` without the matching `go.sum` (breaking reproducibility), and to reach for `replace` directives as a quick fix that then ships to production and points at a local path. Each is a small local convenience that becomes a build or supply-chain incident.

## Core Rules

### Understand Minimum Version Selection: The Build Uses Minimums, Not Latests

Go's version selection algorithm is Minimum Version Selection: for each module, the build uses the minimum version that satisfies every requirement in the graph — the maximum of the minimum versions each dependent asks for. It does not pick the newest available version, it does not solve for a "best" set, and it does not upgrade transitive dependencies unless something requires a higher version. This makes builds reproducible (the same `go.mod` graph yields the same selected versions) and predictable (you can read the selected version off the requirements), but it means the build can be older than what `latest` would give.

The practical consequences: `go get example.com/lib@latest` upgrades that one module to latest and updates its requirements in `go.mod`, but it does not upgrade lib's transitive dependencies unless you also ask (`go get -u` upgrades the module and its transitive deps; `go get -u ./...` does the whole module; `go get -u=patch` upgrades only patch versions). A build that "should" pick a newer transitive version but does not is almost always MVS selecting the minimum that the graph requires — to get the newer version, some requirement in the graph must demand it, or you must explicitly `go get` it. Do not expect `go build` to upgrade anything; it only reads. Upgrades are explicit `go get` operations.

### Treat The Module Path As A Versioned Contract, Including The Major-Version Suffix

Go uses semantic import versioning: a module whose major version is 2 or higher must have a major-version suffix in its module path (`example.com/lib/v2`, `example.com/lib/v3`). Version 0 and 1 have no suffix. This is not cosmetic — it means `v2` and `v1` are different import paths and can coexist in the same build, and it means a `v2` module whose `go.mod` says `module example.com/lib` (no suffix) is broken and will produce "module declares its path as ... but was required as ..." errors. When you publish a v2+, the module path in `go.mod` must include the suffix, and the git tag must be `v2.0.0` (not `2.0.0`).

When consuming, match the path to the version: import `example.com/lib/v2` for v2, `example.com/lib` for v0/v1. When publishing, before tagging a major version, update the module path in `go.mod`, update all internal imports to use the suffix, and tag with the `vN.0.0` form. The most common publishing bug is tagging a v2 without changing the module path, which makes the tag unusable. The most common consuming bug is `go get example.com/lib@v2.0.0` against a module whose path has no `/v2`, which fails confusingly.

### Keep go.mod And go.sum In Sync And Committed; Never Edit Either By Hand Casually

`go.mod` and `go.sum` are both source-controlled and both load-bearing. `go.mod` records the module path, Go version, and requirements (with minimum versions); `go.sum` records the cryptographic hashes of every module's content (and its `go.mod`) so that future builds verify the same bytes. A `go.mod` change without the corresponding `go.sum` update, or vice versa, breaks reproducibility and triggers "missing go.sum entry for ..." errors in CI or on a coworker's machine. The fix is mechanical: run `go mod tidy`, which adds missing entries and removes unused ones for both files.

Never hand-edit version strings or checksums in either file to "make it work." A hand-edited `go.sum` entry that does not match the real module content will fail the checksum database verification (see below) on every machine but yours. A hand-edited `go.mod` requirement can select a version that does not actually satisfy the graph. Use `go get`, `go mod tidy`, and `go mod edit` (for structural changes like the Go version) as the only ways to mutate these files. Commit both together; a PR that changes `go.mod` but not `go.sum` is incomplete.

### Use replace And exclude Sparingly, And Never Ship A Local replace

`replace` and `exclude` directives in `go.mod` override the normal resolution: `replace example.com/lib => ../myfork` points a dependency at a local path or a different version, and `exclude example.com/lib v1.2.3` forbids a specific version. Both are legitimate tools — `replace` for developing a dependency and its consumer together, or for pinning a forked fix before upstream merges it; `exclude` for blocking a known-broken version. Both are also footguns: a `replace` pointing at a local filesystem path (`=> ../somelib`) works only on your machine and breaks every other build including CI, and a `replace` or `exclude` left in a published module affects only that module's own build, not consumers (consumers ignore your replaces/excludes except for the module's own `go.mod`).

The rules: never commit a local-path `replace` to a shared branch (it will break CI and coworkers); if you must use one, keep it on a local uncommitted change or in a `go.work` workspace file (which is not committed for this purpose). Use `replace` to point at a published fork only with a plan to remove it once upstream fixes the issue, and document why. Remember that a module's `replace`/`exclude` directives do not propagate to consumers — if you need consumers to avoid a version, use a retraction (see below), not an exclude. Treat any `replace` in a `go.mod` as a smell to justify and a debt to clear.

### Use Workspaces (go.work) For Multi-Module Local Development, And Do Not Commit It

A `go.work` file defines a workspace: a set of local modules whose main modules override their dependencies during local development. This is the modern way to develop several modules together without `replace` directives — you edit module A and module B in sibling directories, `go.work` lists both, and a build in A uses the local B instead of the published B. It is the correct tool for multi-repo local development and for monorepos with several modules.

The discipline: `go.work` is for local development and should generally not be committed to a module repository, because a committed `go.work` forces every consumer and CI into the workspace's view. The toolchain warns (`go: warning: go.mod file found in vendor directory, ...` style) and behaves as if the workspace were authoritative. Use `go work sync` to propagate workspace-selected versions back into the individual `go.mod` files before committing. If you find yourself wanting a permanent workspace across teams, that is usually a sign the modules should be one module or that you should use `replace` with published versions, not a committed `go.work`.

### Set The Minimum Go Version Deliberately; It Is A Build Contract

The `go X.Y.Z` line in `go.mod` declares the minimum Go language version and toolchain the module requires. This is a contract: consumers building with an older Go will fail, and the toolchain uses it to decide language features (generics need 1.18+, range-over-func needs 1.23+, etc.) and some standard library behavior. Bumping the minimum Go version is a breaking change for consumers on older toolchains, even though it is not a major-version bump under semver — decide it deliberately and document it.

Modern Go (1.21+) also supports a `toolchain` directive that can request a specific toolchain version, and the `GOTOOLCHAIN` environment variable controls whether the toolchain auto-downloads the requested version. Understand this interaction: a `go 1.22` line means "this module requires at least 1.22," and a `toolchain go1.22.3` means "prefer this exact toolchain." Do not bump the minimum Go version casually to silence a linter or use one feature; it excludes consumers. Conversely, do not leave it artificially low if you use features that require a newer version — the build will fail for consumers in a confusing way. Match the minimum to what the code actually requires.

### Use The Module Proxy And Checksum Database, And Understand What They Protect

By default, Go fetches modules through the module proxy (`proxy.golang.org`) and verifies them against the checksum database (`sum.golang.org`), a tamper-evident log of module content hashes. This is a supply-chain protection: it prevents a compromised upstream repository from serving different content to different builds (because the hash is recorded in the public log), and it provides availability (the proxy caches modules even if the upstream disappears). A "verifying X: checksum mismatch" error means the content the proxy or upstream returned does not match the recorded hash — a real integrity signal, not a fluke.

Do not disable these protections casually. Setting `GOFLAGS=-insecure` or `GOPROXY=direct` with `GOSUMDB=off` removes the supply-chain verification and should be reserved for genuinely private modules that cannot use the public infrastructure. For private modules, configure `GOPRIVATE` (or `GONOSCHECK`/`GONOSUMDB`) to tell the toolchain which paths skip the proxy and checksum DB — this is the correct way to handle private repos, not disabling globally. Understand that once a version is published to the public proxy and checksum DB, its content is immutable — you cannot republish the same version with different bytes, which is why fixes require a new version (and bad versions use retraction).

### Tag Releases Correctly And Retract Known-Bad Versions

Releasing a module means tagging a git commit with a semantic version tag (`v1.4.2`), and the tag must match the module path's major version (a `/v2` module gets `v2.x.y` tags). Pre-release versions use suffixes (`v1.0.0-beta.1`), and pseudo-versions are auto-generated for untagged commits (`v0.0.0-20231015120000-abcdef123456`). Do not delete or rewrite tags after publication — the proxy and checksum DB have cached them, and a rewritten tag produces checksum mismatches for everyone who already fetched it. If a published version is broken, you cannot un-publish it; you retract it.

Retraction (Go 1.16+) is the mechanism: add a `retract` directive to `go.mod` listing the bad version(s) with a reason, publish a new version containing the retraction, and the toolchain will warn consumers who select a retracted version. This is the correct response to a broken release — not tag deletion, not republishing the same version, and not an `exclude` (which does not propagate to consumers). Publish retractions promptly for security-critical or build-breaking releases, and include a clear reason and the version to use instead. A retracted version remains fetchable (for reproducibility) but is flagged, so consumers know to move.

## Common Traps

### Running `go get -u` Without Understanding It Upgrades Transitives

`go get -u` upgrades the named module and its dependencies, which can pull in newer (and possibly breaking-in-minor-ways) transitive versions. Use `go get example.com/lib` (just the module) or `go get -u=patch` (patch only) when you do not want transitive churn.

### Ignoring The Major-Version Suffix On Module Paths

Importing `example.com/lib` for a v2 module, or publishing a v2 without the `/v2` path. Match import path to major version; update the module path before tagging a new major version.

### Committing go.mod Without go.sum (Or Vice Versa)

A requirement without its checksum entry breaks reproducibility and CI. Always run `go mod tidy` and commit both files together.

### Hand-Editing Version Strings Or Checksums

Editing `go.sum` to silence a mismatch hides a real integrity problem. Use `go mod tidy` and the toolchain commands; never fabricate checksums.

### Shipping A Local-Path replace Directive

`replace example.com/lib => ../somelib` committed to a shared branch breaks every build but yours. Use `go.work` for local development; do not commit local replaces.

### Expecting replace/exclude To Affect Consumers

A module's `replace`/`exclude` directives apply only to that module's own build, not to downstream consumers. To influence consumers, retract the bad version or publish a fix.

### Deleting Or Rewriting A Published Tag

The proxy and checksum DB have cached the tag's content; rewriting it causes checksum mismatches for everyone. Retract broken versions instead; publish new versions for fixes.

### Disabling GOPROXY/GOSUMDB Globally Instead Of Using GOPRIVATE

`GOFLAGS=-insecure` or `GOSUMDB=off` removes supply-chain protection for all modules. Use `GOPRIVATE` for private modules only; keep verification on for public dependencies.

### Bumping The Minimum Go Version Casually

A `go 1.22` line excludes consumers on older toolchains. Bump it only when the code requires newer features, and document it as a compatibility change.

### Committing go.work To A Module Repository

A committed `go.work` forces consumers and CI into the workspace view. Keep `go.work` for local development; use `go work sync` to propagate versions back to `go.mod` files.

## Self-Check

- [ ] The build's selected versions are understood as MVS results (the maximum of the minimum versions each requirement asks for), and upgrades are explicit `go get` operations rather than expected from `go build`; `go get -u` is used knowingly because it upgrades transitives.
- [ ] The module path matches its major version (no suffix for v0/v1, `/v2`+ for v2 and above), imports use the versioned path, and any new major version was published with the path updated before tagging.
- [ ] `go.mod` and `go.sum` are always committed together and in sync (`go mod tidy` run), and no version strings or checksums have been hand-edited to force a resolution.
- [ ] `replace` and `exclude` directives are justified, documented, and not local-path replaces on shared branches; consumers are not expected to be affected by them (retractions are used for consumer-facing version blocking).
- [ ] `go.work` is used for multi-module local development and is not committed to module repositories; `go work sync` propagates workspace versions back to individual `go.mod` files before commit.
- [ ] The minimum Go version (`go` directive) reflects what the code actually requires and was bumped deliberately with consumer compatibility considered; the `toolchain` directive and `GOTOOLCHAIN` interaction are understood.
- [ ] The module proxy and checksum database remain enabled for public dependencies; private modules use `GOPRIVATE` rather than globally disabling verification; checksum mismatches are treated as real integrity signals.
- [ ] Releases are tagged with semver tags matching the major-version path, published tags are never deleted or rewritten, and known-bad versions are retracted (with reason and replacement version) via a `retract` directive in a new release.
