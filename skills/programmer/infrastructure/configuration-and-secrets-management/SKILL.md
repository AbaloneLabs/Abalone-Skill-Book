---
name: configuration_and_secrets_management.md
description: Use when the agent is designing how applications receive configuration and secrets, layering config by environment versus application versus team, deciding what is a secret versus ordinary config, planning secret rotation and dynamic short-lived secrets with Vault or a cloud secrets manager, injecting secrets into CI/CD pipelines and containers without leaking them, preventing plaintext secrets in git repos or environment variables or container images, diagnosing config drift across dev/staging/prod, treating feature flags as a config layer, or applying immutable-infrastructure principles to runtime configuration. Covers the configuration and secrets judgment problem — the boundary between config and secrets, rotation, drift, and the failure mode of secrets committed to source control — distinct from how infrastructure itself is delivered as code.
---

# Configuration And Secrets Management

Configuration and secrets look like the same problem — both are values the application reads at startup or runtime — and they are not. Configuration is the set of values that describe how the system should behave: timeouts, feature toggles, endpoint URLs, pool sizes, log levels. Secrets are the set of values whose disclosure is itself the harm: credentials, API keys, private keys, tokens. Treating them the same way produces one of two failures. Treat secrets like config and they end up in version control, in container images, in environment variables visible to every process and every log line, and a single leak compromises the system. Treat config like secrets and every harmless change — bumping a timeout, flipping a flag — requires a rotation procedure, an approval gate, and an outage while the change propagates. The discipline is to separate them cleanly, govern each by its actual risk, and never let the convenience of one pattern smuggle in the danger of the other.

Agents tend to reach for the easiest delivery mechanism: environment variables, because the twelve-factor app said so, and because every runtime reads them. Environment variables are fine for non-sensitive config and catastrophic for secrets — they are inherited by every child process, dumped into crash reports and debug output, leaked into CI logs by a forgotten `echo`, and impossible to rotate without a restart. The same convenience that makes environment variables attractive is what makes them a recurring incident source. The deeper failure is treating "where does the value come from" as the whole question, when the real questions are: is this value a secret, how does it rotate, who can read it, what happens when it changes, and how do we know the running system has the value we think it has. Configuration and secrets management is the engineering of those answers, not the choice of a delivery format.

This skill is about the boundary, lifecycle, and delivery of configuration and secrets at runtime. It is distinct from how infrastructure is delivered as code (`infrastructure-as-code-design`, `terraform-and-iac-patterns`), which governs the resources themselves; from network and security-group design (`network-and-security-groups`); and from Kubernetes workload config (`kubernetes-deployment-design`). Those decide what exists and how it runs. This skill decides what values the running system trusts and how those values are kept correct and secret over time.

## Core Rules

### Separate Secrets From Config By Disclosure Risk, And Govern Each By Its Own Rules

The first decision for every value is whether its disclosure is harmful. That decision determines everything else — where it is stored, who can read it, how it rotates, and how it is delivered. Conflating the two is the root of most config and secrets incidents.

- **Classify each value as secret or config.** A database password, an API key, a signing key, a TLS private key, an OAuth client secret — disclosure is the harm, so these are secrets, governed by rotation, access control, and audit. A timeout, a feature flag, an endpoint URL, a pool size, a log level — disclosure is harmless, so these are config, governed by change management and review. When unsure, default to secret; the cost of over-classifying is mild friction, the cost of under-classifying is a leak.
- **Store secrets in a secrets manager, never in the same store as config.** A secrets manager (Vault, AWS Secrets Manager, GCP Secret Manager, Azure Key Vault) provides encryption at rest, access audit, rotation hooks, and short-lived retrieval. A config file, a `.env`, a ConfigMap, or a CI variable marked "secret" does not provide these — it provides storage that happens to be less visible, which is not the same as managed. Put config in version-controlled config stores; put secrets in a secrets manager and reference them by pointer.
- **Reference secrets, do not inline them.** Whether in application code, IaC, a container manifest, or a CI workflow, the artifact should record *which* secret (a name, an ARN, a path) and let the runtime fetch the value. An inlined secret is in the artifact forever — in git history, in the image layer, in the CI log. A referenced secret can be rotated without touching the artifact.
- **Apply least privilege to secret access.** A secret should be readable only by the identity (service account, IAM role, pod identity) that needs it, for the window it needs it. A secret readable by every workload in the environment is a secret whose blast radius is the whole environment. Scope secret access as narrowly as the runtime allows.

### Layer Configuration So That The Right Values Reach The Right Environment Without Drift

Configuration that is flat — one bag of values applied everywhere — produces either drift (prod differs from staging by undocumented hand-edits) or rigidity (every environment is forced identical, including where it should differ). The strong pattern is layered configuration where each layer overrides or supplements the one below, and the differences between environments are explicit and minimal.

- **Layer by scope: defaults, environment, application instance, and overrides.** Sensible defaults live in the code or a base config; environment-specific values (endpoints, sizing, log verbosity) live in a per-environment layer; per-instance values (a specific replica's assignment) live at the top. Each layer overrides the previous, and the final resolved config is the composition. This makes the differences between environments a small, reviewable set of overrides rather than a separate copy of everything.
- **Make environment differences explicit and minimal.** Prod and staging should differ in the values that must differ (sizing, retention, real versus test credentials, region) and be identical in topology and shape. A staging that is "mostly like prod except for these hand-tuned things" stops predicting prod. The differences should be a documented set of override values, not a divergent codebase.
- **Detect and prevent config drift.** Drift is when the running config no longer matches the intended config — a hand-edited value, a stale override, a flag flipped in an emergency and never reconciled. Detect drift by resolving the intended config from source and comparing to what the running system reports, on a cadence, and alert on unexplained divergence. Config drift, like infrastructure drift, is found routinely or discovered during an incident.
- **Resolve config once, at a defined point, and make it observable.** Whether config is baked at build time, resolved at deploy time, or fetched at runtime, the resolution point should be explicit and the resolved values should be queryable (without exposing secrets) for debugging. A service whose running config cannot be inspected is a service whose behavior cannot be explained.

### Rotate Secrets, And Prefer Dynamic Short-Lived Secrets Over Long-Lived Ones

A secret that never changes is a secret that, once leaked, stays valid forever. Rotation bounds the window of exposure; dynamic secrets bound it to minutes. The rotation story is not optional — it is the difference between a leak that is contained and a leak that persists.

- **Every secret should have a rotation plan, even if rotation is infrequent.** The plan defines how the secret changes, who or what performs it, whether it requires downtime, and how consumers follow the new value. A secret with no rotation plan is a secret that will be leaked and then cannot be changed without an outage, which is exactly when it will be leaked.
- **Prefer dynamic, short-lived secrets where the system supports it.** Vault and several cloud secrets managers can issue credentials that are valid for minutes and tied to a specific lease or identity — a database login that expires, a short-lived token, a per-request credential. A short-lived secret that is leaked is useless within minutes; a static secret that is leaked is useful until a human notices and rotates it. Where dynamic secrets are feasible, they are strictly stronger than static ones.
- **Design consumers to follow rotation without a restart where possible.** If a secret rotation requires restarting every consumer, rotation becomes a coordinated outage and is therefore done rarely, which defeats the purpose. Consumers should periodically re-fetch secrets (or be notified of changes) and use the current value for new connections, so rotation propagates without a deploy. Where a restart is unavoidable, automate it so rotation is still routine.
- **Rotate on a schedule and on personnel change.** Scheduled rotation catches leaks that nobody noticed; rotation when a team member leaves or a vendor relationship ends revokes access that should no longer exist. Treat "this secret has not been rotated in a year" as a defect, not a sign of stability.

### Keep Secrets Out Of Git, Images, Env Vars, And CI Logs

The delivery path is where secrets leak, and each stage has a specific, recurring failure mode. The defenses are concrete and must be applied at every stage.

- **Never commit secrets to version control.** A secret in a commit is in the history forever, accessible to everyone with repo read, and survives deletion of the working copy. Use pre-commit hooks and secret-scanning CI checks to catch this before it lands; if a secret is committed, treat it as compromised and rotate it — removing it from history does not un-leak it.
- **Do not bake secrets into container images.** A secret in an image layer is in every pull of that image, in every registry it is pushed to, and survives the image's deletion from the local cache. Inject secrets at runtime (mounted volume, fetched from a manager) rather than building them in. An image should be identical across environments; environment-specific secrets are a runtime concern.
- **Avoid environment variables for secrets.** Environment variables are inherited by child processes, appear in process listings and crash dumps, and are trivially leaked by a debug statement or a misconfigured CI step that prints the environment. For non-sensitive config they are acceptable; for secrets, prefer a mounted file or a secrets-manager fetch that the application reads directly.
- **Scrub secrets from CI/CD logs and artifacts.** CI systems can mask known secret values in logs, but only values they know about; a secret printed in an unexpected form is printed in full. Design pipelines so secrets are injected as masked inputs, never echoed, and never passed to steps that log their arguments. Audit CI log retention for accidental exposure.

### Treat Feature Flags As Config With Its Own Lifecycle, And Apply Immutable-Config Principles

Feature flags are configuration that changes behavior at runtime without a deploy, and they carry their own risks: flags that stay on forever, flags whose state differs across environments with no record, flags that gate critical paths and are flipped without coordination. Treat flags as a config layer with governance, not as a free switch.

- **Flags are config, so manage them like config: versioned, reviewed, and eventually removed.** A flag that has been on for six months is no longer a flag; it is dead code that should be removed. A flag with no owner and no expiry accumulates until the codebase is incomprehensible. Give each flag an owner, a purpose, and a sunset date.
- **Distinguish kill switches from release flags from experiment flags.** A kill switch (disable a broken feature) must be fast and globally authoritative; a release flag (roll out a new feature gradually) must support targeting and percentage rollout; an experiment flag (A/B test) must be tied to measurement. Using one mechanism for all three produces a system that is bad at each.
- **Apply immutable-config principles where stability matters.** Immutable infrastructure means a deployed artifact is never modified in place — changes produce a new version. The same principle applies to config: rather than mutating a running service's config, ship a new resolved config and redeploy, so the running state is always traceable to a version. This trades some flexibility for reproducibility and auditability, and it is the right tradeoff for systems where "what config was running at the time of the incident" must be answerable.

## Common Traps

### Secrets In Environment Variables Because Twelve-Factor Said So

The twelve-factor guidance to put config in environment variables is sound for non-sensitive config and dangerous for secrets. Environment variables leak into child processes, crash reports, process listings, and CI logs, and they cannot be rotated without a restart. Use a secrets manager for secrets; reserve environment variables for harmless config.

### A Secret Committed "Just For Local Dev" Or "Just Once"

A database password is pasted into a `.env` committed to the repo, or a CI secret is echoed into a workflow file for debugging, and the secret is now permanent in history. Treat any committed secret as compromised and rotate it immediately; deleting the file or the commit does not revoke the value. Pre-commit hooks and secret scanning exist precisely because this happens constantly.

### Long-Lived Static Secrets With No Rotation Plan

A service account key created at launch, never rotated, with broad permissions, is the credential most likely to be leaked and most damaging when it is. Every secret needs a rotation plan; where possible, replace static secrets with dynamic short-lived credentials that expire in minutes.

### Config Drift Across Environments From Hand-Edits And Emergency Overrides

A flag is flipped in prod during an incident, the fix works, and the override is never reconciled into the config source. Prod now differs from staging by an undocumented value, staging stops predicting prod, and the next deploy either reverts the fix or fails. Track config overrides as debt reconciled into source, and detect drift on a cadence.

### Feature Flags That Become Permanent, Unowned, Or Divergent Across Environments

A flag is added for a rollout, the rollout completes, and the flag stays on forever with no owner and no removal plan. Or the flag is on in prod and off in staging with no record of why. Flags are config with a lifecycle: each needs an owner, a purpose, and a sunset date, and the state across environments must be deliberate and recorded.

### Secrets Baked Into Container Images For "Convenience"

A secret is added to an image layer so the image "just works" in a given environment. The secret is now in the registry, in every pull, and in every copy of the image, and it cannot be rotated without rebuilding and repushing. Inject secrets at runtime; an image should be environment-agnostic.

### One Shared Secret Readable By Every Workload

A single database credential stored as one secret, mounted into every service in the environment, so that any compromised service can reach the database as any other service. Scope secret access to the identity that needs it; the blast radius of a leaked secret is the set of workloads that can read it.

## Self-Check

- [ ] Every value was classified as secret or config by disclosure risk, secrets are stored in a secrets manager (not alongside config), secrets are referenced by pointer rather than inlined in code/IaC/images/CI, and secret access is scoped to the least-privilege identity that needs it.
- [ ] Configuration is layered (defaults, environment, instance, overrides) so environment differences are an explicit, minimal, reviewed set of overrides rather than divergent copies, staging and prod share topology and differ only in documented values, and config drift is detected on a cadence by comparing resolved config to running config.
- [ ] Every secret has a rotation plan defining how, when, and by whom it changes, dynamic short-lived secrets are preferred where the system supports them, consumers re-fetch secrets so rotation propagates without a forced outage, and rotation runs on a schedule and on personnel or vendor changes.
- [ ] No secrets are committed to version control (pre-commit hooks and secret scanning are in place, and any committed secret was rotated rather than merely deleted from history), no secrets are baked into container images, secrets are not delivered through environment variables, and CI logs and artifacts are scrubbed of secret values.
- [ ] Feature flags are treated as config with a lifecycle — each has an owner, a purpose, and a sunset date, kill switches and release flags and experiment flags use mechanisms suited to their purpose, and flag state across environments is deliberate and recorded rather than divergent by accident.
- [ ] Immutable-config principles are applied where reproducibility matters — changes produce a new resolved config and a redeploy rather than mutating a running service in place — so the config running at any past moment is traceable to a version.
- [ ] The resolved runtime config is observable for debugging (the running system can report its non-secret config values) without exposing secrets, so behavior can be explained and incidents can be diagnosed against the actual config that was in effect.
