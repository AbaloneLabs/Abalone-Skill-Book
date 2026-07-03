---
name: issue_logging_and_triage.md
description: Use when the agent is logging project issues, triaging issues by urgency and impact, prioritizing the issue queue, running an issue intake and triage process, or deciding which problems that have already occurred deserve immediate attention and resources.
---

# Issue Logging and Triage

An issue is a problem that has already happened or is happening now, unlike a risk, which may happen. Because issues are real and current, they compete for the same finite attention and resources, and the queue can overwhelm the team if it is not controlled. The judgment problem is that every issue feels urgent to the person raising it, but not every issue is urgent to the project. Without disciplined triage, the team chases the loudest or most recent problems while the high-impact ones quietly compound, or it tries to solve everything at once and solves nothing well. Logging and triage are what turn a flood of complaints into a prioritized, owned, actionable queue.

Agents tend to treat logging as bureaucratic overhead and triage as a quick sort. The skill is to recognize that a well-kept issue log is the project's operational nervous system, and that triage is a deliberate prioritization decision based on urgency and impact, not a reaction to who is upset. Good intake captures issues completely enough to act on; good triage ensures the right problems get attention first.

## Core Rules

### Capture Issues Completely at Intake

An issue logged as "build is broken" or "vendor problem" is not actionable. Each entry should record what happened, when it was detected, who is affected, the current impact, and the reporter. Capture enough detail that someone unfamiliar with the situation could understand it, because the resolver may not be the person who raised it. Incomplete intake forces follow-up questions that delay response and lose context. Standardize the intake fields so that every issue arrives with the minimum information needed to triage.

### Separate Issues From Risks and Change Requests

Keep the categories distinct so each gets the right process. An issue is happening now and needs resolution; a risk may happen and needs mitigation; a change request is a proposed alteration to the baseline. The same event can appear in multiple forms, but the response discipline differs. Logging a risk as an issue triggers premature resolution work; logging an issue as a risk delays action on something real. Classify at intake and let items move between categories as their nature clarifies.

### Triage on Urgency and Impact, Not on Loudness

Urgency is how soon the issue causes damage if unaddressed; impact is how much damage it causes. Triage each issue on both axes: high urgency and high impact demands immediate action; low urgency and low impact can wait; the dangerous combinations are high impact with low urgency, which gets neglected until it becomes critical, and high urgency with low impact, which consumes attention disproportionate to its consequence. Resist prioritizing by who shouts loudest or by recency. Use a consistent scoring guide so triage decisions are comparable across issues and over time.

### Assign a Single Owner at Triage

Every triaged issue needs one named owner accountable for driving it to resolution. Ownership does not mean personally solving it; it means coordinating the response, tracking progress, and escalating when stuck. Collective ownership, where an issue belongs to "the team," ensures no one acts. Assign the owner at triage based on who has the authority and capability to resolve, not merely who is available, and confirm the owner accepts the assignment.

### Define Target Resolution Times by Priority

Each priority level should carry an expected response and resolution timeframe: critical issues addressed within hours, high within days, medium within weeks, low as capacity allows. These targets turn the queue into a managed commitment rather than an open-ended list. When an issue ages beyond its target, it becomes a flag for escalation or re-triage. Without target times, issues drift indefinitely and the queue loses meaning.

### Run Triage as a Regular, Timeboxed Discipline

Triage works only if it happens consistently. Hold a short, timeboxed triage session on a fixed cadence to review new issues, re-prioritize the open queue, close resolved items, and flag aged ones. Triage should be fast because the intake is complete and the scoring guide is consistent; if triage meetings drag, the intake or the criteria are weak. Ad hoc triage, only when someone complains, guarantees that quiet high-impact issues are missed.

### Detect Patterns, Not Just Individual Issues

A well-kept log reveals systemic problems that single issues do not. Repeated issues from the same vendor, the same component, or the same process point to a root cause worth fixing once rather than logging repeatedly. Review the log periodically for clusters and trends, and convert recurring patterns into improvement actions or risks. Treating each issue in isolation misses the leverage of fixing the underlying cause.

### Keep the Log Visible and Current

An issue log that no one can see, or that is weeks out of date, provides false assurance. Make the log accessible to the team and relevant stakeholders, and keep it current: close resolved issues promptly, update status and impact as situations evolve, and remove obsolete entries. Currency is what makes the log trustworthy; a stale log is ignored, and once ignored, it stops being fed, and the project loses its operational visibility.

## Common Traps

### Logging Issues Too Vaguely to Act On

Entries like "performance problem" or "stakeholder unhappy" cannot be triaged or resolved without a chase for details, delaying response. The trap is that minimal logging feels faster in the moment. Capture complete intake information so the issue is actionable immediately.

### Prioritizing by Loudness or Recency

The issue raised by the most senior or most vocal person gets attention, while a quieter high-impact issue compounds. The trap is that loudness feels like urgency. Triage on urgency and impact using a consistent guide, and protect the queue from political pressure.

### Collective Ownership

Assigning an issue to a team or a group ensures no individual drives it, so it stalls. The trap is that collective ownership feels collaborative. Assign a single named owner accountable for resolution.

### No Target Resolution Times

Without expected timeframes per priority, issues age indefinitely and the queue becomes a graveyard. The trap is that the list looks managed while nothing closes. Attach target resolution times to each priority and flag aged issues.

### Mixing Issues, Risks, and Changes

Logging everything in one list applies the wrong process to each category, delaying action on issues and triggering premature work on risks. The trap is that a single list feels simpler. Keep categories distinct and let items migrate as their nature clarifies.

### Ad Hoc Triage

Triage happens only when someone complains, so quiet high-impact issues are never reviewed. The trap is that reactive triage feels responsive. Run triage on a fixed cadence so the whole queue is reviewed regularly.

### Treating Issues in Isolation

Each issue is handled separately, so recurring root causes are never fixed and the same problems reappear. The trap is that individual handling feels thorough. Review the log for patterns and convert clusters into root-cause fixes.

## Self-Check

- [ ] Does each logged issue capture what happened, when, who is affected, current impact, and reporter?
- [ ] Are issues, risks, and change requests kept in distinct categories with the right process for each?
- [ ] Is triage based on urgency and impact using a consistent scoring guide, not on loudness or recency?
- [ ] Does every triaged issue have a single named owner accountable for resolution?
- [ ] Are target response and resolution times defined for each priority level?
- [ ] Is triage run on a regular, timeboxed cadence that reviews new, open, and aged issues?
- [ ] Are aged issues beyond their target resolution time flagged for escalation or re-triage?
- [ ] Is the log reviewed periodically for patterns and recurring root causes?
- [ ] Is the issue log visible to the team and stakeholders and kept current?
- [ ] Are resolved issues closed promptly so the queue reflects only active problems?
