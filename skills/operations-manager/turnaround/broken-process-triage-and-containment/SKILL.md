---
name: broken-process-triage-and-containment.md
description: Use when the agent is triaging a broken operational process, containing process failure, isolating defects, prioritizing fixes, protecting customers, deciding temporary workarounds, or stabilizing a workflow before root-cause elimination.
---

# Broken Process Triage And Containment

A broken process often creates visible symptoms: backlog, errors, complaints, rework, missed SLAs, or unsafe shortcuts. The temptation is to redesign immediately, but turnaround requires triage first: identify what is failing, contain the damage, protect high-risk work, and decide which fixes can wait. This skill helps the agent contain process failure without cementing temporary workarounds into the new normal.

## Core Rules

### Define the failure mode precisely

Name what is broken: intake, routing, prioritization, approval, execution, quality review, handoff, communication, system update, vendor step, closure, or measurement. Different failure modes need different containment.

Avoid saying "the process is broken" without identifying where work leaves the expected path.

### Segment impacted work

Group work by risk, age, customer impact, revenue impact, safety, compliance, reversibility, complexity, and effort. Some work needs immediate containment; some can wait; some should be paused or rejected until the process is safe.

Triage prevents high-risk work from being buried under low-risk volume.

### Stop the source of new defects

If new work is entering through the broken path, consider pausing intake, narrowing scope, adding a gate, changing forms, stopping a faulty automation, disabling a bad template, or requiring review before execution. Continuing to feed a broken process makes recovery harder.

Containment should stop the defect source, not only clean up downstream mess.

### Use temporary workarounds deliberately

Workarounds may be necessary: manual review, special queue, extra approval, alternate supplier, direct customer communication, spreadsheet tracking, or expert review. Each workaround needs owner, scope, risk, start date, expiry, and exit condition.

Do not allow workarounds to become permanent because they are familiar.

### Preserve control and evidence

Broken processes often tempt teams to bypass approvals, documentation, privacy controls, financial checks, or quality review. Temporary speed should not erase evidence. Define minimum records and compensating controls.

If control gaps are accepted, record who accepted them and when they expire.

### Diagnose enough to choose containment

Containment does not require full root cause, but it does require enough evidence to avoid making the problem worse. Check recent changes, volume shifts, staffing, training, systems, vendor behavior, policy changes, and handoff points.

Do not anchor on the first explanation. Broken processes usually have multiple contributing factors.

### Communicate changed rules clearly

If priorities, intake, routing, SLA, customer messages, approvals, or ownership change during containment, tell affected teams. Old instructions left in circulation create duplicate work and inconsistent customer treatment.

Use a single source of truth for temporary rules.

### Transition from containment to redesign

Containment ends when defects are controlled, high-risk work is protected, temporary rules are documented, and a permanent-fix backlog exists with owners. Do not keep the team in firefighting mode after the process is stable enough for structured redesign.

Permanent fixes should address process design, tools, staffing, training, policy, vendor, or control weaknesses.

### Protect evidence for root-cause work and decide what not to fix immediately

During containment, preserve examples of failed cases, timestamps, system logs, customer messages, decision points, and workaround records. Cleanup can accidentally erase the evidence needed to design a permanent fix.

Do not allow teams to overwrite, close, or merge problem records without retaining enough information to understand the pattern.

Turnaround teams cannot fix every process weakness at once. Explicitly defer lower-risk improvements with owner and review date. This prevents distraction while keeping the issue visible for later hardening.

Unowned deferral is not prioritization; it is disappearance.

## Common Traps

- Calling a process broken without locating the failure mode.
- Treating all backlog or errors as equal.
- Cleaning old defects while new defects continue entering the system.
- Using workarounds without owner, expiry, or exit condition.
- Bypassing controls and losing evidence during urgent cleanup; anchoring on one cause before checking volume, staffing, tools, policy, vendors, and handoffs
- Changing priority or routing rules without telling affected teams; leaving old templates, forms, and instructions active during containment
- Treating containment as the permanent fix; keeping crisis rhythm after the process is stable enough for redesign
- Cleaning up records so aggressively that root-cause evidence is lost; letting lower-risk fixes disappear instead of deferring them with owner and review date

## Self-Check

- Is the failure mode located in intake, routing, priority, approval, execution, quality, handoff, communication, system update, vendor step, closure, or measurement?
- Is impacted work segmented by risk, age, customer, revenue, safety, compliance, reversibility, complexity, and effort?
- Has the source of new defects been paused, gated, narrowed, reviewed, disabled, or corrected?
- Are temporary workarounds defined with owner, scope, risk, start date, expiry, and exit condition?
- Are minimum records, approvals, privacy, financial, quality, and compensating controls preserved?
- Are accepted control gaps documented with authority and expiry?
- Has evidence been checked for recent changes, volume, staffing, training, systems, vendors, policy, and handoffs?
- Are temporary priority, intake, routing, SLA, communication, approval, and ownership rules communicated through one source of truth?
- Are old forms, templates, and instructions retired or flagged during containment?; is there a transition from containment to permanent redesign with owners and backlog?
- Is evidence preserved for root-cause work before records are closed, merged, or overwritten?; are deferred fixes recorded with owner, reason, and review date?
