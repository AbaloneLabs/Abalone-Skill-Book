---
name: semantic_versioning_and_compatibility.md
description: Use when the agent is choosing or bumping a version number, deciding whether a change is breaking, identifying the public API of a library or service, writing pre-release or build metadata, specifying version ranges in a manifest, or diagnosing why a consumer broke after an upgrade that looked compatible.
---

# Semantic Versioning And Compatibility

A version number is a promise about compatibility, and the promise is the only thing standing between a consumer and a silent breakage. When a consumer sees a dependency move from 1.2.0 to 1.3.0, they are entitled to assume their code will keep working — that is what the version number promised. When that assumption holds, upgrades are safe and the ecosystem composes. When it breaks — when a "minor" upgrade changes behavior the consumer relied on — every consumer learns to pin and never upgrade, the ecosystem fragments, and the version number becomes noise. Semantic versioning is the discipline of making the number mean what it says, so that compatibility is communicated rather than discovered the hard way.

Agents tend to treat versioning as a labeling exercise: bump the number, ship the release. The hard part — deciding what counts as the public API, judging whether a given change is breaking, and applying the rule consistently — is where the meaning is created or destroyed. The judgment problem is to identify what consumers actually depend on, to classify changes honestly against that surface, to handle the special cases (0.x, pre-releases, build metadata) correctly, and to specify version ranges that are neither so loose they invite breakage nor so tight they defeat the point of versioning.

## Core Rules

### Identify The Public API Explicitly Before Versioning It

Semantic versioning version is a contract over a *defined* public API, and you cannot honor a contract you have not identified. The first step in any versioning discipline is to make the public API explicit: which types, functions, endpoints, behaviors, and formats are promised to consumers, and which are internal and free to change. Without this boundary, every change is a judgment call about whether someone might have depended on it, and the answer is usually "maybe," which makes every change feel breaking and freezes the library.

Make the boundary concrete:

- **Document what is public** — an explicit API surface, exported symbols, documented endpoints, stable formats.
- **Mark what is internal** — unexported modules, underscore-prefixed names, "experimental" labels, anything in a namespace reserved for implementation.
- **State the stability of each part** — stable (covered by the version contract), experimental (may change in any release), deprecated (scheduled for removal).

Consumers depend only on the public, stable surface by contract; depending on internals is at their own risk. This boundary is what makes it possible to evolve the library: you can change internals freely, and you version only the changes to the promised surface.

### Classify Changes Against The Public API Honestly

Once the public API is defined, version bumps follow the change's effect on that surface:

- **PATCH (1.2.0 → 1.2.1)** — backward-compatible bug fixes. Internal changes that do not affect the public API, and fixes that correct behavior to match documented contracts, are patches.
- **MINOR (1.2.0 → 1.3.0)** — backward-compatible additions. New functions, new endpoints, new optional parameters, new capabilities that do not break existing consumers. The hallmark is that a consumer written against 1.2.0 still works against 1.3.0.
- **MAJOR (1.2.0 → 2.0.0)** — breaking changes. Anything that can break a consumer relying on the public API: removed or renamed public symbols, changed signatures, changed behavior of documented contracts, narrowed accepted inputs, broadened thrown errors.

The trap is the change that *looks* compatible but *breaks* behavior. Widening an accepted input type is additive and minor; narrowing it is breaking. Adding a new case to a returned enum is usually minor; changing when existing cases are returned can be breaking. Fixing a bug is a patch — unless consumers relied on the buggy behavior, in which case the fix is breaking to them. The test is always: could a reasonable consumer, relying on the documented public API, have their code break? If yes, it is breaking, regardless of how small the diff looks.

### Treat Behavior And Contracts As Part Of The API

The public API is not only types and signatures; it includes the documented behavior of those types — ordering guarantees, error conditions, side effects, performance characteristics where promised, and the meaning of return values. A change that leaves every signature identical but alters a documented guarantee is a breaking change, because consumers relied on the guarantee.

This is the most commonly missed category. A library that silently changes the order of items in a returned collection, or starts returning an error where it previously returned a default, or changes a retry behavior, has broken consumers who relied on the documented (or even consistently-observed) behavior, even though no signature moved. When evaluating a change, ask not only "did the signatures change" but "did any documented or consistently-observed behavior change" — and version accordingly.

### Handle 0.x As Explicitly Unstable

Versions before 1.0 (0.x) are, by convention, explicitly unstable: the public API is not yet fixed, and any release may break compatibility, with the minor version acting as the breaking-change indicator. This is not a loophole to avoid versioning discipline; it is a signal to consumers that they are adopting something still in flux and should pin tightly.

Use 0.x honestly:

- **Reserve it for genuinely unstable work** where the API is still being discovered. Promote to 1.0 once the public API is settled enough to promise.
- **Do not stay in 0.x forever** as a way to avoid committing to compatibility. A library that is 0.x for years has either not decided on its API or is using the prefix as a shield against the obligations of a real release.
- **Communicate the instability to consumers** so they know to pin and to expect breaking changes between minor versions.

Reaching 1.0 is a decision: it declares that the public API is stable and that the versioning contract now applies in full. Make that decision deliberately, document the public API as it stands at 1.0, and honor the contract thereafter.

### Use Pre-Release And Build Metadata Correctly

Pre-release versions (1.0.0-alpha, 1.0.0-beta.2) and build metadata (1.0.0+build.42) extend the version for specific purposes. Pre-release versions denote a version that precedes the release and has *lower* precedence than the release itself (1.0.0-alpha < 1.0.0). Build metadata denotes variants of the same release and does not affect precedence. These have specific semantics that consumers and resolvers depend on.

Use them as intended:

- **Pre-release** for versions that are not yet the final release and may change before it. Consumers who opt into pre-releases accept that instability; consumers who do not should not receive them through a normal range.
- **Build metadata** for variants of the same version (different builds, compilers, or environments) that are semantically the same release. Do not use build metadata to sneak in behavior changes, because it does not affect precedence and resolvers treat builds as equivalent.

A common error is using pre-release or build tags to dodge a version bump for a breaking change. A breaking change is breaking regardless of the tag; tagging it as a pre-release does not absolve the obligation to bump major once released.

### Specify Version Ranges That Match The Trust Level

As a consumer specifying a dependency version range, the range should reflect how much you trust the dependency's compatibility discipline. A range of `^1.2.0` (accept any compatible 1.x at or above 1.2.0) trusts that the dependency honors its minor and patch contract; a range of `~1.2.0` (accept only 1.2.x) trusts only the patch contract; a pinned exact version trusts nothing and accepts no changes. Each is appropriate in different contexts.

Guidance:

- **Libraries should specify permissive ranges** (compatible minors) so that consumers can resolve a compatible set across the dependency graph, trusting the dependency's contract.
- **Applications should pin exact versions** (with a lockfile) so that the deployed artifact is exactly what was tested, regardless of the dependency's contract.
- **Avoid unbounded ranges** (`*` or `>= 1.0`) that accept any future version, including breaking majors, unless you genuinely accept that risk.
- **Be cautious with ranges spanning 0.x**, where the compatibility contract is weaker and minor bumps may break.

The range is a statement about trust. Specifying a range that assumes more compatibility than the dependency provides guarantees breakage; specifying one tighter than necessary defeats the purpose of versioning and creates resolution conflicts.

## Common Traps

### Versioning Without A Defined Public API

Bumping versions without an explicit public API boundary forces every change to be a guess about what consumers depend on, usually resulting in over-cautious major bumps or under-cautious breaking minors. Define the public API first; version only changes to it.

### The Behavior Change That Looks Compatible

Changing documented or consistently-observed behavior while leaving signatures identical breaks consumers who relied on the behavior, even though the diff looks additive. Treat behavior and contracts as part of the API and version behavior changes as breaking when they alter guarantees.

### Calling A Breaking Change A Minor Or Patch

Renaming, removing, narrowing, or changing the contract of a public element is breaking, however small the diff. Mislabeling it as minor or patch ships a silent breakage to every consumer who upgrades, destroying trust in the versioning discipline.

### Living In 0.x Indefinitely

Staying in 0.x to avoid committing to compatibility either signals the API is not decided or uses the prefix as a shield. Decide on the API, document it, and promote to 1.0; do not use 0.x as a permanent escape from the versioning contract.

### Using Pre-Release Or Build Tags To Dodge A Bump

Tagging a breaking change as a pre-release or burying a behavior change in build metadata does not remove the obligation to version it correctly. The tag describes the release's stability, not a loophole in the contract.

### Unbounded Or Over-Permissive Version Ranges

Ranges that accept any future version, or that span 0.x where the contract is weak, invite breakage when the dependency releases a breaking change that the range happily accepts. Match the range to the trust level and the dependency's actual compatibility discipline.

### Fixing A Bug That Consumers Relied On Without A Major Bump

A bug fix that changes behavior consumers depended on is breaking to those consumers, even though it is "correct." Evaluate whether the behavior was part of the observed contract; if so, the fix may require a major bump or a deprecation path rather than a silent patch.

## Self-Check

- [ ] The public API is explicitly identified and documented, with stable, experimental, deprecated, and internal surfaces distinguished, so versioning decisions have a defined boundary to apply against.
- [ ] Each version bump matches the change's actual effect on the public API — patch for compatible fixes, minor for compatible additions, major for anything that could break a consumer relying on the documented surface.
- [ ] Behavior, contracts, ordering, error semantics, and consistently-observed guarantees are treated as part of the API, and changes to them are versioned as breaking when they alter what consumers relied on.
- [ ] 0.x is reserved for genuinely unstable work, the instability is communicated, and promotion to 1.0 is a deliberate decision to commit to the public API.
- [ ] Pre-release and build metadata are used for their intended semantics (pre-release precedence, build-equivalence) and not to dodge version bumps for breaking changes.
- [ ] Version ranges specified as a consumer match the trust level — permissive for libraries trusting the contract, pinned for applications, never unbounded — and are cautious across 0.x.
- [ ] A bug fix that changes behavior consumers relied on was evaluated for breaking impact and given a major bump or deprecation path rather than a silent patch.
- [ ] A consumer relying only on the documented public API would not be broken by a minor or patch upgrade.
