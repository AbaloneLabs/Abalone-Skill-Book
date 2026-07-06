---
name: problem-definition-and-evidence-boundary.md
description: Use when the agent is framing an operational problem for root cause analysis, defining the defect or incident boundary, separating symptoms from causes, gathering evidence, deciding what data is reliable, or preventing premature conclusions before a corrective action is chosen.
---

# Problem Definition And Evidence Boundary

Root cause work fails when the problem is defined casually. If the team investigates "the process is broken" or "the vendor failed," it will find a convenient story rather than the conditions that created the failure. Agents often jump straight to causes or fixes because the visible symptom is compelling. This skill helps the agent define the problem, evidence boundary, and uncertainty before analysis begins.

## Core Rules

### State The Problem In Observable Terms

Define what happened, where, when, how often, who or what was affected, and how it differed from the expected standard. Use operational language: wrong disposition, missed SLA, duplicate shipment, unsafe handoff, reconciliation mismatch, queue aging, unapproved exception, or repeated rework.

Avoid broad labels such as "communication issue," "training problem," or "human error." Those may be possible causes, but they are not good problem statements.

### Define The Boundary Of Analysis

Clarify what is in scope and out of scope: time period, locations, teams, vendors, customers, systems, work types, product versions, shifts, and process steps. A boundary prevents the analysis from expanding endlessly or missing the affected segment.

Also define comparison cases. Knowing where the problem did not occur can be as useful as knowing where it did.

### Preserve A Timeline

Create a chronological timeline of detection, triggering events, decisions, handoffs, system changes, communications, escalations, and recovery actions. Timelines reveal sequence, latency, and missed decision points that static summaries hide.

Separate event time from discovery time. A defect may have started days before it was detected, and the detection gap may be part of the root cause.

### Gather Multiple Evidence Types

Use case samples, logs, queue data, audit trails, SOP versions, training records, staffing schedules, vendor notices, customer reports, system changes, screenshots, chat decisions, and quality findings. Each evidence type has blind spots.

Do not base root cause on the loudest anecdote or a dashboard alone. Combine quantitative pattern and qualitative examples.

### Rate Evidence Reliability

Identify which evidence is verified, incomplete, inferred, stale, manually edited, sampled, self-reported, or contradicted. A root cause built on weak evidence should be labeled as provisional.

If data quality is poor, that may itself be a contributing cause. Do not hide weak evidence because the organization wants closure.

### Separate Symptoms, Causes, And Consequences

Symptoms are what appeared; causes are conditions that made the failure likely; consequences are harm after the failure. Backlog, complaints, and escalations may be consequences rather than causes. A missing field may be symptom of poor intake design, unclear policy, or tool constraint.

Use this separation to avoid treating cleanup work as prevention.

### Identify The Decision To Be Made

Root cause analysis should support a decision: corrective action, containment lift, policy change, staffing adjustment, vendor escalation, control redesign, training, automation, or risk acceptance. If the decision is unclear, the analysis may become a narrative exercise.

State who will decide and what evidence they need. High-risk findings may require compliance, legal, safety, security, finance, or executive review.

### Protect Against Blame Bias

People are visible; systems are less visible. Before naming an individual or team as the cause, examine process design, workload, tools, incentives, supervision, training, access, handoffs, priority conflicts, and control gaps. Individual behavior can matter, but it should be described in context and with evidence.

Blame-heavy analysis reduces reporting and hides future risk.

### Decide When Evidence Is Enough

Perfect certainty is rarely available. Decide what level of confidence is enough for the risk and reversibility of action. Low-risk fixes may proceed with moderate confidence; high-stakes corrective actions need stronger evidence and specialist review.

If uncertainty remains, choose reversible action, additional monitoring, or staged corrective action rather than pretending certainty.

## Common Traps

- Starting analysis from a broad complaint instead of a precise operational failure.
- Treating the first visible symptom as the problem boundary.
- Ignoring where the issue did not occur, which could reveal process or segment differences.
- Mixing event time and discovery time, hiding detection failure.
- Using only metrics or only anecdotes instead of combining evidence types.
- Building conclusions on manually edited, incomplete, or stale data without labeling uncertainty.
- Calling backlog or complaints the cause when they may be consequences; letting the analysis become a story with no decision owner
- Naming human error before checking workload, tool, incentive, handoff, and control design; demanding impossible certainty for reversible fixes or accepting weak evidence for high-risk actions

## Self-Check

- Is the problem statement observable, specific, and separated from suspected causes?
- Is the analysis boundary clear by time, location, team, system, work type, and process step?
- Are unaffected comparison cases considered where useful?
- Does the timeline distinguish event, detection, decision, communication, and recovery points?
- Are multiple evidence types used rather than one anecdote or one dashboard?
- Is evidence reliability labeled, including gaps and contradictions?
- Are symptoms, causes, and consequences separated?
- Is the decision the analysis must support explicit?
- Has blame bias been checked against process, tool, workload, incentive, training, access, and handoff factors?
- Is the confidence level appropriate for the risk and reversibility of the corrective action?
