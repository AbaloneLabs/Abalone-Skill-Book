---
name: feedback_synthesis.md
description: Use when the agent is synthesizing design feedback, consolidating critique notes, reconciling conflicting reviewer opinions, merging user research findings with stakeholder input, turning raw feedback into prioritized design decisions, or deciding which feedback to act on, defer, or reject, and must distinguish signal from noise, weigh evidence, and avoid both over-reacting to loud voices and ignoring quiet but important observations.
---

# Feedback Synthesis

Receiving feedback is easy; making sense of it is hard. A design round produces a tangle of comments: some specific, some vague, some contradictory, some grounded in user evidence, some rooted in personal taste, some urgent, some trivial. The judgment problem is not collecting feedback, which is the easy part. It is deciding what the feedback actually means, which comments reflect real user problems versus reviewer preference, how to reconcile comments that disagree, and how to avoid the two opposite failures: changing the design in response to every loud comment until it loses coherence, or dismissing feedback as ignorance until the design ships with known problems. Agents tend to fail by treating all feedback as equal, by over-weighting the most recent or most emphatic comment, by reading a cluster of preference comments as a defect, and by failing to trace feedback back to the user consequence that determines whether it matters.

Use this skill when consolidating feedback from a critique, a usability test, stakeholder review, beta, or any source of design input, and when deciding what to change, defer, or reject. The goal is a prioritized, reconciled set of decisions that improves the design without surrendering its coherence.

## Core Rules

### Trace Every Piece Of Feedback To A User Consequence

Feedback becomes actionable only when you understand what user problem it points at. "The header is too big" is a comment; the user consequence might be that the primary content is pushed below the fold on small screens, or it might be nothing more than the reviewer's aesthetic preference. Before deciding how to respond, ask what would actually go wrong for the user. If there is no user consequence, the feedback is preference; if there is one, it is a candidate defect.

For each comment, determine:

- what user problem it describes, if any;
- whether the problem is grounded in evidence, observation, or inference;
- how severe the consequence is, from minor friction to task failure;
- how many users it likely affects.

Feedback without a traceable user consequence should be weighted far lower than feedback with one, no matter how confidently it was delivered.

### Cluster Feedback To Find Underlying Themes

Individual comments are often symptoms of a single underlying issue. Five reviewers saying different things about a screen, one about clutter, one about unclear hierarchy, one about not knowing where to click, one about too many options, one about feeling overwhelmed, may all be pointing at the same root cause: the screen lacks a clear primary action. Treating each comment as separate produces five scattered fixes; clustering reveals one structural problem.

Cluster by:

- the underlying problem each comment points at, not its surface wording;
- the screen, flow, or stage the comments relate to;
- the type of issue: hierarchy, clarity, error, performance, consistency, emotion.

Clustering turns a long list of comments into a small set of themes, each of which may be solved by one coherent change rather than many piecemeal ones.

### Reconcile Conflicting Feedback By Examining The Evidence

Conflicting feedback is common and is not a reason to give up. One reviewer says the flow is too long; another says it needs more steps for clarity. The resolution lies in the evidence behind each view: which reviewer is closer to the real user, which is grounded in data versus assumption, and whether the conflict reflects two different user segments with genuinely different needs. Do not resolve conflict by averaging or by picking the loudest voice; resolve it by weighing the evidence and, where possible, testing.

When feedback conflicts:

- identify what each side is optimizing for, such as speed versus safety;
- check which view is closer to observed user behavior;
- consider whether both can be right for different segments or contexts;
- where the stakes warrant it, test rather than guess.

Conflict is often a signal that two legitimate needs are in tension, which is itself a design insight.

### Distinguish Signal From Noise And Loud From Important

Not all feedback carries equal weight, and volume is not importance. A senior stakeholder's emphatic comment, the most recent piece of feedback, and the comment repeated most often all feel weightier than they may be. The disciplined approach is to weight feedback by evidence and user impact, not by who said it, how loudly, or how many times. A single quiet, evidence-grounded observation about a real user failure matters more than ten loud preferences.

Guard against:

- recency bias, overweighting the last comment received;
- authority bias, treating seniority as correctness;
- frequency bias, treating repetition as confirmation when several people may be echoing one source;
- volume bias, treating emphasis as importance.

Re-weight the feedback by user consequence and evidence before deciding what to act on.

### Separate Defects, Preferences, And Questions Before Prioritizing

The same labeling discipline that improves critique improves synthesis. Defects, comments with real user consequences, belong at the top of the priority list. Preferences belong in a separate, lower-weighted bucket. Questions belong in a clarification bucket that may resolve misunderstandings without any design change. Mixing them produces a priority list where taste competes with real problems and the real problems lose.

Sort into:

- defects to address, ranked by severity and reach;
- preferences to consider, weighted low and optional;
- questions to answer, which may dissolve the apparent issue entirely.

This separation prevents preference feedback from displacing defect fixes when time is limited.

### Decide Explicitly What To Change, Defer, And Reject

Synthesis must end in decisions, not a list. For each cluster or theme, decide explicitly: change now, defer to a later round, or reject with a reason. A decision to defer or reject is still a decision, and it should be recorded with its rationale so the same feedback is not relitigated later. Vague "we will consider this" resolves nothing and leaves the design exposed.

Record for each theme:

- the decision: change, defer, or reject;
- the rationale, tied to user consequence and evidence;
- the owner and timing if it is a change;
- the condition under which a deferred item would be revisited.

Explicit decisions close the loop and protect the design from both over-reaction and drift.

### Preserve Design Coherence Across Changes

Acting on feedback one comment at a time, without considering the whole, fragments the design. Each fix must be checked against the overall structure, hierarchy, and interaction model so that solving one problem does not create another. A change that improves one screen but breaks consistency with the rest of the product is a net loss. After deciding on changes, review them as a set for coherence before implementing.

## Common Traps

### Treating All Feedback As Equal

Weighting every comment the same drowns real defects in preference noise. Trace each to a user consequence and weight accordingly.

### Over-Reacting To The Loudest Voice

Seniority, volume, and recency feel important but are not evidence. Re-weight by user impact, not by who spoke loudest or last.

### Reading Preference Clusters As Defects

Several people disliking a color is still preference, not a user problem. Distinguish preference clusters from defect clusters before prioritizing.

### Fixing Comments Instead Of Themes

Treating each comment as a separate fix produces scattered changes. Cluster to find the underlying issue and solve it once, coherently.

### Resolving Conflict By Averaging

Splitting the difference between conflicting views satisfies no one and solves nothing. Resolve by weighing evidence and testing where stakes are high.

### Leaving Decisions Vague

"We will consider this" resolves nothing and lets problems recur. Make explicit change, defer, or reject decisions with recorded rationale.

### Ignoring Quiet But Important Observations

The most useful feedback is sometimes the quietest. Collect input independently so quiet, evidence-grounded observations are not drowned out.

### Fragmenting The Design With Piecemeal Fixes

Solving comments one by one without checking coherence breaks consistency and hierarchy. Review the full set of changes together before implementing.

## Self-Check

- [ ] Each piece of feedback was traced to a user consequence, and comments without one were weighted as preference.
- [ ] Feedback was clustered by underlying theme rather than treated as isolated comments.
- [ ] Conflicting feedback was reconciled by examining evidence and segment needs, not by averaging or deferring to volume.
- [ ] Feedback was re-weighted by user impact and evidence, correcting for recency, authority, and frequency bias.
- [ ] Defects, preferences, and questions were separated before prioritization.
- [ ] Each theme resulted in an explicit change, defer, or reject decision with recorded rationale.
- [ ] Deferred items have a stated condition for revisiting, and rejected items have a stated reason.
- [ ] The full set of changes was reviewed for coherence against the overall structure before implementation.
- [ ] Quiet but evidence-grounded observations were not lost to louder but weaker feedback.
- [ ] The synthesis ended in prioritized decisions, not an unresolved list of comments.
