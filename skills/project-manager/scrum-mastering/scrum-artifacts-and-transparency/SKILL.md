---
name: scrum_artifacts_and_transparency.md
description: Use when the agent is managing Scrum artifacts such as the product backlog, sprint backlog, or increment, ensuring artifact transparency, defining or evolving the definition of done, or maintaining the integrity and commitment semantics of Scrum artifacts.
---

# Scrum Artifacts And Transparency

Scrum's artifacts exist to make work and progress transparent so that inspection produces good decisions. The product backlog makes desired work visible and ordered, the sprint backlog makes the current sprint's plan and goal visible, and the increment makes the actual outcome visible against a definition of done. When these artifacts are stale, hidden, ambiguous, or owned by no one, the entire inspect-and-adapt loop degrades: decisions are made on bad information, commitments lose meaning, and done becomes a matter of opinion. Agents tend to treat artifacts as passive containers rather than as commitments whose integrity must be actively defended.

Use this skill before managing or redesigning a backlog, defining or changing a definition of done, assessing artifact transparency, or diagnosing why planning and review keep producing poor decisions. The goal is to prevent the agent from allowing artifacts that look complete but conceal ambiguity about priority, scope, or completeness.

## Core Rules

### Treat Each Artifact As A Commitment With An Owner

Each Scrum artifact carries a commitment that gives it meaning. The product backlog is committed to the product goal, the sprint backlog to the sprint goal, and the increment to the definition of done. Each artifact has a clear owner: the product owner is accountable for the product backlog, the developers for the sprint backlog, and the team collectively for the increment. When ownership is diffuse, the artifact drifts. Make accountability explicit before trying to improve the artifact's content.

If you cannot say who owns an artifact and what it is committed to, transparency has already failed.

### Make The Product Backlog A Single Ordered Source Of Truth

The product backlog must be the one place where all desired work appears, ordered by the value and priority the product owner sets. Work that lives only in side spreadsheets, email threads, or individual to-do lists escapes ordering and transparency. Consolidate these sources, and ensure every item is visible, described at a level appropriate to its priority, and ordered. Items near the top should be more refined than items far down, because only near-term work needs to be actionable.

Ordering is a decision, not a sort. If two items are tied, force the priority call rather than leaving it ambiguous.

### Refine Continuously, Not In A Single Big Batch

Backlog refinement is the ongoing work of adding detail, estimates, and sizing to items so they are ready for a future sprint. It is not a one-time grooming event. Spread refinement across the sprint so that the top of the backlog stays ready without consuming the whole team's time. Refinement should clarify intent, acceptance, and size; it should not pre-solve the implementation in a way that removes developer autonomy.

Items that are not ready when planning starts force rushed decisions or carryover. Make readiness a visible property of each item.

### Keep The Sprint Backlog Aligned To The Sprint Goal

The sprint backlog is the plan and the selected items that serve the sprint goal, plus the current understanding of how to achieve it. It should evolve during the sprint as the team learns; it is not frozen. But every change to scope should be tested against the sprint goal: does this change still serve the goal, or has the goal itself changed? If the goal is no longer viable, the product owner may cancel or renegotiate the sprint rather than silently drifting.

Make the sprint backlog visible to the whole team daily so that progress toward the goal is obvious.

### Define Done As A Real Standard, Not A Slogan

The definition of done is the agreement that makes the increment inspectable and releasable. It must be specific, testable, and shared: code reviewed, tests passing, documented where required, deployed to a target environment, acceptance criteria met, and any organizational standards satisfied. A vague definition of done lets work be declared complete that is not actually releasable, which destroys trust in the increment and in velocity.

If two team members would disagree on whether an item is done, the definition is not specific enough.

### Evolve The Definition Of Done Upward

The definition of done should get stricter over time as the team matures and as quality standards rise. Adding criteria is a retrospective outcome; removing criteria to go faster is almost always a mistake that creates hidden debt. When pressure tempts the team to soften done, make the tradeoff explicit and visible rather than letting it happen silently. Track items declared done that later need rework; that gap exposes where done is weak.

A definition of done that never changes usually means no one is using it.

### Make The Increment Genuinely Releasable

The increment is the sum of done work at the end of the sprint, and it must meet the definition of done and be in a useable, releasable state regardless of whether it is actually released. An increment that is done except for integration, done except for testing, or done except for deployment is not an increment. This standard is what makes the sprint review meaningful and what makes the product owner's release decision real.

If the team cannot show a working increment, the problem is upstream: scope, definition of done, or technical practice.

### Defend Transparency Against Pressure

Transparency is often eroded under pressure: backlogs are pruned to hide unpopular items, definitions of done are quietly relaxed, or incomplete work is presented as done to protect velocity. Each erosion corrupts the decision-making the artifacts exist to support. The Scrum Master's role is to make these pressures and their consequences visible, so that tradeoffs are conscious and owned rather than invisible.

## Common Traps

### Diffuse Ownership Of Artifacts

When everyone and no one owns the backlog, it fills with stale and duplicate items. Assign accountability explicitly.

### Side Backlogs And Shadow Lists

Work tracked outside the product backlog escapes ordering and transparency. Consolidate into one source of truth.

### Ambiguous Priority And Ties

Refusing to order items leaves priority implicit and lets the loudest voice win. Force the ordering decision.

### Big-Batch Refinement

Doing all refinement in one exhausting session leaves the top of the backlog underprepared most of the time. Refine continuously.

### Sprint Backlog Drift Without Goal Check

Adding and dropping items without testing against the sprint goal turns the sprint into a random bundle. Re-anchor to the goal.

### Vague Or Aspirational Definition Of Done

Done that means different things to different people lets incomplete work pass as complete. Make it specific and testable.

### Softening Done Under Pressure

Relaxing the definition of done to hit velocity creates hidden debt and destroys trust in the increment. Make the tradeoff visible.

### Increment That Is Done Except

Work that is complete apart from integration, testing, or deployment is not releasable and not a real increment. Fix upstream.

## Self-Check

- [ ] Does each artifact have a clear owner and an explicit commitment (product goal, sprint goal, definition of done)?
- [ ] Is the product backlog the single ordered source of truth, with side lists consolidated?
- [ ] Are backlog items near the top more refined and ready than items further down?
- [ ] Is refinement continuous rather than a single batch event?
- [ ] Does every change to the sprint backlog get tested against the sprint goal?
- [ ] Is the definition of done specific, testable, and shared so two members would agree on completeness?
- [ ] Is the definition of done evolving upward, with softening under pressure made visible?
- [ ] Is the increment genuinely releasable, meeting done without except-for caveats?
- [ ] Are items declared done that later need rework tracked to expose weaknesses in done?
- [ ] Are pressures that erode transparency surfaced rather than absorbed silently?
