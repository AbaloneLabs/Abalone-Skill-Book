---
name: experiment_design_and_randomization.md
description: Use when the agent is designing an A/B test, choosing the randomization unit and assignment strategy, defining experiment eligibility and exposure logic, handling interference between users, or planning the structural integrity of an experiment before it runs.
---

# Experiment Design And Randomization

An experiment is decided long before any data is collected. The structural choices made at design time, the randomization unit, the eligibility rules, the exposure logging, and the handling of interference, determine whether the eventual numbers can be trusted at all. Get these wrong and the readout is a precise-looking lie.

The judgment problem is that structural flaws are invisible in the final dashboard. A mismatched randomization unit, a leaky eligibility filter, or an unhandled network effect all produce clean charts with biased estimates, and nothing in the tooling warns the team that the unit of assignment is wrong. Agents tend to copy the last experiment's configuration, randomize at the unit that is easiest to log, and assume independence between users in products where users plainly influence each other. The harm is shipped features that were never validated, killed features that were never disproven, and a culture that learns to distrust experimentation.

Use this skill before writing an experiment brief, choosing how users are assigned, defining who is eligible, or planning exposure logging. The goal is to prevent structural errors that make the eventual result meaningless regardless of how much traffic flows through.

## Core Rules

### Match The Randomization Unit To The Effect And The Metric

The unit of randomization is the unit at which a user is assigned to control or treatment, and it must match both the unit at which the effect occurs and the unit at which the metric is computed. Mismatching these is the most damaging and most common error in experimentation, because the dashboard reports significance without any warning that the unit is wrong.

User-level randomization assigns each user once and is appropriate when the feature changes a persistent experience and the outcome is measured per user. Session-level randomization re-assigns on each visit and suits one-shot, non-persistent changes, but it dilutes any metric that accumulates across sessions. Account-level or cluster-level randomization is required when users interact, share state, or influence each other, because user-level assignment leaks treatment across the boundary. Ask three questions: could a single user be exposed to both variants, is the metric computed at the same grain as the assignment, and can users contaminate each other. If any answer is yes, the unit must change.

### Define Eligibility Before Assignment, Not After

Eligibility is the rule that decides who enters the experiment population, and it must be computed before randomization so that assignment is not conditioned on post-treatment behavior. Defining eligibility after the fact lets the team shape the population to flatter results.

State the eligibility criteria in the brief: which surfaces, which segments, which new versus existing users, which devices, and any exclusion rules such as internal employees or beta cohorts. Decide whether the population is intent-based, for example users who reach a checkout page, or attribute-based, for example all logged-in users. Intent-based populations are more sensitive but smaller; attribute-based populations are larger but diluted. The choice changes the effect you can detect and the sample you need.

### Log Exposure At The Moment Of Treatment, Not At Session Start

Exposure logging records when a user actually received the treatment, and it must fire at the moment the treatment is experienced, not when the page loads or the session begins. Logging too early inflates the denominator with users who never saw the change; logging too late drops users who saw it and acted on it.

Define the exposure event precisely and instrument it before launch. For client-side changes, account for the fact that some users will have ad blockers, slow connections, or stale clients that never receive the new code. Decide whether unexposed assigned users are excluded from analysis or counted, and apply that rule consistently. The analysis population is the exposed population, not the assigned population, when exposure is the meaningful trigger.

### Handle Interference And Network Effects Explicitly

Many products are not independent. Users invite each other, share content, transact in two-sided markets, bid in the same auction, or see the same marketplace. Standard A/B testing assumes no interference between units, and when that assumption breaks, the estimate is biased and the direction of bias is hard to predict.

Diagnose interference by asking whether a treated user can affect a control user's experience or metric. If yes, use cluster randomization that assigns by city, school, account, or social group, geo experiments that vary treatment by region, or switchback designs that vary treatment over time within the same cluster. These designs have less power and more complexity, so plan for larger samples and longer runtimes. Ignoring interference does not remove it; it only hides the bias in a number that looks precise.

### Pre-Declare The Analysis Plan To Prevent Post-Hoc Shaping

Pre-registration locks the hypothesis, metrics, randomization, sample size, eligibility, and analysis method before data collection begins. It is the structural defense against the human tendency to peek, re-slice, and redefine once results are visible.

Record the plan in a shared, timestamped artifact. State the stopping rule, the analysis population, the handling of exclusions, the segment breakdowns you intend to report, and the metric definitions. If the team wants to deviate after seeing data, that is allowed, but the deviation must be documented as exploratory analysis, not as the original test. The original, pre-registered result is the one the ship decision rests on.

### Separate Rollout Mechanics From Experiment Design

Staged rollout and controlled experimentation solve different problems and should not be conflated. Rollout is a risk-management tool that limits blast radius; experimentation is a learning tool that estimates a causal effect. A rollout with monitoring can tell you nothing broke, but it cannot tell you whether the feature moved the metric you hoped for.

If you need a causal estimate, run a real experiment with a held-out control. If you cannot hold out a control for ethical or operational reasons, use a quasi-experiment such as a difference-in-differences against a historical or geographic baseline, and label the conclusion as lower-confidence. Do not present a monitored rollout as if it were an A/B test.

### Evaluate The Ethics Of Withholding Treatment

An experiment requires that some users not receive the treatment. Withholding is ethically fine when the treatment is unproven and equally distributed, but it is not fine when the treatment is a known benefit, a safety fix, or a right.

Ask whether control users are harmed by not receiving the feature, whether randomization is fair across protected groups, whether consent or notice is required, and whether the feature touches sensitive areas such as health, finance, or safety. If withholding is unacceptable, use staged rollout with monitoring instead of a controlled experiment. The experiment is a means, not an end; if the means is wrong, choose a different method.

## Common Traps

### Randomizing At The Wrong Grain

Randomizing per session while measuring per user, or per user while users share an account, produces correlated observations that shrink effective sample size and inflate false significance. The trap is that the dashboard reports a clean p-value without any warning that the unit is wrong, so the team trusts a biased estimate. Always check that assignment grain, effect grain, and metric grain agree.

### Defining Eligibility After Seeing The Data

It is tempting to exclude the segment that dragged the result down and call the trimmed population the real one. The trap is that every exclusion rule applied after results are visible is a degree of freedom the analyst used to manufacture significance. Eligibility must be fixed before launch; post-hoc exclusions are exploratory and must be labeled as such.

### Logging Exposure At The Wrong Moment

If exposure fires on page load, users who bounced before the new element rendered are counted as treated and dilute the effect. If it fires on a click that only happens after the treatment works, the population is conditioned on success. The trap is that the metric looks clean while the denominator is wrong in a direction that biases the estimate.

### Assuming No Interference In A Social Product

In marketplaces, social networks, and collaborative tools, the no-interference assumption is often false. The trap is running a clean-looking user-level A/B test that estimates a diluted or reversed effect, then shipping based on the biased number. The dashboard never flags interference; you must reason about it from product knowledge.

### Copying The Last Experiment's Configuration

Reusing the previous brief's randomization, eligibility, and exposure settings feels efficient but silently inherits flaws when the new feature differs. The trap is that the team treats the config as a template rather than a fresh decision, and a unit that was correct for the last test is wrong for this one. Every experiment deserves its own structural review.

### Conflating Rollout With Experimentation

Shipping to one percent with monitoring and calling it an A/B test gives the team false confidence. The trap is that rollout confirms nothing broke but provides no causal estimate, yet the conclusion is reported as if the feature were validated. If you need an effect estimate, hold out a control.

### Withholding A Known Benefit As If It Were Neutral

If the treatment is a bug fix, a safety improvement, or an established best practice, randomizing some users to not receive it can be ethically wrong and can damage trust if discovered. The trap is treating every change as experimentally neutral when some changes carry an obligation to ship to everyone.

## Self-Check

- [ ] The randomization unit matches the unit of effect and the unit of metric computation, with no cross-unit contamination.
- [ ] Eligibility criteria are defined before assignment and are not conditioned on post-treatment behavior.
- [ ] The exposure event fires at the moment of treatment, and the analysis population rule for unexposed assigned users is explicit.
- [ ] Interference and network effects were considered, and cluster, geo, or switchback designs are used where users influence each other.
- [ ] The analysis plan is pre-registered with hypothesis, metrics, sample size, stopping rule, eligibility, and exclusions before data collection.
- [ ] Staged rollout is not confused with controlled experimentation; a held-out control exists where a causal estimate is needed.
- [ ] The ethics of withholding treatment were evaluated, including fairness across groups and sensitivity of the domain.
- [ ] The experiment configuration was reviewed fresh for this feature, not copied blindly from the previous brief.
- [ ] Any post-hoc segment, exclusion, or metric change is documented as exploratory, not as the original test.
- [ ] The structural choices would still be defensible if the result came back opposite to what the team hoped.
