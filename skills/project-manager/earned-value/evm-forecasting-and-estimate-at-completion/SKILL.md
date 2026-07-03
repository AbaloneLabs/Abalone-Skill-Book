---
name: evm_forecasting_and_estimate_at_completion.md
description: Use when the agent is forecasting EAC ETC VAC and TCPI, choosing among EAC formulas by situation, communicating completion cost forecasts and confidence to stakeholders, or deciding whether remaining work will proceed at planned rate or current efficiency.
---

# EVM Forecasting And Estimate At Completion

Forecasting is where earned value management earns or loses its credibility. The Estimate at Completion (EAC), Estimate to Complete (ETC), Variance at Completion (VAC), and To-Complete Performance Index (TCPI) translate today's performance into a prediction of where the project will land. The danger is that these formulas produce a single precise-looking number that hides a critical assumption: will the remaining work proceed at the planned rate, at the current efficiency rate, or at some blend? Different assumptions yield dramatically different forecasts, and stakeholders rarely realize that the "EAC" they were given depends entirely on which assumption the forecaster chose. Agents tend to compute one EAC, present it as the answer, and move on.

The judgment problem is how to choose the right forecasting formula for the situation, how to state the assumption explicitly so the forecast is auditable, how to pair every forecast with a confidence level and the risks that could move it, and how to use TCPI to test whether a target is even achievable. A forecast without a stated assumption and confidence is a number without meaning.

## Core Rules

### Match The EAC Formula To The Situation, Not To Convenience

There are several legitimate EAC formulas, and each embeds a different assumption. EAC equals BAC divided by CPI assumes the current cost efficiency continues for all remaining work. EAC equals AC plus (BAC minus EV) assumes the remaining work proceeds at the planned rate, treating the variance to date as a one-time event. EAC equals AC plus the bottom-up ETC replaces the remaining estimate with a fresh re-estimate. EAC combining SPI and CPI assumes both schedule and cost efficiency persist. Choose based on what you believe about the cause of variance: a permanent condition warrants the rate-based formula; a discrete, non-recurring event warrants the plan-based formula. Never pick a formula because it produces a more comfortable number.

### State The Assumption Behind Every Forecast

A forecast is meaningless without its assumption. When you report EAC, state explicitly whether it assumes the remaining work goes as planned, at current CPI, at a blended rate, or per a fresh bottom-up estimate. The same project can show an EAC near budget under one assumption and far over budget under another. Stakeholders must be able to see which assumption produced the number, because that assumption is where the real uncertainty lives. An EAC reported without its assumption is a hidden bet.

### Pair Every Forecast With Confidence And Risk

A single EAC figure implies false certainty. Instead, present a forecast with a confidence level and the specific risks that could move it: pending change requests, late-arriving actuals, unrealized risks, vendor price exposure, or scope negotiations. Distinguish a most-likely EAC from a best case and a worst case when the spread is meaningful. Decision makers need the range and the drivers, not a point estimate dressed up as certainty. The honest forecast says "we expect to land here, with this confidence, unless these things move."

### Use TCPI To Test Whether A Target Is Achievable

The To-Complete Performance Index measures the cost efficiency required for the remaining work to hit a target, usually the BAC or the current EAC. TCPI for the BAC is (BAC minus EV) divided by (BAC minus AC); TCPI for the EAC uses (EAC minus AC) in the denominator. If the required TCPI is far above the historical CPI, the target is likely unrealistic: you are asking the team to suddenly perform far better than they ever have. TCPI is a reality check on commitments. When TCPI exceeds what the team has demonstrated, treat the target as aspirational and renegotiate scope, budget, or schedule rather than pretending the efficiency will appear.

### Re-Estimate The Remaining Work When Conditions Have Shifted

When the original estimate's basis has changed, whether due to scope growth, lost assumptions, new complexity, or a different team, the plan-based and rate-based EAC formulas both inherit a broken foundation. In that case, a bottom-up ETC, re-estimating the remaining work from current knowledge, is more honest than extrapolating from a stale BAC. Bottom-up re-estimation is more effort, but it is the right response when the world no longer matches the plan. Do not let the convenience of a formula override the fact that the plan's basis is gone.

### Treat VAC As A Decision Signal, Not Just A Number

Variance at Completion, EAC minus BAC, is the projected overrun or underrun. Its size and direction should drive a conversation, not merely a report line. A large negative VAC should trigger scope, schedule, resource, or budget decisions before the overrun arrives. A positive VAC should be scrutinized: is it real efficiency, or deferred work that will show up later? VAC reported without a corresponding decision request wastes the forecast's purpose.

### Forecast Schedule Completion Separately From Cost

EVM forecasting is strongest on cost. Schedule completion forecasting needs complementary tools, because SPI degrades near project end. Pair the EAC with an explicit forecast completion date derived from the critical path and remaining work, and state the schedule confidence separately from the cost confidence. A project can land on budget but months late; conflating the two forecasts hides that.

### Communicate Forecasts In Language Stakeholders Can Act On

Stakeholders rarely need the formula; they need the implication. Translate the forecast into decisions: "At current performance we will exceed budget by X, with Y confidence; to hit the original budget we would need TCPI of Z, which exceeds anything we have achieved." Frame the forecast around the choices it enables, the confidence behind it, and the triggers that would change it. A forecast that does not change a decision was not worth computing.

## Common Traps

### Defaulting To The Most Comfortable EAC Formula

Choosing the formula that yields the lowest EAC, often the plan-based one that treats variance as one-time, hides a likely overrun. The trap is that the forecast looks reassuring while assuming away the very problem the indices are flagging. Match the formula to the cause.

### Reporting EAC Without The Underlying Assumption

A bare EAC hides whether it assumes recovery or trend continuation. The trap is that two stakeholders can hear the same number and assume different futures. Always state the assumption.

### Point Estimate Without Confidence Or Range

A single EAC with no confidence or risk context implies certainty that does not exist. The trap is that decision makers plan around a number that has a wide possible spread. Give the range and drivers.

### Ignoring TCPI When It Signals An Impossible Target

When TCPI for the BAC far exceeds historical CPI, the original budget is likely unreachable. The trap is committing to it anyway and being surprised later. Use TCPI to renegotiate early.

### Extrapolating From A Stale BAC When The Basis Has Changed

If scope, team, or assumptions have shifted, rate-based and plan-based EAC both inherit a broken denominator. The trap is a tidy forecast built on a plan that no longer reflects reality. Re-estimate bottom-up when the basis is gone.

### Treating VAC As A Report Line Rather Than A Decision Trigger

A large projected variance reported without a decision request wastes the forecast. The trap is that the overrun arrives on schedule because no one acted on the early warning. Pair VAC with a decision.

### Conflating Cost And Schedule Forecast Confidence

Reporting one blended completion forecast hides that cost and schedule can diverge sharply. The trap is that a project on budget but late, or early but over budget, looks fine in the aggregate. Forecast them separately.

### Formula Precision Exceeding Data Quality

Presenting EAC to the dollar when actuals lag and EV is subjectively assessed gives false precision. The trap is that the apparent rigor suppresses healthy skepticism. Round to a defensible precision and name the uncertainty.

## Self-Check

- [ ] Is the EAC formula chosen to match the believed cause of variance (permanent condition vs discrete event vs changed basis), not to produce a comfortable number?
- [ ] Does every reported EAC state explicitly whether it assumes planned rate, current CPI, a blend, or a bottom-up re-estimate?
- [ ] Is the forecast paired with a confidence level and the specific risks that could move it, rather than a single point estimate?
- [ ] Has TCPI been computed for the target BAC and compared against historical CPI to test whether the target is achievable?
- [ ] When the original estimate's basis has changed, has a bottom-up ETC replaced extrapolation from the stale BAC?
- [ ] Is a large VAC paired with an explicit decision request about scope, schedule, resources, or budget?
- [ ] Are cost and schedule completion forecasts reported separately, with separate confidence levels?
- [ ] Is the forecast communicated in terms of the decisions it enables, the confidence behind it, and the triggers that would change it?
- [ ] Has the precision of the reported forecast been kept within what the underlying data quality supports?
- [ ] Could a stakeholder, reading the forecast, tell which assumption produced it and what would have to be true for it to hold?
