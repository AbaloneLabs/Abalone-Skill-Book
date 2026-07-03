---
name: estimation_techniques_selection.md
description: Use when the agent is estimating product work, choosing estimation techniques, deciding between story points and time estimates, planning how to estimate unfamiliar work, or determining which estimation method fits a given type of work and team.
---

# Estimation Techniques Selection

Estimation is unavoidable in product work, because decisions about sequencing, commitments, and capacity all depend on some sense of how long things will take. Yet estimation is consistently done poorly, and the poor practice is often invisible because the team has never compared its estimates to actuals. The most common failure is reaching for a single estimation technique by habit, regardless of the work's nature, the team's familiarity, and the stage of knowledge. A team that uses the same method for a well-understood bug fix and a novel research-and-build feature is mis-estimating at least one of them, because the two demand different techniques.

Selecting an estimation technique is a judgment about what kind of uncertainty you face and what level of precision the decision requires. Some work is well-understood and can be estimated tightly. Some is genuinely novel, and any single number is a fiction. Some estimation is needed for sequencing, where relative sizing suffices. Some is needed for a commitment to a customer or a market window, where calendar accuracy matters. The technique should match the work and the purpose, not be a one-size default.

Use this skill before estimating work, before choosing or changing an estimation method, before committing to dates based on estimates, or when estimates have proven consistently wrong. Ask: what kind of work is this, and how well do we understand it? What decision depends on this estimate, and how precise must it be? Are we using a technique that fits, or defaulting to habit? Are we estimating at a stage where estimation is even meaningful?

## Core Rules

### Match The Technique To The Type Of Work

Different work has different estimation characteristics, and the technique should follow. Well-understood, repetitive work, such as routine bug fixes or small feature additions in a familiar area, can be estimated with reasonable precision using the team's historical experience. Moderately novel work, where the approach is known but the specifics vary, benefits from decomposition into smaller pieces that are each more estimable. Genuinely novel work, involving research, new technology, or unsolved problems, cannot be meaningfully estimated as a single number and requires a different approach.

Classify the work before estimating. If it is well-understood, a direct estimate may work. If it is moderately novel, decompose it and estimate the pieces, being honest that the decomposition itself carries uncertainty. If it is genuinely novel, do not pretend a point or time estimate is meaningful; instead, estimate the time to reduce uncertainty, such as a spike or prototype, and re-estimate once the unknowns are clearer. Forcing a single technique across all three types produces confident-looking numbers that are wrong in predictable ways.

### Match Precision To The Decision's Needs

Not every decision requires the same estimation precision, and over-precise estimates waste effort while creating false confidence. A roadmap decision about sequencing needs only relative sizing: is this bigger or smaller than that. A capacity-planning decision for next quarter needs rough sizing. A commitment to a customer or a regulatory deadline needs calendar accuracy. Estimating everything to maximum precision wastes time on work where rough sizing suffices, and estimating everything roughly fails commitments that needed accuracy.

Identify what decision the estimate serves and how precise it must be. For sequencing and rough planning, relative techniques like story points or t-shirt sizing are efficient and appropriate. For commitments, translate to calendar time with explicit uncertainty ranges. Do not let the availability of a precise-looking number substitute for the precision the decision actually requires. A precise estimate used for a decision that needed only rough sizing imposes false accountability; a rough estimate used for a commitment that needed accuracy creates broken promises.

### Decompose To Reduce Estimation Error

Large items are harder to estimate accurately than small ones, because the uncertainties compound and the item spans multiple unknowns. Decomposing a large item into smaller pieces, each more concrete and estimable, reduces estimation error, because the pieces are easier to reason about and their uncertainties partially cancel rather than fully compound. This is one of the most reliable ways to improve estimation quality.

Decompose until each piece is small enough that the team can estimate it with confidence, and then aggregate. Be honest that decomposition has limits: if the pieces are interdependent, their uncertainties do not cancel, and if the decomposition itself is uncertain, the aggregate inherits that uncertainty. Decomposition helps most for work that is complex but not fundamentally unknown; for genuinely novel work, the decomposition may be as uncertain as the whole, and a spike to reduce unknowns is more valuable than finer decomposition.

### Use Relative Sizing For Sequencing

Relative sizing techniques, such as story points or t-shirt sizes, estimate work relative to other work rather than in absolute time. Their strength is that humans are better at relative judgment than absolute prediction, and that relative sizing abstracts away the per-engineer speed differences that make absolute estimates contentious. Their weakness is that they do not directly answer calendar questions without a conversion based on historical velocity.

Use relative sizing when the decision is about sequencing or capacity allocation, where knowing that item A is bigger than item B is what matters. Avoid using relative sizing as if it were a time estimate, and avoid the common failure of treating velocity as a stable constant when team composition and work mix change. Convert relative estimates to calendar time only through historical data, and treat the conversion as uncertain. Relative sizing is a tool for certain decisions, not a universal estimation method.

### Use Time Estimates For Commitments

When the decision requires calendar accuracy, such as a customer commitment, a market window, or a regulatory deadline, time estimates are necessary despite their imperfection. The discipline is to produce them honestly, with uncertainty made explicit, rather than as single misleadingly precise numbers. A time estimate should be a range with a confidence level, not a point.

Produce time estimates by combining decomposition, historical actuals for similar work, and explicit identification of unknowns. Express the result as a range, such as four to seven weeks, with the assumptions that would move it. Be honest about confidence: a range with low confidence is different from a range with high confidence, and the decision should account for that. Single-point time estimates are almost always wrong and create false accountability; ranged estimates with stated assumptions support honest commitments and contingency planning.

### Estimate At The Right Stage

Estimates produced too early, before the work is understood, are guesses dressed as estimates, and they tend to harden into commitments that cannot be met. Estimates produced too late, after the work is fully designed, arrive when they are least useful for planning. The right stage is when enough is known to estimate meaningfully but before so much is invested that the estimate cannot change the plan.

Resist pressure to estimate novel work precisely before it is understood. Instead, estimate the cost of reducing uncertainty, and commit to a re-estimate once the unknowns are clearer. For work that must be estimated early, label the estimate as low-confidence and revisit it as understanding grows. Treat early estimates as provisional inputs to sequencing, not as commitments, and let them be updated as the work becomes clearer. An estimate that cannot change as knowledge grows is not an estimate; it is a trap.

### Track Actuals To Improve Calibration

Estimation skill improves only through feedback, and feedback requires comparing estimates to actuals. Teams that estimate without tracking actuals repeat the same errors indefinitely, because they never learn which estimates were wrong and why. Tracking actuals, even lightly, creates the feedback loop that lets estimation improve over time.

Record estimates and actuals, and review the gaps periodically. Look for systematic biases: are estimates consistently low for certain types of work, certain areas, or certain team members? Are unknowns consistently underestimated? Use these patterns to adjust future estimation, not to assign blame. A team that calibrates its estimation through honest actuals tracking becomes more accurate over time; a team that does not remains stuck at its initial error level, whatever that is.

## Common Traps

### One Technique For All Work

Using the same estimation method for routine fixes and novel research. The trap is confident numbers that are wrong for at least one category.

### Over-Precision For The Decision

Estimating to maximum precision when the decision needed only rough sizing. The trap is wasted effort and false accountability.

### Single-Point Time Estimates

Expressing commitments as exact dates without uncertainty. The trap is broken promises when the estimate, predictably, was wrong.

### Estimating Before Understanding

Producing precise estimates for novel work before the unknowns are resolved. The trap is guesses that harden into commitments.

### Relative Sizing Treated As Time

Using story points as if they were days. The trap is broken calendar commitments when the conversion proves unstable.

### No Actuals Tracking

Estimating without ever comparing to what happened. The trap is repeating the same estimation errors indefinitely.

## Self-Check

- [ ] The estimation technique was matched to the type of work: direct estimate for routine, decomposition for complex, spike-first for novel.
- [ ] The precision of the estimate matches what the decision requires, neither over-precise nor under-precise.
- [ ] Large items were decomposed into smaller estimable pieces where decomposition adds value.
- [ ] Relative sizing is used for sequencing decisions, and time estimates for commitment decisions, each in its proper place.
- [ ] Time estimates are expressed as ranges with confidence levels and stated assumptions, not single points.
- [ ] Estimates are produced at a stage where the work is understood enough to estimate meaningfully.
- [ ] Novel work is not estimated precisely before understanding; spikes or prototypes reduce uncertainty first.
- [ ] Estimates and actuals are tracked and reviewed to identify systematic biases and improve calibration.
- [ ] Early estimates are labeled low-confidence and updated as understanding grows.
- [ ] No estimate is treated as a commitment that cannot change when knowledge changes.
