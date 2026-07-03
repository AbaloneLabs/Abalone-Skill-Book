---
name: funnel_and_cohort_analysis.md
description: Use when the agent is building a conversion funnel, analyzing user cohorts over time, diagnosing where users drop off, comparing behavior across segments, or designing retention and engagement analyses.
---

# Funnel And Cohort Analysis

Funnels and cohorts are the two analytical methods product managers use most, and both are easy to run and easy to misread. A funnel compresses a journey into steps and a conversion rate; a cohort groups users by a shared starting point and watches them over time. The tools compute the numbers in seconds, which hides the fact that the setup decisions, not the math, determine whether the result means anything.

The harm this skill prevents is confident misinterpretation. A funnel with the wrong conversion window shows drop-off that does not exist. A retention curve read as a single number hides that one segment is thriving while another is churning. A cohort comparison is presented as causation when it only shows correlation. The method is sound; the judgment around windowing, segmentation, and causality is where analyses go wrong.

Use this skill before building a conversion funnel, reading a retention curve, comparing cohorts or segments, diagnosing drop-off, choosing time granularity, or presenting engagement over time. Ask broadly: what is the journey, what is the window, who is in each group, and does a difference mean what we think it means.

## Core Rules

### Define Funnel Steps Against Real Behavior, Not Screens

A funnel step is a behavior, not a screen. `viewed pricing` is a step only if viewing pricing is a meaningful action users take on the way to purchase. Listing every screen in the flow produces a long funnel where every step looks like failure, while the real drop-off is concentrated in one or two decisions.

Choose steps that represent genuine user decisions: started, committed key information, completed a verification, finished. Keep the funnel short enough that each step is interpretable. If a step moves the conversion rate by less than a percent, ask whether it belongs in the primary funnel or belongs in a diagnostic drill-down.

### Match The Conversion Window To Real Behavior

The conversion window is the time a user is allowed to move from one step to the next, and it changes the number more than almost any other setting. A same-session window punishes products with considered purchases; a thirty-day window flatters products with impulsive flows. There is no universally correct window; there is only the window that matches how users actually behave.

Estimate the window from historical time-to-convert data, not from intuition. If the median user converts in two minutes but the long tail stretches to a week, decide whether you are measuring immediate intent or eventual conversion, and use a different window for each. Always state the window alongside the rate; a conversion rate without a window is meaningless.

### Choose Cohort Type By The Question Being Asked

Cohorts answer different questions depending on how members are grouped, and the wrong grouping answers the wrong question. Acquisition cohorts group users by when they signed up and reveal whether newer users retain better, which speaks to product or acquisition-channel changes over time. Behavioral cohorts group users by what they did, such as completed onboarding or invited a teammate, and reveal which behaviors correlate with retention. Time cohorts within a behavioral group reveal whether a recent change shifted behavior.

Name the cohort type explicitly in any readout. A statement like "retention improved" is uninterpretable unless the listener knows whether the cohort is by signup date, by first action, or by plan tier.

### Read Retention Curves By Shape, Not Just D1 D7 D30

Retention is too often reduced to three day-markers, but the shape of the curve carries the real signal. A curve that decays continuously toward zero indicates a product that does not create lasting value. A curve that drops sharply then plateaus indicates a product that retains a core audience; the plateau level is the real retention number. A curve that rises after an initial dip indicates re-engagement or network effects kicking in.

Read D1, D7, and D30 as checkpoints, but always look at the full curve and name its shape. Compare curves by overlaying them; two cohorts with the same D30 can have radically different shapes, one decaying and one plateauing, and only the overlay reveals which is healthier.

### Compare Segments Instead Of Trusting Averages

An average retention or conversion number can hide that the result is driven by one segment while another is failing. New users may convert at twice the rate of returning users; mobile may retain far worse than web; one acquisition channel may bring high-intent users while another brings low-intent users who churn immediately.

Always segment by the dimensions that plausibly change behavior: acquisition source, platform, geography, plan tier, first-action, and tenure. When a segment is small, flag the sample size rather than reporting a dramatic rate from twenty users. The goal is not to find a winning segment to celebrate, but to understand where the product works and where it does not.

### Use Time-To-Event And Survival Intuition For Drop-Off

Conversion rates hide how long users take, and that latency is often the real story. Time-to-event analysis shows whether users convert quickly or drift in over days. Survival analysis intuition shows the share of users who have not yet converted at each point in time, which is more honest than a fixed-window rate because it does not penalize users who simply need longer.

When a funnel shows drop-off, ask whether users abandoned or whether they have not finished yet. A thirty-day conversion rate treats both the same, but the actions to take are opposite: abandonment needs friction reduction, while slow conversion needs patience or nudges. Plot the survival curve before deciding.

### Diagnose Drop-Off By Cause, Not Just Location

Knowing that sixty percent drop between two steps is a starting point, not a conclusion. The next question is why. Drop-off has different causes: confusion about what to do next, a hard requirement such as payment or verification, a technical error, a value mismatch where the user decided the product is not worth it, or an interruption where the user intended to return.

Combine the funnel with qualitative evidence, error rates, time-on-step, and re-entry behavior to distinguish these causes. A step with high drop-off but high re-entry is friction; a step with high drop-off and no return is rejection. The intervention depends entirely on the cause.

### Distinguish Correlation From Causation In Cohort Differences

Cohort comparisons tempt causal claims: users who completed onboarding retain better, therefore onboarding causes retention. The cohort that completed onboarding may also be the cohort that was more motivated to begin with, and the same motivation drives retention. The behavior and the outcome share a cause; one does not cause the other.

Before claiming a behavior drives an outcome, check for confounders: would these users have retained anyway, is there a selection effect, is there a third variable explaining both? Use holdouts, quasi-experiments, or at minimum a skeptical reading. Present cohort differences as correlations unless an experiment isolates the effect.

## Common Traps

### Reading The Top Of The Funnel As Success

A wide top-of-funnel inflates absolute conversion counts and can mask collapse later in the flow. A funnel that converts more users overall but at a worse rate per step is not necessarily winning. Always read the full funnel and the per-step rates, not just the final count.

### Using A Default Conversion Window Without Justification

The same funnel at a one-day versus thirty-day window tells different stories, and the default in the tool is not a justification. Pick the window from observed time-to-convert behavior and state it. A window chosen for convenience produces a number chosen for convenience.

### Treating D1 D7 D30 As The Whole Retention Story

Three day-markers cannot show whether retention is decaying or plateauing. Two cohorts can share a D30 number while one is heading to zero and the other is stable. Always look at the full curve and name its shape before quoting the markers.

### Reporting Averages That Hide Segment Failure

A blended conversion or retention number can improve while an important segment collapses, because a growing segment can outweigh a failing one. Segment before you summarize, and check whether the headline number is true for the segments that matter most.

### Confusing Slow Conversion With Abandonment

A fixed-window rate treats a user who converts on day eight the same as a user who never converts. If the product has a long consideration cycle, this mislabels patient users as lost. Use survival or time-to-event views to separate slow converters from true abandoners.

### Over-Segmenting Into Tiny Samples

Segmenting by every dimension at once produces cells with a handful of users and wildly unstable rates. A ninety-percent conversion rate from eleven users is noise, not insight. Keep segments large enough to interpret, and flag small samples explicitly rather than charting them as if they were stable.

### Inferring Causation From Cohort Differences

Users who do an action and users who do not often differ in motivation before the action, so the action-outcome link is confounded. Treating the cohort difference as proof that the behavior causes retention leads to building for the wrong lever. State cohort findings as correlation until an experiment shows otherwise.

### Choosing Time Granularity That Hides The Pattern

Daily granularity is noisy for low-volume products and hides the trend in day-to-day jitter; monthly granularity is too coarse to catch a recent change. Match granularity to volume and to the decision timeline, and look at more than one granularity before concluding.

## Self-Check

- [ ] Each funnel step represents a genuine user decision, not merely a screen, and the funnel is short enough that each step is interpretable.
- [ ] The conversion window is chosen from observed time-to-convert behavior and is stated alongside every rate.
- [ ] The cohort type (acquisition, behavioral, time) is named explicitly and matches the question being asked.
- [ ] Retention is read by curve shape, including whether it decays, plateaus, or re-engages, not only by D1, D7, and D30 markers.
- [ ] Results are segmented by dimensions that plausibly change behavior, and small-sample segments are flagged rather than reported as stable.
- [ ] Drop-off is diagnosed by cause using qualitative and error evidence, distinguishing friction, rejection, and interruption.
- [ ] Time-to-event or survival views are used to separate slow conversion from true abandonment where the consideration cycle is long.
- [ ] Cohort differences are presented as correlation unless an experiment or holdout isolates the causal effect.
- [ ] Time granularity matches product volume and the decision timeline, and more than one granularity was reviewed.
- [ ] The readout states window, cohort definition, segment sample sizes, and curve shape so the numbers are interpretable by others.
