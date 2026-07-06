---
name: config_management_and_hierarchy.md
description: Use when the agent is designing a configuration system, layering defaults with files and environment variables, validating configuration at startup, separating environments, deciding what should be static versus dynamic, diagnosing config drift across environments, or taming a configuration surface that has grown too large.
---

# Config Management And Hierarchy

Configuration is where the codebase meets the unpredictable realities of where it runs. The same artifact must behave differently in staging and production, must connect to different databases, must expose different feature sets, and must do all of this without being rebuilt. Configuration is the mechanism for that variation, and the way it is structured determines whether the system is operable or opaque. Done well, configuration is explicit, validated, and traceable: an operator can see exactly what a running instance believes and why. Done poorly, it is a tangle of overlapping sources, silent overrides, and values that differ across environments for reasons no one can reconstruct — the classic "works in staging, breaks in production" failure, rooted in configuration that was never made legible.

Agents tend to treat configuration as a bag of values to be read wherever needed, with no overall model of precedence, validation, or source of truth. Environment variables override files that override defaults in an order no one documented; invalid values crash the system deep in startup or, worse, silently misbehave; the same setting means different things in different places. The judgment problem is to design configuration as a deliberate hierarchy with clear precedence, to validate it at the boundary so failures are loud and early, to keep environments aligned so differences are intentional rather than accidental, and to resist the tendency for the configuration surface to grow until it is unmaintainable.

## Core Rules

### Define A Clear Precedence Hierarchy And Document It

When configuration can come from multiple sources — defaults, files, environment variables, command-line flags, dynamic stores — the system must apply them in a defined, documented order, and that order must match operator intuition. The common and robust hierarchy, from lowest to highest precedence: compiled-in defaults, then config files, then environment variables, then command-line flags or dynamic overrides. Each higher layer overrides the lower for the same key, and the final value is the result of applying the hierarchy.

Documenting the hierarchy is not optional. An operator debugging a misconfigured instance needs to know which source wins when two disagree; without a documented precedence, the answer is "read the code," which is unacceptable for an operable system. State the hierarchy in one place, and make the running instance able to report the resolved value and its source for each key, so the question "where did this value come from?" has an answer that does not require guessing.

### Validate Configuration At The Boundary, Not Deep In The Code

Invalid configuration should fail loudly and early, at startup or at the moment a value is read, with a message that names the key, the invalid value, and what was expected. The failure mode to avoid is the silent or late one: a malformed value that is accepted at load time and causes a confusing error deep in a request path, or worse, causes subtly wrong behavior that no one notices until it has corrupted data.

Treat configuration parsing as a transformation from untyped input (strings from files and environment) into typed, validated values. Validate types, ranges, required-versus-optional, mutual exclusions, and cross-key constraints (e.g., "if TLS is enabled, a cert path is required") at the boundary, and produce a fully resolved, validated configuration object that the rest of the code consumes. Code that uses configuration should receive typed values, not raw strings to parse and check repeatedly. This concentrates validation in one place and makes misconfiguration a startup error rather than a runtime surprise.

### Keep Environments As Similar As Possible

The most dangerous configuration problems come from environments that differ in ways no one intended. A system that passes every test in staging and then fails in production usually does so because production differs from staging in some configuration the tests never exercised: a different feature flag, a different timeout, a different dependency version, a different data shape. The goal of environment management is to minimize these differences so that behavior observed in one environment predicts behavior in another.

Strategies for parity:

- **Use the same configuration schema and code paths in every environment**, varying only the values. Avoid environment-specific code branches that exist only in production.
- **Make every environment difference explicit and version-controlled**, not the result of manual edits to a running system. Configuration that exists only in someone's memory or in a manual console edit is invisible and drifts.
- **Minimize the number of differences.** Each difference is a place staging fails to predict production. The ideal is that environments differ only in values that must differ (resource addresses, credentials, scale), not in structure or behavior.
- **Test against production-like configuration** where feasible, so the configuration itself is exercised, not just the code.

Configuration drift — the slow divergence of environments through untracked changes — is a leading cause of production-only bugs. Treat drift as a defect to be detected and corrected, not as a normal state.

### Separate Static Configuration From Dynamic Configuration

Configuration varies not only in source but in when it can change. **Static configuration** is fixed at startup and changing it requires a restart or redeploy — resource addresses, thread pool sizes, schema definitions. **Dynamic configuration** can change while the system runs — feature flags, thresholds, routing weights. These two have different reliability profiles and must be handled differently.

Static configuration is simple and safe: it is read once, validated, and immutable for the process lifetime, so the code can rely on it not changing. Dynamic configuration is powerful but dangerous: it can change at any moment, so code that reads it must be prepared for the value to change between reads, and the system must handle the transition gracefully (a new value taking effect mid-operation, a flag flipping while a request is in flight). Be explicit about which settings are static and which are dynamic, and design dynamic settings with their mutability in mind — atomic reads, sensible transitions, and clear semantics about when a change takes effect.

Resist the temptation to make everything dynamic for flexibility. Dynamic configuration adds complexity (caching, propagation, consistency across instances) and risk (a bad change takes effect immediately in production). Make a setting dynamic only when its value must change without a redeploy; otherwise keep it static and simple.

### Centralize And Bound The Configuration Surface

As a system grows, configuration accumulates. Every tunable becomes a setting, every environment difference becomes a key, and over time the configuration surface becomes so large that no one understands it, defaults are wrong, and operators cannot predict the effect of a change. Configuration explosion is a real and common form of technical debt.

Counteract it with discipline:

- **Centralize configuration definition** in one schema or module, so the full surface is visible and so adding a setting is a deliberate act in a known place, not a scattered `getenv` call.
- **Prefer fewer, well-chosen defaults over more knobs.** Every setting is a decision an operator must make correctly; a sensible default removes that burden. Add a setting only when the value genuinely varies across deployments and cannot be derived.
- **Remove settings that are no longer used or that should never vary.** A setting that always has the same value is a hardcoded value in disguise, and a setting no one sets is dead surface.
- **Group related settings** so the surface is comprehensible (database settings together, feature settings together) rather than a flat namespace of hundreds of keys.

A configuration surface that an operator can hold in their head is operable; one they cannot is a source of incidents.

### Respect Scope and Escalation Boundaries

Know where the agent's authority and competence end. When the question requires a license, a specialist's judgment, a final approval, or expertise the agent does not hold, the correct action is to escalate rather than to produce a confident answer that overreaches. Scope discipline protects the recipient from harm caused by an unqualified conclusion and protects the agent from liability. State explicitly when the output is advisory and must be confirmed by the qualified person.

## Common Traps

### Undocumented Or Surprising Precedence

When multiple sources can override each other and the order is undocumented, operators cannot predict which value wins, leading to misconfiguration that is discovered only when something breaks. Document the hierarchy and let the running system report each resolved value's source.

### Late Or Silent Validation Failure

Configuration that is parsed lazily or not validated at the boundary fails deep in a request path or causes silent misbehavior, far from the misconfigured key. Validate at startup into a typed, checked object so misconfiguration is a loud, early error.

### Environment Drift From Untracked Changes

Configuration that differs across environments because of manual edits, console tweaks, or forgotten differences produces production-only bugs that staging cannot catch. Make every difference explicit and version-controlled, and minimize the number of differences.

### Making Everything Dynamic For Flexibility

Dynamic configuration adds caching, propagation, and consistency complexity and lets a bad change take effect immediately in production. Make a setting dynamic only when it must change without a redeploy; keep the rest static.

### Configuration Explosion

Adding a setting for every tunable produces a surface so large that no one understands it, defaults go wrong, and operators cannot predict the effect of changes. Prefer sensible defaults over knobs, centralize the schema, and remove unused or invariant settings.

### Reading Raw Strings Throughout The Code

Code that re-parses and re-checks configuration values at every use site duplicates validation and invites inconsistency. Parse and validate once at the boundary into typed values the rest of the code consumes.

### Secrets Handled Like Ordinary Config

Treating secrets (passwords, keys, tokens) as ordinary configuration values leads to them being logged, exposed in error messages, or stored in version control. Handle secrets through dedicated secret management with restricted access, audit, and rotation, distinct from ordinary configuration.

## Self-Check

- [ ] A documented precedence hierarchy defines how defaults, files, environment variables, flags, and dynamic sources combine, and the running instance can report each resolved value and its source.
- [ ] Configuration is parsed and validated at the boundary into a typed, checked object, so invalid values fail loudly at startup with a message naming the key, value, and expectation — not deep in a request path.
- [ ] Environments share the same schema and code paths, differing only in values, and every difference is explicit and version-controlled rather than the result of manual edits.
- [ ] Each setting is explicitly static or dynamic; dynamic settings are designed for mutability (atomic reads, graceful transitions, clear effect timing), and settings are not made dynamic merely for flexibility.
- [ ] The configuration surface is centralized in one schema, bounded in size, favors sensible defaults over knobs, and unused or invariant settings have been removed.
- [ ] Cross-key constraints and mutual exclusions are validated at the boundary, not left for runtime discovery.
- [ ] Secrets are handled through dedicated secret management distinct from ordinary configuration, with restricted access and no logging of secret values.
- [ ] An operator can determine, without reading the code, what a running instance believes and where each value came from.
