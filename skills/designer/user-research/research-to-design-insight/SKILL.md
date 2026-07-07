---
name: research_to_design_insight.md
description: Use when the agent is translating user research findings into design implications, turning research themes into actionable design directions, prioritizing which problems to solve, writing problem statements or opportunity areas from research, or bridging raw evidence and design decisions without leaping to unsupported solutions.
---

# Research To Design Insight

Research findings do not design the product. They describe the problem space, and the gap between a research finding and a design decision is where most of the value, and most of the error, occurs. A finding like "users lose track of where they are in a multi-step flow" does not by itself dictate a solution; it opens a set of possible design directions, each with tradeoffs.

Agents tend to fail at this bridge in two opposite ways: by leaping straight from a finding to a single favored solution without examining alternatives, or by handing over raw findings with no design implication at all, leaving the team to redo the synthesis. The judgment problem is converting evidence into a ranked set of design implications that are traceable to the research, honest about uncertainty, and actionable without overstepping what the evidence supports.

## Core Rules

### Convert Findings Into Problem Statements, Not Solutions

The first output of translating research is not a design but a sharpened statement of the problem. For each significant finding, write a problem statement that names the user, the situation, the difficulty, and the consequence.

A strong problem statement includes:

- the user or role involved;
- the situation in which the problem occurs;
- the difficulty they encounter;
- the consequence, in errors, delay, frustration, or abandonment.

"Customer support agents handling three chats at once lose context when switching tabs, causing slower responses and errors" is a problem statement; "add a sidebar that shows all active chats" is a solution posing as one. Problem statements keep the solution space open and prevent the team from locking onto the first idea.

### Separate What The Research Shows From What It Suggests

Research evidence and design implications are different things, and conflating them destroys traceability. Maintain a clear chain through three layers.

Keep these layers distinct:

- the finding, what the data actually showed;
- the implication, what that suggests about the design;
- the opportunity, the direction it opens.

A finding that "users export data manually every Friday" supports the implication that an automated schedule would reduce repetitive work, but the finding does not prove the implication; the implication is a hypothesis the design must test. Keeping these layers separate lets the team disagree with an implication without rejecting the underlying evidence.

### Prioritize Problems By Impact, Frequency, And Severity

Not every finding deserves design attention. Rank problems along dimensions that reflect real cost to users and the business.

Rank problems by considering:

- frequency: how often the problem occurs across the user population;
- impact: how severely it blocks the user's goal when it does occur;
- severity: whether it causes errors, data loss, abandonment, or merely friction;
- population: how many users, and which segments, are affected;
- strategic fit: whether solving it serves a priority the product has committed to.

A rare, low-impact problem that affects a power-user edge case should not crowd out a frequent, high-severity problem affecting new users. Make the ranking explicit so the team can challenge it.

### Frame Opportunities As "How Might We" Without Embedding A Solution

When translating problems into design directions, frame them as open questions that invite multiple solutions. "How might we help users recover when they have made an error in a multi-step form" opens the space; "how might we add an undo button" closes it by embedding a solution.

Good opportunity frames are:

- grounded in the problem;
- specific enough to be actionable;
- open enough to permit divergent ideation.

Review each frame for an embedded solution and rewrite any that pre-commit the design. The quality of the opportunity frame often determines the quality of the ideas that follow.

### Preserve The Evidence Link For Every Implication

Every design implication should be traceable to the research that produced it. Record which finding, and ideally which participants or data, supports each implication.

Evidence links matter because they:

- let the team judge how much weight to put on the implication;
- let them revisit the implication when new research arrives;
- distinguish a research-backed implication from a bare opinion.

An implication whose evidence has been superseded should be revisited. Build the traceability into the deliverable, not as an afterthought.

### Acknowledge Gaps And Uncertainty Explicitly

Research never covers the entire design space, and honest translation names what is not known.

Handle uncertainty by:

- flagging implications as hypotheses to be tested rather than directions to be built, where the evidence is thin;
- identifying the gaps that the next round of research or design validation should address;
- distinguishing what the research established from what it merely did not contradict.

Overclaiming certainty from thin evidence produces confident decisions that fail in testing; naming uncertainty produces decisions that are appropriately provisional and that get validated before they ship.

### Connect Insights Across Studies, Not Just Within One

A single study's findings are stronger when placed alongside related research. Connect new insights to prior findings, to analytics, to support data, and to known design principles.

Cross-study connection helps because:

- triangulation across sources increases confidence;
- it reveals when a new finding confirms, refines, or contradicts earlier understanding;
- a finding that fits a pattern the team already recognizes is easier to act on than one that stands alone.

Make the connections explicit so the accumulated evidence, not just the latest study, informs the design.

## Common Traps

### Leaping From Finding To Favored Solution

Jumping from a research result to a single design idea skips the problem statement and the alternative solutions, locking the team into an unexamined choice.

### Handing Over Raw Findings With No Implication

Delivering research with no design translation forces the team to redo the synthesis and often loses the nuance the researcher captured.

### Embedding Solutions In Opportunity Frames

"How might we add X" closes the solution space. Frames should open possibilities, not pre-commit to a feature.

### Prioritizing By Vividness Instead Of Impact

A dramatic quote from one participant can dominate priorities over a frequent, severe problem that was described more quietly. Rank by impact and frequency, not memorability.

### Overclaiming Certainty From Thin Evidence

Treating a tentative implication as a proven direction leads to building the wrong thing with confidence. Label hypotheses as hypotheses.

### Losing The Evidence Link

An implication with no traceable finding becomes an untethered opinion that cannot be revisited when conditions change.

### Treating One Study In Isolation

Findings gain strength from cross-study patterns. Ignoring prior research treats every study as if starting from scratch and misses triangulation.

## Self-Check

- [ ] Each significant finding has been converted into a problem statement that names user, situation, difficulty, and consequence, without specifying a solution.
- [ ] Findings, implications, and opportunities are kept as separate layers, not collapsed into one.
- [ ] Problems are prioritized by frequency, impact, severity, population, and strategic fit, with the ranking made explicit.
- [ ] Opportunity frames are written as open "how might we" questions with no embedded solution.
- [ ] Every design implication is traceable to the finding and evidence that supports it.
- [ ] Gaps and uncertainties are named, and provisional implications are labeled as hypotheses to test.
- [ ] New insights are connected to prior research, analytics, and supporting data rather than presented in isolation.
- [ ] No implication oversteps what its underlying evidence and sample can support.
- [ ] The deliverable lets a designer act and lets a reviewer trace each direction back to the research.
- [ ] The translation opens multiple solution directions rather than locking onto one.
