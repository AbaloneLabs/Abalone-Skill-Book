---
name: estimation_and_forecasting.md
description: Use when the agent is estimating project effort or duration, producing forecasts, sizing work packages, choosing estimation techniques, setting confidence ranges, or reviewing whether schedule and cost estimates are realistic enough to commit to.
---

# Estimation And Forecasting

An estimate is not a promise. It is a reasoned forecast under uncertainty. Projects fail repeatedly not because people estimate badly in isolation, but because estimates are converted into commitments without acknowledging their range, their assumptions, or the capacity that constrains them. The project manager must choose appropriate techniques, express uncertainty honestly, separate effort from duration, and avoid the false precision that turns a guess into a deadline.

Use this skill before sizing work, producing schedule or cost estimates, preparing a forecast for stakeholders, committing to dates, or reviewing whether an estimate is credible. The goal is to prevent the agent from delivering single-point numbers that look authoritative but conceal large uncertainty.

## Core Rules

### Distinguish Effort, Duration, And Cost

These are different quantities and conflating them is the most common estimation error.

- Effort is the amount of labor required, usually in person-hours or person-days.
- Duration is the elapsed calendar time until the work is complete, affected by availability, sequencing, waiting, and review.
- Cost is the financial impact, affected by effort, rates, materials, vendor pricing, and overhead.

Two days of effort can become two weeks of duration when the owner is shared. A low-effort task can carry high cost when it requires a premium specialist or vendor. Always state which quantity an estimate refers to.

### Match The Technique To Uncertainty And Detail

Different estimation techniques suit different stages. Using a detailed technique on undefined work produces false precision. Using a rough technique on well-understood work wastes the chance to be accurate.

Common techniques and when they fit:

- Analogous estimation: compare to a similar past deliverable. Useful early, weak when context differs.
- Parametric estimation: apply a rate or unit model, such as cost per unit or hours per screen. Useful when a stable model exists.
- Bottom-up estimation: estimate each work package and roll up. Accurate but expensive, and only valid once the breakdown exists.
- Three-point estimation: optimistic, most likely, pessimistic. Useful for exposing range and risk.
- Expert judgment and Delphi: gather independent estimates and reconcile. Useful for novel work.

State which technique was used and why.

### Express Estimates As Ranges With Confidence

A single number invites it to be treated as a commitment. Express important estimates as ranges tied to a confidence level, such as a most-likely value plus a range that captures realistic variation.

For example, an effort estimate might be presented as a most likely of 10 days with a likely range of 7 to 16 days. For commitments, distinguish a committed date from a forecast date. A committed date should carry enough buffer or scope flexibility to be defensible.

### Separate Estimates From Targets

A target is a desired outcome, often driven by a deadline or budget. An estimate is what the work likely requires. When a target is tighter than the estimate, the gap is a constraint that must be resolved through scope, resources, quality, risk acceptance, or sequencing. Do not let the target silently overwrite the estimate.

Make the gap visible. If leadership sets a date, show what tradeoffs are required to hit it rather than pretending the estimate matches.

### Account For Non-Productive Time And Capacity

Estimates that assume full uninterrupted focus on one task are systematically optimistic. Real capacity is reduced by meetings, support, context switching, administrative work, interruptions, holidays, leave, and parallel projects.

Apply realistic utilization factors. A specialist allocated 50 percent to the project is not producing five full days of project work per week. Calibrate against historical actuals where possible.

### Build In Contingency Where Uncertainty Is Real

Contingency is reserve held against identified uncertainty within a work package or phase. It is not padding hidden to make numbers look safe. Allocate contingency explicitly, proportional to risk, and manage who can consume it.

Reserve analysis distinguishes contingency, held by the project manager for identified risks, from management reserve, held above the project for unidentified work. Both should be visible in the budget and schedule, not buried.

### Re-Estimate As Reality Accumulates

Initial estimates are forecasts based on limited information. As work proceeds, actuals provide evidence. Re-estimate at meaningful points rather than clinging to the original number.

Track estimate accuracy over time. If estimates are consistently low, calibrate future estimates upward or investigate the cause. Forecasting should improve as the project learns.

### Document Assumptions And Dependencies

Every estimate rests on assumptions about scope stability, resource availability, dependency timing, tooling, and external responses. Record these. When an assumption breaks, the estimate should be revisited rather than defended.

Dependencies on vendors, approvals, or other teams are especially prone to optimistic assumptions. Make their timing explicit and flag the risk.

## Common Traps

### Single-Point Estimates Treated As Commitments

One number with no range becomes a deadline. Ranges and confidence levels protect against this.

### Confusing Effort With Duration

Assuming one person-day of effort equals one calendar day of duration ignores availability, waiting, and sequencing.

### Anchoring On A Target Date

When a deadline is stated first, subsequent estimates drift toward it regardless of the underlying work. Estimate independently before reconciling to the target.

### Optimism Bias And Excluding Rework

Estimates often assume smooth execution with no rework, no defects, no review cycles, and no waiting. Real projects include all of these.

### Using Bottom-Up On Undefined Work

Detailed estimates on vague scope create precision theater. Decompose and clarify before applying bottom-up techniques.

### Hidden Padding Instead Of Visible Reserve

When contingency is hidden inside line items to avoid challenge, it cannot be managed. Make reserve explicit and governed.

### Ignoring Historical Data

Estimates built from hope rather than past actuals repeat the same errors. Use historical completion data to calibrate.

### Forgetting Integration And Non-Development Work

Estimates that cover only the core build often omit integration, testing, documentation, training, migration, support readiness, and handoff, which can be a large fraction of total effort.

## Self-Check

- [ ] Effort, duration, and cost are distinguished and each estimate states which quantity it refers to.
- [ ] The estimation technique is chosen to match the level of uncertainty and available detail.
- [ ] Important estimates are expressed as ranges with confidence levels, not single-point numbers.
- [ ] Targets are separated from estimates, and gaps are resolved through explicit tradeoffs.
- [ ] Realistic capacity and utilization factors account for meetings, support, leave, and parallel work.
- [ ] Contingency and management reserve are explicit and governed, not hidden as padding.
- [ ] Estimates are re-evaluated as actuals accumulate and forecasting improves over time.
- [ ] Assumptions and dependencies, especially external ones, are documented and revisited when they break.
- [ ] Historical actuals were used to calibrate rather than relying on hope.
- [ ] Integration, testing, documentation, training, migration, and handoff effort are included in the estimate.
