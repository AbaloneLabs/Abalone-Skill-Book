---
name: infrastructure_as_code_design.md
description: Use when the agent is provisioning cloud or infrastructure resources, writing Terraform / Pulumi / CloudFormation / CDK, deciding whether a resource should be managed as code or changed by hand, designing modules, managing state files and backends, handling state locks or corrupted state, promoting infrastructure across dev/staging/prod, detecting or remediating configuration drift, injecting secrets into IaC, importing manually-created resources, planning a rollback of infrastructure, or reviewing whether an infrastructure change is safe. Also covers declarative vs imperative provisioning, reproducibility, environment parity, drift, the risks of out-of-band manual changes, dynamic and external resources that resist IaC, and the boundary between IaC and runtime configuration. Use before any change to production infrastructure, when auditing infra reproducibility, or when a manual console fix has silently diverged from the code.
---

# Infrastructure As Code Design

Infrastructure as Code is the decision that every cloud resource — the database, the network, the IAM role, the load balancer, the DNS record — should be created and changed by versioned, reviewable, reproducible code rather than by a human clicking through a console. The judgment problem is not "can I write a Terraform file for this." It is two questions asked together: *should this resource be governed by code at all, and if so, how do I keep the code, the state, and the live infrastructure actually aligned over time?* IaC that is written once and then maintained by hand in the console is worse than no IaC, because it creates a false record: the code says one thing, the console says another, and nobody knows which is true.

Agents tend to treat IaC as a one-time authoring task — write the config, run `apply`, the resource exists, move on — because the resource appears immediately and the harm is invisible. The harm shows up later: a state file that drifts until `apply` wants to delete a production database; a manual "quick fix" in the console that the next deploy silently reverts or, worse, fails on; a secret committed to a repo because it was easier than wiring a secrets backend; an environment that cannot be recreated because three resources were hand-tuned and never recorded; a state file corrupted or lost because it lived on someone's laptop. The discipline of IaC is not writing the code; it is keeping code, state, and reality in a trustworthy relationship for the life of the system.

## Core Rules

### Decide Whether A Resource Belongs In IaC, And Make The Decision Explicit

Not everything should be governed by IaC, and pretending otherwise creates fragility. IaC is strongest for resources that are stable, enumerable, and meaningfully part of the system's architecture — and weakest for resources that are inherently dynamic, short-lived, or owned by another system. Before writing code for a resource, classify it:

- **Governs well in IaC:** networks, databases, storage, IAM and roles, load balancers, DNS zones, queues, Kubernetes clusters, the cloud accounts themselves. These change infrequently, have architectural significance, and benefit from review, versioning, and reproducibility.
- **Resists IaC (manage carefully or exclude):** ephemeral resources created at runtime by the application (a per-user bucket, a short-lived job, an auto-scaled node), resources owned and mutated by another control plane (Kubernetes managing its own objects, a platform managing its own tenants), and resources whose lifecycle is driven by data rather than code.
- **Must never be in IaC:** anything whose creation is itself the secret (raw credentials, private keys) — these are injected, not declared.

For resources that resist IaC, the strong choice is usually to let the owning system manage them (e.g., the app creates its per-tenant resources through an SDK) and keep IaC for the *container* (the cluster, the IAM permissions, the quotas). Mixing lifecycles in one IaC config produces configs that are half architecture and half runtime data, and the runtime half will fight the declarative model. Make the boundary explicit and document why each excluded resource is excluded.

### Treat The State File As A Critical, Shared, Append-Only Artifact

Most IaC tools track the relationship between your code and the live resources in a state file. The state is not a cache or a build artifact — it is the source of truth for what the tool believes exists, and it drives every `plan` and `apply`. Losing or corrupting state can mean the tool tries to recreate (and thus destroy) resources it thinks are missing. Treat state accordingly:

- **Store state in a shared, durable remote backend** (object storage with locking, a managed state service), never on a local disk. Local state means one person's machine holds the truth, cannot be shared, and is lost when the laptop is.
- **Enable locking.** Concurrent `apply` operations against the same state corrupt it. A backend that supports locking (DynamoDB lock, native locking) prevents two engineers from mutating the same infrastructure simultaneously.
- **Encrypt and restrict access.** State frequently contains sensitive values — secrets marked in config, resource passwords, connection strings — even when the source config uses secret variables. Treat the state backend as a secret store: encrypt it, restrict who can read it, and audit access.
- **Never hand-edit state to "fix" a mismatch.** Editing state directly to silence a diff is how production resources get orphaned or destroyed. If state and reality disagree, reconcile through the tool's import, move, and remove commands, and record why.

State is the most operationally dangerous part of IaC. A team that has never practiced state recovery will discover, during an incident, that a lost or locked state file blocks every infrastructure change.

### Treat The Console As Read-Only, And Make Out-Of-Band Changes An Incident

The single most common way IaC fails in production is a human making a "quick fix" directly in the cloud console — changing a security group, bumping an instance size, editing a rule — without updating the code. This is an out-of-band change, and it creates drift: the live infrastructure no longer matches the code, so the next `apply` either reverts the change (silently undoing the fix) or fails (because the resource is in a state the code does not expect). The longer drift persists, the more dangerous the next `apply` becomes.

- **Make the console read-only by convention and, where possible, by policy.** IAM permissions can deny direct mutation of IaC-governed resources, forcing changes through code.
- **When an emergency forces a console change, treat it as a debt that must be reconciled immediately.** The change must be back-ported into code (or the code adjusted) before the next deploy, and the drift reconciled deliberately rather than ignored.
- **Detect drift proactively.** Run periodic drift detection (the tool's `plan` with no changes, or a dedicated drift checker) and alert on unexplained drift, so out-of-band changes are found before they cause a surprise `apply`.

The rule: if a resource is in IaC, its only source of truth is the code. A console change to a governed resource is not a fix; it is a divergence that must be reconciled.

### Make Environments Reproducible From The Same Code, And Limit Hardcoded Differences

The value of IaC is that an environment can be recreated from code. That value is destroyed when environments diverge — when staging is "mostly like prod except for these hand-tuned things nobody wrote down." Design for parity:

- **Use the same modules and code paths across environments**, parameterized by environment-specific variables (account, region, sizing, feature flags). The differences between dev and prod should be a set of variables, not a different codebase.
- **Keep the differences meaningful and minimal.** Prod may have more replicas, larger instances, multi-AZ, and stricter retention; dev may scale to zero. But the *topology* — which resources exist and how they connect — should be the same shape, so staging actually tests what prod will do.
- **Avoid one-off resources that exist in only one environment.** A resource that is in prod but not in code (or in code but only for prod) breaks the reproducibility contract and is usually the source of "it worked in staging" failures.

The strong choice is one module, parameterized, applied N times — so that promoting from staging to prod is "apply the same code with prod variables," not "translate a different config." The weak choice is copy-pasted per-environment configs that drift independently and share nothing but resemblance.

### Modularize By Boundary, Not By File Count

Modules (or templates, or stacks) are the unit of reuse and review in IaC. A good module corresponds to a coherent architectural boundary — "a web service with its load balancer, IAM role, and alarms" or "a database with its networking and backups" — so that it can be reused, reviewed, and changed as a unit. A bad module is an arbitrary file split that forces every consumer to wire internal details.

- **A module should encapsulate a deployable unit or a coherent capability**, exposing inputs for the things that vary (size, name, tags) and outputs for the things consumers need (endpoint, role ARN). A module that exposes 40 inputs is not encapsulating; it is a parameterized copy.
- **Version modules like libraries.** Consumers should pin a module version, and module changes should be reviewable and backward-compatible (or clearly breaking with a version bump). A shared module changed without versioning can break every consumer's next `apply`.
- **Resist the "everything" module.** A monolithic module that defines the entire system is hard to review and hard to reuse. Split by boundary, and let composition (calling modules from a root config) build larger systems from coherent parts.

### Keep Secrets Out Of Code And Out Of State Where Possible

Secrets in IaC are a recurring incident source. A secret in a config file is in version control forever; a secret in state is readable by anyone with state access. The discipline:

- **Reference secrets, do not declare them.** Point IaC at a secrets manager (Vault, AWS Secrets Manager, cloud KMS) by name or ARN, and let the runtime fetch the value. The code records *which* secret, not *what* it is.
- **Mark sensitive values** so they are not printed in plans or logs, and understand that marking a value sensitive in the config does not remove it from state — it only hides it from output. State still needs protection.
- **Never commit real secrets.** Use placeholder or dummy values in examples and tests, and scan the repo for leaked credentials. A secret that was committed must be treated as compromised and rotated, not merely deleted from history.

### Validate Before Apply, And Make The Plan Reviewable

An IaC change that destroys a resource is often one misconfigured line away. The defenses are validation and review of the plan before anything touches production:

- **Validate statically** (format, type-check, policy-as-code checks for forbidden patterns — e.g., an S3 bucket without encryption, a security group open to 0.0.0.0/0) in CI, before a human ever sees a plan.
- **Review the plan.** The plan shows exactly what will be created, changed, and destroyed. For production, a human should read the plan and specifically confirm any destructive action (`destroy`, force-replace) is intended. Auto-apply to production without a reviewed plan is how a typo deletes a database.
- **Apply the same change to staging first** when feasible, so the plan's effects are observed before they reach prod. Parity (above) is what makes this meaningful.

### Plan Rollback As "Apply The Previous Code," And Know When It Cannot Work

IaC rollback is conceptually simple — revert the code and `apply` — but it shares the same trap as application rollback: state changes may not be reversible. Before relying on rollback:

- **Destructive resource changes are not cleanly reversible.** A change that replaced a resource (force-new) has destroyed the old one; reverting the code recreates it, but its data, its identity (IP, hostname), and its state are gone. Treat force-replace changes with the same care as destructive migrations.
- **External state may block rollback.** If the application has already written data assuming the new topology, reverting to the old infrastructure may leave the app unable to operate.
- **Keep prior states recoverable.** Remote backends with versioning let you roll back state, not just code. Know how to restore a prior state before you need to.

## Common Traps

### Writing IaC Once, Then Maintaining It In The Console

The config is written at launch, the resource is created, and every subsequent change is made by clicking in the console because it's "faster." The code becomes a stale lie, and the next `apply` is a gamble. If a resource is in IaC, the console is read-only; if the console is the real source of truth, remove the resource from IaC rather than leaving a false record.

### State On A Local Disk Or In The Repo

Storing the state file in the repository or on a laptop. It cannot be shared, it cannot lock, it gets merged-conflicted, and it leaks secrets into git history. State belongs in a shared, locking, encrypted remote backend.

### Hand-Editing State To Make A Diff Disappear

The plan shows a change the engineer does not want, so they edit the state file to make the diff go away. This orphans or misrepresents real resources and is the direct path to `apply` later destroying something. Reconcile through the tool's import/remove commands and record the reason; never edit state by hand.

### An Emergency Console Fix That Is Never Reconciled

A production issue is fixed in the console at 2am, the fix works, and nobody updates the code. The next deploy reverts the fix (or fails on the drifted resource), and the original incident recurs. Any console change to a governed resource must be tracked as debt and reconciled before the next apply.

### `apply` Without Reviewing The Plan In Production

Running `apply` directly, or auto-approving, without reading what will be destroyed. A single misconfigured `count` or `for_each` can delete every replica. For production, the plan is reviewed and destructive actions are confirmed; auto-apply is for non-production or for changes with no destructive component.

### Committing Secrets Because The Variable Was "Easier"

Putting the database password directly in a `.tfvars` or config file because wiring the secrets manager took longer. The secret is now in version control permanently. Reference secrets by pointer; never declare the value. If committed, rotate it — deleting it from history does not un-leak it.

### Treating State As Non-Sensitive Because The Config Uses Secret Variables and forcing Every Dynamic Resource Into IaC

The config reads the secret from a manager, so the engineer assumes state is clean. But state often captures resolved values, resource passwords, and connection strings regardless. Encrypt and restrict state access as if it contained the secrets, because it often does.

Trying to govern per-user, per-tenant, or ephemeral runtime resources in the same IaC as the architecture. The config balloons, `apply` slows, and the declarative model fights the dynamic lifecycle. Govern the container in IaC; let the owning system create the dynamic instances.

### Copy-Pasted Environments That Drift Independently and a Force-Replace Change Treated As Non-Destructive

Three near-identical config files for dev/staging/prod, each hand-edited over time, none matching the others. A fix applied to prod never reaches staging, and staging stops predicting prod. Use one parameterized module applied per environment; the differences should be variables, not separate codebases.

A config change that looks small but forces a resource to be replaced (e.g., changing an immutable attribute). The plan says "destroy then create," the engineer reads "change," and a stateful resource with data is destroyed. Read the plan's actions specifically, and treat force-replace of stateful resources as a destructive operation requiring migration planning.

## Self-Check

- [ ] Each resource was classified as governs-well-in-IaC, resists-IaC, or must-never-be-in-IaC; resources that resist IaC (dynamic, runtime, or owned by another control plane) are managed by their owning system with IaC governing only the container, and excluded resources are documented with a reason rather than left ambiguous.
- [ ] State is stored in a shared, durable, locking, encrypted remote backend — not on a local disk, not in the repository — and access to the state backend is restricted and audited because state may contain resolved secret values even when the config references a secrets manager.
- [ ] The cloud console is treated as read-only for governed resources (enforced by IAM policy where possible); any emergency out-of-band change is tracked as debt to be reconciled before the next apply, and drift detection runs periodically with alerts on unexplained divergence.
- [ ] Environments are reproducible from the same parameterized module/code applied per environment — differences are variables (sizing, region, retention), not separate divergent codebases, and topology is consistent so staging actually predicts prod.
- [ ] Modules correspond to coherent architectural boundaries, expose a minimal input/output surface, are versioned so consumers pin and changes are backward-compatible or clearly breaking, and there is no monolithic "everything" module or arbitrary file split.
- [ ] No real secrets are declared in code or committed to version control; secrets are referenced by pointer to a secrets manager, sensitive values are marked so they are hidden from plans and logs, and any previously committed secret was rotated rather than merely deleted from history.
- [ ] Changes are statically validated (format, types, policy-as-code forbidding unencrypted buckets or open security groups) in CI, and the plan is reviewed before any production apply — with destructive actions (destroy, force-replace) explicitly confirmed rather than auto-approved.
- [ ] Rollback was planned as "revert code and apply," and the cases where rollback cannot work (force-replace that destroyed a resource, external state written by the app) were identified in advance; prior states are recoverable from a versioned backend.
- [ ] No state file is hand-edited to silence a diff — mismatches between state and reality are reconciled through the tool's import/remove/move commands with the reason recorded.
