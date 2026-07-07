---
name: schedule_estimation_techniques_and_accuracy.md
description: Use when the agent is estimating task or project duration, choosing between estimation techniques like analogous, parametric, and three-point estimating, addressing estimation uncertainty, calibrating estimates against actuals, or diagnosing why project estimates are consistently inaccurate and lead to schedule overruns.
---

# Schedule Estimation Techniques And Accuracy

Estimates are the foundation of the schedule, and inaccurate estimates make every downstream plan unreliable. Yet estimation is often done casually, by asking the person doing the work for a single number, or by copying from a past project without adjustment. The judgment problem is that estimation is inherently uncertain, and the technique used must match the uncertainty, the available data, and the consequences of being wrong. A single-point estimate hides uncertainty; an estimate based on optimism rather than data sets the project up for systematic overrun. The skill is choosing the right technique, representing uncertainty honestly, and improving accuracy over time through calibration.

Use this skill before estimating task or project duration, before selecting an estimation technique, when estimates have proven systematically inaccurate, or when establishing estimation processes for a team. The goal is to prevent the agent from producing single-point estimates that hide uncertainty, from using inappropriate techniques for the level of uncertainty, from succumbing to optimism bias, or from failing to learn from past estimation errors.

## Core Rules

### [ ] Match The Estimation Technique To The Context

Different techniques suit different situations. Analogous estimating uses historical data from similar work and is fast but less precise. Parametric estimating uses statistical relationships and is more precise when data exists. Bottom-up estimating sums detailed task estimates and is most accurate but most effortful. Three-point estimating captures uncertainty through optimistic, pessimistic, and most likely values. Choose based on data availability, required accuracy, and effort.

- [ ] Use analogous estimating when historical data exists and precision needs are moderate.
- [ ] Use parametric estimating when statistical relationships and data are available.
- [ ] Use bottom-up estimating when accuracy is critical and detail is available.
- [ ] Use three-point estimating when uncertainty is high.

### [ ] Represent Uncertainty With Ranges, Not Single Points

A single-point estimate implies false precision. Estimates should be expressed as ranges or distributions that capture uncertainty. A range of "four to eight weeks" is more honest and more useful than "six weeks," because it communicates the uncertainty that drives buffering and risk decisions.

- [ ] Express estimates as ranges or distributions, not single points.
- [ ] Use three-point estimates (optimistic, most likely, pessimistic) for uncertain work.
- [ ] Communicate the assumptions behind the range.
- [ ] Avoid false precision that misleads stakeholders.

### [ ] Base Estimates On Data, Not Optimism

Estimates driven by hope, "the team is good, so it will be fast," systematically understate duration. Estimates based on historical data, what similar work actually took, are more reliable. When data is unavailable, acknowledge the added uncertainty rather than substituting optimism.

- [ ] Base estimates on historical actuals where possible.
- [ ] Avoid optimism-driven estimates not grounded in data.
- [ ] Acknowledge and quantify uncertainty when data is sparse.
- [ ] Adjust analogous estimates for differences between past and current work.

### [ ] Account For Non-Productive Time And Dependencies

Estimates often assume uninterrupted, focused work, ignoring meetings, email, context switching, holidays, and waiting on dependencies. Estimates that ignore these realities are systematically optimistic. Build in realistic allowances for overhead and dependency wait time.

- [ ] Account for meetings, communication, and administrative overhead.
- [ ] Include wait time for dependencies, approvals, and handoffs.
- [ ] Adjust for holidays, vacations, and part-time availability.
- [ ] Avoid estimating as if work happens in uninterrupted focus.

### [ ] Use Bottom-Up Estimating For Accuracy When Detail Exists

Bottom-up estimating, breaking work into detailed tasks and summing the estimates, produces the most accurate results because errors at the task level tend to partially offset. It requires more effort and detail but is appropriate for high-stakes or well-understood work.

- [ ] Break work into detailed, estimable tasks for bottom-up estimating.
- [ ] Sum task estimates, recognizing that errors partially offset.
- [ ] Use bottom-up when accuracy is critical and the WBS is detailed.
- [ ] Avoid bottom-up when detail is insufficient, as it produces false precision.

### [ ] Apply Contingency Based On Uncertainty

Contingency, additional time added to account for uncertainty, should be proportional to the estimate's uncertainty. Well-understood work needs little contingency; novel or complex work needs more. A flat contingency percentage across all tasks misallocates buffer.

- [ ] Size contingency to the uncertainty of each task or phase.
- [ ] Add more contingency for novel, complex, or high-risk work.
- [ ] Avoid flat contingency percentages that ignore per-task uncertainty.
- [ ] Track contingency consumption to inform future sizing.

### [ ] Involve The People Who Will Do The Work

Estimates produced by the people who will perform the work incorporate their knowledge of the task and are more likely to gain their commitment. Estimates imposed from above, or produced by someone unfamiliar with the work, are less accurate and less owned.

- [ ] Involve the doers in producing estimates.
- [ ] Leverage their knowledge of the task's specifics.
- [ ] Build commitment through participation in estimation.
- [ ] Avoid imposing estimates from above without doer input.

### [ ] Avoid The Planning Fallacy And Optimism Bias

The planning fallacy is the tendency to estimate based on the ideal scenario rather than typical experience. Optimism bias leads to systematic underestimation. Counter these by grounding estimates in base rates and historical data rather than ideal projections.

- [ ] Ground estimates in historical base rates, not ideal scenarios.
- [ ] Explicitly consider what could go wrong, not just the happy path.
- [ ] Use reference class forecasting: how long did similar work actually take?
- [ ] Recognize and counter systematic optimism bias.

### [ ] Calibrate Estimates Against Actuals Over Time

Estimation accuracy improves only with feedback. Track estimated versus actual duration and analyze systematic biases. If estimates are consistently 30% low, adjust the process or apply a calibration factor. Without calibration, the same errors repeat.

- [ ] Track estimated versus actual duration for all significant tasks.
- [ ] Analyze systematic biases (consistent over- or under-estimation).
- [ ] Apply calibration factors or process changes based on findings.
- [ ] Maintain an estimation database to improve future estimates.

### [ ] Re-Estimate As Uncertainty Resolves

Initial estimates are made with limited information. As work progresses and uncertainty resolves, re-estimate remaining work based on what has been learned. Holding to stale estimates ignores valuable new information.

- [ ] Re-estimate remaining work as the project progresses.
- [ ] Incorporate lessons from completed work into remaining estimates.
- [ ] Update the schedule based on refined estimates through change control.
- [ ] Avoid holding to stale estimates when better information exists.

## Common Traps

### [ ] Single-Point Estimates

Implying false precision by giving one number without uncertainty.

### [ ] Optimism-Driven Estimates

Estimating based on hope or ideal scenarios rather than data.

### [ ] Ignoring Overhead And Dependencies

Estimating as if work happens in uninterrupted focus with no wait time.

### [ ] Flat Contingency

Applying the same contingency percentage regardless of task uncertainty.

### [ ] Estimates Imposed From Above

Producing estimates without input from the people who will do the work.

### [ ] Planning Fallacy

Estimating the ideal path rather than typical experience.

### [ ] No Calibration

Failing to track actuals and learn from estimation errors.

### [ ] Stale Estimates

Holding to initial estimates despite better information from progress.

## Self-Check

- [ ] Is the estimation technique matched to data availability, accuracy needs, and uncertainty?
- [ ] Are estimates expressed as ranges or distributions that capture uncertainty?
- [ ] Are estimates based on historical data rather than optimism?
- [ ] Do estimates account for overhead, dependencies, and non-productive time?
- [ ] Is bottom-up estimating used for accuracy when sufficient detail exists?
- [ ] Is contingency sized to each task's uncertainty, not applied as a flat percentage?
- [ ] Are the people who will do the work involved in producing estimates?
- [ ] Are estimates grounded in base rates to counter the planning fallacy?
- [ ] Is estimation accuracy tracked and calibrated against actuals over time?
- [ ] Are remaining estimates refined as uncertainty resolves and information improves?
