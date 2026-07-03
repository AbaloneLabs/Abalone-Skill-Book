---
name: lifecycle-stage-definition.md
description: Use when the agent is defining user lifecycle stages, mapping the stages from new user to advocate, deciding what distinguishes one stage from another, choosing stage transition criteria, or establishing a shared lifecycle model that the whole team uses to segment users and target interventions.
---

# Lifecycle Stage Definition

The user lifecycle is the spine of how a product team thinks about its users over time. Defining it well gives the team a shared map: where each user is, what they need at that point, and what would move them forward. Defining it poorly — with vague stages, arbitrary boundaries, or a model that does not match how users actually behave — produces a map that misleads every team that uses it, leading to interventions targeted at the wrong users, metrics that measure the wrong transitions, and a fragmented understanding of the user base. The lifecycle model is foundational, and the discipline of defining it rigorously is worth the investment because everything built on top of it inherits its quality or its flaws.

This skill covers the judgment needed to define lifecycle stages: how many, what distinguishes them, what marks the transitions, and how to keep the model honest and useful.

## Core Rules

### Define stages by behavioral state, not by time elapsed

The most common lifecycle modeling failure is defining stages by time ("new user," "user in first 30 days," "user over 90 days") rather than by behavioral state. Time-based stages feel objective and are misleading, because two users at the same elapsed time can be in completely different behavioral states — one deeply engaged, one dormant — and treating them as the same stage produces useless interventions.

- Define each stage by the user's behavioral relationship with the product: what they have done, what they are doing, what their engagement pattern is.
- A user is "new" because they have not yet established a usage pattern, not because they signed up recently. A user who signed up six months ago and never returned is not "new" in any useful sense; they are dormant.
- A user is "active" because they engage with a pattern that indicates ongoing value, not because they logged in within an arbitrary window.

Behavioral definitions make the stages meaningful and the interventions targeted. Time-based definitions are easy and frequently wrong.

### Choose stage boundaries that correspond to meaningful shifts, not arbitrary thresholds

Stage boundaries should mark points where the user's relationship with the product genuinely changes, and where the appropriate intervention, messaging, or metric changes with it. Arbitrary boundaries (day 7, day 30, login count 5) that do not correspond to real shifts produce stages that feel different to the team but are indistinguishable to the user.

- Identify the points in the user journey where behavior, needs, or value perception shift meaningfully: the transition from trying to adopting, from single-use to multi-use, from individual to collaborative, from active to declining.
- Define boundaries at these shifts, using behavioral criteria that can be measured (performed a milestone action, reached a usage threshold, showed a change in pattern).
- Validate that the boundaries are real by checking that users on either side of a boundary actually behave differently and respond differently to interventions.

A boundary that does not correspond to a behavioral difference is a line on a diagram, not a useful stage distinction.

### Use the minimum number of stages that capture meaningful differences

Lifecycle models tend to accrete stages as different teams add the distinctions they care about, until the model is so granular that it is unusable. Each stage adds complexity, measurement burden, and intervention design load, and the value of a stage must justify its cost.

- Start with the smallest set of stages that captures the meaningful behavioral differences in your user base. Often this is four to seven stages.
- Add a stage only when it corresponds to a behavioral state that requires different treatment and that is not served by an existing stage.
- Collapse stages that are theoretically distinct but that the team treats identically. If two stages receive the same interventions and metrics, they are one stage in practice.

A simpler model that the whole team uses consistently beats a complex model that each team interprets differently.

### Make stage definitions measurable and consistent across teams

A lifecycle model that is defined qualitatively ("users who are getting value") is interpreted differently by every team and cannot be used for measurement or targeting. Each stage must have a precise, measurable definition that every team applies identically.

- Define each stage with explicit behavioral criteria that can be computed from data (performed action X within window Y, used feature Z at least N times).
- Document the definitions and make them the single source of truth that all teams reference.
- Reconcile the definitions when teams use the same stage name for different criteria, which is a common source of cross-team confusion and conflicting metrics.

The value of a lifecycle model is that it creates a shared language. If the language is ambiguous, the value is lost.

### Map the lifecycle to the actual paths users take, including non-linear ones

Lifecycle models are often drawn as a linear progression (new to active to loyal to advocate), but real user behavior is not linear. Users skip stages, regress, loop, and take paths the linear model does not anticipate. A model that assumes linearity misses these realities and targets interventions as if every user were on a single conveyor belt.

- Map the actual paths users take through the stages, including the common non-linear ones: activation followed by dormancy and return, direct jumps to deep engagement, regressions from active to declining.
- Design interventions that account for the common paths, not only the idealized forward progression.
- Identify the decision points and triggers that move users between stages, which is where interventions have leverage.

### Connect stages to the interventions and metrics appropriate to each

The purpose of defining stages is to act differently at each. A lifecycle model that does not change what the team does at each stage is academic. For each stage, define what the user needs, what intervention would help, and what metric indicates success.

- For each stage, identify the user's primary need and the obstacle to progressing.
- Define the intervention appropriate to that need: onboarding for new users, depth-building for active users, re-engagement for declining users, recognition for advocates.
- Define the metric that indicates success at each stage, distinct from the metrics of other stages. The metric that matters for a new user (activation) differs from the metric that matters for a mature user (depth or retention).

### Revisit the lifecycle model as the product and user base evolve

A lifecycle model that fit the product at one stage of its life may not fit later. As the product adds capabilities, serves new segments, or changes its value proposition, the stages that matter and the transitions between them can shift. A model frozen at launch drifts out of alignment and misleads.

- Revisit the model when the product changes substantially, when new segments emerge, or when the metrics stop making sense (often a sign that the stage definitions no longer match behavior).
- Be willing to revise stage definitions, boundaries, and even the number of stages when the evidence warrants, while maintaining consistency over time so trends remain interpretable.

## Common Traps

### Time-based stage definitions

Defining stages by elapsed time rather than behavioral state misleads, because users at the same time can be in completely different states. Define stages by behavior.

### Arbitrary boundaries that do not correspond to behavioral shifts

Boundaries at day 7 or login 5 that do not mark a real change produce stages that are indistinguishable to the user and that do not respond differently to intervention. Place boundaries at meaningful shifts.

### Too many stages accumulating over time

Each team adds the distinctions it cares about until the model is unusable. Use the minimum stages that capture meaningful differences.

### Qualitative definitions interpreted differently by each team

Stages defined without measurable criteria are applied inconsistently and cannot be used for targeting or measurement. Make definitions explicit and documented.

### Assuming linear progression through stages

Real users skip, regress, and loop through stages. A linear model misses these paths and targets interventions as if all users were on one conveyor belt.

### Stages that do not change what the team does

A lifecycle model that does not lead to different interventions and metrics per stage is academic. Connect each stage to the appropriate action and measure of success.

### A model frozen at launch

The lifecycle model that fit the early product drifts out of alignment as the product and user base evolve. Revisit and revise when the evidence warrants.

### Stage names that imply judgment

Names like "good user" or "churn risk" that imply moral judgment shape how teams treat users and can bias interventions. Use neutral, descriptive stage names.

## Self-Check

- Are my stages defined by behavioral state rather than by time elapsed since signup?
- Do my stage boundaries correspond to meaningful behavioral shifts, validated by checking that users on either side behave differently?
- Am I using the minimum number of stages that capture meaningful differences, resisting accretion?
- Are stage definitions explicit, measurable, documented, and applied consistently across all teams?
- Have I mapped the actual non-linear paths users take, not only the idealized forward progression?
- Does each stage have a defined user need, an appropriate intervention, and a distinct success metric?
- Have I revisited the model as the product and user base evolved, rather than freezing it at launch?
- Are stage names neutral and descriptive, free of implicit judgment that could bias treatment?
- Could a new team member use my lifecycle model to correctly identify a user's stage and know what to do for them?
- If the model were wrong, how would I detect it — what signal would tell me the stages no longer match behavior?
