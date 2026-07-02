---
name: build_system_and_reproducibility.md
description: Use when the agent is setting up or debugging a build system, fixing CI-versus-local discrepancies, improving build speed or caching, making builds deterministic and reproducible, choosing incremental versus clean builds, isolating the build environment, or diagnosing why the same code builds differently across machines or runs.
---

# Build System And Reproducibility

A build system's job is to turn source into artifacts deterministically: the same input should produce the same output, every time, everywhere. When that holds, builds are trustworthy — a failure means the code changed, a passing build means the artifact is sound, and two engineers running the same commit get the same result. When it breaks, the build becomes a source of superstition. "It works on my machine," "try a clean build," "re-run CI it sometimes passes," and "delete the cache and try again" are all symptoms of a build whose outputs depend on hidden state rather than on the source. Each one is a small lie that the system is reproducible when it is not.

Agents tend to treat the build as a black box that either works or does not, reaching for workarounds (clean everything, bump the cache, re-run) whenever it misbehaves. The workarounds mask the real problem — a non-deterministic step, a stale cache, an environment difference, a hidden dependency on the machine — and the underlying fragility remains, ready to resurface during a release or an incident. The judgment problem is to build and maintain a build system whose results you can trust because they are derived only from the inputs, and to diagnose the specific cause when they are not.

## Core Rules

### Make Builds Deterministic First, Fast Second

A fast build that produces different artifacts from the same input is worse than a slow build that is correct, because the non-determinism is invisible until it breaks something downstream. Before optimizing for speed, establish that the same source, the same toolchain, and the same inputs produce the same outputs. Determinism is the foundation; speed is built on top of it.

Common sources of non-determinism to eliminate:

- **Time and date embedded in artifacts** — build timestamps, file ordering by filesystem listing, or random IDs baked into output.
- **Unordered iteration** — iterating a hash map or set and writing results in iteration order produces output that varies across runs.
- **Absolute paths** — embedding the build machine's absolute paths into artifacts makes them differ across machines.
- **Undeclared inputs** — a build step that reads files outside its declared inputs (environment variables, home directory, system config) depends on state the build system cannot track.
- **Network access during build** — fetching dependencies or data at build time pins nothing and varies by run.

Each of these makes the build depend on something other than the source. Eliminate them or declare them as explicit inputs, so the build system can track and cache them correctly.

### Achieve CI-Local Parity

The most common build failure pattern is "it builds locally but fails in CI" (or the reverse). The cause is almost always an environment difference: different toolchain version, different OS, different environment variables, different installed system libraries, or a local cache that CI lacks. The fix is not to patch each discrepancy as it appears but to make the environments genuinely identical.

Strategies for parity:

- **Pin the toolchain.** Specify the exact compiler, runtime, and tool versions in a version file the build reads, so local and CI use the same versions.
- **Define the environment as code.** Use a container image, a Nix derivation, or an asdf/mise configuration that both local and CI consume, so the environment is the same artifact in both places.
- **Declare all inputs.** Dependencies, system libraries, and configuration the build needs are declared and installed the same way everywhere, not assumed to be present.
- **Run the same commands.** The CI build should invoke the same build entrypoint a developer runs locally, not a parallel script that drifts.

When local and CI diverge, treat the divergence as a bug in the environment definition, not as a reason to add a CI-only workaround. A workaround that makes CI pass while leaving local different guarantees the next divergence will be harder to find.

### Use Caching Correctly, Not Superstitiously

Caching is what makes incremental builds fast, but a cache that returns stale or incorrect results is worse than no cache, because it produces wrong artifacts silently. The rule for a correct cache is that cache validity must be a function of the inputs to the step being cached: if the inputs are unchanged, the cached output is valid; if any input changed, the cache must be invalidated. The build system must know the full set of inputs to compute this correctly.

This is where undeclared inputs bite. A step that reads a file it does not declare as an input will return a stale cached result when that file changes, because the build system does not know to invalidate. Symptoms include "I changed the code but the build used the old version" and "clean build fixes it." Both mean the cache key is missing an input.

Practices for trustworthy caching:

- **Declare all inputs** to each step, including source, dependencies, tool versions, and configuration.
- **Make cache keys content-addressed** — based on a hash of the inputs — rather than based on timestamps or names, which can collide or miss changes.
- **Distrust "clean build fixes it."** That phrase means the incremental cache is incorrect; find the missing input rather than institutionalizing clean builds.
- **Validate cache correctness periodically** by comparing cached and clean builds for a sample of changes; divergence reveals a missing input.

### Isolate The Build Environment

A build that depends on the state of the machine it runs on is not reproducible. Builds should run in an environment that contains exactly the declared inputs and nothing else, so that the result is independent of who ran it or what else is on the machine.

Isolation means:

- **No implicit system dependencies.** If the build needs a library, it is declared and provided, not assumed present on the host.
- **No leakage from the user environment.** Environment variables, shell configuration, credentials, and home-directory state do not influence the build unless explicitly declared as inputs.
- **Sandboxed execution where possible.** Build systems that sandbox file access and network can detect undeclared inputs by failing when the step reaches for something it did not declare.

Isolation catches the hidden inputs that break reproducibility. A build that runs only because the developer happens to have a certain tool installed will fail the moment it meets a clean machine — usually CI, usually at a bad time.

### Make Incremental Builds Correct Before Relying On Them

Incremental builds (rebuild only what changed) are essential for developer velocity, but they are correct only when the build system tracks the full dependency graph of inputs to outputs. An incremental build that misses a dependency rebuilds too little and produces a stale artifact; one that over-invalidates rebuilds too much and loses the speed benefit. Both are common.

Trust an incremental build only after confirming it produces the same result as a clean build for representative changes. If they ever differ, the dependency graph has a gap — usually an undeclared input or a generated file that is not tracked. Fix the graph rather than training developers to clean-build "just in case," which is a sign the incremental build is not trusted and therefore not delivering its value.

### Optimize Build Time As A First-Class Concern

Build time is a productivity multiplier. A build that takes a minute runs many times an hour; one that takes thirty minutes runs rarely and shapes the whole workflow around avoiding it. Slow builds push developers toward large, risky changes and away from the fast feedback loop that catches bugs early.

Effective optimizations, in order of leverage:

- **Eliminate unnecessary work** — remove unused build steps, dead configuration, and redundant processing before making the remaining work faster.
- **Parallelize** — build independent steps concurrently; most build systems can, but only if the dependency graph is correctly declared.
- **Cache aggressively and correctly** — reuse results across runs and across machines via remote caching, with content-addressed keys.
- **Reduce the scope of what rebuilds** — tighten dependency graphs so a change in one module does not rebuild the world.
- **Profile before optimizing** — measure where the time goes; the bottleneck is rarely where intuition suggests.

Measure build time continuously and treat regressions as bugs. A build that slowly crept from two minutes to ten did so one change at a time, and each increment felt acceptable in isolation.

## Common Traps

### "Clean Build Fixes It" As A Workaround

When a clean build fixes a failure, the incremental build is broken — a missing input in the cache key or dependency graph. Institutionalizing clean builds hides the bug and surrenders the speed of incremental builds. Find and declare the missing input instead.

### CI And Local Divergence Treated As Normal

"It works on my machine" is a symptom of an environment difference, not a defense. Each divergence is a defect in the environment definition. Achieve parity through a shared, code-defined environment rather than tolerating drift.

### Caching By Timestamp Or Name Instead Of Content

A cache key based on file modification time or filename misses changes that preserve the timestamp or name, returning stale results. Cache keys must be content-addressed — a hash of the actual input contents — to be correct.

### Undeclared Inputs Silently Breaking Reproducibility

A build step that reads environment variables, home-directory files, or system config without declaring them produces output that varies by machine. Sandboxing or strict input declaration catches these; without it, they surface as intermittent failures.

### Network Or Time Dependencies In The Build

Fetching dependencies at build time without pinning, or embedding build timestamps in artifacts, makes outputs vary across runs even for identical source. Pin all fetched inputs and eliminate or fix embedded non-determinism.

### Optimizing The Wrong Bottleneck

Build time optimizations driven by intuition often target steps that are visible but not dominant, leaving the real bottleneck untouched. Profile the build to find where time is actually spent before optimizing.

### Trusting The Incremental Build Without Verifying

Assuming the incremental build matches a clean build, without ever checking, lets a stale-result bug persist until it causes a confusing failure. Periodically compare incremental and clean outputs to catch missing dependencies in the graph.

## Self-Check

- [ ] The same source, toolchain, and declared inputs produce byte-identical (or semantically identical) artifacts across runs and machines, with no time, ordering, path, or network non-determinism.
- [ ] Local and CI build environments are defined by the same code (container, version files, manifests) and run the same commands, so divergences are treated as bugs in the environment definition.
- [ ] Cache keys are content-addressed over the full set of declared inputs; "clean build fixes it" triggers a search for a missing input rather than becoming routine.
- [ ] The build environment is isolated from implicit system state — no undeclared environment variables, home-directory files, or system dependencies influence the output.
- [ ] The incremental build has been verified to produce the same results as a clean build for representative changes; any divergence was traced to a missing input in the dependency graph.
- [ ] All build inputs, including fetched dependencies and tool versions, are pinned and declared, with no unpinned network fetches at build time.
- [ ] Build time is measured continuously, regressions are treated as bugs, and optimizations were driven by profiling toward the actual bottleneck.
- [ ] A developer on a clean machine, given only the source and the environment definition, can reproduce the CI artifact without undocumented steps.
