---
name: infrastructure_as_code_and_config.md
description: Use when the agent is writing or reviewing infrastructure-as-code (Terraform, Pulumi, CloudFormation, Ansible), managing configuration (config files, environment variables, secrets), deciding what belongs in IaC vs application config vs runtime config, handling environment-specific configuration, secrets management, config drift, or designing the config/IaC workflow. Covers IaC principles (declarative, idempotent, versioned), config layering (build-time vs deploy-time vs runtime), secrets handling, drift detection, module composition, and the blast-radius and review discipline that IaC demands because it can destroy infrastructure as easily as create it.
---

# Infrastructure As Code And Config

Infrastructure-as-code (IaC) makes infrastructure a software system: declarative definitions, version control, review, and automated application. Its power is that infrastructure becomes reproducible, reviewable, and disposable — a new environment is a config change away, and drift from the intended state is detectable and correctable. Its danger is the same power applied in reverse: an IaC change can destroy production infrastructure as easily as it creates it, and because IaC applies changes automatically, a mistaken deletion, a broadened scope, or a misconfigured import can tear down databases, drop queues, or disconnect networks before anyone reviews the consequence. The defining discipline of IaC is treating it with the rigor of production code — versioned, reviewed, tested, and applied with blast-radius awareness — because the "deploy" of IaC modifies the foundation everything else runs on.

Agents tend to write IaC imperatively ("create this then that") rather than declaratively, to hardcode environment-specific values, to store secrets in plaintext IaC or config, and to apply changes without reviewing the plan of what will change. The judgment problem is recognizing that IaC is a high-blast-radius system whose correctness depends on declarative idempotence, that configuration has a layering (build, deploy, runtime) that determines what can change when, and that secrets, drift, and review are the failure surfaces that distinguish reliable IaC from dangerous IaC. This skill covers the discipline of infrastructure-as-code and configuration: declarative principles, config layering, secrets management, drift, module composition, and the review and blast-radius awareness the power demands.

## Core Rules

### Write IaC Declaratively, Idempotently, And Versioned

Declarative IaC (describe the desired state; let the tool reconcile) is more reliable than imperative (script the steps), and idempotence (applying the same config twice produces the same result) is what makes it safe to re-apply.

- **Declare the desired state, not the steps to reach it.** Describe what the infrastructure should look like (a database with these settings, a network with these subnets); let the IaC tool compute and apply the diff. Avoid imperative scripts that encode "create X, then modify Y" — they are not idempotent and drift from reality.
- **Ensure idempotence: applying the config repeatedly produces the same result.** A config that creates a resource on first apply but errors or duplicates on second apply is not idempotent and is unsafe to re-run. Test that re-applying a converged config is a no-op.
- **Version-control all IaC.** Infrastructure definitions belong in version control with the same review discipline as application code — pull requests, review, history. The version-controlled state is the source of truth; manual changes to infrastructure (click-ops) create drift that the next apply will revert or conflict with.
- **Separate the definition from the state.** The IaC definition (the code) describes the desired state; the actual state (what exists) is tracked by the tool (state file, backend). Protect the state (it is the source of truth for what exists); back it up; never lose it.

### Review The Plan Before Applying

Because IaC applies changes automatically, reviewing the plan (the diff between desired and actual state) before applying is the primary safeguard against destructive changes.

- **Always review the plan (terraform plan, pulumi preview) before applying.** The plan shows exactly what will be created, changed, and destroyed. A change that unexpectedly destroys resources is caught here, before it is applied. Never apply blindly without reviewing the plan.
- **Scrutinize deletions especially.** A resource marked for destruction in the plan is the highest-risk change — it may be a database, a queue with messages, a network dependency. Understand why each resource is being destroyed; a deletion due to a renamed resource or a scope change can destroy data if the tool treats it as destroy-and-recreate.
- **Bound the blast radius of a change.** A single IaC apply can touch many resources across many environments. Scope changes to the minimum affected resources; apply to one environment at a time; use the plan to confirm the scope before applying.
- **Require review and approval for production applies.** Production infrastructure changes should go through pull-request review (a human reviews the IaC diff) and the plan should be reviewed before the apply. Do not auto-apply to production without human review of the plan.

### Layer Configuration By When It Can Change

Configuration exists at multiple layers — build-time, deploy-time, runtime — and putting the wrong config at the wrong layer creates inflexibility or risk.

- **Build-time config (baked into the image/artifact): environment-agnostic, rarely changing.** Values that are the same across all environments (the application binary, default settings) are baked into the build artifact. Do not bake environment-specific values (database URLs, feature flags) into the artifact, or you need a different artifact per environment.
- **Deploy-time config (applied with the deployment): environment-specific, changing per release.** Values that differ by environment but are stable within a release (database endpoints, log levels, resource limits) are applied at deployment, via environment variables, config files, or IaC. This allows one artifact to deploy to many environments.
- **Runtime config (changeable without redeployment): dynamic, frequently changing.** Values that must change without a redeploy (feature flags, thresholds, dynamic settings) are fetched at runtime from a config service or feature-flag system. Reserve runtime config for what genuinely needs to change without a deploy; over-use creates unreviewed, unversioned configuration drift.
- **Keep the layering clear and documented.** Confusion about which layer holds a given config leads to values baked into artifacts that should be deploy-time, or runtime config that should be versioned. Document the layering; enforce it in review.

### Manage Secrets Securely, Never In Plaintext Config

Secrets (passwords, API keys, certificates) in plaintext IaC, config files, or version control are a security incident waiting to happen. They must be managed by a secrets manager.

- **Never store secrets in version control or plaintext config.** A secret committed to a repository is exposed to everyone with access, survives in history, and is a common breach source. Use placeholder references in IaC and config; resolve them at apply/deploy/runtime from a secrets manager.
- **Use a secrets manager (Vault, AWS Secrets Manager, GCP Secret Manager).** Secrets are stored in the manager, accessed by the application or IaC at runtime/apply time via fine-grained permissions. The manager provides rotation, auditing, and access control that plaintext cannot.
- **Grant least-privilege access to secrets.** Each application or service accesses only the secrets it needs; access is scoped by role or identity. Broad access to all secrets amplifies the blast radius of a compromised service.
- **Rotate secrets regularly.** Secrets that never rotate remain exposed if leaked. Rotate periodically and on personnel changes; design the application to handle rotation (fetch the secret at runtime, not bake it in).

### Detect And Correct Drift

Drift — the actual infrastructure diverging from the IaC definition (via manual changes, out-of-band fixes, or external modifications) — makes the IaC unreliable. Detecting and correcting it keeps the definition as the source of truth.

- **Detect drift by comparing actual state to the definition regularly.** Run the IaC tool's plan against the current state periodically; a non-empty plan (when no change was intended) indicates drift. Alert on drift so it is investigated.
- **Correct drift by reconciling, not by updating the definition to match the drift (unless intended).** If drift is unintended (a manual change), re-apply the definition to restore the intended state. If drift is intended (an emergency fix), update the definition to match, so the definition remains the source of truth.
- **Prevent drift by forbidding manual infrastructure changes.** Manual changes (click-ops in the cloud console) create drift that the next IaC apply reverts or conflicts with. Route all infrastructure changes through IaC, even emergencies (apply via IaC, then document).
- **Import existing infrastructure into IaC before managing it.** Applying IaC to infrastructure it does not know about can conflict or destroy it. Import existing resources into the IaC state before managing them, so the tool knows not to recreate them.

### Compose With Modules And Reusable Components

IaC at scale benefits from modularity — reusable components (modules) that encapsulate common patterns and are composed into environments.

- **Use modules for repeated infrastructure patterns.** A standard VPC, a standard database setup, a standard service deployment — encapsulate these as modules, parameterized for the specific use. Modules reduce duplication, enforce consistency, and allow a fix to one pattern to propagate to all uses.
- **Version modules to allow controlled evolution.** A module that changes affects all environments using it. Version modules (pin to a specific version per environment) so environments update deliberately, not when the module changes.
- **Test modules in isolation.** A module is reusable infrastructure code; test it (apply it to a test environment, verify the result) before relying on it in production. A broken module breaks every environment using it.
- **Do not over-modularize early.** Modules add indirection; premature modularization creates abstraction before the pattern is stable. Extract a module when a pattern is repeated and stable, not speculatively.

## Common Traps

### Applying Without Reviewing The Plan

Applying IaC changes without reviewing the plan, missing a destructive change (a resource marked for destroy). Always review the plan; scrutinize deletions.

### Imperative Or Non-Idempotent IaC

Scripting steps rather than declaring state, or configs that error or duplicate on re-apply. Write declaratively; ensure idempotence.

### Secrets In Plaintext Config Or Version Control

Secrets committed to the repository or stored in plaintext config, exposing them to breach. Use a secrets manager; reference, do not inline.

### Environment-Specific Values Baked Into Artifacts

Database URLs or feature flags baked into the build artifact, requiring a different artifact per environment. Layer config: environment-specific values at deploy or runtime, not build.

### Drift From Manual Changes

Manual infrastructure changes (click-ops) that diverge from the IaC definition, causing conflicts or reversion on the next apply. Route all changes through IaC; detect and correct drift.

### Unbounded Blast Radius

A single apply touching many resources across environments, amplifying a mistake. Scope changes minimally; apply per-environment; use the plan to confirm scope.

### Runtime Config For Values That Should Be Versioned

Feature flags or settings changed at runtime without review or versioning, creating untracked configuration drift. Reserve runtime config for what genuinely needs dynamic change; version the rest.

### Lost Or Corrupted State

The IaC state file lost or corrupted, making the tool unaware of existing infrastructure (risking recreation or destruction). Protect and back up the state.

## Self-Check

- [ ] IaC is written declaratively (desired state, not steps), is idempotent (re-applying a converged config is a no-op), is version-controlled with review discipline, and the state is protected and backed up.
- [ ] Every apply is preceded by a reviewed plan (terraform plan, pulumi preview), deletions in the plan are scrutinized and understood, the blast radius is bounded (minimal scope, per-environment), and production applies require human review and approval.
- [ ] Configuration is layered correctly: build-time config is environment-agnostic (baked into the artifact), deploy-time config is environment-specific (applied at deployment), runtime config is reserved for genuinely dynamic values, and the layering is documented and enforced.
- [ ] Secrets are never in plaintext config or version control — they are stored in a secrets manager, referenced by IaC and config, accessed with least-privilege, and rotated regularly with the application designed to handle rotation.
- [ ] Drift is detected (regular plan comparison, alerting on unintended divergence), corrected by reconciling to the definition (or updating the definition if the drift is intended), and prevented by routing all infrastructure changes through IaC rather than manual console changes.
- [ ] Repeated infrastructure patterns are encapsulated as versioned, tested modules, composition is deliberate (not over-modularized prematurely), and existing infrastructure is imported into IaC state before being managed.
- [ ] The IaC workflow includes testing (apply to a test environment first), staged promotion (dev to staging to production), and rollback capability (the previous version of the IaC can be re-applied to revert).
- [ ] The blast-radius awareness extends to the tool's destructive capabilities — imports, scope changes, force-destroy flags, and resource replacement are understood and reviewed — because IaC can destroy infrastructure as easily as it creates it.
