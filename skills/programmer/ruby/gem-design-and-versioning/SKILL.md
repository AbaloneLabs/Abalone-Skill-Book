---
name: gem_design_and_versioning.md
description: Use when the agent is designing, publishing, or maintaining a Ruby gem, defining its public API, declaring gem dependencies and version constraints, cutting releases, handling semver and breaking changes, supporting multiple Ruby versions, or diagnosing dependency conflicts and load-order issues in Ruby applications.
---

# Gem Design and Versioning

A Ruby gem is a published contract, and most gem maintenance pain comes from treating the gem like an internal library. Once a gem is on RubyGems, its public API, its dependency constraints, and its version number become a load-bearing part of every application that depends on it. The judgment problem is that Ruby's dynamic dispatch, monkey-patching, and loose module boundaries make it trivially easy to publish a gem that "works for me" but breaks consumers through implicit dependencies, overly tight version pins, or a public surface that quietly expands with every release.

The recurring failure mode is a maintainer who ships a "minor" or "patch" release that changes behavior, renames a method, or bumps a transitive dependency, and then discovers that hundreds of applications break because they treated the gem as stable. The reverse failure is just as common: a gem author pins dependencies so tightly (`~> 1.2.3`) that the gem cannot coexist with anything else in the dependency graph, producing "Bundler could not find compatible versions" errors that are extremely hard to resolve. Versioning and dependency declaration are design decisions, not release mechanics.

## Core Rules

### Define the public API explicitly and treat it as immutable

Ruby has no `private` enforcement that survives `send`, so "public API" is a social contract. Decide deliberately what is public (methods you document, classes you list in the readme) versus internal (everything prefixed with `_`, marked `@private`, or in a `Internal` module). Once public, treat it as immutable: do not rename, remove, or change signatures in non-major releases. Use `Kernel#warn` deprecation cycles for at least one minor release before removing anything. Document the boundary; consumers cannot guess it.

### Declare dependencies with the loosest constraint that is actually safe

A gem's `spec.add_dependency` constrains every consumer's resolution. The default should be `~>` (pessimistic) on the minor version (`~> 1.2`) for libraries that follow semver, which allows patch and minor updates but blocks majors. Tighter pins (`~> 1.2.3` = patch only) are appropriate only when you have verified breakage across minors. Avoid exact pins (`= 1.2.3`) in published gems; they force conflicts. Test against a matrix of dependency versions, not just the latest, to justify your lower bound.

### Use `add_runtime_dependency` vs `add_dependency` vs `add_development_dependency` correctly

Runtime dependencies are required to use the gem; development dependencies are required only to develop it. Putting a test-only gem in runtime dependencies bloats every consumer. Putting a runtime requirement in development dependencies means the gem fails at runtime for consumers. Audit each dependency for its actual role.

### Follow semantic versioning and earn the version number

SemVer in Ruby gems: major for breaking API/behavior changes, minor for backward-compatible features, patch for backward-compatible fixes. The trap is the "behavioral change that feels small." Renaming an internal constant that consumers happen to reference, changing the order of callback execution, or altering a default value are all breaking even if no method signature changed. When in doubt, bump minor for any behavior change and major for anything that could break a consumer's existing code. Never rewrite history or republish a version number.

### Support a deliberate Ruby version matrix

Declare `required_ruby_version` to fail fast on unsupported Rubies rather than failing mysteriously at runtime. Test against the oldest Ruby you support, not just the newest, because language features and stdlib changes break older runtimes. A gem that uses Ruby 3 pattern matching but declares `required_ruby_version >= 2.5` will fail on 2.5. CI matrix testing across the supported range is the only reliable check.

### Keep the gem's load footprint minimal and explicit

`require`ing a gem should not execute heavy side effects, monkey-patch core classes, or make network calls. Lazy-load optional dependencies (e.g., `require "redis"` only when the Redis adapter is used) so consumers who do not need that path do not pay the load cost or the dependency conflict. If the gem monkey-patches core classes, make it opt-in (e.g., a `require "mygem/core_ext"`) rather than the default, and document it loudly.

### Design for the Bundler resolution graph, not just your gem

Every constraint you add participates in every consumer's `bundle install`. A gem that pins `rails (~> 6.1.4)` cannot be used in a Rails 7 app; a gem that pins `faraday (~> 1.0)` conflicts with a gem that needs `faraday (~> 2.0)`. Before tightening a constraint, ask whether the tighter bound is justified by a real incompatibility or just by "that is what I tested." Prefer wider bounds verified by a test matrix.

### Provide a changelog and deprecation path

Maintain a `CHANGELOG.md` with Breaking, Added, Changed, Fixed sections per release. For breaking changes, ship a deprecation in the prior minor release that warns on the old behavior, then remove in the major. Consumers who read the changelog and fix warnings upgrade smoothly; those who do not at least get a clear failure with a version they can pin to.

## Common Traps

### Shipping a behavioral change as a patch

Changing a method to return `nil` instead of `false`, or to raise a different exception class, breaks consumers who branched on the old behavior. Treat any observable behavior change as at least a minor, and breaking ones as major.

### Over-tight dependency pins justified by "it works"

Pinning `~> 1.2.3` because that is the only version tested creates artificial conflicts across the ecosystem. Test a matrix and widen the bound. The ecosystem-wide cost of over-tight pins is large.

### Monkey-patching core classes by default

`class String; def my_method; end; end` in a gem's default load path pollutes every consumer and creates conflicts with other gems that define the same method. Make core extensions opt-in and namespaced.

### Exposing internals that consumers then depend on

A constant, class, or method you did not mark private becomes public API the moment someone references it. Aggressively namespace internals (`MyGem::Internal::...`) and document the public boundary, because once consumers depend on internals you lose the freedom to refactor.

### Yanking versions that consumers depend on

`gem yank` removes a version from the index, breaking any Gemfile.lock or install that references it. Yank only for security or critical breakage, and even then prefer pushing a fixed version. Yanking a version consumers have locked is a breaking change to the ecosystem.

### Ignoring the gemspec's effect on load order

Code in the gemspec or in a top-level `require` runs at load time. Side effects, network calls, or heavy computation at require time slow every consumer and break offline/locked environments. Keep require-time code to declarations and autoloads.

### Treating `bin/` executables as part of the public API

A gem's executable is a separate contract from its library API. Changing the CLI flags or output format breaks shell scripts even when the library API is stable. Version and document the CLI independently if it is a supported interface.

## Self-Check

- Is the public API boundary documented and enforced by convention (namespacing, `@private`), with internals clearly separated from stable surface?
- Does each runtime dependency use the loosest constraint justified by a test matrix, rather than a tight pin justified only by "what I tested"?
- Is `required_ruby_version` declared and tested against the oldest supported Ruby, not just the newest?
- For the most recent release, does the version bump (major/minor/patch) match the actual change classification, including behavioral changes?
- Are breaking changes preceded by a deprecation warning in a prior release and documented in the changelog?
- Does requiring the gem avoid side effects, monkey-patching, network calls, and heavy computation by default?
- Are core-class extensions opt-in and documented, rather than applied on default load?
- Have you avoided yanking versions that consumers may have locked, and is there a clear policy for security-only fixes on old versions?
