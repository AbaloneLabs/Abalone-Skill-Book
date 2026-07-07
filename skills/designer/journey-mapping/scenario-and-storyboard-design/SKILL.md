---
name: scenario_and_storyboard_design.md
description: Use when the agent is writing user scenarios, designing storyboards, crafting narrative use cases, illustrating sequences of interaction through frames, defining the context and motivation behind a task, or translating abstract requirements into concrete situations a user acts within, and must decide scenario realism, persona fit, narrative scope, and how much detail a storyboard needs to communicate intent without over-specifying the solution.
---

# Scenario And Storyboard Design

Scenarios and storyboards turn abstract requirements into concrete human situations. A requirement says "the user can export a report"; a scenario says "a finance manager, closing the month on a tight deadline, needs to pull the same variance report she exports every Friday, this time from her phone in a taxi." The scenario makes the design problem real: it reveals context, motivation, constraints, and the moments that matter. The judgment problem is not writing a story that sounds vivid. It is choosing situations that genuinely stress the design, anchoring them in a real persona and context, keeping the narrative scoped to what the design must solve, and deciding how much visual detail a storyboard needs to communicate without prematurely locking the solution. Agents tend to fail by writing flattering scenarios where everything goes smoothly, by inventing a persona's motivation instead of grounding it, by telling stories so broad they no longer inform specific design decisions, and by over-rendering storyboards that close the design space before exploration is done.

Use this skill when writing scenarios, building storyboards, defining narrative use cases, or translating research and requirements into concrete situations for design, critique, or testing. The goal is scenarios and storyboards that expose real design problems and communicate intent without over-constraining the answer.

## Core Rules

### Write Scenarios That Stress The Design, Not Flatter It

A scenario's job is to reveal where the design must work hard. A scenario in which the user is calm, unhurried, on a fast connection, with perfect data, and doing the obvious thing tests nothing. The valuable scenarios are the ones that introduce realistic pressure: a deadline, a distraction, an interruption, a degraded connection, incomplete information, a mistake to recover from, or a motivation that conflicts with the obvious path.

Choose scenarios that exercise:

- the most common task under normal conditions, as a baseline;
- the same task under realistic stress or constraint;
- edge cases that expose assumptions, such as empty data or first-time use;
- recovery situations where something has already gone wrong;
- cross-device or interrupted sessions.

A set of scenarios that only covers the easy version of the task will produce a design that fails the moment reality intrudes.

### Anchor Every Scenario In A Specific Persona And Context

A scenario without a specific protagonist is a generic walk-through that reveals nothing about the user's real motivations and constraints. Name the persona, their goal, their prior experience, their environment, and what is at stake for them. The same task performed by a novice under time pressure and by an expert at leisure produces two different design problems.

Before writing, define:

- who the persona is and what they already know;
- what they are trying to accomplish and why it matters to them;
- where and when this happens, including device, environment, and time pressure;
- what they expect based on prior experience with similar products;
- what could go wrong or interrupt them.

Context is not flavor text; it is what makes the scenario diagnostic. A scenario stripped of context cannot distinguish a design that works for the real user from one that works only in the abstract.

### Scope The Narrative To The Design Decision At Hand

A scenario can expand indefinitely if left unchecked, drifting into the user's entire day, their relationship history, and unrelated tasks. Each scenario should serve a specific design question and stay scoped to it. If the question is how a user recovers a failed payment, the scenario should set up enough context to make the recovery meaningful and then focus on the recovery, not narrate the entire purchase history.

Define the narrative boundary:

- which part of the journey this scenario illuminates;
- where the scenario starts and ends;
- which design decisions it is meant to inform;
- what lies outside the scope and belongs to a different scenario.

A scenario that tries to cover everything informs nothing, because the relevant detail is buried under irrelevant narrative.

### Separate The Situation From The Solution

A scenario describes the problem situation: the user, the context, the goal, and the obstacles. It should not prematurely dictate the solution. When a scenario specifies exact interactions, screens, or wording, it stops being a tool for exploring the design space and becomes a specification that forecloses alternatives. Keep the scenario at the level of need, motivation, and obstacle, and let multiple design responses remain possible.

Resist writing "the user taps the export button in the header" when the design question is how the user reaches export at all. Write "the user needs to get this report out of the system quickly, without navigating a deep menu." The first commits to a solution; the second keeps the design space open.

### Choose Storyboard Fidelity To Match The Decision Being Communicated

Storyboards can be rough sketches, structured frames, or polished illustrations, and the right fidelity depends on what the storyboard must accomplish. A rough storyboard for internal exploration should be fast and disposable, because polish signals commitment and discourages critique. A storyboard for stakeholder buy-in may need more finish to be legible to a non-design audience. Over-rendering early storyboards wastes effort and, worse, makes the team reluctant to change direction because the work looks finished.

Match fidelity to purpose:

- rough thumbnails for exploring many directions quickly;
- annotated frames for communicating interaction sequence and logic;
- polished frames only when the direction is settled and the audience needs clarity.

The mistake to avoid is jumping to high fidelity before the sequence and intent are settled, because rendered frames feel final and suppress the iteration the concept still needs.

### Make Each Frame Carry Decision-Relevant Information

Every frame in a storyboard should communicate something the reader needs to understand the design problem or response: a context detail, a user emotion, a key interaction, a transition, or a moment of friction. Frames that exist only to pad the sequence, or that repeat the same information, dilute the storyboard and hide the important moments. If a frame could be removed without losing meaning, remove it.

For each frame, confirm it shows:

- what the user is doing or experiencing;
- what context or constraint is in play;
- what emotion, confusion, or relief is present;
- what the key interaction or decision is;
- how this frame connects to the next.

A tight storyboard of decision-relevant frames communicates more than a long one full of filler.

### Use Scenarios And Storyboards To Surface Assumptions, Not To Replace Validation

Scenarios and storyboards are tools for thinking and communicating, not evidence that a design works. A vivid scenario can make a flawed design feel inevitable and correct, because narrative is persuasive. Treat each scenario as a hypothesis about the user and the design, and flag the assumptions it rests on. Where the stakes are high, the scenario should be followed by testing with real users in something close to the described context.

Document the assumptions each scenario makes about the user's knowledge, motivation, environment, and tolerance, so the team knows what must be true for the design to succeed and what to validate.

## Common Traps

### Flattering Scenarios With No Stress

A scenario where everything goes smoothly validates nothing. Introduce realistic pressure, interruption, and error to expose where the design must hold up.

### Generic Protagonists Without Context

A scenario about "the user" with no persona, motivation, or environment reveals no real design constraints. Anchor every scenario in a specific person and situation.

### Narrative Sprawl

A scenario that narrates the user's entire day buries the relevant detail. Scope each scenario to the specific design decision it serves.

### Embedding The Solution In The Problem

When a scenario specifies exact interactions or screens, it forecloses design alternatives. Keep the scenario at the level of need and obstacle.

### Over-Rendering Early Storyboards

High-fidelity frames signal commitment and suppress iteration. Use rough fidelity while the direction is still being explored.

### Frames That Carry No Information

Padding a storyboard with frames that repeat or add nothing dilutes the message. Keep every frame decision-relevant.

### Treating The Scenario As Proof

A vivid narrative is persuasive, not evidentiary. Flag the assumptions a scenario rests on and validate them where the stakes are high.

### One Scenario Treated As The Whole Truth

Designing for a single scenario produces a solution tuned to one situation. Use a set of scenarios that spans common, stressful, and edge situations.

## Self-Check

- [ ] Scenarios include realistic stress, interruption, error, and constraint, not only smooth happy paths.
- [ ] Each scenario is anchored in a specific persona with defined motivation, context, device, and stakes.
- [ ] The narrative is scoped to a specific design decision, with a clear start, end, and out-of-scope boundary.
- [ ] Scenarios describe the problem situation without prematurely dictating the interaction or screen solution.
- [ ] Storyboard fidelity matches the purpose: rough for exploration, polished only when the direction is settled.
- [ ] Every frame carries decision-relevant information about context, emotion, interaction, or transition.
- [ ] Assumptions embedded in each scenario are identified and flagged for validation rather than treated as proven.
- [ ] A set of scenarios spans common, stressful, and edge situations rather than relying on a single narrative.
- [ ] The scenario or storyboard exposes a real design problem the team can act on, rather than merely telling a pleasing story.
