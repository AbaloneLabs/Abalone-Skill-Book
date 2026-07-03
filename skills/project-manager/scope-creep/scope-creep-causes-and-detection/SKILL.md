---
name: scope_creep_causes_and_detection.md
description: Use when the agent is diagnosing why scope is growing, detecting gold plating and incremental scope creep, distinguishing uncontrolled creep from legitimate scope discovery, or reviewing whether the sources of scope growth are being identified rather than only the symptoms.
---

# Scope Creep Causes And Detection

Scope creep is rarely a single dramatic event; it is the accumulation of small additions that each look reasonable and together break the project. Because each increment is small and well-intentioned, it escapes the change process that would have caught a large request. By the time the schedule or budget is visibly broken, the cause is buried under dozens of accepted micro-additions, and the team cannot reconstruct how they got there. Detection is therefore a continuous diagnostic discipline, not a one-time audit, and it must look at causes, not just at the growing list of features.

The judgment problem is recognizing the patterns that produce creep, separating genuine scope discovery from uncontrolled growth, and reading the early signals before they compound. Agents tend to notice scope growth only when it is large enough to threaten commitments, by which point the causes, ambiguous requirements, sponsor drift, gold plating, and silent acceptance, are hard to unwind. The skill is to detect creep at the increment level and to name the mechanism driving it.

## Core Rules

### Detect Creep At The Increment, Not At The Collapse

Scope creep is invisible at the level of any single addition and obvious only in aggregate. Build a detection mechanism that flags each small addition as it happens: a running log of "small" requests, a periodic comparison of in-flight work against the baseline, and a review of completed work for unrequested extras. Waiting until the schedule slips means detecting the disease at stage four. The increment is where creep is still cheap to reverse.

### Distinguish Creep From Legitimate Scope Discovery

Not all growth is creep. Scope discovery is the legitimate refinement of understanding within the agreed boundaries, where earlier ambiguity is resolved into concrete deliverables. Creep is growth beyond the agreed boundaries, however small. The test is twofold: is the addition within the spirit and letter of the approved scope, and has it gone through the change process? Discovery stays inside the fence and is made visible; creep crosses the fence or bypasses review. Mislabeling creep as discovery is how growth becomes invisible.

### Identify The Dominant Cause Before Treating It

Creep has several distinct causes, and each requires a different response. Ambiguous or incomplete requirements generate additions as the team fills gaps. Sponsor or stakeholder drift generates new wants as priorities shift. Gold plating by the delivery team adds unrequested polish. Poor change discipline lets small requests enter without evaluation. Technical discovery reveals work no one anticipated. Diagnose which mechanism is dominant before acting, because tightening change control does nothing for gold plating, and clarifying requirements does nothing for sponsor drift.

### Watch For Gold Plating As A Specific Pattern

Gold plating is the delivery team adding features, polish, or robustness that were not requested, usually from a desire to exceed expectations or from genuine craft pride. It feels generous but consumes effort, expands the test and maintenance surface, and can introduce defects into otherwise acceptable work. Detect it by comparing delivered work to the agreed scope and asking, for each extra, who requested it and through what decision. Treat unrequested additions as changes requiring the same evaluation as any other.

### Read The Early Warning Signals

Creep announces itself before it breaks anything. Signals include a rising count of "quick" requests, scope discussions that reference verbal agreements rather than the baseline, requirements documents that keep growing without version control, stakeholders expressing surprise that something is "not included," and team members working on items no one formally approved. Treat these as diagnostic, not as noise. Each signal points to a cause: verbal-only agreements indicate weak baselining; surprise exclusions indicate expectation gaps; unapproved work indicates weak change discipline.

### Separate Scope Growth From Effort Growth

Sometimes scope is stable but effort grows, because the work turned out harder than estimated. This is an estimation or risk problem, not scope creep, and conflating the two leads to the wrong fix. Ask whether the deliverable itself changed or whether the same deliverable simply costs more. If the deliverable is unchanged but the effort rose, address estimation and risk; if the deliverable expanded, address scope control. Both can coexist, but they must be diagnosed separately.

### Trace Each Addition To A Decision And A Source

For every increment of growth that has entered the work, record who requested it, what decision approved it, and what source it came from, a stakeholder, the team, a dependency, or a regulation. Additions with no traceable decision are the signature of creep. This trace is what allows later diagnosis of which cause is dominant and which control is weakest. Without it, creep is a feeling; with it, creep is a manageable pattern.

## Common Traps

### Only Detecting Creep After The Schedule Breaks

The agent waits for slippage to investigate scope, by which point the causes are buried under months of additions. The trap is treating creep as an outcome to measure rather than a process to monitor. Detect at the increment.

### Labeling Creep As Discovery To Avoid Conflict

Calling uncontrolled growth "discovery" or "refinement" makes it feel legitimate and removes the pressure to evaluate it. The trap is using a positive word to bypass control. Apply the boundary and process test honestly.

### Treating All Growth With One Generic Fix

The agent responds to every scope increase by "tightening change control," but if the real cause is gold plating or ambiguous requirements, the fix misses. The trap is a single-tool response to a multi-cause problem. Diagnose the dominant cause first.

### Ignoring Gold Plating Because It Looks Like Initiative

Unrequested extras from a keen team feel like value, so they go unchallenged. The trap is rewarding behavior that expands test surface and risk without approval. Compare delivered work to agreed scope.

### Accepting Verbal Agreements As Scope

A stakeholder says "just add this" in a meeting, the team does it, and it never enters the baseline. The trap is that verbal scope is invisible scope, immune to control. Require additions to be recorded and baselined.

### Conflating Effort Growth With Scope Growth

The work costs more, so the agent assumes scope crept, when the deliverable is unchanged and the estimate was simply wrong. The trap is applying a scope fix to an estimation problem. Separate the two diagnoses.

### No Traceability For Small Additions

Small requests are absorbed without a record, so the pattern of who and why is lost. The trap is that creep becomes undiagnosable. Log every addition with source and decision, however minor.

## Self-Check

- [ ] Is scope growth detected at the increment level through a running log and periodic baseline comparison, rather than only after slippage?
- [ ] Is each addition tested against both criteria, within agreed boundaries and through the change process, to distinguish creep from discovery?
- [ ] Has the dominant cause of growth, ambiguous requirements, sponsor drift, gold plating, weak discipline, or technical discovery, been diagnosed before a fix is chosen?
- [ ] Is gold plating detected by comparing delivered work to agreed scope and tracing each extra to a request and decision?
- [ ] Are early warning signals, rising quick requests, verbal agreements, surprise exclusions, unapproved work, treated as diagnostic rather than ignored?
- [ ] Is scope growth separated from effort growth, so that estimation problems are not misdiagnosed as creep?
- [ ] Does each addition carry a traceable record of source, requester, and approving decision?
- [ ] Are verbal agreements converted to recorded, baselined scope rather than absorbed invisibly?
- [ ] Is the scope baseline versioned so that growth is visible over time rather than absorbed silently?
- [ ] When growth is detected, is the cause named specifically enough to choose the correct corrective action?
