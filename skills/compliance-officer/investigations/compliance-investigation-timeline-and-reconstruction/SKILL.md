---
name: compliance_investigation_timeline_and_reconstruction.md
description: Use when the agent is building a timeline or reconstructing the fact pattern of a compliance investigation, sequencing events, corroborating accounts against documents and logs, resolving conflicting dates, identifying gaps and anomalies, or testing whether a narrative is internally consistent and supported.
---

# Compliance Investigation Timeline And Reconstruction

An investigation's conclusion lives or dies in its timeline. Findings are not isolated verdicts; they are reconstructions of who did what, when, in what order, and with what knowledge. A timeline that sequences events correctly can expose an impossible approval, a backdated document, or a coordination pattern that no single interview revealed. A timeline built sloppily, mixing time zones, trusting uncorroborated recollections, or anchoring to the subject's narrative, can manufacture a false story that drives wrong discipline and flawed disclosure. Reconstruction is where partial evidence hardens into a fact pattern, and it is where bias and confirmation pressure do their most dangerous work.

Use this skill when reconstructing the sequence of events in an investigation, when building a master timeline from documents and interviews, when corroborating or challenging a witness account, when resolving conflicting dates and timestamps, when identifying gaps that may indicate deletion or concealment, or when testing whether a proposed narrative is internally consistent. The goal is to make the agent treat the timeline as an evidence-driven analytical product, not a summary of what people said happened.

## Core Rules

### Build A Single Master Timeline From All Sources

A common failure is to keep timelines in interview notes, in separate reviewer memos, or in the investigator's head. Reconstruction requires one consolidated, source-cited timeline that integrates every evidentiary input.

The master timeline should capture for each entry:

- the date and, where relevant, the time;
- the event or action described;
- the source, document identifier, or interview citation;
- the custodian or actor;
- the system or location of the record;
- the reliability and corroboration status;
- any conflict with another entry.

A single timeline forces contradictions into view. Two entries that cannot both be true appear adjacent and demand resolution, whereas scattered notes let them coexist unnoticed. Maintain the timeline as a living document, updated as evidence arrives, with version control so the evolution is auditable.

### Anchor To Contemporary Records Over Recollection

Memory is unreliable and self-serving, especially under investigation pressure. Where a contemporary document and a later recollection conflict, the contemporary record usually carries more weight, but even documents can be wrong, incomplete, or backdated.

Establish a hierarchy of reliability:

- system-generated timestamps and logs, such as access records and transaction commits, generally strongest;
- contemporaneous communications, such as emails and chats sent at the time;
- dated approvals, contracts, and formal records;
- later-created documents, such as memos reconstructing events;
- witness recollection, weighted by corroboration and bias;
- the subject's own account, weighted for self-interest.

Apply the hierarchy consistently. Do not prefer a document when it helps the conclusion and prefer a recollection when the document is inconvenient. Document the reasoning when you depart from the hierarchy, because a regulator or opposing party will probe every such choice.

### Corroborate Every Material Fact

A material fact is one that matters to the conclusion: who approved, who knew, when action was taken, whether a control was bypassed. Material facts should not rest on a single uncorroborated source if corroboration is reasonably available.

For each material fact, ask:

- how many independent sources support it;
- whether the sources are truly independent or share a common origin;
- whether documentary evidence confirms or contradicts it;
- whether system logs place the actor or action at the relevant time;
- whether the fact is consistent with the surrounding sequence;
- what would disprove it, and whether that evidence exists or is missing.

Distinguish corroboration from repetition. Three witnesses who all heard the same rumor are not three independent sources. True corroboration comes from independent evidence that converges on the same fact.

### Resolve Conflicts Deliberately, Not By Default

Investigations surface conflicts: two dates, two accounts of who said what, two versions of an approval. The temptation is to resolve conflicts in favor of the preferred conclusion. Discipline requires resolving them on evidence and reasoning.

For each conflict:

- state the competing versions precisely;
- identify the evidence supporting each;
- assess the reliability of each source using the documented hierarchy;
- test each version against the surrounding sequence for plausibility;
- consider motive to misrepresent on each side;
- determine whether one version makes other events impossible or improbable;
- record the resolution and the reasoning.

Where a conflict cannot be resolved, say so. An honest inconclusive finding is more defensible than a forced conclusion. Labeling something unable to determine is a legitimate outcome when the evidence does not support more.

### Normalize Time Zones, Clocks, And Calendars

Timestamps from different systems live in different time zones, and a failure to normalize creates false sequences. An email sent at 9 PM in one zone and received at 8 AM the next day in another can look like a response preceded the request.

Normalize:

- the time zone of each system, including server time versus user local time;
- daylight saving transitions around the relevant dates;
- calendar system differences where international dates are involved;
- clock skew between systems that may be out of sync;
- the difference between send time, server receipt time, and client display time.

Pick a single reference time zone for the master timeline and convert every entry to it, while preserving the original. Note any clock skew that affects ordering. A regulator will notice a sequencing error that an internal team missed.

### Identify Gaps As Actively As Events

What is missing matters as much as what is present. A gap can indicate deletion, a control that was bypassed, a communication channel that went off-record, or simply incomplete collection. Treat gaps as analytical signals.

Look for:

- periods of silence where activity would be expected;
- missing approvals in a sequence that otherwise shows them;
- absent communications between people who were plainly coordinating;
- documents referenced but not produced;
- time windows with no logs where logs should exist;
- custodians whose data is incomplete;
- a sudden shift from documented channels to undocumented ones.

Distinguish a true gap from an expected absence. Not every silence is suspicious, but a silence in a context that demands activity often is. Document each gap, its possible explanations, and whether collection could fill it before concluding.

### Test The Narrative For Internal Consistency

Before finalizing findings, stress-test the reconstructed narrative. A plausible story can collapse under its own contradictions.

Test by asking:

- does the sequence allow each actor to have done what is claimed, given timing and access;
- are there events that must have happened for the narrative to hold but for which there is no evidence;
- does the narrative depend on an assumption of knowledge that the record does not support;
- would an alternative sequence fit the evidence equally well;
- what is the strongest counter-narrative, and why is it rejected;
- does the narrative rely on a single contested fact that, if flipped, changes the conclusion?

Constructing and rejecting the strongest alternative is a discipline that prevents confirmation bias. If you cannot articulate why the alternative fails, the conclusion is not as solid as it appears.

### Document Assumptions And Confidence Levels

Reconstruction always involves inference. The investigator fills gaps with reasonable assumptions. The danger is treating assumptions as findings.

For the timeline and conclusions:

- separate established facts from inferences;
- label each inference with its basis and confidence;
- identify the load-bearing assumptions, those on which the conclusion depends;
- note where additional evidence could change the conclusion;
- distinguish high-confidence reconstructions from speculative ones.

A finding that rests on three well-corroborated facts is different from one that rests on a single inference bridging a gap. The report should make the difference visible so decision-makers and regulators understand what is proven and what is reasonably inferred.

## Common Traps

### Anchoring To The Subject's Narrative

Building the timeline around the subject's explanation lets them frame the sequence. Start from documents and independent witnesses, then test the subject's account against the reconstruction.

### Trusting Timestamps Blindly

System timestamps are strong but not infallible. Clock skew, time zones, and display settings can mislead. Normalize and verify before treating a timestamp as dispositive.

### Confusing Repetition With Corroboration

Multiple people repeating the same account from a common source is not independent corroboration. Seek convergent evidence from independent origins.

### Resolving Conflicts Toward The Preferred Conclusion

The pull to resolve ambiguity in favor of the expected outcome is confirmation bias. Resolve conflicts on evidence and reasoning, and document both the conflict and the resolution.

### Ignoring Gaps As Neutral

A missing approval or a silent period is not merely absent evidence; it can be the signal of deletion or bypass. Investigate gaps actively before treating them as meaningless.

### Forcing A Conclusion From An Inconclusive Record

Declaring misconduct where the evidence only supports suspicion, or exoneration where the evidence is simply incomplete, misstates the record. Use unable to determine where appropriate.

### Mixing Established Fact With Inference

Letting assumptions blend into findings hides the load-bearing inferences from decision-makers. Separate fact from inference and label confidence.

## Self-Check

- Is there a single master timeline integrating all sources, with each entry cited to a document or interview, corroboration status noted, and version control maintained?
- Is a documented reliability hierarchy applied consistently, preferring contemporary records over recollection, with departures from the hierarchy explained?
- Is each material fact corroborated by genuinely independent sources, with repetition distinguished from corroboration?
- Are conflicts between dates or accounts stated precisely, resolved on evidence and reasoning, and the resolution documented, with unable to determine used where appropriate?
- Have time zones, daylight saving, clock skew, and server versus local time been normalized to a single reference, with originals preserved?
- Are gaps treated as active analytical signals, with possible explanations, collection attempts, and distinctions between suspicious and expected absences documented?
- Has the narrative been stress-tested for internal consistency, including the strongest counter-narrative and why it is rejected?
- Are established facts separated from inferences, with load-bearing assumptions and confidence levels labeled?
- Does the reconstruction allow each actor the timing, access, and knowledge the narrative requires, without unsupported assumptions?
- Would flipping any single contested fact change the conclusion, and if so, is that fragility disclosed?
