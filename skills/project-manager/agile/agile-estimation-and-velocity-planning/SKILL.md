---
name: agile_estimation_and_velocity_planning.md
description: Use when the agent is estimating agile work using story points or other relative techniques, establishing and using velocity for planning, running planning poker or similar estimation sessions, calibrating estimates over time, or diagnosing why sprint commitments are consistently missed or velocity is erratic.
---

# Agile Estimation And Velocity Planning

Agile estimation replaces deterministic, hour-based estimation with relative, effort-based estimation and uses the team's empirical velocity to plan iterations. The judgment problem is that relative estimation is often misunderstood or misused: story points are treated as hours in disguise, velocity is treated as a performance target rather than a measurement, and estimation is gamed to hit commitments. When used well, relative estimation and velocity provide a lightweight, honest basis for forecasting that improves over time. When used poorly, they become another form of false precision that produces missed commitments and erodes trust in the process.

Use this skill before establishing agile estimation, before using velocity for planning, before running estimation sessions, or when sprint commitments are consistently missed or velocity is erratic. The goal is to prevent the agent from conflating story points with hours, from treating velocity as a target, from using velocity to compare or pressure teams, or from letting estimation decay into gaming.

## Core Rules

### [ ] Estimate Relatively, Not Absolutely

Story points estimate effort relative to other work, not in absolute hours. A story estimated at 5 points should take roughly five times the effort of a 1-point story. Relative estimation leverages the fact that people compare more reliably than they measure absolutely, and it accommodates the uncertainty inherent in software work.

- [ ] Estimate effort relative to reference stories, not in absolute hours.
- [ ] Use a shared understanding of reference stories to anchor the scale.
- [ ] Recognize that points capture effort, complexity, and uncertainty together.
- [ ] Avoid converting points to hours, which defeats the purpose.

### [ ] Use Velocity As A Measurement, Not A Target

Velocity is the empirical rate at which the team completes work, measured over past iterations. It is a planning input, forecasting how much the team can likely complete, not a performance target to hit. Treating velocity as a target incentivizes inflating estimates or cutting quality to hit a number.

- [ ] Use velocity as a measurement of past throughput for forecasting.
- [ ] Avoid setting velocity targets that the team must hit.
- [ ] Recognize that velocity varies naturally and is not a quality metric.
- [ ] Do not use velocity to compare or rank teams.

### [ ] Base Forecasts On A Range, Not A Single Number

Velocity varies from iteration to iteration due to normal variability. Forecast using a range, such as the average plus or minus variation, or a statistical distribution, rather than a single number. Ranges communicate uncertainty honestly and produce more realistic forecasts.

- [ ] Forecast using a velocity range or distribution, not a single average.
- [ ] Account for historical variability in the range.
- [ ] Communicate forecasts as probabilities, not certainties.
- [ ] Update the range as new velocity data arrives.

### [ ] Use The Whole Team For Estimation

Estimation by the whole team, through techniques like planning poker, leverages diverse perspectives and builds shared understanding. Single-person estimation misses knowledge distributed across the team and fails to build commitment. Consensus-based estimation produces more accurate and owned estimates.

- [ ] Involve the whole team in estimation, not a single estimator.
- [ ] Use planning poker or similar techniques to surface and reconcile perspectives.
- [ ] Discuss divergent estimates to surface hidden assumptions and risks.
- [ ] Build shared ownership of the estimates through participation.

### [ ] Keep Estimates Independent Of The Estimator

If the same person always estimates a particular type of work, estimates reflect that person's speed, not the team's. Rotate estimation and use the team's collective judgment so estimates reflect a team-average effort, which aligns with velocity measured across the team.

- [ ] Estimate as a team so estimates reflect team-average effort.
- [ ] Avoid assigning estimates based on who will do the work.
- [ ] Rotate who leads estimation for different work types.
- [ ] Recognize that velocity assumes team-average effort.

### [ ] Re-Estimate Only When Understanding Changes

Estimates should be stable once made; re-estimating to make velocity look better is gaming. Re-estimate only when the team's understanding of the work has genuinely changed, and document why. Stable estimates produce a velocity trend that is meaningful.

- [ ] Keep estimates stable; do not re-estimate to manipulate velocity.
- [ ] Re-estimate only when understanding genuinely changes, with documented rationale.
- [ ] Preserve historical estimates to maintain a meaningful velocity trend.
- [ ] Avoid pressure to revise estimates to hit targets.

### [ ] Calibrate The Scale With Reference Stories

A relative scale is only useful if the team shares an understanding of what a 1-point or 5-point story looks like. Maintain a set of reference stories, agreed examples at each point value, to anchor the scale and keep estimation consistent over time.

- [ ] Maintain reference stories that anchor each point value.
- [ ] Use reference stories to calibrate new estimators.
- [ ] Periodically review and update reference stories.
- [ ] Ensure the team shares a common understanding of the scale.

### [ ] Split Large Stories For Reliable Estimation

Large stories are hard to estimate reliably because they bundle multiple uncertainties. Splitting large stories into smaller, independently valuable ones improves estimation accuracy and enables incremental delivery. A story too big to estimate is too big to deliver in an iteration.

- [ ] Split stories that are too large to estimate or deliver in one iteration.
- [ ] Ensure each story is independently valuable and testable.
- [ ] Use splitting to reduce estimation uncertainty.
- [ ] Avoid estimating stories so large that the estimate is meaningless.

### [ ] Use Velocity For Medium-Term Forecasting, Not Short-Term Control

Velocity is most useful for forecasting releases and milestones over multiple iterations. It is less useful for controlling a single iteration, where commitment should be based on the specific work and the team's confidence. Do not over-rely on velocity for micro-planning.

- [ ] Use velocity for release and milestone forecasting across iterations.
- [ ] Base iteration commitment on specific work and team confidence, not just velocity.
- [ ] Avoid using velocity to dictate single-iteration commitments.
- [ ] Recognize the limits of velocity for short-term planning.

### [ ] Diagnose Erratic Velocity Rather Than Ignoring It

Erratic velocity, large swings iteration to iteration, signals a problem: unstable team composition, inconsistent estimation, scope churn, or external disruptions. Investigate the cause rather than averaging it away. Stable velocity is a sign of a healthy process.

- [ ] Investigate the causes of erratic velocity rather than ignoring it.
- [ ] Look for team instability, estimation inconsistency, or scope churn.
- [ ] Address the underlying cause to stabilize velocity.
- [ ] Treat velocity stability as a process health indicator.

## Common Traps

### [ ] Points As Hours

Treating story points as hours in disguise, defeating relative estimation.

### [ ] Velocity As Target

Setting velocity targets that incentivize estimate inflation or quality cutting.

### [ ] Single-Number Forecasts

Forecasting with a single average rather than a range, hiding uncertainty.

### [ ] Single-Person Estimation

Estimating without the whole team, missing distributed knowledge.

### [ ] Estimator-Dependent Estimates

Estimating based on who will do the work rather than team-average effort.

### [ ] Re-Estimating To Game Velocity

Changing estimates to make velocity look better rather than reflecting understanding.

### [ ] No Reference Stories

Estimating without shared anchors, producing inconsistent scales.

### [ ] Large Un estimable Stories

Estimating stories so large that the estimate is meaningless.

## Self-Check

- [ ] Is effort estimated relatively using story points, not in absolute hours?
- [ ] Is velocity used as a measurement for forecasting, not as a performance target?
- [ ] Are forecasts based on a velocity range that accounts for variability?
- [ ] Does the whole team participate in estimation through consensus techniques?
- [ ] Are estimates independent of who will do the work, reflecting team-average effort?
- [ ] Are estimates kept stable, re-estimated only when understanding genuinely changes?
- [ ] Is the estimation scale calibrated with agreed reference stories?
- [ ] Are large stories split for reliable estimation and incremental delivery?
- [ ] Is velocity used for medium-term forecasting rather than single-iteration control?
- [ ] Is erratic velocity diagnosed for its underlying causes rather than ignored?
