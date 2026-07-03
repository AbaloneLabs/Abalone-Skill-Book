---
name: incident-root-cause-analysis.md
description: Use when the agent is conducting or contributing to a post-incident root cause analysis, facilitating a blameless retrospective, distinguishing proximate cause from systemic cause, deciding corrective actions, or determining whether an incident reveals a deeper product or process failure.
---

# Incident Root Cause Analysis

After an incident is stabilized, the work that determines whether it was a one-off or the first of many begins: root cause analysis. The purpose is not to assign blame or to produce a document for the file. It is to understand why the incident was possible, so that the organization can change the conditions that allowed it. A shallow analysis names a proximate trigger and closes the case; a meaningful analysis traces the chain of decisions, assumptions, and systemic gaps that made the trigger consequential. The difference determines whether the same class of incident recurs.

This skill covers the judgment needed to run or contribute to a root cause analysis that actually prevents recurrence rather than producing theater.

## Core Rules

### Pursue systemic cause, not just proximate trigger

Every incident has a proximate trigger: the specific action, change, or event that set it off. Stopping at the trigger is the most common analysis failure, because the trigger is usually a symptom of a deeper condition. The valuable question is why the trigger had the impact it did.

- A bad deploy is a trigger. The systemic causes might be: the deploy lacked automated safeguards, the change was too large to review safely, the rollback path was untested, or the feature was insufficiently instrumented to detect the problem early.
- A third-party outage is a trigger. The systemic causes might be: excessive dependency on a single provider without fallback, no circuit breaker to isolate the failure, or no monitoring of the dependency's health.

For each proximate trigger, ask "why was this possible?" at least three to five times, tracing backward through the conditions that allowed each step. Stop only when you reach a cause that, if addressed, would have prevented the class of incident, not just this instance.

### Run the analysis blameless

People involved in an incident must be able to speak honestly about what they saw, decided, and did, or the analysis cannot reach the truth. If the process assigns blame, people become defensive, withhold information, and frame events to protect themselves, and the systemic causes stay hidden.

- Frame the analysis around what happened and why the system allowed it, not who did what wrong. Replace "why did you do X?" with "what information or incentives led to X being the reasonable choice at the time?"
- Assume that everyone acted reasonably given the information and pressures they faced. If an action seems unreasonable, the question is what context made it seem reasonable, not what was wrong with the person.
- Treat human error as a signal, not a cause. "Human error" means the system allowed a human to make a mistake with serious consequences; the fix is to change the system, not to exhort humans to be more careful.

A blameless culture is not a consequence-free culture. It is a diagnostic posture that recognizes blame suppresses the information needed to prevent recurrence. Performance and accountability issues are handled separately, through normal management, not through the incident review.

### Reconstruct the timeline from evidence, not memory

Memory of an incident is unreliable, especially under stress. Participants conflate the order of events, misremember timings, and fill gaps with plausible but incorrect detail. Reconstruct the timeline from contemporaneous evidence: logs, deploy records, alerts, chat messages, ticket timestamps.

- Build the timeline collaboratively, with each participant contributing what they observed and when, validated against the evidence.
- Mark uncertainties explicitly. Where evidence and memory conflict, trust the evidence, but investigate the conflict; it may reveal that the evidence itself is misleading.
- Capture not just what happened but when people knew it. Many incidents involve a gap between when the problem started and when it was detected or understood, and that gap is often the most actionable finding.

### Identify the contributing factors, not just the single cause

Real incidents rarely have a single cause. They result from a convergence of factors, any one of which, if absent, might have prevented or reduced the incident. Reducing the analysis to "the cause" loses the actionable contributing factors.

- List all the conditions that contributed: the trigger, the safeguards that failed or were absent, the detection delay, the communication gaps, the environmental factors (load, time of day, recent changes).
- For each contributing factor, ask whether addressing it would have prevented the incident, reduced its impact, or reduced its duration. This prioritizes which factors are worth investing in fixing.
- Resist the pressure to name a single root cause for the executive summary. A single-cause narrative is satisfying and usually wrong, and it leads to fixing one thing while the other contributing conditions remain.

### Distinguish fixable systemic gaps from inherent residual risk

Not every contributing factor can or should be fixed. Some reflect inherent tradeoffs: a fast deployment pipeline carries more risk than a slow one; a feature-rich product has more surface area than a minimal one; a dependency on a third party cannot be fully eliminated. The analysis should distinguish systemic gaps that are worth closing from risks that are inherent and accepted.

- For each finding, ask: is this a gap we should close, an accepted tradeoff we should make more visible, or an inherent risk we should monitor but cannot eliminate?
- Be honest about cost. A safeguard that would prevent this class of incident but slow every future release by 30% may not be worth it; the analysis should surface the tradeoff, not pretend the fix is free.
- Make accepted residual risk explicit and owned. Risks that are silently accepted resurface as surprises in the next incident.

### Ensure corrective actions are specific, owned, and resourced

The most common reason incidents recur is that the corrective actions from the previous review were never completed. A list of action items with no owners, no due dates, and no resources is ceremony. Each corrective action must be specific enough to be actionable, owned by a named person, and resourced within the planning cycle.

- Each action should describe what will change, not just the goal. "Improve monitoring" is not an action; "add an alert for anomaly X on service Y, owned by Z, by date W" is.
- Each action needs an owner accountable for completion and a place in the backlog with priority.
- Track actions to completion and review overdue ones. If actions consistently go uncompleted, the problem is in the resourcing and prioritization process, not in the analysis.

### Decide deliberately how much to generalize from a single incident

A single incident can reveal a systemic problem worth broad investment, or it can be a one-off that does not justify sweeping change. Over-generalizing leads to bureaucratic overreaction (heavy process added for a rare event); under-generalizing leads to recurrence. Decide deliberately.

- Look for whether the contributing factors are specific to this incident or structural. A one-off trigger with unique circumstances may warrant targeted fixes; a gap in safeguards or detection that this incident exposed likely affects other areas too.
- Resist both the "we must prevent this ever happening again at any cost" reaction and the "this was a fluke, move on" reaction. Both are usually wrong.

## Common Traps

### Stopping at the proximate trigger

Naming the trigger and closing the case feels like resolution but prevents nothing systemic. The trigger will recur; the question is whether the conditions that made it harmful have changed. Always trace to the systemic conditions.

### "Human error" as a root cause

Labeling the cause as human error ends the inquiry at the least useful point. Humans will always make errors; the system must be designed so that errors are caught, isolated, or limited in impact. Treat human error as the start of the investigation, not the conclusion.

### Blame suppressing honesty

If participants fear blame, the timeline is sanitized, the uncomfortable facts are omitted, and the analysis confirms a comfortable narrative that prevents nothing. Blameless process is a prerequisite for useful analysis, not optional politeness.

### Single-cause narratives for complex incidents

Complex incidents are convergences of factors. Reducing them to one cause for simplicity leads to fixing one factor and being surprised when the others cause the next incident. Preserve the contributing factors in the analysis.

### Corrective actions that are never completed

The review produces action items that sit in a tracker forever, providing false reassurance that the problem is being addressed. If actions are not resourced and tracked to completion, the review has changed nothing. Track completion honestly and escalate chronic non-completion.

### Over-reacting to a rare incident

A vivid, high-profile incident creates pressure to add heavy process so it can never recur. But the cost of the process is paid forever, across every future release, while the incident was rare. Weigh the cost of prevention against the realistic frequency and impact, and prefer targeted safeguards over blanket process.

### Under-reacting to a systemic signal

Conversely, dismissing an incident as a one-off when its contributing factors are structural guarantees recurrence, possibly in a more damaging form. Look for whether the safeguards, detection, and response gaps are specific to this incident or reflect broader weaknesses.

### Analysis focused on producing a document rather than change

If the goal of the review is to produce a write-up for leadership, the analysis will be shaped to look thorough rather than to drive change. The goal is changed conditions; the document is a byproduct. Evaluate the review by what actually changed afterward, not by the quality of the write-up.

## Self-Check

- Did I trace beyond the proximate trigger to the systemic conditions that made the incident possible and consequential?
- Was the analysis conducted blamelessly, so participants could speak honestly about decisions and context?
- Is the timeline reconstructed from contemporaneous evidence, with uncertainties marked and the detection gap identified?
- Did I capture all contributing factors rather than reducing the incident to a single cause?
- For each finding, did I distinguish fixable systemic gaps from accepted tradeoffs and inherent residual risk, and made accepted risk explicit?
- Are corrective actions specific, owned by named individuals, resourced in the backlog, and tracked to completion?
- Did I deliberately weigh whether to generalize from this incident or treat it as targeted, resisting both over- and under-reaction?
- Did I treat "human error" as a signal to investigate the system, not as a conclusion?
- Is the review being evaluated by what conditions actually changed, rather than by the document it produced?
- If this same class of incident recurred in six months, would the analysis and actions from this one have prevented or reduced it?
