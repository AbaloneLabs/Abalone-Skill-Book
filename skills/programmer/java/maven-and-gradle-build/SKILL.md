---
name: maven_and_gradle_build.md
description: Use when the agent is creating or editing a Java/JVM build with Maven (pom.xml, plugins, profiles, dependencyManagement, BOMs, multi-module reactor) or Gradle (build.gradle(.kts), plugins DSL, configurations, dependency locks, version catalogs), choosing between Maven and Gradle, managing transitive dependency conflicts and version alignment, configuring build plugins (compile, test, shade/assembly, fat jar, Spring Boot plugin), reproducible and hermetic builds, or diagnosing dependency convergence failures, "cannot find symbol" from version skew, slow builds, flaky caching, dependency-downloads-at-runtime, or conflicting transitive versions. Covers the build lifecycle, dependency resolution and conflict mediation, BOMs/version catalogs, reproducibility, and the tradeoffs of Maven's convention vs Gradle's flexibility.
---

# Maven And Gradle Build

A JVM build system does two things: it orchestrates the compile-test-package lifecycle, and it resolves a graph of dependencies with conflicting versions into a single classpath. Both look mechanical and both are where builds become slow, flaky, non-reproducible, or subtly wrong. A transitive dependency that resolves to an older version than your code expects produces "cannot find symbol" at compile time or `NoSuchMethodError` at runtime — on a coworker's machine or in CI but not yours, because the local cache had a different resolution. A build that downloads dependencies at runtime (rather than vendoring or locking them) is non-reproducible and fails in air-gapped or cold-cache environments. A Gradle script that grew organically into imperative Groovy logic is impossible to reason about and caches incorrectly. The build is infrastructure that every developer touches daily and that determines whether "it builds" is a reliable fact or a coin flip.

The judgment problem is not "how do I add a dependency" but "what version will every environment actually resolve to, is that resolution stable and reproducible, is the build declarative enough to reason about, and what plugin/lifecycle behavior am I relying on." Agents tend to add dependencies without checking what they pull in transitively, to "fix" version conflicts by forcing a version somewhere inconsistent, to write imperative build logic where declarative configuration would do, and to ignore reproducibility (caching, lock files, hermeticity) until a build breaks in a way no one can reproduce. Each produces a build that works locally and fails elsewhere, or works today and breaks when an upstream releases.

## Core Rules

### Understand How Dependency Resolution And Conflict Mediation Actually Work

Both Maven and Gradle resolve a dependency graph by walking each dependency's transitive dependencies and mediating conflicts where two paths require different versions of the same artifact. Maven uses "nearest wins": the version closest to the root of the graph is selected, and at equal depth, the first declared wins. Gradle uses "highest wins" by default: the highest version among all requested is selected (unless a constraint or BOM forces otherwise). These are different algorithms, and migrating a build between them (or reasoning across them) without understanding the difference produces silent version changes. A build that "was fine in Maven" and breaks in Gradle (or vice versa) is usually a conflict-mediation difference surfacing a latent conflict.

The practical discipline: know what version each environment resolves to, not what you think you declared. Use `mvn dependency:tree` or `gradle dependencies` to inspect the resolved graph, and use `mvn dependency:tree -Dverbose` or Gradle's dependency insight (`gradle dependencyInsight --dependency foo`) to see why a specific version was chosen. When you force a version (Maven `<dependency>` direct declaration, Gradle a constraint or `force`), do it in one canonical place and understand it overrides mediation. Do not scatter version overrides across modules — that produces a graph where the "real" version depends on declaration order and is impossible to audit. The resolved classpath is the truth; the declared dependencies are a wish.

### Align And Pin Versions Centrally With BOMs (Maven) Or Version Catalogs (Gradle)

Version skew — different modules or different transitive paths pulling different versions of the same library family — is the dominant cause of `NoSuchMethodError`, `ClassCastException`, and `IncompatibleClassChangeError` at runtime. Library families that must be version-aligned (Spring, Jackson, Mockito, protobuf, gRPC, AWS SDK v2) should be imported as a BOM (Bill of Materials) in Maven (`<dependencyManagement>` with `<scope>import</scope>`) or a version catalog / platform in Gradle (`platform(...)` or `java-platform`). A BOM centralizes the versions of a family so that any transitive request for a member resolves to the BOM's version, preventing skew.

Use BOMs/platforms as the primary version-management mechanism for families that publish them, and use a Gradle version catalog (`libs.versions.toml`) or a Maven `<properties>` block for the rest, so versions are declared once and referenced by name. Avoid hardcoding version strings in every module's dependencies — that guarantees drift. When you upgrade a family, upgrade the BOM version in one place and the whole family moves together. When you must override a single member's version (a patch not yet in the BOM), do it explicitly and document why, because it is a deliberate skew. The goal is that the entire build's versions are auditable from one or a few files.

### Prefer Declarative, Convention-Based Configuration Over Imperative Scripting

Both Maven and Gradle are more reliable when the build is declarative — you describe what you want (apply the Java plugin, set the Java version, declare dependencies, configure the test task) and the tool's conventions handle the how. Maven is declarative by design (the POM is a description, plugins do the work); Gradle supports both declarative configuration and imperative Groovy/Kotlin scripting, and the imperative path is where builds become unmaintainable. A Gradle script full of `doLast { ... }`, programmatic file manipulation, conditional logic on environment variables, and ad-hoc task dependencies is a program, not a build description, and it is impossible to cache correctly and hard to reason about.

The discipline: use the plugins DSL (`plugins { id("org.springframework.boot") version "..." }`) rather than the legacy `apply plugin:`. Use the Kotlin DSL (`build.gradle.kts`) where possible — its type safety catches configuration errors at edit time and makes the build navigable in an IDE. Express configuration declaratively (set properties, use the documented extension blocks) rather than reaching for `project.afterEvaluate { ... }` hooks that run after configuration and defeat the build system's understanding. When you need custom logic, write a convention plugin (a `buildSrc` or included build plugin) that encapsulates it, rather than inlining imperative code in every project's build script. A build that reads top-to-bottom as configuration is maintainable; one that is a program is a liability.

### Make Builds Reproducible And Hermetic: Lock, Cache, And Pin

A reproducible build is one that, given the same source, produces the same output (or at least the same resolved dependency set) regardless of when or where it runs. Reproducibility fails when the build depends on the state of the local cache, the time of day (a dynamic version resolved to something newer), or the network (a dependency not cached). The defenses are: pin all dependency versions explicitly (no `LATEST`, no `RELEASE`, no `+` dynamic versions in Gradle), use dependency lock files (Gradle `--write-locks` / `gradle.lockfile`; Maven does not have native locks but the enforcer plugin's `requireUpperBoundDeps` and a checked-in `.mvn` can help), and make the build hermetic (all inputs are declared, the build does not reach out to the network at runtime except through the configured, cacheable repositories).

Reproducibility matters because non-reproducible builds break in CI, in air-gapped environments, and months later when a dynamic version moves. A `version = "1.+"` in Gradle or a `RELEASE`/`LATEST` in Maven resolves to whatever is newest at build time — fine for a throwaway, fatal for a released artifact whose build cannot be reproduced. Use the build cache (Gradle's local and remote build cache, Maven's incremental and the `mvnd` daemon) to speed builds, but understand that caching is correct only when task inputs are fully declared (a Gradle task that reads a file without declaring it as an input will cache incorrectly and produce stale output). Pin the JDK version (toolchains in both Maven and Gradle) so the build does not silently change language level with the developer's installed JDK.

### Manage The Multi-Module Reactor Deliberately

Both Maven and Gradle support multi-module builds (a reactor in Maven, a settings + included builds in Gradle), and the discipline is to keep module boundaries aligned with dependency boundaries. A multi-module build lets you build a set of related modules together with consistent versions and inter-module dependencies resolved to the locally-built version (not a published one). The traps are: circular dependencies between modules (A depends on B depends on A — the build system will refuse or produce confusing errors), inconsistent versions between modules (module A's published artifact is 1.2 but module B expects 1.1), and over-modularization (splitting one logical unit into many tiny modules that must always change together, multiplying build time and configuration overhead).

Keep modules at the granularity of genuinely independent deployables or clearly separated concerns, not one module per package. Use a parent POM (Maven) or a convention plugin / `buildSrc` (Gradle) to share configuration across modules so that the Java version, plugin versions, and common dependencies are declared once. Build the whole reactor in CI to catch inter-module breakage, but use incremental/incremental-build features to avoid rebuilding unchanged modules in local development. When a module is published, ensure its coordinates (groupId, artifactId, version) are consistent and its dependencies are either internal (same reactor, same version) or external (pinned), never ambiguous.

### Configure Plugins For The Output You Actually Need: Fat Jars, Shades, Reproducible Archives

The packaging step determines what artifact you ship, and the default (a thin jar of your classes, depending on dependencies on the classpath) is rarely what a deployed service needs. Spring Boot's plugin repackages into an executable fat jar; Maven Shade/Assembly and Gradle's `shadow`/`application` plugins produce fat jars or distributions; each has options for what to include, how to handle duplicate files, and whether to relocate (shade/rename) dependencies to avoid conflicts. Configure these deliberately: a fat jar that includes two versions of a dependency (because both were on the classpath) ships a conflict; a shaded jar that does not relocate a commonly-conflicting library (Guava, Netty) causes `NoSuchMethodError` when composed with other shaded jars in the same process.

Reproducible archives matter too: jar files include timestamps and ordering that can make the build output non-dreproducible byte-for-byte. Configure `reproducibleFileOrder` and `reproducibleTimestamp` (Maven jar plugin, Gradle's `reproducibleFileOrder`/`preserveFileTimestamps=false`) if byte-reproducibility matters (it does for supply-chain verification and for caching). Understand what your packaging plugin produces and verify it (open the jar, check the manifest, check for duplicate entries, check the dependency versions actually included) rather than trusting the default. A misconfigured fat jar is a deployment that fails at runtime in a way the local build never revealed.

### Keep Builds Fast: Parallelism, Caching, Daemon, And Avoiding Pathological Patterns

Build speed is a developer-experience and CI-cost concern, and slow builds compound across a team. The levers: parallel module building (`mvn -T 1C`, Gradle `--parallel`), the Gradle daemon and `mvnd` (keep the JVM warm), incremental compilation and task input tracking (Gradle is incremental by default; Maven needs plugin support), the build cache (Gradle local and remote cache; reuse outputs across machines), and avoiding pathological patterns. Pathological patterns include: a build that recompiles everything because task inputs are not declared; a build that runs all tests on every change because the test task is not incremental; a build that downloads dependencies on every CI run because the cache is not persisted; a build that does heavy work in `doFirst`/`doLast` hooks that bypass caching.

Profile the build (`gradle --profile` or `--scan`, `mvn -Dmaven.profile` / build-time profiling) to find where time goes before optimizing. The usual culprits are tests (run only affected tests locally, all tests in CI), dependency resolution (pin and cache), and non-incremental tasks. Do not sacrifice correctness for speed — a fast build that produces wrong output is worse than a slow correct one — but do not tolerate a multi-minute local build that should be seconds, because it drives developers to avoid running it.

## Common Traps

### Dynamic Versions (`LATEST`, `RELEASE`, `1.+`) Breaking Reproducibility

A dynamic version resolves differently over time or across machines. Pin explicit versions; use lock files or a BOM/catalog to keep them aligned.

### Version Skew Across A Library Family

Different modules pulling different Jackson/Spring/Mockito versions, causing `NoSuchMethodError` at runtime. Import a BOM (Maven) or platform/version catalog (Gradle) to align the family.

### Scattered Version Overrides Producing An Un-Auditable Graph

Version forced in five different places, so the "real" resolution depends on order. Manage versions in one place (BOM, catalog, parent POM, convention plugin).

### Imperative Gradle Scripting That Defeats Caching

`doLast { ... }` blocks and `afterEvaluate` hooks that do work the build system cannot track, causing incorrect caching or stale output. Use declarative configuration and convention plugins.

### Forgetting To Declare Task Inputs (Gradle)

A custom task that reads a file without declaring it as an input caches incorrectly, returning stale output. Declare all inputs and outputs; use the build scan to verify.

### Conflicting Transitive Versions Surfacing As NoSuchMethodError

Two paths pull different versions of a dependency, conflict mediation picks one, and code compiled against the other breaks at runtime. Inspect `dependency:tree` / `dependencies`; align with a BOM or force consistently.

### Misconfigured Fat/Shaded Jar Shipping Conflicts

A fat jar bundling two versions of a library, or a shaded jar not relocating a conflicting one. Verify the packaged jar's contents; relocate commonly-conflicting dependencies.

### Non-Reproducible Archive Contents

Jar timestamps and file order varying per build, breaking byte-reproducibility and caching. Set `reproducibleFileOrder`/`preserveFileTimestamps=false`.

### Building Everything When One Module Changed

No incremental/parallel build, so every change rebuilds the world. Enable parallelism, incremental compilation, the build cache, and the daemon; profile to find the slow spots.

### Assuming Maven And Gradle Resolve Conflicts The Same Way

Maven "nearest wins" vs Gradle "highest wins" produce different versions for the same graph. Inspect the resolved graph after any migration or cross-tool reasoning.

## Self-Check

- [ ] All dependency versions are explicit (no `LATEST`/`RELEASE`/`+`), the resolved graph is inspected (`dependency:tree` / `dependencies`) and understood, and any version override is in one canonical place rather than scattered.
- [ ] Version-aligned library families (Spring, Jackson, Mockito, protobuf, etc.) are imported via BOM (Maven) or platform/version catalog (Gradle), and versions are managed centrally so the build's versions are auditable from a few files.
- [ ] The build is declarative: Gradle uses the plugins DSL and Kotlin DSL where possible, custom logic lives in convention plugins rather than inline imperative scripting, and Maven relies on the standard lifecycle rather than ad-hoc plugin execution.
- [ ] The build is reproducible and hermetic: dependency lock files or equivalent pin the resolved set, the JDK version is pinned via toolchains, dynamic versions are absent, and the build does not depend on local cache state or undeclared network access.
- [ ] Multi-module boundaries align with genuinely independent concerns (not one module per package), there are no circular inter-module dependencies, and shared configuration lives in a parent POM or convention plugin.
- [ ] Packaging (fat jar, shade, Spring Boot repackage) is configured deliberately, the produced artifact's contents are verified (no duplicate/conflicting dependencies, correct manifest), and archive output is reproducible (file order and timestamps).
- [ ] Build speed has been profiled and addressed: parallelism, the daemon/mvnd, incremental compilation, the build cache, and persisted dependency caching in CI are used; pathological non-incremental patterns have been removed.
- [ ] The build was verified to be reproducible (same inputs → same resolved dependencies and, where required, same output bytes) across a clean cache and a second machine, and the Maven-vs-Gradle conflict-mediation difference was accounted for in any cross-tool reasoning.
