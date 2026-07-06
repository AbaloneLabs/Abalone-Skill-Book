---
name: deprecation_and_removal_process.md
description: Use when the agent is deprecating an API, feature, or configuration option, planning a removal timeline, building a compatibility shim, communicating a breaking change to users, writing an end-of-life policy, or deciding how aggressively to remove old code that users still depend on.
---

# Deprecation And Removal Process

Removing something that works is harder than building something new. A feature, API, or configuration option that is being retired almost certainly still has users — often users you cannot see, on schedules you do not control, in systems you cannot inspect. Rip it out abruptly and you break those users, on their timeline, with no warning and no path forward. Leave it forever and it becomes permanent debt: maintenance burden, a drag on every change, a confusing surface where new users cannot tell what is current and what is abandoned. The deprecation process is the discipline of navigating between these failures — retiring the old thing on a schedule that lets users move, with a path that makes moving possible, and with a removal that actually happens.

Agents tend to fail at deprecation in two opposite directions. Some remove immediately, treating "we announced it" as sufficient notice and breaking consumers who had no realistic chance to react. Others deprecate and then never remove, accumulating a graveyard of deprecated-but-immortal APIs that everyone is afraid to touch and no one can kill. The judgment problem is to design a deprecation as a process with a real timeline and a real end — one that gives users enough notice and help to migrate, that provides a compatibility bridge where the cost is justified, that communicates clearly so no one is surprised, and that has the discipline to actually complete the removal rather than letting it drift into permanence.

## Core Rules

### Deprecate With A Path, Not Just A Warning

A deprecation is only useful if it tells the user not only that the old thing is going away but *what to do instead*. A deprecation warning that says "this is deprecated" without pointing to the replacement leaves the user stuck: they know they must change, but not to what. The first job of a deprecation is to make migration obvious and cheap.

For each deprecation:

- **Name the replacement** explicitly, in the warning message, the docs, and the release notes. A user who sees the warning should immediately know the alternative.
- **Provide a migration guide** for non-trivial changes — the steps, the mapping from old to new, the edge cases, and ideally a code example or automated tool.
- **Make the replacement genuinely better or at least equivalent**, so users have a reason to move beyond mere obligation. A deprecation to a replacement that is worse or incomplete invites users to stall until forced.
- **Offer automated migration where feasible** — a codemod, a linter rule, a config rewriter — so that the cost of moving is minutes rather than days.

The deprecation warning itself should be actionable at the point of use: it should appear when the deprecated path is exercised, name the deprecated thing, point to the replacement, and (where relevant) state the removal timeline. A warning buried in release notes that no one reads does not reach the user; a warning that fires in their running system does.

### Set A Removal Timeline That Gives Real Notice

Removal on a realistic schedule is what distinguishes a deprecation from a permanent zombie. The timeline must be long enough that users who depend on the old thing have a genuine opportunity to learn of the deprecation, plan the migration, and execute it — but not so long that the deprecation becomes meaningless and the removal never happens.

Factors that should set the timeline:

- **How visible the deprecation is.** A warning in every use reaches users fast; a release note reaches only those who read them. Match the notice period to the visibility.
- **How hard the migration is.** A one-line rename needs short notice; a migration that touches architecture or data needs long notice and possibly assistance.
- **Who the users are.** Internal users on a shared schedule can be migrated quickly; external users on their own release cycles need notice measured in their release cadence, not yours.
- **The versioning contract.** A breaking removal in a SemVer-major release is expected; a removal that breaks within a minor version violates the contract and must not happen regardless of timeline.

State the timeline explicitly at the moment of deprecation: the version or date of removal, and any milestones along the way (warning-only period, then louder warnings, then removal). A timeline that is "eventually" is not a timeline; it is an excuse to defer removal indefinitely.

### Build A Compatibility Bridge When Migration Is Costly

When the migration from old to new is expensive for users, a compatibility bridge — a shim, an adapter, a translation layer — can let the old path keep working while users migrate, at the cost of maintaining the bridge. The bridge translates old calls to the new implementation, so users see no breakage even as the system evolves underneath.

Use bridges deliberately:

- **Build them when the migration cost is high and the user base is large or hard to coordinate.** The bridge buys time for migration without forcing a synchronized cutover.
- **Make them clearly temporary**, with the same removal timeline as the deprecation. A bridge that becomes permanent is just a second implementation that doubles the maintenance.
- **Track usage of the bridge** so you know when migration has progressed enough to remove it. A bridge with no remaining callers can be removed; one with active callers cannot, and the callers should be notified.
- **Weigh the maintenance cost.** A bridge that is complex to maintain, or that prevents the new design from reaching its potential, may not be worth it; in those cases, a longer notice period and migration assistance may be better than a shim.

The bridge is a tool, not a default. For simple deprecations, a clear migration path and a timeline are enough; a shim adds maintenance without proportional benefit.

### Communicate So Users Are Not Surprised

The worst outcome of a deprecation is a user who is surprised by the removal — who did not know the thing was deprecated, did not see the warnings, and discovers the breakage when their system stops working. Communication is how that surprise is prevented, and it must reach users through the channels they actually use.

Layer the communication:

- **In-product warnings** that fire when the deprecated path is used, reaching users at the moment of impact.
- **Release notes and changelogs** that call out deprecations prominently, not buried, reaching users who track releases.
- **Documentation** that marks the deprecated thing clearly and links to the migration path, reaching users who read the docs.
- **Direct outreach** for high-impact deprecations — notifying known major users, posting announcements, offering help — reaching users who would otherwise miss the signals.

The standard is that a user who reasonably tracks the project's communications should not be surprised by a removal. If a removal would surprise such a user, the communication was insufficient, regardless of how many channels were used.

### Define And Enforce An End-Of-Life Policy

A deprecation without an enforced end-of-life is permanent debt. The policy should define, for each class of deprecation, how long the notice period is, what the milestones are, and what happens at removal — and it should be enforced, so that removal actually occurs rather than being perpetually deferred.

The policy should address:

- **The notice period** by class (internal API, public API, configuration, entire product) reflecting the user impact.
- **The escalation** — how warnings intensify as removal approaches (deprecation warning, then louder warning, then error in a pre-release, then removal).
- **The removal action** — what "removed" means (code deleted, endpoint returns 410 Gone, config ignored with error) so there is no ambiguity.
- **The exceptions process** — how to handle a user who genuinely cannot migrate in time, whether by extending the bridge, providing a one-off path, or accepting a documented break.

The discipline to actually remove is the hardest part. Each removal will have some user who asks for more time, and each concession extends the debt. Set the policy, give generous notice, provide real help, and then remove on schedule — because a project that never removes has no credible deprecation, and users learn to ignore its warnings.

### Respect Scope and Escalation Boundaries

Know where the agent's authority and competence end. When the question requires a license, a specialist's judgment, a final approval, or expertise the agent does not hold, the correct action is to escalate rather than to produce a confident answer that overreaches. Scope discipline protects the recipient from harm caused by an unqualified conclusion and protects the agent from liability. State explicitly when the output is advisory and must be confirmed by the qualified person.

## Common Traps

### Deprecating Without A Replacement Or Path

A deprecation that says "this is going away" without naming the alternative or how to migrate leaves users stuck and invites them to ignore the warning. Always provide the replacement and a migration path; a deprecation without a path is just a threat.

### Removing Without Real Notice

Removing on a timeline that no reasonable user could have reacted to — because the warning was invisible, the period too short, or the migration too hard — breaks users and destroys trust in the deprecation process. Match the notice to the visibility and the migration cost.

### The Permanent Deprecation

Deprecating and then never removing, because someone always asks for more time or because removal is never convenient, accumulates immortal deprecated code that burdens every change and confuses new users. Set an end-of-life policy and enforce it; removal must actually occur.

### A Compatibility Bridge That Becomes Permanent

A shim meant to be temporary that is never removed becomes a second implementation with its own maintenance and its own bugs, doubling the cost without progressing the migration. Track bridge usage and remove it when callers have migrated.

### Communication That Does Not Reach Users

Warnings buried in release notes, deprecations announced only on channels no one reads, or docs that are not updated leave users surprised by removal. Layer communication across in-product warnings, release notes, docs, and direct outreach so a reasonable user is not caught off guard.

### Removing Within A Compatible Version Range

A removal that breaks consumers within a minor or patch release violates the versioning contract, regardless of how long the deprecation was announced. Breaking removals belong in a major version (or the project's equivalent breaking-change boundary).

### No Usage Tracking For The Deprecated Path

Without knowing who still uses the deprecated path, removal is a gamble — it may break unknown users, or it may be deferred forever because the usage is assumed. Track usage where possible so removal decisions are informed.

## Self-Check

- [ ] Each deprecation names an explicit replacement and provides a migration path (guide, mapping, examples, and automated tooling where feasible), not just a warning.
- [ ] The removal timeline reflects the deprecation's visibility, the migration difficulty, and the users' release cadence, and is stated explicitly at deprecation time rather than left as "eventually."
- [ ] Compatibility bridges are built only when migration cost justifies them, are clearly temporary, track remaining usage, and are removed once callers have migrated.
- [ ] Communication is layered across in-product warnings, prominent release notes, updated docs, and direct outreach for high-impact cases, so a user who reasonably tracks the project is not surprised by removal.
- [ ] An end-of-life policy defines notice periods, escalation, the removal action, and an exceptions process, and removals are enforced on schedule rather than perpetually deferred.
- [ ] Breaking removals occur only at the versioning contract's breaking-change boundary (major version), never within a compatible range.
- [ ] Usage of deprecated paths is tracked where possible, so removal decisions are informed by actual remaining dependence rather than assumption.
- [ ] No deprecation in the project is immortal — each has a real removal date or version, and the oldest deprecations have already been removed.
