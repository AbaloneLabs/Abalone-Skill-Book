---
name: dev_environment_and_parity.md
description: Use when the agent is setting up or standardizing development environments, achieving local-CI-production parity, adopting devcontainers or Docker-based dev environments, diagnosing works-on-my-machine divergence, reducing onboarding setup time, or ensuring that code behavior is consistent across local, CI, staging, and production.
---

# Development Environment and Parity

"Works on my machine" is the most common and most expensive class of integration defect, and it is not a joke about careless developers; it is the predictable outcome of environments that are allowed to drift. When local, CI, and production environments differ in OS, library versions, locale, timezone, file system semantics, or available services, defects hide in the gaps and surface only at the worst time. Agents often treat the development environment as a personal preference (each engineer configures their own machine) and discover, after a string of production bugs that no one could reproduce locally, that environment divergence is an architectural problem with real cost.

The judgment problem is that perfect parity is impossible and the pursuit of it can become an end in itself. Production has scale, real data, real network conditions, and real failure modes that no local environment can fully reproduce. The goal is not identical environments but controlled, understood differences: the differences that remain must be deliberate, documented, and bounded so that a defect reproducible in one environment is reproducible in the others, and so that the environments do not silently disagree about what correct behavior is. The agent must decide what to make identical (language runtime, dependency versions, service versions, config schema), what to approximate (data volume, load, network), and what to accept as irreducibly different (scale, real third-party state), and must make those decisions explicitly rather than letting them happen by neglect.

## Core Rules

### Make parity the default and divergence an explicit, documented decision

The baseline should be that local, CI, and production agree on everything that affects behavior: language runtime version, dependency versions, operating system family and version, service versions (database, cache, queue), and the shape of configuration. Any divergence from this baseline should be a deliberate choice with a reason ("we use SQLite locally and Postgres in production because X") rather than an accident. Document the divergences and review them periodically, because each one is a place where a defect can hide. The cost of a divergence is paid every time a bug appears in only one environment and must be chased across the gap.

### Containerize the environment to make parity mechanical

The most reliable way to achieve parity is to run the same container image in local development, CI, and (ideally) production. Docker, devcontainers, or equivalent tools let you define the OS, system libraries, language runtime, and service dependencies as code that is shared across environments. This makes parity a property of a versioned artifact rather than a matter of each engineer manually matching a wiki. Where full production parity is not feasible (e.g., production runs on a managed cloud service), at least share the application's runtime image across local and CI, and version the service dependencies (database version, etc.) explicitly.

### Version and pin every dependency, including transitive and system-level ones

Parity requires that the same code sees the same dependencies everywhere. Pin direct dependencies via lockfiles, and ensure lockfiles are honored in CI (not bypassed by a fresh resolve that picks different versions). Do not forget system-level dependencies: the OS version, the C library, the SSL library, the database version, the time zone database. A defect that appears only in production because production runs a different OpenSSL version is a parity failure. Use containerization or infrastructure-as-code to pin these, and treat a version bump as a deliberate, tested change, not a side effect of rebuilding an image.

### Treat configuration shape as part of the contract, not just values

Parity is not only about software versions; it is also about configuration. If local uses a single-region, single-replica, in-memory setup while production is multi-region with replicas and external caches, the code paths exercised locally are different from the ones that run in production. Configuration that changes behavior (feature flags, caching strategy, retry policy, circuit breaker settings) should have the same shape across environments, with only the values differing. A feature that is enabled in production but cannot be enabled locally is a feature that is never tested locally, and its first real test is in production.

### Make local dependencies (databases, queues, external services) realistic

A common parity failure is substituting a simplified local stand-in for a real dependency: SQLite for Postgres, an in-memory queue for a real broker, a mocked external API for the real service. These substitutions are sometimes justified, but each one is a place where behavior can diverge (SQL dialect differences, transaction semantics, failure modes, concurrency behavior). Prefer running the real dependency locally (via Docker) over a simplified stand-in, and where a stand-in is unavoidable, understand and document the semantic differences. Never let a local stand-in silently swallow errors that the real dependency would surface.

### Reproduce environmental factors that affect behavior: locale, timezone, file system

Behavior-affecting environmental factors are easy to overlook because they are invisible until they break. Locale affects string comparison, sorting, and number formatting. Timezone affects every date and time computation. File systems differ in case sensitivity, path length limits, and permission semantics (macOS is case-insensitive by default; Linux is case-sensitive; Windows differs again). If local development runs in one locale/timezone/filesystem and production in another, you will ship bugs that are unreproducible locally. Standardize these in the development container and in CI, and test with values matching production.

### Diagnose parity failures at the root, not the symptom

When a defect appears in only one environment, the temptation is to patch it there. The correct response is to find the environmental difference that caused it, decide whether to close the gap (so it reproduces everywhere and the fix is testable) or to document the difference as accepted, and fix the root cause. Patching a production-only bug without understanding the environmental cause leaves the divergence in place and guarantees the next bug will hide in the same gap. Treat every environment-specific defect as a signal about a parity gap, not just as a bug to fix.

### Measure onboarding time as a parity signal

The time it takes a new hire to get a working, building, testing environment is a direct measure of environment health. If setup takes days and requires tribal knowledge, the environment is fragile and parity is likely poor. A healthy environment sets up in under an hour via a scripted or containerized process. Track onboarding time and treat friction as a bug to fix in the environment, not as a cost to pass on to each new engineer.

## Common Traps

### Letting each engineer configure their own machine

Personal environments drift from each other and from production, and defects hide in the differences. Standardize via containerization or infrastructure-as-code so parity is a property of a shared artifact.

### Substituting a simplified local dependency without understanding the differences

SQLite-for-Postgres or in-memory-queue-for-real-broker substitutions change SQL dialect, transaction semantics, and failure modes. Prefer the real dependency locally, and document differences where a stand-in is used.

### Bypassing the lockfile in CI

If CI resolves dependencies fresh rather than honoring the lockfile, it can pick different versions than local development and break parity. Pin and honor lockfiles everywhere.

### Accepting configuration divergence that changes code paths

Feature flags, caching, and retry settings that differ across environments mean different code runs in each. Keep config shape consistent and only vary values.

### Patching environment-specific bugs at the symptom

A production-only bug patched in production leaves the parity gap in place. Find the environmental cause, close or document the gap, and fix the root so it is testable everywhere.

### Forgetting locale, timezone, and filesystem differences

These invisible factors affect string handling, date math, and path resolution. Standardize them in the dev container and CI to match production, or ship unreproducible bugs.

### Treating onboarding friction as normal

Multi-day setup requiring tribal knowledge is a symptom of a fragile, low-parity environment. Script and containerize setup, and treat friction as a bug.

## Self-Check

- Is environment parity the default, with every divergence from local/CI/production an explicit, documented, and periodically reviewed decision rather than an accident?
- Is the application runtime shared across environments via containerization (Docker, devcontainer) or infrastructure-as-code, so parity is a property of a versioned artifact?
- Are all dependencies pinned and honored everywhere (lockfiles, system libraries, OS, database versions), with version bumps treated as deliberate, tested changes?
- Does configuration have the same shape across environments (feature flags, caching, retry policy), with only values differing, so the same code paths run everywhere?
- Are local dependencies the real services (via Docker) rather than simplified stand-ins, and where stand-ins are used, are the semantic differences documented?
- Are locale, timezone, and filesystem semantics standardized across local, CI, and production to prevent invisible behavior divergence?
- When a defect appears in only one environment, do you find and close (or formally accept) the environmental gap rather than patching the symptom?
- Is new-hire setup time under roughly an hour via a scripted or containerized process, with friction treated as an environment bug to fix?
