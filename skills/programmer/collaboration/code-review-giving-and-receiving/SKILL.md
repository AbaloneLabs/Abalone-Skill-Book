---
name: code_review_giving_and_receiving.md
description: Use when the agent is performing, requesting, responding to, or sequencing code reviews, writing PR review comments, deciding what to block on, calibrating feedback tone, handling review requests, triaging nitpicks, setting approval criteria, or managing async review latency and reviewer load across a team.
---

# Code Review Giving And Receiving

A code review is a judgment act with two simultaneous goals: catch the problems that matter, and keep the author able to ship and to keep contributing. Agents tend to optimize one at the expense of the other. Some treat review as a rubber stamp that approves anything that compiles and passes tests, letting real defects and design risks slip into main. Others treat it as a performance of thoroughness, burying the author in stylistic nits and personal preferences until the review becomes a tax on every change and the author learns to avoid review altogether.

The harder failure is misreading what a review is for. A review is not a gate for "would I have written it this way." It is a check that the change is correct enough, safe enough, and legible enough to live in the codebase and be maintained by someone who was not there when it was written. That means the reviewer's job is to prioritize ruthlessly, to separate the few things that must change from the many things that could be different, and to deliver feedback in a form the author can act on without losing context or morale. The author's job is to make the change reviewable, to respond to feedback without defensiveness, and to know when to push back. Both sides fail when they treat review as a contest to be won rather than a shared decision about what enters the codebase.

## Core Rules

### Prioritize By Severity, Not By What You Noticed First

Not every comment carries equal weight. A review that lists thirty observations in the order the reviewer happened to read them forces the author to re-derive which ones matter. Sort your feedback by what actually threatens the change.

A workable priority order, highest to lowest:

- **Correctness** — logic errors, wrong edge cases, broken invariants, race conditions, data loss. These block.
- **Security and safety** — injection, auth bypass, secret leakage, unsafe operations, resource exhaustion. These block.
- **Behavior contract changes** — silent API changes, altered error semantics, changed ordering, removed guarantees. These block unless intentional and communicated.
- **Design and maintainability** — coupling introduced, abstraction leaks, missing tests, unclear ownership. These often block, sometimes defer.
- **Readability** — naming, structure, comments needed where intent is non-obvious. Address when they materially impair future reading.
- **Style and nits** — formatting, minor wording, personal preference. Optional, and ideally automated away.

State the priority explicitly. A comment tagged "blocking: race on shared counter" reads completely differently from the same sentence tagged "nit." When everything is presented as equally important, nothing is, and the author either over-reacts to trivia or under-reacts to a real defect.

### Make Comments Actionable And Specific

A useful review comment tells the author what is wrong, where, why it matters, and what would resolve it. A comment that says only "this is bad" or "consider refactoring" forces the author to guess and invites a round of clarification that delays the whole change.

Strong comments are concrete: they name the file and line, describe the failure mode or risk, and either propose a fix or describe the property the fix must have. "If `users` is empty here, `users[0]` throws; guard for empty or document why the caller guarantees non-empty" is actionable. "Be careful with empty lists" is not. When you propose an alternative, mark whether it is a suggestion, a requirement, or an idea the author may discard, so the author knows whether to implement, defend the current approach, or simply consider it.

### Separate Preference From Defect

Much review friction comes from presenting taste as obligation. "I would have used a map here" is preference; "this linear scan turns the function from O(1) to O(n) on a hot path" is a defect. Preference comments are legitimate but must be labeled as such and must be cheap for the author to decline. Defect comments must be justified by a concrete consequence — performance, correctness, maintainability, security — not by appeal to how the reviewer would have done it.

When you find yourself writing "consider," ask whether the consideration is driven by a real risk or by your own habits. If it is habit, either drop it or file it as an explicit nit. Reviewers who convert every preference into a blocking comment train authors to either fight every review or stop seeking review.

### Calibrate Tone To The Person And The Stakes

Review comments persist and are read by people who lack the context of the moment, including the author's manager and future contributors. Write comments you would be comfortable seeing quoted. This is not about softness; a blunt comment about a security hole is appropriate. It is about separating the defect from the person and avoiding language that reads as contemptuous, sarcastic, or dismissive.

Concrete practices:

- Comment on the code, not the author. "This loop skips the last element" rather than "you forgot the last element again."
- Ask questions before asserting. "Does this handle the case where the token is expired?" invites explanation; "this doesn't handle expired tokens" may be wrong and puts the author on the defensive.
- Avoid sarcasm and backhanded praise. They do not survive being read without tone.
- Match the severity register to the severity of the issue. Flooding a change with exclamation points over style undermines the signal when a real emergency appears.

### Keep Review Latency Predictable

Async review is a pipeline. A change stuck in review blocks everything that depends on it, and the author cannot tell whether silence means "still reading," "waiting on someone else," or "forgotten." The harm is not only delay but uncertainty, which freezes parallel work.

As a reviewer, respond within whatever cadence your team has agreed to, and if you cannot, say so. A one-line "swamped today, will get to this tomorrow morning" unblocks the author to plan. As an author requesting review, set expectations: tag the right reviewers, describe what kind of review you need (quick sanity check versus deep design review), and call out time sensitivity. A review request that says "this is on the critical path for the release, needs eyes on the auth flow specifically" gets a different and faster response than a bare "PTAL."

### Bound Reviewer Load And Scope

A reviewer's effectiveness drops sharply with the size and complexity of the diff. A two-hundred-line change with a clear description gets a careful read; a two-thousand-line change gets a skim that misses exactly the subtle bugs review is meant to catch. Both authors and reviewers are responsible for keeping reviews scoped.

Authors should split large changes into reviewable units, each making one logical change. Reviewers should push back on changes that are too large to review safely rather than approving them out of fatigue. If a change is genuinely indivisible, that is a signal to invest in tests, documentation, and a walkthrough, not to lower the review bar.

### Agree On What "Approved" Means

Teams stall when approval criteria are implicit and differ per reviewer. Make the bar explicit. Common bars include "no blocking comments unresolved," "all blocking comments resolved and non-blocking ones acknowledged," or "at least one reviewer other than the author signs off." Whatever the rule, it should be knowable in advance and applied consistently.

Distinguish approval from agreement. A reviewer can approve a change they would have built differently, as long as the change meets the agreed bar. Conversely, a reviewer who disagrees with the design but cannot point to a concrete consequence should not block; they should raise the design question in a forum where it can be decided on its merits, not held hostage in a single PR.

## Common Traps

### Nitpicking As A Substitute For Review

Filling a review with formatting and naming comments creates the appearance of thoroughness while avoiding the harder work of checking logic, security, and design. A review with twenty nits and no correctness analysis has not protected the codebase. Lead with the highest-severity issues; nits are filler, not the meal.

### Approving To Clear The Queue

When review is a bottleneck, the temptation is to approve quickly to reduce the backlog. This converts review from a safety check into a rubber stamp and teaches authors that review is a formality. If you do not have time to review properly, say so and ask for another reviewer rather than approving what you did not read.

### Blocking On Style That Should Be Automated

If a style rule matters enough to block on, it should be enforced by a linter or formatter, not by a human reading diffs. Reviewers who manually police formatting waste their attention and introduce inconsistency, since different reviewers enforce different rules. Push style into tooling and reserve human review for what tooling cannot judge.

### The Drive-By Architecture Rewrite

A reviewer who uses a focused bug fix as the occasion to demand a large restructuring has confused review with design. Even if the restructuring is desirable, forcing it into an unrelated change inflates the diff, delays the fix, and entangles risk. Raise structural improvements as their own proposal with their own justification.

### Defensiveness That Turns Review Into Negotiation

An author who argues every comment, including correct ones, turns review into a haggling process that exhausts reviewers and slows the team. Distinguish "this comment is wrong, here is why" from "I do not want to change this." The first is valuable pushback; the second is friction. Reserve disagreement for comments that are genuinely incorrect or harmful, and accept the rest even when they are not how you would have done it.

### Letting Reviews Drift Until They Stale

A review left open for weeks rots: the codebase moves, conflicts appear, context is lost, and the change either gets force-merged without re-review or abandoned. If a review is stalled, the author should escalate or close it; the reviewer should not let it sit silently. Stale reviews are debt that compounds.

## Self-Check

- [ ] Comments are sorted or tagged by severity, and blocking issues (correctness, security, contract changes) are clearly distinguished from optional nits.
- [ ] Each comment names the location, describes the concrete consequence, and is actionable — the author knows what to do without a follow-up round.
- [ ] Preference comments are labeled as preference or nit and are cheap to decline; blocking comments are justified by a real consequence, not taste.
- [ ] Tone addresses the code and the problem, avoids sarcasm and personal framing, and would be acceptable if quoted without context.
- [ ] Review latency is predictable: the reviewer acknowledges requests they cannot immediately handle, and the author sets expectations about urgency and the kind of review needed.
- [ ] The diff is sized so it can actually be reviewed; oversized changes were split or given extra test and documentation support rather than a lowered bar.
- [ ] Approval criteria are explicit and applied consistently, and approval does not require the reviewer to agree with every choice, only to confirm the change meets the bar.
- [ ] Style and formatting issues that recur were moved into automated tooling rather than re-litigated in every review.
- [ ] Stalled reviews were escalated or closed rather than left to rot, and structural concerns raised in review were separated out as their own proposals when they exceeded the change's scope.
