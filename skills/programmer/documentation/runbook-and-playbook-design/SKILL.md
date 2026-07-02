---
name: runbook_and_playbook_design.md
description: Use when the agent is writing or revising a runbook, playbook, or operational procedure, designing incident response steps, creating diagnostic and escalation guides, documenting recovery procedures for an outage, or maintaining operational documentation so it stays accurate as systems change.
---

# Runbook and Playbook Design

A runbook is the document an on-call engineer reaches for at 3 a.m. when something is broken and every minute counts. Its quality is measured under those conditions, not in calm review. A runbook that reads well in a planning meeting but omits the one command that actually fixes the problem, that references dashboards that no longer exist, or that buries the critical step on page four is worse than no runbook, because it wastes the most expensive resource in an incident: focused attention under stress. Agents tend to write runbooks like design documents (comprehensive, narrative, explanatory) when they should be written like checklists (procedural, scannable, copy-pasteable).

The judgment problem is that a runbook is operational documentation with a hostile reader: tired, stressed, possibly unfamiliar with the system, and under time pressure. Every design choice must serve that reader. The document must be discoverable (you must be able to find it from the alert that triggers it), actionable (each step must be something you can actually do, with the exact command or click path), ordered (do this, then this, with explicit decision points), and verifiable (how do you know the step worked). It must also be maintained, because a runbook that drifts from the system it describes will mislead during the exact moment it is needed. A runbook is a living contract with the on-call rotation, and an unmaintained one is technical debt that manifests as outages.

## Core Rules

### Write for the 3 a.m. reader, not the planning meeting

The reader is stressed, tired, time-pressured, and may not be the system's author. Optimize for them: short sentences, imperative voice, numbered steps, copy-pasteable commands with the actual values filled in or clearly marked placeholders, and no walls of explanatory prose. Put the most likely fix first. If background is needed, put it in an appendix, not in the critical path. A runbook that requires reading three paragraphs of architecture before reaching the first action fails the reader who needs to act now.

### Link every runbook to the alert or symptom that triggers it

A runbook that cannot be found from the alert is useless. Every alert, dashboard anomaly, or recurring symptom should link directly to the runbook that handles it, and the runbook's title should name the symptom the engineer will see ("API 500s spiking," "queue depth growing unbounded"), not an internal component codename. The path from "pager goes off" to "right runbook open" must be zero-click. If on-call engineers routinely search for the right runbook, the linking is broken.

### Make every step actionable, ordered, and verifiable

Each step must be three things at once. Actionable: it is a concrete action (a command, a click path, a call), not a vague instruction ("investigate the database"). Ordered: steps are numbered and sequenced, with explicit decision points ("if the error is X, go to step 5; if Y, go to step 8"). Verifiable: after each step, state how to confirm it worked ("the queue depth should begin dropping within 2 minutes; if it does not, proceed to escalation"). A step whose success cannot be checked leads to false confidence and wasted time. Avoid conditional logic buried in prose; use explicit if/then branches.

### Include exact commands, paths, and values, not descriptions of them

"Restart the service" is not a runbook step; it is a summary of one. The step is the exact command (`systemctl restart foo`), the exact host or cluster, the exact credentials or permission needed, and the expected output. Provide copy-pasteable commands with real values or clearly marked placeholders (`<env>`, `<cluster>`). An engineer who must reverse-engineer the command from a description is doing original work under stress, which is exactly what the runbook exists to prevent.

### Define escalation criteria and contacts explicitly

A runbook must state when to escalate, to whom, and how. Not "escalate if it's serious" but "if queue depth exceeds 10,000 for more than 15 minutes, or if any step in this runbook fails twice, page the storage on-call via <rotation> and notify the incident commander." Include the escalation path for each layer (on-call peer, team lead, vendor support) and the information to provide when escalating (symptom, steps already tried, current state). An engineer who does not know when to stop and escalate will either give up too early or struggle too long.

### Document the diagnostic path, not just the fix

Many incidents require diagnosis before a fix is possible. The runbook should guide diagnosis: which dashboards to check first, which logs to grep and for what, which queries reveal the scope (how many users affected, which regions), and how to distinguish the common causes from each other. A decision tree ("if error rate is uniform across regions, suspect config; if regional, suspect infrastructure") is more useful than a flat list. The goal is to move the engineer from symptom to cause quickly and reliably.

### Make the runbook executable and testable

The best runbooks are not just documents but executable procedures. Where possible, link to or embed the actual scripts, runbooks-as-code, or automation that performs the steps, so the engineer runs a verified tool rather than retyping commands. Even without full automation, runbooks should be exercised: run a game day or chaos exercise that follows the runbook end to end and reveals where it is wrong, incomplete, or references stale resources. A runbook that has never been used is an untested hypothesis.

### Assign ownership and a maintenance cadence

Every runbook has an owner (the team responsible for the system) and a review cadence. Systems change, dashboards move, commands change, runbooks rot. Tie runbook review to system changes: any change that affects operational behavior should require a runbook update in the same PR or change ticket. Schedule periodic reviews (e.g., quarterly) to catch drift. A runbook with no owner is an orphan that will mislead; treat ownership as a first-class property, documented in the runbook itself.

## Common Traps

### Writing a design doc instead of a procedure

Runbooks are not the place for architecture explanation or design rationale. Background prose buries the actions the reader needs. Move explanation to an appendix and keep the critical path procedural.

### Referencing dashboards, links, or commands that no longer exist

A runbook that points to a deleted dashboard or a renamed command wastes time and erodes trust in the document. Tie runbook updates to system changes and review on a cadence to catch drift.

### Vague steps that require interpretation under stress

"Check the database" or "investigate latency" are not steps. Provide the exact command, query, or click path, and the expected result, so the engineer acts rather than investigates.

### No explicit escalation criteria

Without "escalate when X," engineers either give up too early or struggle too long. Define the trigger, the contact, and the information to provide.

### Putting the likely fix on page four

The 80% case should be addressed in the first few steps. If the most common resolution is buried after rare-case diagnostics, every common incident pays the cost of reading past it.

### A runbook that has never been exercised

Untested runbooks contain wrong commands, dead links, and missing steps that only surface during a real incident. Run game days that follow the runbook end to end.

### No owner, so drift is never corrected

A runbook without an assigned owner rots silently. Document ownership in the runbook and tie updates to system changes.

## Self-Check

- Is the runbook written for a stressed, time-pressured reader, with short imperative steps, copy-pasteable commands, and the most likely fix first, rather than narrative explanation?
- Can the runbook be reached in zero clicks from the alert or symptom that triggers it, and does its title name the symptom the engineer will see?
- Is every step actionable (a concrete command or action), ordered (numbered, with explicit if/then decision points), and verifiable (how to confirm it worked)?
- Are exact commands, paths, hosts, and values provided or clearly marked as placeholders, rather than described in prose?
- Are escalation criteria explicit (trigger threshold, contact, rotation, and information to provide), so the engineer knows when to stop and whom to call?
- Does the runbook guide diagnosis (dashboards, log queries, scope-determining queries, a decision tree distinguishing common causes) rather than only stating a fix?
- Has the runbook been exercised end to end (game day, chaos exercise, or real use), and are wrong commands, dead links, or missing steps corrected?
- Is there a documented owner and a maintenance cadence, with runbook updates tied to system changes so the document does not drift from the system?
- Is background and architecture moved to an appendix so it does not bury the procedural critical path?
