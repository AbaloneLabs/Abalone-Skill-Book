---
name: ab_test_design_and_analysis.md
description: Use when the agent is designing, running, or analyzing an A/B test (online controlled experiment); calculating required sample size and power; choosing primary and guardrail metrics; deciding experiment duration; interpreting statistical significance versus practical significance; handling multiple comparisons; diagnosing bias from improper randomization, peeking, or sample-ratio mismatch; or deciding whether to ship a variant based on experiment results. Also covers the failure modes of underpowered tests, peeking at results and stopping early, multiple-testing inflation of false positives, confounding from broken randomization, Simpson's paradox in aggregated results, and the recurring mistake of treating a single statistically significant metric as proof the variant is better.
---

# A/B Test Design And Analysis

An A/B test is a randomized controlled experiment: split users into control and treatment, expose only treatment to the change, and measure the difference. The mechanics are simple; the judgment is hard, because a dozen subtle errors each produce a confident wrong conclusion. An underpowered test cannot detect the effect you care about, so a "no significant difference" is mistaken for "no effect." Peeking at results daily and stopping when significance appears inflates false positives. Testing twenty metrics and celebrating the one that moved guarantees false positives by chance. Broken randomization (a bug in assignment) confounds the variant with some other difference and the "effect" is an artifact. The judgment problem is that an A/B test produces a number that looks like proof, and most of the work is in ensuring the number actually means what it appears to mean.

Agents tend to miss these problems because the tooling produces a clean result — a p-value, a lift, a confidence interval — and a significant result feels like a decision. The harm is decisions made on noise. A team ships a variant because one of fifteen metrics moved significantly (it was the one-in-twenty chance). A feature is killed because the test "showed no effect" when it was underpowered to detect the real, modest effect. A result reverses when sliced by segment (Simpson's paradox) and the aggregate pointed the wrong way. A test is stopped on day 3 because the metric crossed significance, when it would have regressed by day 14. The judgment problem is to treat the experiment design — sample size, duration, metrics, randomization, analysis plan — as the thing that determines whether the conclusion is trustworthy, and to read results conservatively in light of the many ways they can mislead.

This skill covers sample size and power, statistical vs practical significance, multiple comparisons, bias and validity threats, and early stopping. It complements the experiment-metrics-and-guardrails skill (metric and guardrail design) and the statistical-pitfalls skill (general interpretation errors).

## Core Rules

### Calculate Sample Size And Duration Before Running, Not After

An experiment's ability to detect an effect is fixed by its sample size, which is determined before it runs. Deciding duration by "let's run it a week and see" produces tests that are either underpowered (cannot detect the real effect) or wastefully long:

- **Power analysis sets the required sample size.** Given the minimum effect size you care about (the minimum detectable effect), the baseline metric's variance, the desired significance level (α, usually 0.05), and power (1−β, usually 0.8), compute the sample size needed per arm. Smaller detectable effects and noisier metrics require larger samples — often far larger than intuition suggests.
- **If the required sample size exceeds what you can collect, the test cannot answer the question.** A test that needs 500,000 users per arm but reaches 50,000 in a reasonable window is underpowered; its "no significant difference" is not evidence of no effect. Decide upfront whether the question is even answerable at your traffic.
- **Duration is sample size divided by traffic, with a floor for cyclic effects.** Run long enough to cover full weekly cycles (user behavior differs by day of week), plus any novelty/learning period. A 3-day test misses weekly seasonality; a test ending on a holiday is confounded.
- **Pre-register the analysis plan.** Decide the primary metric, the success criterion, the duration, and the analysis method before looking at results. Pre-registration prevents the post-hoc rationalization ("well, metric X is what we really cared about") that turns noise into a story.

### Distinguish Statistical Significance From Practical Significance

A statistically significant result is not necessarily worth shipping, and a non-significant result is not necessarily no effect:

- **Statistical significance answers "is the effect real (unlikely due to chance)?"** At α=0.05, a significant result has a 5% false-positive rate *per test* — meaning roughly one in twenty true-null tests will appear significant. Significance is a threshold, not a measure of importance.
- **Practical significance answers "is the effect large enough to matter?"** A 0.01% lift in click-through may be statistically significant with enough traffic and commercially irrelevant. Define the minimum effect worth shipping *before* the test; a significant result below that threshold is not a ship decision.
- **The confidence interval is more useful than the p-value.** It shows the range of plausible effect sizes. A significant result whose interval is [0.01%, 0.03%] is real but tiny; a non-significant result whose interval is [−0.5%, +2%] is inconclusive (could be a meaningful gain or a small loss), not "no effect."
- **Beware a "significant" result from a huge sample.** At massive scale, even negligible differences are statistically significant. Significance without practical magnitude leads to shipping changes that move a metric by an amount no user or the business notices.

### Correct For Multiple Comparisons Or You Will Ship Noise

Every metric and every segment you test is a separate hypothesis, and each carries its own false-positive probability. Test enough of them and some will appear significant by chance alone:

- **The family-wise error rate inflates fast.** At α=0.05 per test, testing 14 independent metrics gives a ~50% chance that at least one is falsely significant. Testing dozens of segment × metric combinations guarantees false positives.
- **Pre-designate one primary metric.** The ship decision rests on the pre-registered primary metric; other metrics are exploratory. This keeps the false-positive rate bounded for the decision that matters.
- **Apply correction for secondary/exploratory analyses.** If you act on multiple metrics, correct (Bonferroni, Benjamini-Hochberg) or treat them as hypothesis-generating, not confirmatory. An uncorrected "we found a winning segment" is very likely noise.
- **Beware segment chasing (subgroup analysis).** Slicing results by segment until something is significant ("the variant wins for mobile users on Tuesday") is multiple testing in disguise. Pre-specify the segments you care about, and treat others as exploratory.

### Ensure Validity: Randomization, Sample-Ratio Mismatch, And Interference

A test's conclusion is only as trustworthy as its validity. Three threats invalidate results silently:

- **Verify randomization and check sample-ratio mismatch (SRM).** If assignment is truly random, the arm sizes should match the intended split (e.g., 50/50) within sampling error. A significant mismatch (e.g., 52/48 when you expected 50/50) signals a bug in assignment, tracking, or exposure — and invalidates the comparison. Always run an SRM check; a failing SRM means the result is uninterpretable regardless of the p-value.
- **Guard against interference (network effects).** A/B tests assume stable unit treatment value — one user's treatment does not affect another's control outcome. This fails for two-sided markets (a treatment that increases rider demand affects driver availability for control riders), social features (a treatment changes what a user's friends see), or shared resources. Where interference is likely, use cluster-randomized or geo-experiments, not user-level A/B.
- **Confirm the treatment was actually delivered.** Exposure bugs (a feature flag that did not roll out, a client that did not receive the variant) produce a "no effect" that is actually "no treatment." Verify exposure rates and that the variant behaved as intended before interpreting metrics.

### Do Not Peek And Stop Early — Or Use A Valid Sequential Method

Monitoring a test daily and stopping the moment the metric crosses significance is the most common way teams ship noise. The reason is mathematical: each look is a chance to cross the threshold by chance, and repeated looks inflate the false-positive rate far above the nominal α:

- **Fixed-horizon testing: decide duration upfront, look once.** The clean approach: compute the duration, run to completion, then evaluate. No early stopping.
- **If you must monitor continuously, use a valid sequential method.** Sequential testing (always-valid p-values, group sequential designs, mSPRT) adjusts thresholds to allow continuous monitoring without inflating false positives. Use these if the business needs early stopping; do not apply fixed-horizon p-values to peeked data.
- **Novelty and learning effects distort early results.** Users react to change (novelty effect: a new layout spikes engagement briefly; or change aversion: it dips briefly). Early results often do not reflect the steady-state effect; this is a reason to run to the planned duration, not to stop on an early spike.

### Read Results Conservatively And Replicate When Stakes Are High

A single experiment is one estimate with uncertainty. For high-stakes decisions, treat it as evidence, not proof:

- **Look at the full picture, not one metric.** Did the primary metric improve within the practical-significance threshold? Did guardrail metrics hold (no regression in latency, retention, errors)? Did the effect hold across key segments, or only in aggregate (or only in one segment)?
- **Check for sign reversals across segments (Simpson's paradox).** A variant can win in aggregate and lose in every segment (or vice versa) when segment composition differs between arms — a sign of SRM or interference, not a real effect.
- **Replicate for high-stakes decisions.** A follow-up test on a different time window or population that reproduces the effect is far stronger evidence than a single significant result. For decisions that are costly to reverse, replication is worth the additional time.

## Common Traps

### Running An Underpowered Test And Reading "No Effect"

A test too small to detect the real effect, whose non-significant result is misread as "the variant has no effect" rather than "we could not measure it." Compute sample size upfront; if unreachable, the question is not answerable at current traffic.

### Peeking And Stopping On First Significance

Monitoring daily and stopping when the metric crosses p<0.05, inflating false positives well above 5%. Use a fixed horizon with one look, or a valid sequential method.

### Multiple Testing Without Correction

Testing many metrics or segments and acting on the significant ones, guaranteeing false positives by chance. Pre-designate a primary metric; correct or treat others as exploratory.

### Confusing Statistical With Practical Significance

Shipping a variant because a tiny effect was statistically significant at massive scale, when the lift is commercially irrelevant. Define the minimum practical effect upfront and read the confidence interval.

### Ignoring Sample-Ratio Mismatch

Failing to check whether arm sizes match the intended split, so an assignment or tracking bug silently invalidates the comparison. Always run an SRM check; a failing SRM voids the result.

### Interference In Two-Sided Markets Or Social Features

User-level A/B in a setting where one user's treatment affects another's outcome, violating the no-interference assumption. Use cluster or geo-randomization where interference is likely.

### Stopping Early Due To Novelty Or Change Aversion and simpson's Paradox In Aggregated Results

Killing or shipping based on an early spike or dip that is a transient reaction to change, not the steady-state effect. Run to the planned duration to clear novelty/learning effects.

An aggregate effect that reverses across segments due to composition differences, pointing the decision the wrong way. Slice by key segments and check for sign reversals.

### Post-Hoc Metric Switching and treating A Single Significant Result As Proof

Changing the "primary" metric after seeing results to the one that happened to win, turning noise into a story. Pre-register the analysis plan and stick to it.

Acting on one experiment's significant metric as definitive, without checking guardrails, segments, or replication. Read conservatively; replicate for high-stakes decisions.

## Self-Check

- [ ] Sample size and duration were computed before the test via power analysis (minimum detectable effect, baseline variance, α, power), and if the required sample is unreachable, the test is acknowledged as underpowered rather than run as-is.
- [ ] Duration covers full weekly cycles and any novelty/learning period, and the analysis plan (primary metric, success criterion, duration, method) was pre-registered before looking at results.
- [ ] Statistical and practical significance are distinguished: a minimum practically-significant effect was defined upfront, and the confidence interval (not just the p-value) is read to judge both reality and magnitude.
- [ ] A single primary metric drives the ship decision; secondary metrics and segment analyses are corrected (Bonferroni/Benjamini-Hochberg) or treated as exploratory, not as confirmatory.
- [ ] Validity was checked: sample-ratio mismatch test passed (arm sizes match the intended split within sampling error), randomization/exposure verified (the treatment was actually delivered), and interference considered (cluster/geo-randomization used where user-level A/B is invalid).
- [ ] No peeking-and-stopping on fixed-horizon p-values; either a single end-of-test look, or a valid sequential method if continuous monitoring is required.
- [ ] Results were read conservatively: primary metric within the practical threshold, guardrails held, key segments checked for sign reversals (Simpson's paradox), and high-stakes decisions replicated.
- [ ] The highest-risk cases were verified — an underpowered "no effect," peeking inflation, uncorrected multiple testing, an SRM voiding the result, interference in a two-sided market, and a novelty-spike early stop — not only the single significant metric that looks like a win.
