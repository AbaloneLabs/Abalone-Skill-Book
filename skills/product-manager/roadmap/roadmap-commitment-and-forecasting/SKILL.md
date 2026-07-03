---
name: roadmap_commitment_and_forecasting.md
description: Use when the agent is deciding what to commit to on a roadmap, forecasting delivery with confidence ranges, balancing predictability against discovery, or setting confidence levels and review cadence for roadmap items.
---

# Roadmap Commitment And Forecasting

Committing on a roadmap is a different act from narrating one. A commitment is a promise that consumes trust when it is kept and destroys trust when it is missed, so the mechanics of forecasting matter as much as the choices themselves. Product managers fail at commitment by confusing exploration with delivery, by forecasting from wishful dates rather than from capacity, by treating a single point estimate as truth, or by never updating a commitment when reality has already moved. Good forecasting is not about being right once; it is about being calibrated, honest, and responsive over time.

Use this skill before answering broad questions such as "what should we commit to", "how should we forecast delivery", "how confident should we be in this roadmap item", or "how do we balance predictability with discovery". The goal is to prevent the agent from producing firm commitments on uncertain work, point-estimate dates that ignore variability, or commitments that are never revisited as facts change.

## Core Rules

### Separate The Commitment Horizon From The Discovery Horizon

Not everything on a roadmap carries the same kind of promise. Some work is firm delivery with real obligations; other work is exploration that should not be committed as if it were certain. Mixing the two corrupts both the commitment and the discovery.

Distinguish the horizons:

- firm commitment: well-understood, capacity-confirmed, low-variability work that the team is actively delivering;
- planned: directionally committed and being scoped, but with residual uncertainty in scope or timing;
- exploratory: discovery or bets whose outcome, and possibly whose existence on the roadmap, is not yet settled.

The further out and the less discovered an item is, the less it should resemble a commitment. Labeling the horizon prevents stakeholders from reading an exploration as a promise and prevents the team from feeling bound to a plan that discovery may overturn.

### Forecast From Capacity, Not From Wishful Dates

A forecast derived from a desired date is not a forecast; it is a hope. Real forecasts start from capacity and lead time and work forward, then compare against any external deadline to reveal whether the deadline is achievable.

Build the forecast from:

- available capacity after maintenance, support, and overhead;
- historical throughput, stated as a range;
- the size and uncertainty of the work;
- known dependencies and their own lead times;
- planned discovery or spikes that must precede delivery.

When a deadline is real and external, the forecast tells you whether to descope, add capacity, or negotiate. When a deadline is self-imposed, the forecast should set the date, not the reverse. A date chosen first and justified later is the most common source of missed commitments.

### Label Confidence Per Item And Make It Meaningful

Confidence labels only help if they mean something consistent. A roadmap where every item is "high confidence" teaches stakeholders to ignore the label. Confidence must reflect real evidence and be calibrated against the team's history of hitting forecasts.

Meaningful confidence considers:

- how well-understood the work is;
- how stable the scope is;
- how reliable the capacity assumption is;
- how many dependencies threaten it;
- how variable the team's historical lead time has been;
- whether discovery has happened or is still pending.

Calibrate against reality: if items labeled "high confidence" are routinely missed, the labels are inflated. Confidence that is never wrong is probably too conservative; confidence that is often wrong is dishonest. Track calibration and adjust the labeling standard over time.

### Respect Lead Time Variability; Point Estimates Lie

A single date hides the range of plausible outcomes. Lead time is variable because of rework, dependencies, incidents, scope discovery, and integration surprises, and that variability is the real story. A point estimate presented as the forecast sets an expectation the team will usually miss.

Forecast with ranges:

- express delivery as an earliest-to-latest window, not a single date;
- communicate the median or expected outcome separately from the tail risk;
- show what drives the width of the range: dependencies, unknowns, shared resources;
- narrow the range as discovery reduces uncertainty.

A range is not vagueness; it is honesty about variability. Stakeholders who plan against a range make better decisions than those surprised by a missed point date.

### Set Realistic Review And Commitment Cadence

Commitments decay if they are never revisited. A healthy roadmap has a cadence that matches its uncertainty: firm commitments reviewed often, exploratory items reviewed as discovery produces learning, and the overall portfolio rebalanced on a regular rhythm.

Set cadence by type:

- firm commitments: reviewed each delivery cycle for slip signals;
- planned items: reviewed when scoping completes or dependencies resolve;
- exploratory items: reviewed at discovery milestones, not on a fixed clock;
- portfolio: rebalanced quarterly or when a major assumption breaks.

Cadence prevents two failures: stale commitments that no one updates until they are missed, and constant re-litigation that makes every commitment feel provisional. The right cadence stabilizes trust.

### Weigh The Cost Of Overpromising Against The Cost Of Underpromising

Both errors carry costs, and the product manager must choose deliberately rather than defaulting to one. Overpromising erodes trust and forces quality-destroying rushes; underpromising leaves capacity unused and makes the team look slow or unambitious.

Consider both directions:

- overpromising cost: missed dates, lost stakeholder trust, rushed and fragile delivery, burned-out teams;
- underpromising cost: unused capacity, reduced organizational confidence in the team, pressure to add more work that then overcommits;
- context that shifts the balance: contractual deadlines, competitive pressure, reliability concerns, team morale.

The aim is calibrated ambition: commitments that stretch the team realistically and that are met often enough to build trust. A team that always hits easily is underforecasting; a team that always misses is overcommitting.

### Surface Dependencies That Threaten Commitments

Most missed commitments are not caused by the team's own work but by a dependency that arrived late, changed, or failed. Forecasting that ignores dependencies forecasts only the isolated best case.

Identify dependency risk:

- which teams, services, vendors, or approvals does this work depend on?
- have those dependencies confirmed their own commitment, or is it assumed?
- what is the integration or handoff risk?
- what is the fallback if a dependency slips?

Dependencies should be visible on the roadmap, and a commitment should be conditional on its critical dependencies being confirmed. An unconfirmed dependency is a hidden risk that converts a confident forecast into a hopeful one.

### Update Commitments When Reality Changes

A commitment is not a vow to ignore new information. When scope, capacity, dependencies, or discovery results change, the commitment must be updated promptly and the change communicated, not hidden until the original date arrives.

Update commitments by:

- detecting slip signals early, not at the deadline;
- revising the forecast and confidence as soon as evidence warrants;
- communicating the change with the reason and the new expectation;
- distinguishing a re-estimate from a true slip in performance;
- preserving the audit trail so stakeholders see the reasoning.

The credibility of a roadmap comes less from never changing and more from changing promptly and transparently when reality demands it. A commitment updated early is far less damaging than one missed silently.

## Common Traps

### Committing Discovery As If It Were Delivery

Promising an outcome from work that has not been discovered treats exploration as certainty. The trap is that discovery often changes the scope or the approach, making the original commitment impossible to honor as stated.

### Forecasting From The Desired Date

Starting from the date leadership wants and working backward produces a plan that fits the deadline on paper but has no grounding in capacity or lead time. These forecasts miss predictably.

### Presenting A Point Estimate As The Forecast

A single date implies false precision and sets an expectation the team will usually miss. The trap is that the point estimate becomes the commitment, and all the variability around it is forgotten until it bites.

### Inflating Confidence To Avoid Hard Conversations

Labeling everything high confidence avoids disappointing stakeholders now but guarantees larger disappointment later. The trap is trading short-term comfort for long-term loss of trust.

### Never Revisiting A Commitment

A commitment made once and never updated becomes stale, and the team delivers against a plan that reality has already overtaken. Silent drift turns into a surprise miss.

### Ignoring Dependencies In The Forecast

Forecasting only the team's own work hides the integration and handoff risks that cause most slips. The trap is a confident number that assumes every other team will be ready on time.

### Treating Every Miss As A Performance Failure

When misses are treated only as failure, teams hide slip signals and inflate forecasts to avoid blame. The trap is a culture that punishes honesty and thereby guarantees worse forecasting over time.

### Underpromising To Look Safe

Consistently forecasting conservatively to avoid misses leaves capacity unused and erodes confidence in the team's ambition. The trap is that underpromising becomes its own kind of unreliability, as stakeholders stop trusting the team's estimates of what is possible.

## Self-Check

- [ ] Firm commitments, planned work, and exploratory work are labeled with distinct horizons, not treated as equally certain.
- [ ] Forecasts are built from capacity and lead time, not from desired dates.
- [ ] Confidence labels are calibrated against the team's history of hitting forecasts, and not uniformly inflated.
- [ ] Delivery is expressed as a range that reflects lead time variability, not a single point date.
- [ ] Review cadence matches item type, with firm commitments reviewed often and exploratory work reviewed at discovery milestones.
- [ ] The costs of overpromising and underpromising were both considered when setting the commitment.
- [ ] Dependencies are surfaced, and commitments are conditional on critical dependencies being confirmed.
- [ ] Commitments are updated promptly when scope, capacity, dependencies, or discovery results change.
- [ ] Slip signals are detected and communicated early rather than hidden until the deadline.
- [ ] Misses are used to improve calibration rather than treated only as performance failure.
