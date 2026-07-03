---
name: design_critique_facilitation.md
description: Use when the agent is running a design critique, preparing designers for critique, deciding who should attend, structuring critique agendas, or determining how to evaluate a design against the problem it is meant to solve rather than personal taste.
---

# Design Critique Facilitation

A design critique is a structured evaluation of work against its goals, not a forum for personal preference or a competition to find the most flaws. The most common failure mode is letting critique drift into opinion swapping, where the loudest voices dominate and the designer leaves with contradictory directions. A second, equally damaging failure is critique that is too gentle, where real problems go unnamed because no one wants to discourage the designer. Both extremes produce worse design and erode trust between product and design.

The product manager's role in critique is usually not the primary critic of craft. It is to hold the problem, the user, and the constraints in view, to ensure the right people are in the room, and to make sure the conversation evaluates effectiveness before it evaluates aesthetics. Facilitation matters as much as the feedback itself. A well-run critique turns a designer's work into better solutions; a poorly run one turns it into noise.

Use this skill before scheduling or running a design critique, before deciding what to bring to critique, or when reflecting on why critiques in your team produce inconsistent results. Ask: does this critique have a clear purpose and a clear artifact under review? Are the right evaluators present, and do they understand their role? Is the conversation anchored to the problem and success criteria, or has it drifted into taste? Is the designer being asked to defend decisions, or helped to see alternatives? Is there a record of what was decided and why?

## Core Rules

### Define The Purpose And Scope Of Each Critique

Not every critique serves the same purpose, and treating them interchangeably causes confusion. A critique early in a design cycle explores direction and asks whether the approach is right. A critique late in the cycle polishes detail and asks whether the execution is right. Giving detailed polish feedback on an early directional concept wastes everyone's time and forces premature commitment. Giving broad directional feedback on a design that is about to ship destabilizes the team.

Before the critique, state explicitly what kind of feedback is needed. Is this directional exploration, mid-flow validation, or pre-handoff polish? What decisions are still open, and which are already settled? What specific questions does the designer want answered? A critique without a stated purpose will default to whatever the loudest attendee wants to discuss, which is rarely what the designer needs.

### Anchor Every Critique To Problem, User, And Success Criteria

The single most important habit is to begin each critique by restating what the design is supposed to achieve. What user problem is being solved? Who is the target user, and what is their context? What does success look like, and what constraints apply? Without these anchors, feedback becomes subjective reaction. With them, feedback can be evaluated against a shared standard.

This is where the product manager adds the most value. The designer owns the craft; the PM owns the problem. If the problem statement itself is ambiguous or contested, surface that immediately rather than letting it contaminate the rest of the critique. A critique cannot rescue a design built on an unclear goal.

### Invite The Right Evaluators With Clear Roles

Critique quality depends on who is in the room and whether they understand why they are there. Common attendees include other designers for craft, engineers for feasibility, researchers for user evidence, and product partners for problem fit. Each brings a different lens, and each should know which lens they are contributing.

Avoid two extremes. The first is the open-invite critique where anyone can drop in; this dilutes focus and invites drive-by opinions from people without context. The second is the closed critique where only design attends; this misses feasibility and problem-fit issues that surface later and more painfully. Choose evaluators deliberately, give them the context in advance, and tell them what you need from them specifically.

### Structure The Conversation In A Priority Order

Effective critique moves from high-level to low-level concerns, because debating button placement when the flow is wrong is wasted effort. A useful sequence is to first ask whether the design solves the right problem, then whether it is usable and understandable for the target user, then whether it handles the full range of states, then whether it is feasible to build, and only then whether it is consistent and polished.

As facilitator, redirect the conversation when it jumps levels. If someone nitpicks visual detail while the core approach is still in question, note the detail and pull the group back to the higher-order concern. This is not dismissive; it is sequencing. Low-level feedback is more useful once the foundation is sound.

### Evaluate Against Evidence, Not Preference

Strong critique distinguishes between feedback grounded in evidence and feedback grounded in personal taste. Evidence includes user research, behavioral data, support trends, usability findings, accessibility requirements, and known constraints. Preference is what an individual happens to like. Both can be valid inputs, but they should be labeled differently and weighted differently.

When someone offers feedback, gently probe the basis. Is this based on something observed in users, or is it a hypothesis, or is it personal preference? A PM who can make this distinction visible helps the whole team give better feedback over time. It also protects designers from being whipsawed by opinions presented as facts.

### Protect The Designer's Ownership Of The Solution

Critique should inform the designer's judgment, not override it. The output of a critique is not a list of changes to implement; it is a set of observations, concerns, and questions the designer uses to iterate. When critique turns into a directive list, the designer stops owning the solution and becomes an executor of committee instructions, and the quality of the work declines.

The PM and design lead should synthesize feedback with the designer, not for the designer. Sort the feedback by importance, identify themes, separate must-address effectiveness issues from optional polish suggestions, and let the designer decide how to respond. It is legitimate and healthy for the designer to reject feedback that does not serve the goals, with reasoning.

### Review The Full Range Of States, Not Only The Happy Path

Critiques routinely approve the success screen and ignore everything that can go wrong. Insist on reviewing empty states, loading and processing states, error states and recovery, validation failures, permission denial, long content and overflow, offline behavior, first-time use, and destructive actions. These states are where users lose trust and where support load originates.

If the states are not yet designed, that itself is a critique finding. A design that only covers the happy path is not ready for handoff, regardless of how polished the happy path looks.

### Document Outcomes And Rationale

Critique produces decisions, and decisions without rationale get relitigated. Record what was reviewed, what was decided, what concerns were raised and deferred, and why. This helps engineers understand intent, helps absent stakeholders catch up, and prevents the same debate recurring when someone new joins.

A lightweight format is usually enough: artifact reviewed, key decisions, open questions, and next steps. The cost is small; the cost of re-deciding the same design question in a month is large.

## Common Traps

### Critique As Opinion Swap

When feedback is unanchored to the problem and unevaluated for evidence basis, critique becomes a competition of preferences. The trap is that it feels productive because lots of feedback is exchanged, but the designer leaves more confused than they arrived.

### Wrong Critique Type For The Stage

Giving polish feedback on a directional concept forces premature commitment to details that may be discarded. The trap is that detailed feedback feels more rigorous, when it is actually mistimed.

### Dominance By The Loudest Voice

In unmoderated critique, seniority or volume overrides quality. The trap is that quieter, more thoughtful contributors self-censor, and the team loses input that would have improved the design.

### Confusing Inclusivity With Decision-Making

Inviting broad input is good, but trying to satisfy every suggestion produces compromise design. The trap is treating rejected feedback as a failure of process, when selective synthesis is the whole point.

### Skipping Feasibility And Accessibility

Critiques that include only designers miss whether the design can be built and whether it is accessible. The trap is discovering these problems at handoff or after launch, when they are far more expensive to fix.

### No Record Of Decisions

Without documentation, the rationale for design choices is lost, and new stakeholders relitigate settled questions. The trap is assuming memory or authority will carry the decision.

## Self-Check

- [ ] The critique had a stated purpose (directional, mid-flow, or polish) and the feedback matched that stage.
- [ ] The conversation began by restating the problem, target user, and success criteria before any reaction.
- [ ] Attendees were chosen deliberately and understood which lens they were contributing.
- [ ] Feedback moved from high-level effectiveness to low-level detail in priority order, and premature detail was deferred.
- [ ] The basis of each piece of feedback (evidence, hypothesis, or preference) was made visible.
- [ ] The designer retained ownership of the solution, and feedback was synthesized with them rather than handed down as a directive list.
- [ ] Empty, loading, error, validation, permission, overflow, offline, and first-use states were reviewed, not only the happy path.
- [ ] Feasibility and accessibility were part of the critique, not deferred to handoff.
- [ ] Decisions, open questions, and rationale were recorded so the critique does not get relitigated.
- [ ] The critique evaluated effectiveness against goals, not whether the design was personally appealing to attendees.
