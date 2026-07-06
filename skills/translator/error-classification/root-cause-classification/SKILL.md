---
name: root_cause_classification.md
description: Use when the agent is investigating why translation errors occurred rather than only what they are, classifying root causes such as source ambiguity, termbase gaps, process failures, tooling defects, or translator knowledge gaps, performing post-mortem analysis on quality failures, or turning recurring error patterns into systemic preventive actions.
---

# Root Cause Classification

Error classification by type and severity answers what went wrong and how badly. Root cause classification answers why it went wrong, which is the question that enables prevention. A mistranslation recorded as an accuracy error tells you a defect exists; knowing that the defect occurred because the source was ambiguous, or because the termbase lacked the term, or because the translator lacked domain knowledge, or because the CAT tool broke a placeholder, tells you what to change so it does not recur. Agents frequently stop at the what and skip the why, because the what is visible in the segment and the why requires investigation. The harm is that the same root causes generate the same defects project after project, improvement efforts treat symptoms rather than causes, and quality plateaus. Root cause classification is the discipline of pushing past the visible defect to the systemic condition that produced it, so that corrective action prevents the whole class rather than re-fixing individual instances. It is the difference between a program that learns and a program that repeats.

Use this skill when analyzing why errors occurred, classifying their root causes, running a post-mortem on a quality failure, or designing preventive actions from error patterns. The goal is to move from symptom to cause so that action prevents recurrence rather than re-litigating individual defects.

## Core Rules

### Distinguish Symptom From Cause Explicitly

The first discipline is to recognize that the classified error is a symptom, not a cause. A terminology error is a symptom; the cause might be a missing termbase entry, an unclear style guide, a tool that did not enforce the term, or a translator who did not check. An omission is a symptom; the cause might be a segmentation that split a sentence invisibly, a process that rushed the translator, or a source defect that made the segment unreadable. State the symptom, then ask why it happened, and keep asking why until the answer is a condition you can act on. Stopping at the symptom produces corrective action that fixes one segment while the underlying condition generates ten more.

### Use A Root Cause Category Set

Adopt a defined set of root cause categories so causes aggregate and patterns emerge. A practical set separates source-related causes such as ambiguity, errors, or defects in the original; resource causes such as missing or incorrect termbase entries, inadequate style guide, or missing reference materials; process causes such as inadequate time, missing briefing, unclear instructions, or skipped self-revision; tooling causes such as segmentation errors, broken placeholders, or QA configuration gaps; competence causes such as translator domain knowledge or language-pair proficiency gaps; and communication causes such as unanswered queries or late client input. Map each defect to a root cause category during analysis. Without a category set, root causes are described in free text that cannot aggregate, and the systemic pattern stays invisible.

### Investigate Before Assigning Cause

Root cause assignment requires evidence, not assumption. Before assigning a cause, examine the source segment, the reference materials available at the time, the termbase and style guide state, the process timeline, the tool configuration, and any translator queries. A defect that looks like a competence gap may actually be a missing reference; a defect that looks like carelessness may actually be a segmentation that hid content. Investigating protects translators from unfair blame and protects the program from fixing the wrong cause. Record the evidence that supports the assigned cause so the classification is auditable and so later analysts can reassess if new information appears.

### Avoid Defaulting To Translator Error

The most common and most damaging failure mode is defaulting every defect to translator error or competence. This is tempting because the translator is the visible last touch, but it is usually wrong or incomplete. Most defects have contributing causes upstream: an ambiguous source, a missing term, an impossible deadline, a tool that failed. Defaulting to translator error blames the individual, demoralizes the team, and leaves the systemic cause in place to generate more defects. Push past the individual to the conditions that made the defect likely. If competence is genuinely the cause, the evidence will show it, and the response is support and training rather than blame.

### Look For Patterns Across Defects, Not Just Single Causes

A single defect's root cause is interesting; the pattern across many defects is actionable. Aggregate root causes by category, by translator, by content type, by locale, and over time. A cluster of terminology errors across translators points to a termbase gap, not to multiple translators; a cluster of omissions in one content type points to a segmentation or process issue; a cluster of accuracy errors from one translator points to a competence or briefing gap. Pattern analysis is where root cause classification earns its keep, because it converts many individual investigations into a few systemic fixes. Review patterns regularly, not only after a failure, so preventive action is proactive.

### Match The Corrective Action To The Cause Level

The corrective action must address the root cause level, or it will not prevent recurrence. A termbase gap is fixed by adding the term and checking related entries; a process cause is fixed by changing the briefing, timeline, or workflow; a tooling cause is fixed by fixing segmentation, configuration, or the tool itself; a competence cause is fixed by training, mentoring, or reassignment; a source cause is fixed by feeding back to the source author or flagging source defects for review. Mismatched action, such as re-training a translator for a defect caused by a missing term, wastes effort and fails to prevent the next defect. Tie each assigned cause to a specific action with an owner and a verification that the action was taken and effective.

### Distinguish Corrective From Preventive Action

Corrective action fixes the specific defect and its immediate cause for the current batch. Preventive action changes the system so the cause cannot generate the defect again, for this batch or any other. Both matter, but preventive action is where quality improves over time. A post-mortem that produces only corrective fixes, rework this segment, re-check this file, leaves the program vulnerable to recurrence. Explicitly ask, for each root cause, what change would prevent this class of defect across all future projects, and assign that preventive action even when the immediate defect is already fixed.

### Close The Loop By Verifying Effectiveness

A root cause classification is not complete until the corrective and preventive actions are verified effective. After action is taken, monitor whether the defect class recurs. If terminology errors drop after the termbase is expanded, the cause was correctly identified and the action worked; if they persist, the cause was mis-assigned or the action was inadequate, and the investigation reopens. Closing the loop turns root cause analysis from a one-time exercise into a learning cycle, and it prevents the program from accumulating actions that look right but do not work.

## Common Traps

### Stopping At The Symptom

Classifying the defect type without asking why leaves the systemic cause in place to generate the same defects repeatedly.

### Free-Text Causes That Cannot Aggregate

Root causes described in prose rather than mapped to a category set stay invisible at the pattern level and cannot drive systemic fixes.

### Assigning Cause Without Investigation

Assuming a cause from the defect alone risks blaming the translator for upstream conditions and fixing the wrong thing.

### Defaulting To Translator Error

Blaming the visible last touch demoralizes the team and leaves source, resource, process, and tooling causes unaddressed.

### Single-Cause Analysis Without Patterns

Investigating defects one at a time misses the systemic pattern that a few preventive actions could resolve across many projects.

### Action Mismatched To Cause Level

Re-training a translator for a termbase-caused defect wastes effort and fails to prevent recurrence because the real cause is untouched.

### Only Corrective, Never Preventive Action

Fixing the current defect without changing the system leaves the program vulnerable to the same defect class in future projects.

### No Verification Of Effectiveness

Actions taken but never checked for results accumulate as activity without improvement, and mis-assigned causes go undetected.

## Self-Check

Before accepting a root cause classification or closing a post-mortem, verify:

- The classified error is treated as a symptom, and the analysis asks why repeatedly until it reaches an actionable condition.
- A defined root cause category set, covering source, resource, process, tooling, competence, and communication, is used so causes aggregate.
- The assigned cause is supported by evidence from the source, references, termbase, process timeline, tooling, and queries, not assumption.
- No defect is defaulted to translator error without evidence, and upstream conditions are investigated before individual blame.
- Root causes are aggregated by category, translator, content type, locale, and time to find systemic patterns, not only single causes.
- Corrective and preventive actions are matched to the cause level, with specific changes such as termbase expansion, process revision, tooling fixes, or training.
- Each post-mortem produces preventive action that changes the system, not only corrective fixes for the current batch.
- Actions have owners and are verified effective by monitoring whether the defect class recurs, reopening the investigation if it does.
- The analysis moves from symptom to cause to prevention, so the program learns rather than repeats.
- No root cause is assigned without investigation, and no action is declared complete without verified effectiveness.
