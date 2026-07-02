---
name: estimation_techniques_and_calibration.md
description: Use when the agent is estimating how long a task will take, sizing work with story points or T-shirt sizes, running a planning poker session, calibrating estimates against past actuals, explaining why estimates were wrong, or deciding whether to give a time estimate versus a range versus a size.
---

# Estimation Techniques And Calibration

An estimate is a prediction about an uncertain future, and the single most reliable fact about software estimates is that they are wrong. They are wrong because the work contains unknowns, because the codebase resists in ways that only surface mid-task, because integration takes longer than building, and because humans systematically under-weight the parts they cannot see. The harm is not that estimates are imperfect — that is unavoidable — but that they are treated as commitments, given as single precise numbers, and never corrected against reality, so the same bias repeats forever and planning stays built on fiction.

Agents tend to fail at estimation by confusing it with a guess to be delivered quickly. The work looks simple at the start, the unknowns are invisible, and a confident single number feels more helpful than a hedged range, so the agent says "two days" and the organization builds a schedule around it. The judgment problem is to produce estimates that honestly represent uncertainty, to use techniques that dampen the systematic biases, and — most importantly — to calibrate against actual outcomes so that each estimate is more accurate than the last. An estimating process that never measures itself cannot improve; it can only repeat its errors.

## Core Rules

### Distinguish Estimate, Target, And Commitment

These three are constantly conflated, and the conflation is the root of most estimation dysfunction. An **estimate** is a prediction of likely effort given what is known. A **target** is a desired outcome ("we want this by the end of the quarter"). A **commitment** is a promise to deliver by a date, accepting the estimate's uncertainty as a risk. When a target is mistaken for an estimate, the team reports the desired date as the estimate, reality disagrees, and trust erodes. When an estimate is mistaken for a commitment, every variance becomes a failure rather than new information.

Keep them separate in communication. State the estimate as a prediction with explicit uncertainty, state the target as a goal, and make commitments only with an explicit accounting of the risk that the estimate will be wrong. A schedule built on an estimate treated as a guarantee is a schedule that will be defended with overtime and corner-cutting when reality arrives.

### Represent Uncertainty With Ranges, Not Point Numbers

A single number ("five days") implies a precision the estimator does not have and invites the listener to treat it as a commitment. A range ("three to ten days, most likely around five") conveys the actual uncertainty and forces the conversation about what drives the spread. The width of the range is itself information: a narrow range means the work is well understood; a wide range means the unknowns dominate and the first job is to reduce them, not to pick a number.

Prefer ranges that capture realistic best and worst cases rather than optimistic ones. Humans anchor on the happy path and under-weight the tail; counteract this by deliberately constructing the pessimistic end from what could go wrong (unknown dependencies, integration friction, rework, waiting on others). A range whose worst case is only slightly worse than the best case is almost certainly over-optimistic.

### Choose A Technique That Matches The Purpose

Different estimation techniques serve different purposes, and using the wrong one produces noise. Match the technique to the decision the estimate supports.

- **T-shirt sizes (S/M/L/XL)** are coarse, fast, and good for relative triage and prioritization when precision is not needed. They communicate "this is much bigger than that" without pretending to know the exact size.
- **Story points** are relative complexity units, often Fibonacci-scaled to reflect that uncertainty grows with size. They work for sprint planning and velocity tracking when the team is stable and estimates are compared to actuals over time. They are meaningless across teams.
- **Time estimates (hours or days)** are the most intuitive and the most abused. They make sense when the work is well-understood and the unit maps to a real calendar, but they invite the precision illusion and the commitment trap.
- **Probabilistic estimates (ranges with confidence levels, or Monte Carlo over a backlog)** are the most honest for planning at scale, because they propagate uncertainty instead of collapsing it. They are more effort but far more reliable for dates that matter.

Coarse techniques for triage, relative techniques for team planning, probabilistic techniques for high-stakes dates. Reaching for a precise time estimate when a T-shirt size would do wastes effort and manufactures false precision.

### Decompose Until The Pieces Are Estimable

A large task cannot be estimated accurately because its unknowns are too many and its components vary too widely. The reliable way to estimate large work is to decompose it into smaller pieces until each piece is small enough that its effort is dominated by visible work rather than invisible unknowns. Then estimate the pieces and aggregate, accepting that the aggregate carries the sum of the pieces' uncertainty.

Decomposition also surfaces the hidden work. Breaking "add reporting" into "design the schema, build the query, add the API, build the UI, handle empty states, add tests, update docs" reveals steps that "add reporting" hid. Each revealed step is an estimate that would otherwise have been silently omitted. If a piece cannot be decomposed or estimated, that is the signal that it is really a research task, and the correct estimate is "we need a spike to find out."

### Run Planning Poker Or Similar To Counter Individual Bias

A single estimator's number is dominated by that person's optimism, their familiarity with the code, and the parts they happened to think of. Group estimation techniques like planning poker counter this by requiring independent estimates first, then surfacing and discussing the disagreements. The disagreement is where the learning is: the person who said "one day" and the person who said "eight days" are seeing different risks, and the conversation reconciles them into a better-shared understanding.

The key mechanic is independent estimation *before* discussion, so a dominant voice or a premature anchor does not capture the group. If everyone just agrees with the first number spoken, the technique has devolved into theater and produces no more accuracy than a single estimate.

### Calibrate Against Actuals Continuously

The most important and most skipped practice is comparing estimates to actuals and adjusting. Without calibration, the same biases repeat indefinitely; with it, the estimator learns where they are consistently off and the organization's estimates converge toward reality over time.

Calibration practices:

- **Record estimates and actuals** for completed work, at a granularity that allows comparison.
- **Review the misses periodically**, not to assign blame but to find patterns — are integration tasks always underestimated? Is testing consistently ignored? Are unknown dependencies the main driver?
- **Adjust future estimates** by the observed bias. If testing reliably takes twice the estimated time, multiply testing estimates by two going forward rather than re-deriving the same error.
- **Track calibration directly** — of the estimates given as "80% likely," do roughly 80% come in under? If only 50% do, the estimator is overconfident, and the ranges need widening.

A team that has been estimating for a year without measuring accuracy has learned nothing. A team that measures learns to estimate better, which is the only path to estimates worth trusting.

### Treat The Unknowns As The First Work Item

When an estimate has a wide spread, the dominant cost is the unknown. The highest-value action is often not to pick a number but to spend a small, time-boxed effort reducing the unknown — a spike, a prototype, a read of the relevant code, a conversation with the dependency owner. After the spike, the estimate narrows, and the remaining work is more predictable.

This reframes estimation from a guessing exercise into a risk-reduction exercise. The wide-range task is not "estimated badly"; it is "not yet understood enough to estimate," and the right move is to understand it before committing to a date.

## Common Traps

### The Precision Illusion

Giving a single precise number ("4 days") implies certainty that does not exist and is treated as a commitment. The number feels helpful but misleads the listener about the real uncertainty. Use ranges, and let the width communicate the confidence.

### Anchoring On The Happy Path

Estimates built by imagining the work going smoothly systematically exclude the integration friction, the rework, the waiting, and the surprises that dominate actual effort. Deliberately construct the pessimistic case from what could go wrong, and let it widen the range.

### The Target Mistaken For The Estimate

When the desired date becomes the estimate, the team reports what leadership wants to hear, reality diverges, and the gap appears late and painfully. Keep the estimate an honest prediction; negotiate the target and the commitment separately and explicitly.

### Estimating Only The Visible Work

Counting the coding but omitting testing, code review, documentation, deployment, bug fixing, and coordination produces estimates that are structurally low. Decompose fully so every category of work is estimated, not just the parts that are fun to imagine.

### Never Measuring Accuracy

An estimation process with no feedback loop cannot improve. Without comparing estimates to actuals, the same biases repeat forever. Record actuals and review them; calibration is the single highest-leverage improvement to estimation.

### Group Agreement That Is Really One Voice

Planning poker or group estimation that collapses to the first spoken number produces no more accuracy than a solo estimate and adds ceremony. Require independent estimates before discussion to surface the real disagreements.

### Treating An Unknowable Task As A Bad Estimate

A task whose scope is genuinely unknown is not badly estimated; it is un-estimated. Forcing a number onto it manufactures false certainty. The correct response is a spike to reduce the unknown, then an estimate with a narrower range.

## Self-Check

- [ ] Estimates, targets, and commitments are distinguished in communication; the estimate is an honest prediction with uncertainty, not a restatement of the desired date.
- [ ] Estimates are given as ranges whose width reflects real uncertainty, with the pessimistic end deliberately constructed from what could go wrong, not anchored on the happy path.
- [ ] The estimation technique matches the purpose — coarse sizes for triage, relative points for team planning, probabilistic ranges for high-stakes dates.
- [ ] Large tasks were decomposed until each piece was small enough to estimate, and the decomposition surfaced hidden categories of work (testing, review, docs, deployment) rather than only the visible coding.
- [ ] Group estimation used independent estimates before discussion, so the result reflects shared understanding rather than the first voice.
- [ ] Estimates are recorded against actuals, misses are reviewed for patterns, and future estimates are adjusted by observed bias; calibration (do 80%-confidence estimates hold ~80% of the time?) is tracked.
- [ ] Wide-range estimates triggered a spike or research step to reduce the unknown before a date was committed, rather than forcing a precise number onto unknowable work.
- [ ] No estimate presented as precise conceals a wide spread of real possibility.
