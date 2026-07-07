---
name: incident_runbooks_and_automation.md
description: Use when the agent is writing, maintaining, or evaluating incident runbooks and incident automation, deciding when a runbook helps versus when it rots, choosing between executable runbooks and static documentation, designing automated mitigation (auto-rollback, auto-scaling, auto-remediation), embedding diagnostic dashboards and links in alerts, or keeping runbooks tested and current through game days. Also covers the failure modes of runbooks that assume a human remembers context the alert did not provide, runbooks that have drifted from the system they describe, automation that takes action no human can override or understand, alerts that point nowhere, and the recurring mistake of treating a runbook as a document to write once rather than a capability to maintain, test, and retire when it no longer applies.
---

# Incident Runbooks And Automation

A runbook is the bridge between an alert and a mitigation: when the pager fires at 3 a.m., the runbook is what tells the on-call what this alert means, what to check, and what to do. The judgment problem is that runbooks are easy to write and hard to keep useful. A runbook written at launch describes a system that, six months later, has been refactored, renamed, and redeployed — and the runbook now points at dashboards that no longer exist, commands that no longer work, and thresholds that no longer apply. Worse, the team trusts it, because it exists. The recurring failure is a runbook that has rotted silently into a document that actively misleads the responder who follows it. The discipline is to treat a runbook not as a document to write once but as a capability to maintain, to make it executable and testable rather than static prose, to embed the diagnostic context (dashboards, logs, queries) directly in the alert so the responder does not start from zero, and to know when automation should replace the runbook entirely.

Agents tend to produce runbooks that read well and help little. They write generic steps ("check the logs," "verify the service is healthy") that assume the responder already knows where the logs are and what healthy means. They describe the happy path and omit the decisions that actually matter under pressure (when to roll back, when to fail over, when to shed load). They write runbooks for alerts that never include a link to the runbook, so the responder has to find it. And they reach for automation without considering that automation that acts without human-in-the-loop safeguards can turn a small anomaly into a large outage when its trigger is wrong. The harm is a responder who, during the worst minutes, is reading a stale document, hunting for a dashboard, or fighting an automated action they cannot understand or override. The judgment is to make runbooks current, executable, linked, and tested, and to design automation with clear triggers, bounds, and override paths.

## Core Rules

### Know When A Runbook Helps And When It Has Rotted

A runbook is valuable when the response to an alert is non-obvious but repeatable — a sequence of checks and actions a competent responder could perform faster with guidance than from memory. It is a liability when it has drifted from the system it describes.

- **A runbook helps when the mitigation is procedural and easy to forget.** If the correct response is a specific sequence (drain a node, check a queue, run a remediation script, roll back a canary), a runbook captures it so the responder does not reinvent it under pressure. If the response is obvious from the alert, a runbook adds friction.
- **A runbook rots when the system changes and the runbook does not.** Renamed services, moved dashboards, changed commands, revised thresholds, and refactored architectures all invalidate a runbook silently. A runbook that points at a dead dashboard or a renamed command is worse than none, because it wastes time and erodes trust.
- **Assign ownership and a review cadence.** Every runbook needs an owner (the team responsible for the service) and a review trigger (a regular schedule, or whenever the service changes). A runbook with no owner is an orphan that will rot.
- **Retire runbooks that no longer apply.** A runbook for an alert that no longer fires, or a system that no longer exists, is clutter that obscures the runbooks that matter. Delete or archive runbooks when their alert or system is retired.

### Prefer Executable Runbooks Over Static Documentation

A runbook whose steps a human must transcribe into a terminal is slower and more error-prone than one whose steps can be executed, copy-pasted, or invoked directly. Executable runbooks reduce the gap between "what to do" and "doing it."

- **Make steps copy-pasteable or invocable.** Provide exact commands, queries, and links rather than descriptions of them. "Run `kubectl drain node-X --ignore-daemonsets`" beats "drain the node." A script the responder can invoke beats a script they must retype.
- **Move repeated procedures into scripts or automation.** If the runbook's steps are always the same, they are a candidate for a script or automated remediation; the runbook then links to the automation rather than describing manual steps. This reduces transcription error and drift.
- **Keep the runbook linked from the alert.** A runbook the responder cannot find from the alert is useless during the incident. Every alert should link directly to its runbook, and every runbook should link to its alert, so the pair is never separated.
- **Version runbooks with the system.** Store runbooks near the code (in the repo) rather than in a separate wiki that drifts, so changes to the system prompt changes to the runbook in the same review.

### Embed Diagnostic Context In The Alert, Not Just In The Runbook

The responder's first question is always "what is happening?" An alert that says "error rate high" and links a runbook still leaves the responder hunting for the dashboard, the logs, and the relevant query. Embedding the context in the alert collapses that hunt.

- **Link the dashboards, logs, and queries directly in the alert.** The alert should carry deep links to the relevant dashboard panels, the log view filtered to the affected service, and the query that would confirm the diagnosis, so the responder lands on the evidence in one click.
- **Include the context needed to triage: scope, severity, and recent changes.** The alert should say what is affected, how badly, since when, and whether a deploy or change correlates, so the responder can triage without a separate investigation.
- **Make the alert self-sufficient for the first decision.** The responder should be able to decide "is this real, and what do I check first" from the alert alone, before opening the runbook. The runbook then guides the response; the alert provides the situation.

### Design Automated Mitigation With Clear Triggers, Bounds, And Overrides

Automation (auto-rollback, auto-scaling, auto-remediation) can mitigate an incident in seconds that a human would take minutes to reach — but automation that acts on a wrong trigger, or that cannot be overridden or understood, can amplify a small anomaly into a large outage.

- **Define the trigger precisely and conservatively.** Automated mitigation should fire on clear, high-confidence signals (a deploy followed by a sustained error-rate spike) with thresholds that avoid false positives, because a false trigger acts on a healthy system. Prefer a slightly slower, more certain trigger over a fast, uncertain one.
- **Bound the action.** Automation should take a bounded action (roll back the last deploy, scale out by N, shed a defined fraction of load) and then stop, not iterate indefinitely. Unbounded automation can cascade when its model of the situation is wrong.
- **Make automation observable and overrideable.** Every automated action must be logged, visible to on-call, and overrideable. Automation that acts invisibly or that a human cannot stop is dangerous when its trigger is wrong; the human must always be able to intervene.
- **Decide deliberately what to automate and what to leave to humans.** Automate reversible, well-understood, high-frequency mitigations (rollback a bad deploy, scale out for load); leave ambiguous, high-stakes, or novel decisions to humans. Automation is a tool for the responder, not a replacement for judgment.

### Keep Runbooks And Automation Tested Through Game Days

A runbook or automation that has never been exercised will, with high probability, fail when invoked, because the conditions of a real incident expose assumptions the runbook made. Testing converts a written procedure into a working capability.

- **Exercise runbooks in game days, not only in incidents.** Run through the runbook's steps under simulated conditions to catch dead links, broken commands, and missing context before the incident does. A runbook that has never been followed is unverified.
- **Test automation under the conditions it is meant for.** Trigger the automated mitigation in a controlled setting to confirm it fires on the right signal, takes the bounded action, and can be observed and overridden. Automation untested is automation assumed.
- **Treat game-day findings as runbook and automation fixes.** When a game day reveals a broken step, a stale link, or a misfiring automation, fix it and re-test. A finding that is logged but not fixed leaves a known-broken capability in the response path.

## Common Traps

### A Runbook That Assumes The Responder Remembers Context

A runbook with generic steps ("check the logs," "verify health") that assumes the responder already knows where the logs are and what healthy means, offering no help during the worst minutes. Embed exact commands, links, and queries, and put the diagnostic context in the alert, not in the responder's memory.

### A Runbook That Has Drifted From The System

A runbook that points at renamed services, moved dashboards, changed commands, or revised thresholds, silently invalidating itself as the system evolves — and that the team still trusts because it exists. Assign ownership, review on change, version with the code, and retire runbooks that no longer apply.

### A Runbook The Responder Cannot Find From The Alert

An alert that fires without a link to its runbook, so the responder must hunt for guidance during the incident. Link every alert directly to its runbook and every runbook to its alert.

### Automation That Acts Without Override Or Understanding

Automated mitigation that fires on a wrong trigger, takes unbounded action, or acts invisibly such that on-call cannot see or stop it, turning a small anomaly into a large outage. Define precise conservative triggers, bound the action, and make every automated action logged, visible, and overrideable.

### Alerts That Point Nowhere

An alert with no link to a dashboard, log view, or query, leaving the responder to reconstruct the situation from scratch. Embed deep links and triage context (scope, severity, since-when, correlated changes) directly in the alert.

### Static Prose Where Executable Steps Would Do

A runbook written as descriptive prose that the responder must transcribe into commands, introducing delay and error. Prefer copy-pasteable commands, invocable scripts, and automation, with the runbook linking to them.

### A Runbook Written Once And Never Tested

A runbook or automation that has never been exercised, so it is assumed to work until an incident reveals it does not. Test runbooks and automation through game days, and fix the findings, so the capability is verified rather than assumed.

## Self-Check

- [ ] Each runbook exists for an alert whose response is non-obvious but repeatable, has a named owner and a review trigger (schedule or on-change), and is retired when its alert or system no longer applies — not left to rot.
- [ ] Runbooks are executable rather than static prose: steps are copy-pasteable commands, queries, and deep links; repeated procedures are moved into scripts or automation the runbook links to; and runbooks are versioned with the code so system changes prompt runbook changes.
- [ ] Every alert links directly to its runbook and every runbook to its alert, so the pair is never separated during an incident.
- [ ] Diagnostic context is embedded in the alert: deep links to dashboards, filtered log views, and confirming queries, plus triage context (scope, severity, since-when, correlated deploys or changes), so the responder can make the first decision from the alert alone.
- [ ] Automated mitigation has precise, conservative triggers, takes bounded actions (roll back one deploy, scale out by N, shed a defined fraction), and is logged, visible to on-call, and overrideable — it is a tool for the responder, not an unsupervised actor.
- [ ] The split between automated and human action is deliberate: reversible, well-understood, high-frequency mitigations are automated; ambiguous, high-stakes, or novel decisions are left to humans.
- [ ] Runbooks and automation are tested through game days, findings are fixed and re-tested, and the highest-risk cases were verified — a drifted runbook, an unfindable runbook, an automation that misfired and could not be overridden, and an alert that pointed nowhere — not only the clean happy path.
