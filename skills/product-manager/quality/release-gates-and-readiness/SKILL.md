---
name: release-gates-and-readiness.md
description: Use when the agent is defining release gates, deciding whether a feature is ready to ship, establishing rollout and rollback plans, setting go/no-go criteria, or determining the staged rollout strategy and monitoring needed before and after a release goes live.
---

# Release Gates And Readiness

The decision to release is the moment when accumulated risk becomes real. Every shortcut taken, every test not written, every edge case not considered is about to meet real users, and the cost of a problem shifts from "fix it before anyone sees" to "fix it while customers are affected." Release gates exist to make this decision deliberately rather than by default, and to ensure that the conditions for a safe release are met before, not after, the consequences begin. Weak gates let problems through; rigid gates that cannot adapt to context become theater that teams work around. The product manager's job is to define gates that are meaningful, calibrated, and enforced.

This skill covers the judgment needed to define release gates, assess readiness, and plan the rollout so that releases are safe, observable, and reversible.

## Core Rules

### Define release gates around risk, not around process compliance

A release gate is a checkpoint that a release must pass to proceed. The most common failure is defining gates as process compliance ("all test cases executed, sign-off obtained") rather than as risk conditions ("the specific risks of this release are understood and mitigated"). Process-compliance gates produce sign-off theater; risk-based gates actually protect the release.

Define gates around the conditions that must be true for the release to be safe:

- **Critical defects resolved or consciously accepted.** No known must-fix issues remain, or any accepted issues have explicit rationale and a plan.
- **Critical paths verified.** The flows users depend on have been tested at the appropriate level and are known to work.
- **Rollback plan exists and is viable.** If the release fails, there is a tested way to reverse it, or an explicit decision that the release is irreversible and the risk is accepted.
- **Monitoring and alerting in place.** Problems will be detected after release, not discovered through customer reports.
- **Support and communication prepared.** The people who will handle customer impact know what is coming and are equipped to respond.

These conditions are risk-based: each addresses a specific way a release can go wrong. A gate that does not address a real risk is ceremony.

### Calibrate gate strictness to risk and reversibility

Not every release warrants the same gates. A configuration flag flip, a copy change, and a major architecture migration carry vastly different risk, and applying identical gates either over-burdens low-risk changes (encouraging teams to bypass the process) or under-protects high-risk ones. Calibrate deliberately.

- **Low-risk, easily reversible changes** (small copy edits, minor UI tweaks, feature flag toggles with instant rollback) warrant minimal gates: basic verification and the ability to revert.
- **Medium-risk, reversible changes** (most product features) warrant the standard gate set: critical paths verified, monitoring in place, rollback plan, support briefed.
- **High-risk, low-reversibility changes** (data migrations, security changes, architecture changes, changes to money or data integrity) warrant the strictest gates: extensive testing, staged rollout, explicit rollback or recovery plan, heightened monitoring, and often a formal go/no-go review.

State the risk class of each release explicitly and apply the matching gate set. The calibration must be honest; relabeling a high-risk change as low-risk to avoid gates is the failure mode the gates exist to prevent.

### Make go/no-go decisions with explicit criteria, not vibes

The release decision is often made in a meeting based on general confidence, which is influenced by schedule pressure, fatigue, and optimism. Replace vibes with explicit criteria that can be checked.

- List the specific conditions that must be met for go: each gate, passed.
- List the known issues and the rationale for any that are accepted rather than fixed.
- Identify the residual risk: what could go wrong, how likely, how detectable, how recoverable.
- Name the decision owner who is accountable for the go/no-go.

A go decision with documented criteria and accepted risks is defensible. A go decision based on "we feel good about it" is not, and when something goes wrong, the lack of rigor is exposed. The discipline is especially important under deadline pressure, which is exactly when vibes-based decisions are most optimistic and most dangerous.

### Plan the rollout to limit blast radius

A release is not a binary event; it is a process of increasing exposure. The rollout strategy determines how many users are affected if something goes wrong, and how quickly it can be stopped. Plan the rollout deliberately to limit blast radius.

- **Staged rollout** exposes the release to a small percentage first, monitors for problems, and expands only if healthy. This catches issues that testing missed while affecting few users.
- **Feature flags** decouple release from exposure, allowing the feature to be deployed dark and enabled gradually or for specific segments.
- **Segment-aware rollout** considers which users to expose first: internal users, then a friendly beta segment, then the broader population. Exposing the most sensitive or highest-value users last reduces risk.
- **Rollback readiness** means the ability to reverse the release quickly, verified to work, not assumed. An untested rollback plan is not a rollback plan.

The rollout plan is part of readiness. A release is not ready if the plan is "ship to everyone at once and hope," regardless of how well-tested the feature is.

### Ensure observability before, not after, release

A release is only safe if problems are detectable. Monitoring, alerting, and the dashboards needed to assess health must be in place before the release goes live, not added afterward when a problem appears. This is a gate, not an afterthought.

- Define what "healthy" looks like for the release: the key metrics, error rates, and behaviors that indicate the feature is working.
- Set alerts that fire when health degrades, with thresholds calibrated to catch real problems without overwhelming with false alarms.
- Ensure the people who will respond to alerts know what to look for and what to do.

A release without observability is a release where the first sign of trouble is a customer complaint, by which point the damage is done.

### Decide rollback versus fix-forward deliberately before release

Before releasing, decide the default response if something goes wrong: rollback or fix-forward. This decision should be made in advance, while thinking clearly, not improvised under pressure.

- Rollback is the default when the release is reversible and rollback is tested. It restores the previous known-good state fastest.
- Fix-forward is the path when rollback is impossible (irreversible data changes), when the fix is small and fast, or when rollback would cause a different serious problem.
- For high-risk releases, rehearse the chosen path. An unrehearsed rollback under pressure often fails.

Making this decision in advance removes a major source of confusion and delay during an actual problem.

### Treat the post-release period as part of the release

The release is not complete when it ships; it is complete when it has been observed behaving well in production under real load, for a defined period. The first hours and days after release are when problems most likely surface, and the team should be positioned to detect and respond.

- Define a monitoring window after release during which health is actively watched.
- Have the team available to respond during that window, not immediately moving to the next thing.
- Define the criteria for declaring the release stable and ending the heightened monitoring.

Treating the post-release period as part of the release closes the loop and catches the problems that staged rollout and monitoring are designed to surface.

## Common Traps

### Gates as process compliance theater

Gates defined as checklists of activities performed ("testing done, sign-off obtained") rather than risk conditions met produce sign-off without protection. Define gates around the risks they address.

### Uniform gates that ignore risk class

Applying identical gates to a copy change and a data migration over-burdens the former and under-protects the latter. Calibrate gate strictness to risk and reversibility, honestly.

### Vibes-based go decisions under deadline pressure

Releasing based on general confidence, especially under deadline pressure, is optimistic and unaccountable. Use explicit, checkable criteria and a named decision owner.

### Ship-to-everyone rollout with no staging

Releasing directly to the full user base maximizes blast radius if something is wrong. Use staged rollout, feature flags, and segment-aware exposure to limit risk.

### Rollback assumed but not tested

A rollback plan that has never been exercised will fail when needed, often because of data migrations or schema changes that are not actually reversible. Test rollback for high-risk releases.

### Observability added after the problem

Monitoring is an afterthought, added when a problem appears, by which point the damage is done. Observability is a release gate, set up before release.

### Post-release attention ending at ship

The team ships and immediately moves on, so problems surfacing in the first hours go undetected. Treat the post-release monitoring window as part of the release, with the team available to respond.

### Relabeling high-risk changes as low-risk to bypass gates and accepted issues without rationale or plan

To move faster, a change is classified as low-risk to avoid strict gates, when it actually carries high risk. This is the exact failure the gates exist to prevent. Classify risk honestly.

Known issues are "accepted" to make the date, without documented rationale or a plan to address them. These accumulate into a quality deficit. Accept issues only with explicit rationale and ownership.

## Self-Check

- Are release gates defined around risk conditions (critical defects, verified paths, rollback viability, monitoring, support readiness) rather than process compliance?
- Is gate strictness calibrated to the risk class and reversibility of this specific release, with the risk class stated honestly?
- Is the go/no-go decision made against explicit, checkable criteria with a named accountable owner, rather than general confidence?
- Does the rollout plan limit blast radius through staging, feature flags, and segment-aware exposure?
- Is the rollback plan tested and viable, or merely assumed? Have I decided rollback versus fix-forward in advance?
- Is observability — monitoring, alerting, dashboards — in place before release, treated as a gate?
- Is the post-release monitoring window defined, with the team available to respond before declaring the release stable?
- Were any known issues accepted with explicit rationale and a plan, rather than to make a date?
- Did I resist pressure to relabel a high-risk change as low-risk to bypass gates?
- If this release caused a serious incident, would the gates and rollout plan have been expected to limit the damage, or would the gap have been foreseeable?
