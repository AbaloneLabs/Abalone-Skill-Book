---
name: terraform_and_iac_patterns.md
description: Use when the agent is writing Terraform or Pulumi configurations and needs concrete patterns for state isolation, module versioning and composition, variable validation, resource dependencies, drift remediation, remote backend setup, environment separation with workspaces or terragrunt, or wiring plan and apply into CI. Covers Terraform workspaces versus separate state per environment, moved blocks and state import, module sources and registry versioning, sensitive outputs, depends_on and replace_triggered_by, S3 with DynamoDB locking, Terraform Cloud, and the plan review workflow. Distinct from the general infrastructure-as-code-design skill which covers whether a resource belongs in IaC at all.
---

# Terraform And IaC Patterns

The companion skill `infrastructure-as-code-design` covers the philosophy: whether a resource belongs in code, why state must be remote and shared, why the console is read-only, and why environments should be reproducible. This skill is the concrete implementation layer underneath that philosophy — the specific Terraform and Pulumi patterns that keep a real codebase safe as it grows past a few resources. An agent can fully agree with "store state remotely and isolate environments" and still ship a `terraform apply` that destroys a production database, because the concrete choices — one workspace or many, one state file or several, how modules are versioned, whether a refactor uses `moved` blocks or a blind `import`, how `replace_triggered_by` is wired — are where the harm actually happens.

Agents tend to reach for whatever pattern the first tutorial showed: a single state file, `terraform workspace` switched per environment, modules pulled from a git branch at `main`, variables typed as `any`, `depends_on` sprinkled where a dependency looks unclear. Each of these is defensible in a toy project and wrong at scale. A single state file makes every change a blast-radius gamble; workspace-per-environment shares a config but hides which environment a change hits; unpinned modules break every consumer when the branch moves; loose typing turns plan review into guesswork. The discipline here is to choose each pattern deliberately, knowing the failure mode it introduces, rather than inheriting it by default.

The judgment problem is not "can I express this in HCL." It is choosing the state shape, the module boundaries, the dependency declarations, and the apply workflow so that a refactor, a new environment, or a drift event is a routine, reviewable operation instead of an incident.

## Core Rules

### Choose State Isolation Strategy By Blast Radius, Not By Convenience

The single most important Terraform decision is how many state files exist and what each one owns. State is the unit of blast radius: every resource in one state file is at risk from a single bad `apply`. The strong pattern is to split state by blast radius and by change cadence, not to put everything in one file.

- **One state per environment, at minimum.** Dev, staging, and prod must never share a state file. A typo in a dev apply must be incapable of touching prod. This is non-negotiable; sharing state across environments is how a `count = 0` mistake deletes production.
- **Split state by independent subsystem or team boundary.** Beyond environment, split by component that changes independently — networking, the data tier, the application platform. The test is "could this component be applied without the others, and would a failure here be contained?" If yes, it earns its own state.
- **Prefer separate directories (or separate backend keys) over `terraform workspace`.** Workspaces share one configuration and one module set and differ only by a workspace label. They are convenient for ephemeral, identical environments (preview environments, per-PR stacks) but dangerous for long-lived dev/staging/prod because the workspace name is the only thing separating a prod destroy from a dev destroy, and it is easy to be in the wrong workspace. For long-lived environments with real differences, use separate directory trees (e.g., `envs/prod/`, `envs/staging/`) each with its own backend configuration. This makes the target environment explicit in the path and the command.

The weak choice is one giant state for the whole org because it is easiest to set up; the strong choice is several small states composed by remote-state references or by a thin root module, each independently applicable and independently recoverable.

### Configure Remote Backends With Locking And Encryption, And Make The Backend Itself Code

The backend configuration — where state lives and how it locks — is itself infrastructure that should be managed deliberately. The common production pattern on AWS is an S3 bucket for state with a DynamoDB table for locking, both encrypted, with versioning and lifecycle policies on the bucket. On Terraform Cloud or a managed runner, state and locking are handled by the platform.

- **Locking is mandatory.** Without it, two concurrent applies corrupt state. DynamoDB locking (or the platform equivalent) is not optional for any state more than one person touches.
- **Encrypt state at rest and in transit, and restrict who can read it.** State contains resolved secret values even when the config references a secrets manager. The bucket policy should deny public access, require TLS, and limit principals. See `infrastructure-as-code-design` for why state is a secret-adjacent artifact.
- **Bootstrap the backend outside the state it serves.** The bucket and lock table cannot manage their own existence cleanly. Bootstrap them with a separate, small config (or manually, once, with the action recorded) and treat that bootstrap as immutable thereafter. Trying to import the state bucket into the state it stores creates a chicken-and-egg trap.
- **Enable versioning on the state object store.** Versioning is what makes state recoverable after corruption or an accidental destructive apply. Without it, a lost state is gone.

### Refactor With moved Blocks And import, Never With Blind Re-Apply

When resources move between modules, change addresses, or need to be brought under management for the first time, the tooling exists to do this without destroy/create. The rule is to always tell Terraform about the move explicitly so the plan shows zero changes, rather than letting it interpret a refactor as "delete the old, create the new."

- **Use `moved` blocks for in-code refactors.** When you rename a resource or move it between modules, add a `moved { from = ... to = ... }` block. The next plan reads "has moved to" instead of "will be destroyed / created." This is the difference between a safe rename and an outage.
- **Use `terraform import` (or `pulumi import`) to adopt existing resources** rather than recreating them. Importing brings an unmanaged resource under state without touching it. Always run a plan immediately after import to confirm state matches config before any apply.
- **Use `removed` blocks or `terraform state rm` to detach without destroying** when a resource should leave Terraform's management but keep existing in the cloud (for example, handing ownership to another system). The default `delete` removes the resource from cloud; `rm`/`removed` removes it from state only. Confusing these is a classic data-loss path.
- **Never edit state by hand to make a diff disappear.** This is restated from the companion skill because it is the recurring failure mode. Reconcile through `moved`, `import`, `state rm`, with the reason recorded in the commit.

### Version And Compose Modules Like Libraries

Modules are the reuse and review unit. A module that is consumed by more than one root config should be versioned and pinned; a module consumed once can live in the same repo. The decision of when to extract a module is a judgment call about reuse and review boundaries.

- **Pin module versions.** Consumers should reference a module at a tagged version or a specific commit, never at a moving branch like `main`. A shared module changed on `main` silently changes every consumer's next plan. The strong pattern is a semver tag in a registry or a git ref.
- **Choose module source deliberately.** The Terraform Registry, a private registry, a git repo, or a local path each have tradeoffs. Registry modules are easy and discoverable but you depend on an upstream maintainer; git gives control but you own versioning; local paths couple consumer and provider release cadence. For anything production-critical and shared, prefer a source you control and pin it.
- **Compose modules in a thin root.** A root config should read inputs (tfvars), call a few modules, and wire their outputs together. Business logic does not belong in the root. The root is the composition layer; the modules are the capability layer.
- **Split a module when it has a distinct, independently reusable intent** — not when a file gets long. A "network" module and a "database" module split cleanly; splitting "database" into "database-main" and "database-outputs" does not. See the companion skill's modularization guidance.

### Type And Validate Inputs, And Mark Sensitive Outputs Explicitly

Variables and outputs are the contract of a module. Loose typing makes plans unreadable and lets bad values reach `apply`. The strong pattern is explicit types, validation rules, and deliberate sensitivity.

- **Give every variable a concrete type, not `any`.** `any` defeats plan review because the plan cannot show whether a value is wrong until apply fails. Use `string`, `number`, `list(...)`, `object(...)`, and prefer `object` with required keys over untyped maps.
- **Add `validation` blocks for invariants the type cannot express** — CIDR ranges, allowed regions, name formats, port ranges. Validation runs at plan time and turns a class of apply-time failures into plan-time failures, which are cheap.
- **Mark sensitive variables and outputs.** A value marked `sensitive` is hidden from plan output and logs. But remember (per the companion skill) this only hides it from display; it is still in state. Marking sensitivity is necessary hygiene, not a secrecy solution.
- **Type outputs deliberately and document them.** Outputs are the module's public API. Changing an output's shape is a breaking change to consumers; version the module when you do.

### Declare Dependencies Explicitly Only When Implicit Detection Fails

Terraform builds a dependency graph from attribute references. Most of the time you do not need `depends_on`, and adding it unnecessarily hides the real data flow and can force serial application that could be parallel. But there are cases where the graph cannot be inferred and an explicit declaration is correct.

- **Prefer implicit dependencies.** If resource B references `aws_a.foo.id`, the dependency is correct and visible. This is the strong choice.
- **Use `depends_on` only when there is no attribute reference but an ordering requirement exists** — for example, a resource that depends on a null_resource provisioner completing, or a service that must exist before a policy is attached but shares no referenceable attribute. Document why in a comment, because `depends_on` is otherwise invisible and confusing.
- **Use `replace_triggered_by` and lifecycle `triggers` to force replacement on a related change.** When a resource must be recreated because something it does not reference changed (a launch template hash, a config hash), wire a `triggers` map or `replace_triggered_by` so the replacement is driven by data, not by a manual `terraform taint`. This is the modern, declarative way to express "redeploy when X changes."

### Make Drift Detection And Plan Review A Routine Workflow, Not An Incident Response

Drift is inevitable; the question is whether you find it routinely or only when an apply fails. Build drift detection into the regular workflow.

- **Run `terraform plan` on a schedule against each state** (daily or per-merge) with no changes applied, and alert on any non-empty plan. This catches out-of-band console changes before they compound.
- **Treat drift as a signal to reconcile, not to auto-correct.** A drift plan that says "will change X back" might be reverting an intentional emergency fix. Drift should be reviewed by a human who decides whether to update the code or update the cloud — never blindly applied.
- **Integrate plan into CI on every pull request.** A PR that changes infra should post the plan as a comment, so reviewers see the exact create/change/destroy actions. The plan is the artifact under review; the code is only what produces it.
- **Gate production apply behind a reviewed plan.** Auto-apply is acceptable for ephemeral or non-production states. For production, a human confirms the plan, and destructive actions (destroy, force-replace) get explicit attention.

## Common Traps

### Using `terraform workspace` For Long-Lived Dev/Staging/Prod

Workspaces feel like free environment separation, but the workspace name is a runtime switch with no presence in the code or the path. An engineer in the wrong workspace runs `apply` against prod thinking it is dev. Workspaces are good for short-lived, identical, ephemeral environments; for long-lived environments with real differences, use separate directories with separate backend configs so the target is structural, not a label.

### One Monolithic State File For The Whole Organization

Everything in one state means every apply risks everything, `plan` is slow, and a single corrupted state blocks all changes. Split by environment first, then by independent component. The convenience of "one state" is paid for the first time a bad apply touches an unrelated subsystem.

### Unpinned Module Sources On A Moving Branch

Referencing a shared module at `branch = "main"` means any merge to that module changes every consumer's next plan, with no review at the consumer side. Pin modules to a tag or commit SHA. When the module needs to change, the consumer bumps the pin in a reviewed change.

### Letting A Refactor Show "Destroy Then Create" Without A moved Block

A resource is renamed or moved between modules, the plan shows "will destroy old / create new," and the engineer applies it because the names look equivalent. For a stateful resource this destroys data and identity (IP, hostname). Always use a `moved` block so the plan shows "has moved" with zero changes; if the move is cross-state, use `state mv` / import deliberately and verify the plan is clean before applying anything else.

### Importing A Resource And Applying Without A Clean Plan

`terraform import` writes to state but does not reconcile config. If the config does not match the imported resource, the very next apply will "fix" the resource to match config — potentially destroying attributes or the resource itself. After any import, run plan and confirm it shows no unexpected changes before applying.

### `terraform state rm` When You Meant `terraform destroy`, Or Vice Versa

`state rm` detaches a resource from state but leaves it running in the cloud (it becomes unmanaged and orphaned). `destroy` deletes it from the cloud. Confusing them either orphans a resource you meant to delete or deletes a resource you meant to keep. Know which you intend; the `removed` block with `destroy = false` makes the intent explicit in code.

### Variables Typed As `any` Reaching Apply Before Anyone Notices They Are Wrong and treating `sensitive = true` As A Secrecy Guarantee

`any` skips validation, so a malformed value passes plan and fails at apply — or worse, succeeds with the wrong value and creates a misconfigured resource. Type every variable and add `validation` for invariants. The cost is a few lines; the benefit is failures at plan time instead of in production.

Marking a variable sensitive hides it from plan output but the value still lands in state, which is readable by anyone with state access. Sensitivity is display hygiene, not access control. Combine it with a restricted, encrypted state backend and prefer referencing secrets from a manager rather than passing them through Terraform at all.

### Forgetting `replace_triggered_by` On Resources That Depend On A Hash and auto-Applying In CI Without A Reviewed Plan For Production

A launch template, init script, or config map changes, but the instances it configures are not recreated because Terraform sees no reference change. The deploy "succeeds" and nothing actually rolls. Wire `replace_triggered_by` or a `triggers` hash so the dependency is data-driven and the replacement happens automatically.

CI is wired to apply on merge, the plan is not posted or reviewed, and a destructive change ships unchallenged. For production, the plan must be a reviewed artifact (posted as a PR comment, approved) and destructive actions must be confirmed. Auto-apply belongs to non-production or no-destructive-component changes only.

## Self-Check

- [ ] State is split by blast radius — at minimum one state per environment, and further split by independent component — and long-lived dev/staging/prod use separate directory trees with separate backend configurations rather than relying on `terraform workspace` labels that can be switched by mistake.
- [ ] Every state uses a remote backend with locking enabled (DynamoDB lock or platform equivalent), encryption at rest and in transit, versioning on the state object store for recoverability, and restricted read access; the backend resources themselves are bootstrapped outside the state they serve.
- [ ] Every refactor that moves or renames a resource uses a `moved` block (or deliberate `state mv` / `import` for cross-state moves) so the plan shows "has moved" with zero changes rather than "destroy then create," and no stateful resource is at risk of replacement during a rename.
- [ ] Existing resources brought under management were imported with `terraform import` / `pulumi import`, and a plan was run immediately after import to confirm state matches config before any apply; nothing was hand-edited in state to silence a diff.
- [ ] Shared modules are pinned to a tagged version or commit SHA (not a moving branch), sourced from a location whose tradeoff was chosen deliberately, composed through a thin root config, and split only along distinct reusable intents.
- [ ] Every variable has a concrete type (not `any`) with `validation` blocks for invariants the type cannot express, and every sensitive variable and output is marked `sensitive` with the understanding that this hides display only and does not protect the value in state.
- [ ] Dependencies are implicit via attribute references by default; `depends_on` is used only where no referenceable attribute exists and is commented with the reason; `replace_triggered_by` or lifecycle `triggers` drive replacement-on-related-change for resources that depend on a hash.
- [ ] Drift detection runs on a schedule (plan with no apply) with alerts on non-empty plans, drift is treated as a signal to reconcile by a human rather than auto-corrected, plan is posted to CI on every pull request as the reviewed artifact, and production apply is gated behind a human-reviewed plan with destructive actions explicitly confirmed.
