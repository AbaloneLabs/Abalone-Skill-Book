---
name: support-model-and-runbook-handoff.md
description: Use when the agent is defining support models, support tiers, runbook handoff, escalation paths, operational documentation, monitoring ownership, incident response, known-error handling, and support readiness for a project moving into operations.
---

# Support Model And Runbook Handoff

Support model handoff determines whether users can get help after a project goes live. A project may have trained users and tested features, but if support tiers, runbooks, monitoring, escalation, and known errors are unclear, every issue becomes a custom investigation. This skill helps the agent make support readiness explicit before project teams withdraw.

## Core Rules

### Define who supports what

Map support tiers, owners, hours, channels, scope, exclusions, and backup coverage. Include frontline support, operations, IT, product, vendor, finance, compliance, facilities, or other teams depending on the project. Each team should know what it owns and what it does not own.

Avoid generic statements such as "support will handle it." Support teams need routing criteria and authority.

### Build runbooks from real support scenarios

Runbooks should cover common questions, known errors, access issues, data discrepancies, failed handoffs, customer-impacting defects, manual workarounds, and escalation conditions. They should include evidence to collect, system checks, decision paths, communication templates, and closure criteria.

Do not write runbooks only from project design assumptions. Use test defects, pilot feedback, training questions, and launch risks.

### Define monitoring and alert ownership

If the project creates dashboards, alerts, reports, logs, or control checks, define who watches them, when, what thresholds matter, what action follows, and how false positives are handled. Monitoring without response ownership creates false comfort.

For manual checks, specify cadence, evidence, owner, backup, and escalation.

### Control known errors and workarounds

Known issues should have severity, impact, affected users, workaround, owner, expected fix, communication, and review date. Workarounds should be safe, documented, and time-bounded where possible.

Do not let known-error lists become an excuse to normalize defects. Users and support teams need to know whether an issue is accepted, being fixed, or requires escalation.

### Ensure support access and permissions

Support teams need access to systems, logs, knowledge articles, ticket categories, test accounts, reports, vendor portals, and contact lists. Access should be appropriate to role and privacy constraints.

Late access means support teams learn in production. If access cannot be granted, define an alternate diagnostic path and its limits.

### Create escalation paths with decision authority

Escalations should specify trigger, severity, required evidence, receiving owner, response expectation, decision authority, and status cadence. Some escalations require product, engineering, vendor, legal, security, finance, or leadership.

Escalation paths must work after the project team rolls off. If the path depends on one project manager, it is not a run-state path.

### Train support through case practice

Support readiness should be validated with realistic cases and role-specific practice. Agents need to find the runbook, gather evidence, choose a path, communicate status, and close or escalate correctly.

Training attendance is not enough. Observe whether the support team can operate without project-team coaching.

### Close support handoff with evidence

Handoff evidence should include support model, runbooks, known-error list, monitoring plan, access confirmation, escalation map, training validation, open issues, vendor contacts, and support metrics. Operations should sign off on support readiness before project closure.

If evidence is missing, transition can still proceed only with accepted risk and hypercare coverage.

### Keep support knowledge maintainable

Runbooks and support articles need owners, review triggers, version dates, and feedback channels. If a project changes after handoff, support knowledge must change with it. Otherwise support quality decays while the artifact still looks official.

Assign who updates guidance after defects, policy changes, vendor changes, and user feedback.

## Common Traps

- Assuming support ownership because a ticket queue exists.
- Writing runbooks from ideal process design rather than real user and defect scenarios.
- Creating dashboards or alerts without response owner and threshold.
- Letting known workarounds become permanent without owner or review.
- Granting support teams access only after go-live issues appear.
- Using project-specific escalation contacts that disappear after closeout; training support teams through presentations but not realistic cases
- Omitting vendor portals, logs, reports, test accounts, and privacy constraints from support readiness; closing the project while support still needs project experts for ordinary issues
- Treating missing support artifacts as documentation cleanup rather than operational risk; handing off runbooks without owner, review trigger, feedback path, or update authority

## Self-Check

- Are support tiers, owners, hours, channels, scope, exclusions, and backups clear?
- Do routing criteria define which team handles each issue type?
- Do runbooks cover common questions, known errors, access, data, handoffs, defects, workarounds, evidence, decisions, communication, and closure?
- Are runbooks informed by test defects, pilot feedback, training questions, and launch risks?
- Are dashboards, alerts, reports, logs, and manual checks assigned to owners with thresholds and actions?
- Are known errors documented with severity, impact, users, workaround, owner, fix plan, communication, and review date?
- Do support teams have required access, permissions, categories, portals, logs, reports, accounts, and contacts?
- Are escalation triggers, evidence, owners, response expectations, authority, and cadence defined for run state?
- Has support practiced realistic cases without relying on project-team coaching?
- Does handoff evidence include model, runbooks, monitoring, access, escalation, training validation, known errors, vendor contacts, open issues, metrics, and signoff?; do support artifacts have owners, review triggers, version dates, feedback channels, and update authority?
