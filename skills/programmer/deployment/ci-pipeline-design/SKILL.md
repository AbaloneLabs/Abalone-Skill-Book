---
name: ci_pipeline_design.md
description: Use when the agent is designing or reviewing a continuous integration pipeline, ordering build and test and lint and security scan stages, configuring dependency or Docker or monorepo caches, parallelizing matrix builds, managing build artifacts and promotion between stages, handling CI secrets with OIDC and masked variables, reducing pipeline runtime, diagnosing flaky tests, or deciding what the merge quality gate should enforce. Use before writing a CI workflow file, when a pipeline is slow or unreliable, or when auditing whether CI actually prevents bad merges.
---

# CI Pipeline Design

A CI pipeline is the automated verification that gates whether a change is allowed to merge. It is not a build script and not a deployment system. Its job is to give a fast, reliable, trustworthy answer to one question: is this change safe to integrate? A weak pipeline fails to catch real defects and lets them through; a slow or flaky pipeline trains the team to ignore or retry failures, which destroys the very thing that makes CI a quality gate — trust in a red signal. The hardest judgment in CI design is not which tools to wire together but how to balance three forces that constantly pull against each other: speed, coverage, and reliability. You can make a pipeline fast by skipping checks, comprehensive by adding every scanner, or reliable by waiting for flakes — but naively optimizing any one degrades the others.

Agents tend to approach CI as a checklist of jobs to add: build, test, lint, scan, done. They under-invest in the decisions that determine whether the pipeline actually works: the *ordering* of stages (cheap fast checks first, expensive deep checks later), the *caching strategy* that keeps runtime sane as the repo grows, the *secret handling* that determines whether the pipeline is itself an attack surface, and the *flakiness budget* that decides whether a red signal means something. The result is pipelines that take forty minutes, fail randomly ten percent of the time, leak tokens into logs, and get bypassed with "merge anyway" because no one trusts them. This skill covers the design of the verification pipeline itself — how code reaches production safely is the domain of the release-strategies skill, and how features are progressively released is the domain of the feature-release skill; this skill is about the automated gate that decides whether a merge may happen at all.

## Core Rules

### Order Stages So Cheap Signal Comes First

The single most impactful CI design decision is stage ordering. A pipeline that runs an expensive integration suite before a cheap lint wastes minutes on changes that would have been rejected in seconds. Order stages by cost-to-signal ratio, cheapest decisive checks first:

- **Pre-merge fast gates first.** Format, lint, type-check, and compile run in seconds to a minute and catch the majority of trivial defects. Put them before anything that needs a database or a container image.
- **Unit tests next.** Fast, isolated, no network. These catch most logic regressions and run in a small fraction of the full suite's time.
- **Integration and contract tests after.** These need real dependencies, containers, or seeded data, and are slower. Run them after the cheap gates have already filtered out obvious breakage.
- **Expensive scans last.** Security scans, dependency audits, end-to-end tests, and performance benchmarks are the slowest and most resource-hungry. Gate them on the changes that survived everything cheaper.

The reason ordering matters is fail-fast economics. If a one-second lint failure is discovered only after a fifteen-minute integration suite, every developer pays fifteen minutes per trivial mistake, and the feedback loop that makes CI valuable is destroyed. Strong choice: a pipeline where a formatting error returns red in under thirty seconds. Weak choice: a single monolithic job that runs everything and reports failure only at the end.

### Parallelize Independent Work, Serialize Dependent Work

A pipeline that runs jobs sequentially when they could run in parallel is artificially slow; one that runs jobs in parallel when they depend on each other produces misleading failures. Identify the dependency graph of checks and parallelize the independent branches:

- **Independent checks run concurrently.** Lint, unit tests, security scan, and license check usually do not depend on each other and can all start at once. A pipeline that runs them one after another is leaving throughput on the table.
- **Dependent checks serialize.** Integration tests depend on a successful build; deployment depends on passing tests. Express these as stage dependencies so a downstream job does not start — and waste resources — when its upstream has already failed.
- **Shard slow suites across runners.** A test suite that takes thirty minutes on one runner can take five minutes across six shards, provided the shards are independent and the results are aggregated into one pass/fail verdict. Sharding is one of the highest-leverage optimizations for mature pipelines.

The trap to avoid is parallelizing for its own sake. Two jobs that share mutable state, or that both publish an artifact, will race and produce flaky failures that are far worse than the slowness they cured. Strong choice: a fan-out/fan-in shape where independent shards run concurrently and a final job aggregates. Weak choice: many jobs launched in parallel that implicitly depend on each other through shared storage.

### Design Caching As A First-Class Concern, Not An Afterthought

As a repository grows, uncached CI becomes unusably slow. Caching is what keeps a ten-minute pipeline at ten minutes instead of forty when dependencies and build outputs are large. Treat each cache layer deliberately:

- **Dependency caches.** Package manager lockfiles (lockfile hash as the cache key) avoid re-downloading and re-resolving dependencies on every run. Invalidate on lockfile change, not on a fixed schedule, so the cache is always correct for the current dependency set.
- **Build caches.** Compilers and bundlers (incremental compilation, transpilation caches) should persist their output directories across runs. The cache key should include the toolchain version so a compiler upgrade does not serve stale artifacts.
- **Docker layer caches.** For image-based pipelines, order Dockerfile instructions so the least-frequently-changing layers come first (base image, then dependencies, then source copy). A Dockerfile that copies source before installing dependencies invalidates the dependency layer on every code change and defeats the cache.
- **Monorepo caches.** In a monorepo, cache per-package build outputs and scope test runs to affected packages using a dependency graph, so a change in one package does not rebuild and retest the entire tree.

Caching introduces its own risk: a stale or poisoned cache produces "works on my CI" failures that are extraordinarily hard to diagnose. Always provide a documented, low-friction way to bust the cache (a key suffix, an environment variable), and key caches on everything that should invalidate them — lockfile, toolchain, OS, and cache schema version. A cache that is never invalidated is a cache that will eventually serve wrong results.

### Treat Secrets As An Attack Surface, Not A Configuration Detail

A CI pipeline runs with credentials to publish packages, deploy to production, and read private dependencies. Those credentials make the pipeline a target, and mishandled secrets are a leading cause of real breaches. The strong defaults:

- **Prefer short-lived, scoped credentials over long-lived tokens.** OIDC federation lets the pipeline assume a role for the duration of a run with no stored secret at all; a long-lived deploy token in the CI settings is a standing credential that leaks wholesale if the CI provider is compromised. Use OIDC wherever the destination supports it.
- **Mask every secret in logs.** Any value registered as a secret should be redacted from log output automatically. Verify this by deliberately printing a canary value in a test run and confirming it appears masked.
- **Scope secrets to the jobs that need them.** A secret available to every job in the pipeline is available to every dependency of every job — including transitive dependencies pulled from npm or PyPI. Bind secrets to specific jobs and environments, and never expose production deploy credentials to pull-request builds from forks.
- **Never commit secrets, even "encrypted".** Encrypted-at-rest secrets in the repo still rotate poorly and are often decrypted by anyone with read access. Use the CI provider's secret store or a vault.

The unifying principle: a secret in CI is a secret on every machine that can trigger a run. Treat the pipeline's trust boundary as seriously as production's.

### Make The Pipeline A Real Quality Gate, Not A Suggestion

CI only prevents bad merges if a red pipeline actually blocks the merge. The enforcement, not the existence of checks, is what makes it a gate:

- **Block merges on required checks.** The checks that protect the main branch must be configured as required status checks, not advisory. A pipeline whose failures can be overridden by a maintainer clicking "merge anyway" is not a gate; it is a suggestion that will be ignored under deadline pressure.
- **Require the same checks on every merge path.** A direct push to main, a fast-forward merge, a squash merge, and a rebase merge must all run the same gate. A bypass for one path is a bypass for all of them in practice.
- **Make the gate the source of truth for "done".** A change is not complete when the author says so; it is complete when the gate passes. Resist "I tested it locally, just merge it" — local environments drift, and the gate exists precisely because local confidence is not sufficient.

The cost of a strict gate is friction when the pipeline itself is the problem (flakiness, slowness). That friction is the signal to fix the pipeline, not to weaken the gate.

### Budget For Flakiness Explicitly And Drive It To Zero

A flaky test is not a minor annoyance; it is an active attack on the pipeline's trustworthiness. Once a failure can be random, every failure becomes plausibly random, and developers start retrying instead of investigating. Treat flakiness as a budget that must be spent down:

- **Quarantine flaky tests immediately.** A test that fails intermittently should be moved out of the blocking path the moment it is identified, with a tracking issue and an owner, so it stops eroding trust in the rest of the suite.
- **Investigate root causes, do not just retry.** Flakiness almost always indicates a real defect: a test that depends on ordering, on wall-clock time, on shared mutable state, on an external service. Retrying masks the defect; the test must be fixed or made deterministic.
- **Track a flakiness metric.** If you do not measure how often the suite fails and then passes on retry, you cannot tell whether you are getting better. A pipeline with no flakiness metric is one where rot is invisible until it is severe.

A pipeline with zero known flakiness is achievable and worth the effort; a pipeline that tolerates flakiness will, over time, be trusted by no one.

## Common Traps

### The Single Monolithic Job That Reports Failure Only At The End

Running build, test, lint, and scan in one job means a formatting error is discovered after the full suite finishes, and the developer waits the entire pipeline duration for feedback they could have had in seconds. Split into ordered stages so cheap decisive checks fail fast.

### Caching Without A Correct Invalidation Key

A cache keyed only on the branch name, or with no key at all, serves stale dependencies and build outputs across toolchain upgrades and lockfile changes, producing failures that vanish on cache bust and recur on the next run. Key caches on lockfile hash, toolchain version, OS, and a schema version, and provide a one-command cache bust.

### Long-Lived Deploy Tokens Instead Of OIDC

Storing a long-lived production deploy token as a CI secret because it is simpler than setting up OIDC federation. The token is a standing credential exposed to every job and every transitive dependency; OIDC gives short-lived, scoped, auditable credentials with no stored secret. The setup cost is paid once; the breach risk is permanent.

### Secrets Leaking Through Fork Builds And Pull-Request Checks

Exposing deploy or publish secrets to pull-request builds, including builds triggered from forks, so that a malicious external contributor can exfiltrate them via a crafted workflow change. Bind secrets to trusted branches and internal contributors only, and treat fork-run workflows as untrusted by default.

### Parallel Jobs Racing On Shared Storage

Two jobs launched "in parallel" that both write to the same artifact bucket path or both publish to the same registry tag, producing intermittent corrupt artifacts or publish races. Parallelize only genuinely independent work, and give each concurrent job an isolated output namespace.

### Docker Layer Cache Defeated By Instruction Order

A Dockerfile that copies the full source tree before installing dependencies, so every code change invalidates the dependency layer and forces a full reinstall. Reorder so dependencies are installed before source is copied, making the expensive layer reusable across code changes.

### Treating Flaky Tests As Normal

Retrying a flaky test until it passes and merging, because investigating the root cause is slower than the retry. Each accepted flake erodes trust in the red signal and guarantees the flakiness will spread. Quarantine and fix flaky tests; never silently retry past them.

### A Gate That Maintainers Can Override Under Pressure

Required checks that can be bypassed with admin privileges, used routinely when the pipeline is slow or flaky. The override becomes the path of least resistance, and the gate exists only on paper. Fix the slowness and flakiness rather than weakening enforcement, and reserve overrides for genuine emergencies with an audit trail.

### Monorepo Pipelines That Rebuild And Retest Everything

Running the entire build and test matrix on every change regardless of which package changed, because per-package scoping was never set up. As the monorepo grows, runtime grows with it until the pipeline is unusable. Use a dependency graph to scope work to affected packages.

## Self-Check

- [ ] Stages are ordered by cost-to-signal ratio with cheap decisive checks (format, lint, type-check, compile) running before expensive ones (integration tests, security scans, benchmarks), so a trivial defect fails fast instead of after the full suite.
- [ ] Independent checks run in parallel and dependent checks serialize correctly, slow suites are sharded across runners with aggregated results, and no parallel jobs race on shared mutable storage or artifact paths.
- [ ] Every cache layer (dependencies, build output, Docker layers, monorepo per-package output) has a correct invalidation key covering lockfile, toolchain version, OS, and cache schema, plus a documented one-command cache-bust mechanism.
- [ ] Dockerfile instruction order puts the least-frequently-changing layers first (base image, dependencies) before source copy, so the dependency layer survives code changes.
- [ ] Secrets use short-lived scoped credentials (OIDC federation) over long-lived tokens, every secret is masked in logs (verified with a canary print), secrets are scoped to the specific jobs and environments that need them, and no secret is exposed to fork or untrusted pull-request builds.
- [ ] The checks that protect the main branch are configured as required status checks with no maintainer-override path under normal pressure, and the same gate runs on every merge path (direct push, squash, rebase, fast-forward).
- [ ] Flaky tests are quarantined immediately on detection with a tracking issue and owner, root causes are investigated rather than retried past, and a flakiness rate is measured so drift is visible.
- [ ] In a monorepo, build and test work is scoped to affected packages via a dependency graph rather than rebuilding and retesting the entire tree on every change.
- [ ] Build artifacts are produced once, stored in an immutable registry or artifact store with a content-addressed or version-pinned identifier, and promoted between stages rather than rebuilt per stage, so the artifact that passed the gate is the artifact that ships.
- [ ] The pipeline's total runtime is bounded and monitored, and the design has not traded speed for coverage or reliability in a way that silently degrades the others (e.g., skipping checks to hit a time budget, or adding scanners without parallelizing).
