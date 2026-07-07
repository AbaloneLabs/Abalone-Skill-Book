---
name: quasi_experiments_and_observational_inference.md
description: Use when the agent is evaluating a change that cannot be A/B tested, designing quasi-experiments, using difference-in-differences or regression discontinuity, drawing causal inference from observational data, or deciding how much confidence to place in non-randomized analysis.
---

# Quasi Experiments And Observational Inference

The cleanest way to know whether a change caused an outcome is a randomized experiment, but a great deal of product work cannot be randomized. A pricing change affects everyone at once, a feature launches to an entire market, a regulatory shift hits a whole region, and network effects make per-user randomization impossible. When randomization is impossible, the question does not disappear, it just becomes harder. Quasi-experimental methods, difference-in-differences, regression discontinuity, interrupted time series, synthetic control, are the tools for drawing causal inference from observational data, and their value is that they make the assumptions explicit. The judgment problem is that these methods can produce convincing-looking causal estimates that rest on untestable assumptions, and a practitioner who does not understand those assumptions will overclaim causation from designs that cannot support it.

Use this skill before analyzing a non-randomized change, before attributing an outcome to a launch that could not be A/B tested, before choosing among quasi-experimental methods, or before presenting observational analysis as evidence. The goal is to prevent the agent from treating observational analysis as equivalent to an experiment, from ignoring the assumptions that make a method valid, or from abandoning causal reasoning entirely because a true experiment is impossible.

## Core Rules

### Recognize When Randomization Is Impossible And Why

The first step is honestly assessing why a randomized experiment cannot run, because the reason determines which quasi-experiment is viable.

Common barriers:

- universal change: pricing, policy, or rebrand affects everyone simultaneously;
- network effects: per-user assignment contaminates through social or data connections;
- legal or ethical constraints: cannot deny a benefit or feature to a control group;
- market or partner constraints: a launch is all-or-nothing by contract;
- historical change: the intervention already happened and only observational data exists.

Naming the barrier clarifies what substitute evidence is possible and what its limits are.

### Match The Method To The Structure Of The Change

Different quasi-experimental methods exploit different structures, and applying the wrong one produces invalid estimates. Choose based on what natural comparison the situation creates.

Common methods and their fit:

- difference-in-differences: a treatment group and a comparable control group, both measured before and after, assuming parallel trends;
- regression discontinuity: treatment assigned by a threshold, comparing units just above and below;
- interrupted time series: a single group measured over many points before and after an intervention;
- synthetic control: constructing a weighted combination of untreated units to approximate the treated unit's counterfactual;
- matching and propensity methods: constructing a comparison group from observables, weakest for unmeasured confounding.

The method must fit the data-generating structure, not be chosen for convenience.

### Make The Identifying Assumption Explicit And Defend It

Every quasi-experiment rests on an assumption that cannot be directly tested, and the credibility of the result lives or dies on that assumption. State it openly and argue for it.

Key assumptions to state and defend:

- difference-in-differences: parallel pre-trends would have continued without treatment;
- regression discontinuity: units near the threshold are equivalent except for treatment;
- interrupted time series: no other change occurred at the same time;
- synthetic control: the donor units are unaffected by the treatment and track the treated unit pre-intervention;
- matching: no unmeasured confounders differ between groups.

If the assumption is implausible, the estimate is not credible regardless of how precise it looks.

### Construct The Most Plausible Counterfactual

Causal inference is fundamentally about what would have happened without the change. The entire effort is constructing a credible counterfactual, the counterfactual group or trend.

Strengthen the counterfactual by:

- choosing comparison units similar on observables and ideally on trends;
- validating that the comparison group tracked the treatment group before the change;
- checking for contamination, spillovers, or shared shocks;
- using multiple comparison groups to test robustness;
- extending the pre-period to establish stable baseline behavior.

A counterfactual that did not track before the change cannot be trusted after it.

### Test For Confounders And Concurrent Changes

The greatest threat to observational inference is a confounder, something else that changed at the same time as the intervention and drove the outcome. Actively hunt for these.

Investigate:

- other product changes, launches, or fixes in the same window;
- marketing campaigns, pricing promotions, or sales motion shifts;
- seasonal, holiday, or cyclical effects;
- competitor moves or market events;
- data or instrumentation changes that could create artificial movement;
- composition shifts in the population.

A difference-in-differences estimate collapses if a concurrent shock hit only the treatment group.

### Use Placebo And Falsification Tests

A strong quasi-experimental analysis tests itself. Placebo and falsification checks probe whether the method would find an effect where none should exist.

Run checks such as:

- placebo treatment dates: apply the method to a pre-period with no intervention and confirm no effect;
- placebo outcomes: test outcomes that should not be affected by the intervention;
- donor pool validation: confirm the synthetic control tracks the treated unit in a held-out pre-period;
- threshold balance: in regression discontinuity, confirm covariates are balanced around the cutoff.

If the method finds effects in placebos, it is not trustworthy for the real analysis.

### Report Effect Sizes With Honest Uncertainty

Quasi-experimental estimates carry more uncertainty than experiments, both statistical and from assumption fragility. Report that uncertainty honestly.

Report:

- point estimates with confidence intervals, not single numbers;
- sensitivity to the identifying assumption;
- results under alternative specifications and comparison groups;
- the magnitude relative to the minimum meaningful effect;
- explicit acknowledgment of what the method cannot rule out.

### Distinguish Causal From Descriptive Conclusions

Be disciplined about what the analysis supports. Observational analysis can be powerfully descriptive and suggestive of causation, but it rarely achieves the certainty of a randomized experiment.

Calibrate claims:

- causal: supported by a method with a defensible identifying assumption and robustness checks;
- suggestive: consistent with causation but with remaining confounders or assumption fragility;
- descriptive: shows association without causal identification;
- insufficient: the data and method cannot support a conclusion.

Match the language to the evidence, especially when communicating to leadership.

### Triangulate Across Methods And Evidence

A single observational estimate is fragile. Triangulation across methods, time periods, and evidence types builds confidence that no single method can.

Triangulate by:

- applying multiple quasi-experimental methods to the same change;
- cross-checking against qualitative evidence and customer behavior;
- confirming with any partial randomization or natural experiments available;
- checking consistency with related metrics and adjacent outcomes;
- building a converging case rather than relying on one estimate.

## Common Traps

### Treating Observational As Experimental

Presenting quasi-experimental estimates with the confidence of a randomized trial, ignoring assumption fragility.

### Wrong Method For The Structure

Applying difference-in-differences without parallel trends, or matching with unmeasured confounders.

### Unstated Identifying Assumption

Running the method without stating or defending the assumption the estimate depends on.

### Ignoring Concurrent Confounders

Attributing an effect to the intervention when another change drove the outcome.

### Weak Counterfactual

A comparison group that did not track the treatment group before the change.

### No Falsification Tests

Skipping placebo and robustness checks that would have exposed method fragility.

### Overprecise Reporting

Presenting point estimates without the uncertainty from assumptions and specification choice.

### Causal Language For Descriptive Findings

Using causal framing for associations the method cannot identify.

## Self-Check

- [ ] The reason randomization is impossible is identified, and the viable quasi-experimental approach follows from it.
- [ ] The method is matched to the data-generating structure of the change, not chosen for convenience.
- [ ] The identifying assumption, parallel trends, threshold equivalence, no concurrent shock, is stated and defended.
- [ ] The counterfactual is constructed from comparison units that tracked the treatment group before the change.
- [ ] Confounders and concurrent changes, product, marketing, seasonal, competitor, data, are actively investigated and ruled out.
- [ ] Placebo and falsification tests confirm the method does not find effects where none should exist.
- [ ] Effect sizes are reported with confidence intervals, sensitivity, and alternative specifications, not single numbers.
- [ ] Conclusions are calibrated as causal, suggestive, descriptive, or insufficient, matching the evidence strength.
- [ ] Multiple methods and evidence types triangulate toward the conclusion rather than relying on one estimate.
- [ ] The analysis acknowledges what it cannot rule out rather than presenting false certainty.
