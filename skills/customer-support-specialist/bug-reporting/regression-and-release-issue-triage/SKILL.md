---
name: regression-and-release-issue-triage.md
description: Use when the agent is triaging possible regressions, release-related defects, rollout issues, beta problems, post-deploy complaints, feature flag mistakes, migration defects, new version failures, or customer reports that product behavior changed unexpectedly after a release.
---

# Regression And Release Issue Triage

Regression triage is time-sensitive because early support contacts may be the first sign that a release broke customer workflows. Agents can miss regressions by treating each ticket as individual confusion, or over-escalate by blaming a release without evidence. This skill helps the agent connect customer reports to release context while preserving evidence and customer trust.

## Core Rules

### Establish what changed and when

Check release date, version, feature flag, rollout cohort, migration, configuration change, dependency update, app store release, content change, pricing change, or backend deploy. Compare it with the customer's first failure time.

Temporal correlation is a signal, not proof.

### Compare previous and current behavior

Ask what worked before, when it last worked, what changed for the customer, and whether other users in the same account see it. Use screenshots, logs, and customer history where available.

Regression reports need a baseline.

### Identify rollout scope

Determine whether the issue affects all customers, one cohort, one region, one plan, one device, one browser, one integration, one tenant, or one migrated data shape. Feature flags and staged rollouts create uneven impact.

This prevents both panic and underreaction.

### Collect release-specific evidence

Include version numbers, build, feature flag state, migration ID, release note, experiment group, app store version, device model, cache state, and recent configuration or data changes. Link related tickets and known issues.

Release triage needs evidence that ties symptom to change.

### Watch high-risk categories

Escalate quickly for data loss, incorrect billing, security or privacy exposure, access failure, order or payment failure, safety issue, accessibility blocker, enterprise SLA impact, public complaint, or launch-critical workflow.

One credible high-risk regression can justify urgent review.

### Avoid premature customer explanations and decide incident, rollback, hotfix, or backlog path

Do not tell customers a release caused the issue until confirmed. Say the team is investigating reports related to recent changes if approved. Provide safe workaround and next update.

Speculation becomes hard to retract.

Support should not decide deployment action alone, but the escalation should state whether impact may justify incident review, rollback, hotfix, feature flag change, migration pause, or backlog handling.

### Track post-release patterns and update launch and support materials

Use tags and watchlists for repeated contacts. Monitor volume, segment, sentiment, workaround success, and whether the issue grows with rollout expansion.

If the regression is confirmed or a workaround exists, update known issue guidance, macros, help center, bot flows, and escalation triggers. Remove them after resolution.

### Verify recovery by customer segment and track affected customers during rollout

A fix may work for new sessions but not cached users, mobile apps, migrated accounts, or data already corrupted. Confirm recovery for affected segments before closing.

When a suspected regression appears, maintain a list of affected customers, cohorts, versions, and promises. If the rollout continues, the affected group may change. Support needs to know who should receive proactive updates, workaround guidance, or post-fix confirmation.

Do not rely only on the engineering ticket to preserve customer follow-up obligations.

### Account for delayed release channels and preserve rollback and workaround decisions

Mobile app stores, desktop clients, browser extensions, marketplace apps, embedded integrations, and cached web assets may update at different speeds. A fix may be available in one channel while customers in another remain affected.

Customer messaging should distinguish release availability from customer adoption of the fixed version.

If product chooses not to rollback or cannot provide a workaround, document the reason at the level support is allowed to know. Customers may ask why the broken behavior remains available.

Support needs approved language for continuing rollout, pausing rollout, disabling a feature, or waiting for a patch. Do not let agents invent explanations under pressure.

## Common Traps

- Treating post-release complaints as isolated user confusion.
- Assuming a release caused the issue without baseline or evidence.
- Ignoring staged rollout, feature flags, app versions, or cohorts.
- Missing high-risk low-volume reports; telling customers the release is the cause before confirmation
- Escalating without version, timing, flag, migration, or environment details; failing to monitor whether impact grows as rollout expands
- Leaving stale known-issue macros after a fix; closing after hotfix without checking affected segments
- Forgetting to capture customer promises during triage; losing track of which customers or cohorts need updates as rollout impact changes
- Saying a fix is available without checking delayed app, extension, client, integration, or cache rollout; leaving agents without approved language for rollback, pause, disablement, workaround, or wait-for-patch decisions

## Self-Check

- Is the relevant release, flag, migration, configuration, app version, or deploy identified?
- Is the customer's first failure time compared with the change time?
- Is prior behavior or last-known-good state captured?
- Is rollout scope described by cohort, region, plan, device, browser, integration, tenant, or data shape?
- Are version, build, flag state, migration ID, experiment group, app store version, device, cache, and recent changes captured where relevant?
- Are high-risk categories escalated quickly?
- Does customer messaging avoid unconfirmed release blame?
- Does escalation indicate whether incident, rollback, hotfix, flag change, migration pause, or backlog review may be needed?; are post-release tags, watchlists, volume, sentiment, and workaround success monitored?
- Is recovery verified for affected segments after fix?; is an affected-customer or cohort list maintained with update promises?
- Are delayed release channels and customer adoption of fixed versions considered?; are rollback, rollout pause, feature disablement, workaround, and patch-wait decisions documented with approved customer language?
