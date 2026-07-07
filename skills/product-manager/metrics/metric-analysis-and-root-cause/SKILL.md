---
name: metric_analysis_and_root_cause.md
description: Use when the agent is analyzing why a metric moved, investigating metric drops or spikes, diagnosing root causes from data, avoiding false correlations, or presenting metric analysis that distinguishes real signal from noise and confounding factors.
---

# Metric Analysis And Root Cause

A metric moved. The natural human response is to find a story that explains it, and the danger is that a story is always available. Any change can be attributed to the launch that happened nearby, the campaign that ran last week, or the feature the team just shipped, and once a plausible explanation is offered, investigation often stops. The judgment problem in metric analysis is that correlation is cheap, confounders are everywhere, and the explanation that feels right is frequently wrong. Rigorous analysis is the discipline of distinguishing a real cause from a convenient one, separating signal from noise, and refusing to claim knowledge the data does not support, while still being useful enough to drive a decision.

Use this skill before explaining why a metric moved, before attributing a change to a cause, before diagnosing a drop or spike, or before presenting analysis to leadership. The goal is to prevent the agent from offering confident causal stories built on coincidence, from ignoring confounding factors, or from paralysis that refuses to conclude even when the evidence is sufficient.

## Core Rules

### Characterize The Change Before Explaining It

Before hunting for a cause, describe the change precisely. Many investigations go astray because the change itself was mischaracterized.

Characterize:

- the magnitude, is it within normal noise or materially outside the band;
- the timing, gradual drift, sudden step, or spike-and-revert;
- the direction and whether it is sustained;
- the scope, all users, one segment, one platform, one geography;
- the baseline and historical variance for context.

A change within normal variance is probably noise and needs no cause. A step change at a precise moment narrows the suspect set dramatically.

### Rule Out Data And Instrumentation First

The most common cause of a shocking metric movement is not behavior change but broken data. Before behavioral explanations, eliminate data causes.

Check for:

- instrumentation changes, new event version, schema change, tracking bug;
- data pipeline delays, missing partitions, backfill effects;
- definition changes, filter or segment edits applied retroactively;
- population changes, bot traffic, internal accounts, test cohorts leaking;
- reporting bugs, dashboard query errors, timezone shifts.

A large fraction of "metric crises" resolve to data issues. Always clear this ground first.

### Separate Signal From Noise Using Historical Variance

Not every wiggle is meaningful. A metric that normally fluctuates plus or minus 3% does not become alarming at minus 3.5%. Establish what normal looks like before declaring abnormal.

Assess significance by:

- comparing the change to historical variance and seasonality;
- checking sample size, small segments fluctuate wildly by chance;
- looking at rolling averages rather than single-day spikes;
- accounting for known cyclical patterns, day-of-week, monthly, seasonal;
- distinguishing a one-day spike from a sustained shift.

### Decompose The Metric To Localize The Change

Aggregate metrics hide where the change actually happened. Decompose to find the segment, channel, or component driving the movement.

Decompose by:

- segment: new vs returning, plan tier, geography, device;
- channel: acquisition source, campaign, referral;
- component: which sub-funnel or feature moved;
- cohort: is it a specific cohort's behavior or a broad shift;
- time: did all time periods shift or only one.

A retention drop that is actually a single geography's payment failure looks completely different from a broad engagement decline, and the aggregate number cannot tell them apart.

### Generate Multiple Hypotheses Before Settling

The first plausible explanation is rarely the only one and often not the best. Force yourself to enumerate alternatives before committing.

For each change, list candidate causes:

- internal: launches, feature changes, config, pricing, outages;
- external: competitor moves, seasonality, market events, holidays;
- data: instrumentation, pipeline, definition, population;
- sampling: small segment noise, composition shift;
- interaction: a combination of factors.

Rank hypotheses by how well they predict the observed pattern, not by how convenient they are.

### Look For Disconfirming Evidence, Not Just Confirming

Confirmation bias makes the favored hypothesis look proven. Actively seek evidence that would falsify it.

For each hypothesis ask:

- if this cause were true, what else should I observe;
- is that other thing observed;
- if this cause were false, could the change still happen;
- what evidence would rule this hypothesis out;
- does the timing actually support causation.

A hypothesis that survives disconfirmation is far more credible than one that merely fits.

### Beware Confounders And Composition Effects

Two things changing together does not mean one caused the other. A third factor may drive both, or the apparent relationship may be a composition artifact.

Watch for:

- Simpson's paradox, where a trend reverses across segments;
- composition shifts, where the mix of users changed, not their behavior;
- common causes, where seasonality or an event drove both metrics;
- lag effects, where the real cause preceded the apparent one;
- survivorship, where the population that remains behaves differently.

### Distinguish Causation From Correlation Honestly

Most business metric analysis cannot achieve scientific proof, and pretending otherwise is dishonest. But useful analysis can still assign reasonable confidence.

State confidence as:

- high: controlled experiment, natural experiment, or strong mechanistic link;
- medium: consistent timing, decomposition, and ruled-out alternatives;
- low: correlation only, with plausible confounders remaining;
- unknown: insufficient evidence to conclude.

Match the strength of the claim to the strength of the evidence.

### Connect Analysis To Action With Explicit Next Steps

Analysis that does not change a decision is academic. Close the loop by stating what should be done given the finding, and what would increase confidence if action is premature.

State:

- the most likely cause and its confidence level;
- the recommended action or watch-and-wait decision;
- the additional data or experiment that would confirm;
- the owner and review point.

## Common Traps

### Narrative Fallacy

Grabbing the first plausible story and stopping investigation before checking alternatives or disconfirming evidence.

### Skipping The Data Check

Attributing a behavioral cause to what is actually an instrumentation or pipeline error.

### Confusing Noise With Signal

Treating normal variance as a crisis and launching investigations or changes that are unnecessary.

### Aggregate Blindness

Analyzing only the top-line number and missing that the change is concentrated in one segment or component.

### Confirmation Bias

Seeking only evidence that supports the favored hypothesis and ignoring what contradicts it.

### Ignoring Confounders

Attributing causation to correlation without ruling out common causes or composition effects.

### Overclaiming Causation

Presenting correlational analysis as proven causation, especially to leadership who will act on it.

### Analysis Paralysis

Refusing to conclude even when evidence is sufficient, delaying action past the useful window.

## Self-Check

- [ ] The change is characterized by magnitude, timing, direction, scope, and historical variance before explanation begins.
- [ ] Data, instrumentation, pipeline, and definition causes are ruled out before behavioral explanations.
- [ ] The change is assessed against historical variance and seasonality to distinguish signal from noise.
- [ ] The metric is decomposed by segment, channel, component, cohort, and time to localize the movement.
- [ ] Multiple hypotheses were generated and ranked before settling on an explanation.
- [ ] Disconfirming evidence was actively sought for the favored hypothesis.
- [ ] Confounders, composition effects, common causes, and lag effects were considered and addressed.
- [ ] The causal claim is matched to the evidence strength, high, medium, low, or unknown.
- [ ] The analysis connects to a recommended action, watch decision, or confirmatory next step.
- [ ] No correlational finding was presented as proven causation beyond what the evidence supports.
