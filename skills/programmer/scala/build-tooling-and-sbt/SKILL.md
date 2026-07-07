---
name: scala_build_tooling_and_sbt.md
description: Use when the agent is setting up or configuring a Scala build (sbt build.sbt, project/ files, plugins in project/plugins.sbt), managing dependencies and resolver/repos, cross-building across Scala versions (crossScalaVersions, %%), publishing artifacts, configuring compilation flags (scalacOptions), using Bloop/Mill as alternatives, setting up multi-module projects, working with sbt tasks and commands, dealing with dependency conflicts and eviction, or is diagnosing "sbt is slow / recompiles everything", "dependency conflict / eviction warning", "cross-version suffix wrong", "plugin not applied", or build reproducibility issues. Covers sbt build structure and settings scoping, dependency management with %% and cross-building, scalacOptions and compiler flags, incremental compilation and the zinc cache, multi-module projects, plugins, and Mill/Bloop alternatives.
---

# Build Tooling And sbt In Scala

sbt is the dominant Scala build tool, and its power (a Scala DSL for builds, scoped settings, incremental compilation via Zinc) is also its complexity. Agents misunderstand settings scoping (a setting in `ThisBuild` vs a project vs a task scope behaves differently), get the cross-version suffix wrong (`%%` adds the Scala version, `%` does not, so a Java dependency uses `%`), trigger full recompiles by misconfiguring incremental compilation or by mutating shared state, and drown in dependency eviction warnings from conflicting transitive versions. The judgment problem is to structure the build correctly (settings at the right scope), to manage dependencies with the right resolver and cross-version operator, to set compiler flags deliberately, to keep incremental compilation effective, and to handle multi-module projects and dependency conflicts cleanly.

Agents put settings at the wrong scope, use `%` where `%%` is needed (or vice versa), ignore eviction warnings, or leave sbt slow because of full recompiles. The remedy is to scope settings deliberately, manage dependencies and versions explicitly, and keep Zinc incremental compilation healthy.

## Core Rules

### Scope Settings Deliberately (ThisBuild / Project / Task / Config)

sbt settings are scoped along three axes: project, configuration (Compile/Test), and task. A setting in `ThisBuild` applies to all projects in the build (good for `organization`, `scalaVersion`, common `scalacOptions`); a setting on a specific project applies only there; a setting scoped to a task (`Test / fork := true`) applies only to that task. Prepend the scope: `ThisBuild / scalaVersion := "2.13.12"`, `Test / fork := true`. Avoid the legacy bare `scalaVersion := ...` (ambiguous scope). When a setting seems ignored, it is almost always a scope mismatch — check which project/config/task it is set in and whether a more specific scope overrides it.

- Scope settings explicitly: `ThisBuild / x` for build-wide, `project / x` for a module, `Config / x` or `Task / x` for scoped.
- Common build-wide settings (`organization`, `scalaVersion`, scalacOptions) belong in `ThisBuild`.
- A "setting ignored" is usually a scope mismatch or a more specific scope overriding.

### Manage Dependencies With The Right Resolver And Cross-Version Operator

- `libraryDependencies += "org.typelevel" %% "cats-core" % "2.10.0"` — `%%` appends the Scala binary version (e.g., `_2.13`), for Scala libraries. Use `%%` for Scala dependencies.
- `libraryDependencies += "org.postgresql" % "postgresql" % "42.6.0"` — `%` (single) for Java/non-Scala dependencies (no version suffix).
- Cross-building: set `crossScalaVersions := Seq("2.12.18", "2.13.12")` and use `+publish`/`+test` to run across all. For Scala 3, the binary version is `3`.
- Resolvers: add `resolvers += ...` for non-Maven-Central repos (a private Nexus, a snapshot repo). Prefer Maven Central; add Sonatype snapshots/releases only when needed.
- Resolve dependency conflicts (eviction): sbt's `evictionError` or the `dependencyTree` plugin (sbt-dependency-graph) shows conflicting transitive versions. Pin a version with `dependencyOverrides += "x" %% "y" % "z"`, or use the stricter `evictionError := true` to fail on conflicts.

### Set Compiler Flags (scalacOptions) Deliberately

`scalacOptions` control warnings, features, and optimizations. Useful flags: `-deprecation` (warn on deprecated APIs), `-unchecked` (additional warnings), `-feature` (warn on feature implicits), `-Xlint`, `-Werror` (fail on warnings, strict projects), `-Ywarn-dead-code`, and for Scala 3 `-explain`, `-no-indent` toggles. Linting flags surface real bugs; `-Werror` enforces a clean build but must be paired with `@nowarn` for unavoidable warnings. Beware flags that are version-specific (a `-Y` flag in 2.12 may not exist in 2.13/3); guard with `scalacOptions ++= { if (scalaVersion.value.startsWith("2.13")) Seq(...) else Seq(...) }`. Do not blindly copy a flag list from a blog; understand each.

- Enable `-deprecation`, `-feature`, `-Xlint`, and consider `-Werror` for strictness, with `@nowarn` for unavoidable warnings.
- Guard version-specific flags (`-Y`/`-X`) by `scalaVersion` for cross-builds.
- Understand each flag; do not copy a flag list blindly.

### Keep Incremental Compilation (Zinc) Effective

sbt's Zinc incrementally recompiles only what changed, making the edit-compile cycle fast. Patterns that defeat it: (1) macros and large generated code that force recompilation of dependents; (2) changing a `build.sbt` or `project/` file reloads the build (slow); (3) `clean` (avoid unless necessary); (4) shared mutable state or non-hermetic tasks. To keep Zinc healthy: avoid unnecessary `clean`, structure modules so a change in one does not recompile the world (split modules by change frequency), and ensure `compile`/`Test/compile` are the tasks used (not `run` which may bypass). For speed, use Bloop (sbt-bloop generates Bloop config; Bloop is a faster compile server) or the Metals IDE backend.

- Avoid unnecessary `clean`; rely on Zinc incremental compilation.
- Split modules by change frequency so a change does not recompile the world.
- Use Bloop (`sbt-bloop`) or Metals for faster edit-compile cycles.

### Structure Multi-Module Projects Cleanly

A multi-module build has a root project aggregating sub-projects (`lazy val core = project.in(file("core"))`, `lazy val app = project.in(file("app")).dependsOn(core)`). `dependsOn` makes a module depend on another's sources (compile order, classpath); `aggregate` makes `sbt compile` run across all. Scope shared settings in `ThisBuild`; per-module settings in each module's `project`. Common patterns: `core` (domain, no deps), `db` (depends on core), `app` (depends on core, db, http). Keep the dependency graph acyclic and shallow; cycles or deep chains slow incremental builds.

- `lazy val x = project.in(file("x")).dependsOn(core).settings(...)`.
- `dependsOn` for source/classpath dependency; `aggregate` for running tasks across modules.
- Keep the module graph acyclic and shallow; shared settings in `ThisBuild`.

### Consider Mill Or Bloop For Specific Needs

sbt is the default, but Mill (a simpler, faster, file-oriented build tool) suits new projects wanting less magic, and Bloop (a compile server) accelerates IDE/CLI compile regardless of sbt/Mill. For publishing, cross-building, and complex plugin ecosystems, sbt remains the most supported. Choose by ecosystem needs: sbt for max compatibility/plugins, Mill for simplicity/speed in greenfield, Bloop as an accelerator on top of either.

## Common Traps

### Settings At The Wrong Scope

A setting in a project does not apply build-wide; a task-scoped setting is overridden by a more specific scope. Scope explicitly.

### Using % Where %% Is Needed (Or Vice Versa)

`%%` for Scala libs (version suffix), `%` for Java/non-Scala deps. Mixing breaks resolution.

### Ignoring Eviction / Dependency Conflicts

Conflicting transitive versions cause runtime `NoSuchMethodError`. Use `dependencyTree`/`evictionError`; pin with `dependencyOverrides`.

### Unnecessary clean Defeating Zinc

`clean` forces full recompilation. Rely on incremental Zinc; split modules to localize change.

### Version-Specific scalacOptions Breaking The Build

A `-Y` flag removed in a newer Scala version fails compilation. Guard by `scalaVersion`.

### Copying scalacOptions Blindly

A flag list from a blog may include deprecated/irrelevant flags. Understand each.

### Deep/Cyclic Module Graphs Slowing Builds

Deep `dependsOn` chains or cycles recompile broadly. Keep the graph acyclic and shallow.

### Treating sbt Reload As Cheap

Editing `build.sbt`/`project/` reloads the build (slow). Batch build changes.

## Self-Check

- [ ] Settings are scoped explicitly (`ThisBuild / x`, `project / x`, `Config/Task / x`); build-wide settings live in `ThisBuild`, and "ignored setting" issues are traced to scope mismatches.
- [ ] Scala dependencies use `%%` (version suffix), Java/non-Scala dependencies use `%`; cross-building uses `crossScalaVersions` and `+` tasks.
- [ ] `scalacOptions` include linting (`-deprecation`, `-feature`, `-Xlint`) and optionally `-Werror` with `@nowarn`, with version-specific flags guarded by `scalaVersion`.
- [ ] Incremental compilation (Zinc) is kept effective (no unnecessary `clean`, modules split by change frequency), and Bloop/Metals is used for fast cycles where applicable.
- [ ] Multi-module projects use `dependsOn` (source/classpath) and `aggregate` (cross-module tasks) with an acyclic, shallow graph and shared settings in `ThisBuild`.
- [ ] Dependency conflicts are surfaced (`dependencyTree`/`evictionError`) and resolved (`dependencyOverrides`), not left as eviction warnings.
- [ ] The build tool choice (sbt/Mill/Bloop) matches ecosystem and speed needs; resolvers are minimal (Maven Central first).
- [ ] Build reproducibility (pinned versions, hermetic tasks, no shared mutable state) has been considered.
