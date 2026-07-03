---
name: hypothesis_and_test_design.md
description: Use when the agent is writing an experiment hypothesis, defining test design and randomization, choosing primary and secondary metrics, deciding on sample size and power, or pre-registering an experiment plan before launch.
---

# Hypothesis And Test Design

An experiment is only as credible as the plan written before it runs. A vague hypothesis, a sloppy randomization unit, an underpowered metric, or an unregistered success criterion all let a team find significance in noise after the fact. The deeper mechanics of experimentation live here, not in the high-level choice of whether to test at all.

The judgment problem is that most experiment failures are decided at design time, before any data is collected. Agents tend to write hypotheses that cannot be falsified, pick the randomization unit that is convenient rather than correct, skip power analysis because the feature is urgent, and leave the success metric undefined until the numbers look favorable. The harm is shipped features that were never actually validated, killed features that were never actually disproven, and a culture that learns to distrust experimentation.

Use this skill before writing an experiment brief, defining randomization, selecting the primary metric, computing sample size, or submitting an experiment for launch. The goal is to prevent unfalsifiable hypotheses, mismatched randomization units, underpowered tests, peeking, and ethical blind spots that make the eventual result meaningless.

## Core Rules

### Write A Falsifiable Hypothesis With All Four Parts

A real hypothesis names the population, the intervention, the predicted effect on a specific metric, and the rationale that connects them. If any part is missing, the experiment cannot be falsified, and a non-falsifiable claim is not a test.

Write the hypothesis as a single sentence with each part explicit. "We believe that showing a progress bar to first-time uploaders will increase 7-day upload completion by at least 3 percentage points, because users currently abandon when they cannot tell how long the upload will take." That sentence tells you who is tested, what changes, what should move, by how much, and why. If you cannot write this sentence, you are not ready to run an experiment; you are ready to do discovery.

Ask whether the hypothesis could be proven wrong. If the result could be interpreted as success no matter what happens, it is a story, not a hypothesis. A good hypothesis pre-commits to a direction and a magnitude.

### Choose The Randomization Unit Deliberately

The unit of randomization is the unit at which a user is assigned to control or treatment, and it must match the unit at which the effect occurs and at which the metric is computed. Mismatching these is one of the most common and most damaging errors in experimentation.

User-level randomization assigns each user once and is appropriate when the feature changes a persistent experience and the outcome is measured per user. Session-level randomization re-assigns on each visit and is appropriate for one-shot, non-persistent changes, but it dilutes measurement of any metric that accumulates across sessions. Account-level or cluster-level randomization is required when users interact, share state, or influence each other, because user-level assignment leaks treatment across the boundary.

Ask whether a single user could be exposed to both variants, whether the metric is computed at the same grain as the assignment, and whether users can contaminate each other. If any answer is yes, the randomization unit must change, or the experiment cannot be trusted.

### Select Primary, Secondary, And Guardrail Metrics At The Experiment Level

An experiment needs exactly one primary metric that the ship decision rests on, a small set of secondary metrics that explain the mechanism, and guardrail metrics that detect harm. Pre-declare all of them before launch.

The primary metric must be the one that would change if the hypothesis were true, sensitive enough to move within the experiment window, and tied to a real user or business outcome. Secondary metrics diagnose why the primary moved or did not; they do not become the primary retroactively. Guardrails catch regression on retention, latency, revenue, support, trust, or safety and should trigger escalation regardless of whether the primary metric improved.

Resist adding many primary metrics. Multiple primaries invite multiple comparisons and let the team pick the winner after seeing results. One primary, a few secondaries, and a guardrail panel is the disciplined shape.

### Run A Power Analysis Before Committing To Runtime

Power analysis converts the smallest effect you care about into the sample size and runtime you need. Skipping it is how teams ship underpowered experiments that can never detect the effect they hoped for, then misread the inconclusive result.

You need four inputs: the baseline rate or mean of the primary metric, the minimum detectable effect (MDE) that would be worth shipping, the significance level (alpha), and the desired power (commonly 0.8). The MDE is the most consequential and most often abused input. Set it to the smallest effect that is practically meaningful, not the effect you hope for. A smaller MDE demands exponentially more traffic, so an unrealistic MDE produces an unrunnable experiment.

Compute the required sample, divide by daily eligible traffic per arm, and that is the minimum runtime. If the runtime exceeds what is feasible, you must raise the MDE, choose a more sensitive metric, accept lower power, or conclude that an A/B test is not the right method for this question.

### Pre-Register The Plan To Prevent Peeking And P-Hacking

Pre-registration means locking the hypothesis, metrics, randomization, sample size, and analysis method before data collection begins. It is the structural defense against the human tendency to peek at results and stop when they look good.

Peeking inflates false positives because every look is another chance to cross the significance threshold by chance. P-hacking inflates false positives by redefining metrics, segments, or windows after the data is visible. Pre-registration removes both by making the decision rule exogenous to the observed data.

Record the plan in a shared, timestamped artifact. State the stopping rule, the analysis population, the handling of exclusions, and the metric definitions. If the team wants to deviate, that is allowed, but the deviation must be documented as exploratory analysis, not as the original test.

### Choose One-Sided Or Two-Sided Based On What You Need To Detect

A two-sided test detects effects in either direction and is the default when you genuinely do not know whether the treatment helps or harms. A one-sided test only detects an effect in the pre-specified direction and has more power in that direction, but it is blind to the opposite.

Choose one-sided only when an effect in the opposite direction would not change your decision and when the direction was chosen before seeing data. Do not switch to one-sided after a two-sided test came back marginal; that is p-hacking dressed in statistical clothing. When in doubt, use two-sided, because the cost of missing a harmful effect usually exceeds the small power gain.

### Handle Interference And Network Effects Explicitly

Many products are not independent: users invite each other, share content, transact in two-sided markets, or see the same marketplace. Standard A/B testing assumes no interference between units, and when that assumption breaks, the estimate is biased and the direction of bias is hard to predict.

Diagnose interference by asking whether a treated user can affect a control user's experience or metric. If yes, use cluster randomization (assign by city, school, account, or social group), geo experiments, or switchback designs that vary treatment over time. These designs have less power and more complexity, so plan for larger samples and longer runtimes.

Ignoring interference does not remove it; it only hides the bias in a number that looks precise.

### Evaluate The Ethics Of Withholding Treatment

An experiment requires that some users not receive the treatment. Withholding can be ethically fine when the treatment is unproven and equally distributed, but it is not fine when the treatment is a known benefit, a safety fix, or a right.

Ask whether control users are harmed by not receiving the feature, whether randomization is fair across protected groups, whether consent or notice is required, and whether the feature touches sensitive areas such as health, finance, or safety. If withholding is unacceptable, use staged rollout with monitoring instead of a controlled experiment, or use a design that gives everyone the feature and compares to a historical baseline with appropriate caveats.

## Common Traps

### Writing A Hypothesis That Cannot Fail

If the hypothesis is "improve engagement," any movement can be claimed as success and any lack of movement can be explained away. The trap is that an unfalsifiable hypothesis feels safe because it cannot be wrong, but it also cannot produce learning. Force a predicted direction and magnitude.

### Randomizing At The Wrong Grain

Randomizing per session while measuring per user, or per user while users share an account, produces correlated observations that shrink effective sample size and inflate false significance. The numbers look clean but the estimate is biased. The trap is that the dashboard reports significance without any warning that the unit is wrong.

### Skipping Power Analysis Because Of Urgency

When a launch is urgent, teams often run the experiment for as long as the calendar allows and declare whatever comes back. The trap is that an underpowered test returns inconclusive, which is then misread as "no effect," and a valuable feature is killed on the basis of a test that could never have detected its value.

### Declaring Multiple Primaries To Hedge

Adding several primary metrics feels like thoroughness, but it multiplies the chance of a false positive and lets the team report whichever metric won. The trap is that the experiment appears rigorous while actually being a fishing expedition. One primary, declared in advance.

### Peeking And Stopping Early On A Promising Look

Early data often shows significance by chance, especially with volatile metrics. Stopping the moment the result looks good biases toward false positives. The trap is that the team feels disciplined because they are watching the data, when the watching itself is the source of bias.

### Switching To One-Sided To Salvage Significance

When a two-sided test returns a borderline result, converting to one-sided can push the p-value under the threshold. The trap is that this is a post-hoc decision dressed as a design choice, and it invalidates the significance claim. Direction must be chosen before data.

### Assuming No Interference In A Social Product

In marketplaces, social networks, and collaborative tools, the no-interference assumption is often false. The trap is running a clean-looking user-level A/B test that estimates a diluted or reversed effect, then shipping based on the biased number.

### Withholding A Known Benefit As If It Were Neutral

If the treatment is a bug fix, a safety improvement, or an established best practice, randomizing some users to not receive it can be ethically wrong and can also damage trust if discovered. The trap is treating every change as experimentally neutral when some changes carry an obligation to ship to everyone.

## Self-Check

- [ ] The hypothesis states population, intervention, predicted effect with magnitude, and rationale, and could be proven false.
- [ ] The randomization unit matches the unit of effect and the unit of metric computation, with no cross-unit contamination.
- [ ] Exactly one primary metric is declared before launch, tied to the hypothesis and sensitive within the experiment window.
- [ ] Secondary metrics diagnose mechanism and guardrails detect harm, all pre-declared.
- [ ] A power analysis was run using a practically meaningful MDE, and the resulting sample size and runtime are feasible.
- [ ] The experiment plan is pre-registered with hypothesis, metrics, sample size, stopping rule, and analysis method before data collection.
- [ ] One-sided versus two-sided was chosen before seeing data, and two-sided is the default when harm in either direction matters.
- [ ] Interference and network effects were considered, and cluster or geo designs are used where users influence each other.
- [ ] The ethics of withholding treatment were evaluated, including fairness across groups and sensitivity of the domain.
- [ ] No success metric, segment, window, or direction was changed after results were visible without being documented as exploratory.
