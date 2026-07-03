---
name: analysis_and_decision_from_results.md
description: Use when the agent is analyzing experiment results, computing statistical significance and confidence intervals, deciding whether to ship, iterate, or kill a feature, or communicating experiment outcomes and their uncertainty to stakeholders.
---

# Analysis And Decision From Results

Once an experiment finishes, the work is to turn numbers into a decision and to communicate that decision honestly. This is not the high-level interpretation of whether metrics moved; it is the statistical and judgmental craft of reading significance, intervals, and segments, then committing to ship, iterate, kill, or investigate, and recording what was learned.

The judgment problem is that results arrive as a seductive summary: a lift, a p-value, a green check. Agents tend to read significance as truth, ignore the confidence interval that shows the plausible range, mine segments until something looks good, and declare victory or defeat without examining novelty effects, concurrent experiments, or whether the effect is large enough to matter. The harm is features shipped on false positives, features killed on underpowered non-results, and a decision record so thin that no one can learn from the experiment later.

Use this skill before reading out an experiment, computing significance, reviewing segment cuts, or making the ship decision. The goal is to prevent over-reading weak significance, ignoring uncertainty, fishing in segments, and recording a verdict without the learning that justifies it.

## Core Rules

### Distinguish Statistical Significance, Practical Significance, And Power

Three different questions are often collapsed into one. Statistical significance asks whether the observed difference is unlikely under the null hypothesis. Practical significance asks whether the difference is large enough to be worth shipping. Power asks whether the experiment could have detected the effect you cared about, given its size.

A result can be statistically significant and practically meaningless when the sample is huge and the effect is trivially small. A result can be practically important and statistically invisible when the experiment was underpowered. Before deciding, state all three: the effect size and its confidence interval, whether that size matters for users or the business, and whether the test had the power to detect the MDE you set at design time. A decision based on only the p-value ignores two of the three.

### Read Confidence Intervals, Not Just Point Estimates And P-Values

A point estimate is a single value; a confidence interval is the range of plausible values consistent with the data. The interval carries the information the point estimate hides: how precise the estimate is, whether it includes zero, and how large the effect could plausibly be at either end.

An interval that is entirely above zero supports a positive effect, but its width tells you whether you are sure it is a 1 percent lift or whether it could be anywhere from 0.2 percent to 5 percent. A wide interval straddling zero means the data is consistent with harm, no effect, and benefit all at once, and that is an inconclusive result, not a negative one. Report intervals to stakeholders so they understand the range of risk, not just the central number.

### Treat Segment Analysis As Exploration, Not Confirmation

Segment cuts are powerful for understanding who the feature helps and who it harms, but every additional cut multiplies the chance of a false positive. Looking at ten segments at alpha 0.05 almost guarantees at least one spurious significant difference.

Use segments to generate hypotheses and to check for harm in important subgroups, not to manufacture a winning segment after the overall result disappoints. If you slice many segments, apply a correction or demand a higher bar, and pre-specify the segments you care about rather than slicing until something shines. A segment result that was not hypothesized in advance is a lead to investigate, not a finding to ship on.

### Account For Novelty And Primacy Effects

Some features show a short-term spike because users are curious about something new, then regress as novelty fades. Others show an initial dip because users must learn a new interface, then improve as they adapt. Both distort a short experiment into a misleading verdict.

Novelty effects inflate early metrics and can cause a team to ship something whose benefit evaporates after a few weeks. Primacy effects depress early metrics and can cause a team to kill something that would have helped once users adjusted. Look at the trend over the experiment window, consider a longer holdout for high-stakes decisions, and explicitly name which effect you suspect and why. Do not extrapolate a first-week curve into a permanent outcome.

### Check For Interaction Effects Between Concurrent Experiments

When multiple experiments run simultaneously, each one's measured effect can be contaminated by the others. An interaction effect means the treatment's impact depends on what else the user is exposed to, and ignoring it attributes the combined behavior to a single feature.

Ask whether other experiments were live during the window, whether they shared population, and whether the features plausibly interact on the same funnel. If a checkout experiment ran at the same time as a pricing experiment, the checkout lift may be partly the pricing effect. Where interactions are plausible, run mutually exclusive experiments or analyze the overlap explicitly rather than assuming independence.

### Recognize When An Inconclusive Result Is Informative

An inconclusive result is not a failure; it is information, if read correctly. It tells you the effect, if it exists, is smaller than your MDE, which itself informs whether the feature is worth the cost of shipping and maintaining.

Distinguish "no significant difference" from "no effect." The former means the data did not rule out a range of possibilities; the latter is a claim the experiment may not have had the power to support. An inconclusive result can justify not shipping when the plausible effect is too small to matter, or it can justify a follow-up with more power, a more sensitive metric, or a different design. Record which interpretation applies and why, rather than defaulting to kill.

### Use An Explicit Ship, Iterate, Kill, Or Investigate Framework

Force the decision into one of four buckets rather than letting it drift into "ship, I guess." Ship means the result meets the bar on the primary metric without breaking guardrails. Iterate means the direction is promising but the feature needs changes before it earns full rollout. Kill means the result is negative, the guardrails broke, or the plausible effect is too small to justify the cost. Investigate means the result is contradictory, surprising, or contaminated and more analysis is needed before any commitment.

Each bucket has a different next action and a different cost. Ship and kill close the loop; iterate and investigate keep it open. Be honest about which one the evidence supports, and resist the pressure to round investigate up to ship because the calendar wants a launch.

### Document The Decision And The Learning

An experiment that produces a decision but no recorded learning wastes the next team's ability to avoid the same question. Write down the hypothesis, the result, the confidence interval, the segments, the suspected novelty or interaction effects, the guardrail status, and the decision with its rationale.

Record what you learned about users and the product, not only what you decided to build. A killed feature with a clear writeup prevents a future team from re-running the same experiment; a shipped feature with a clear writeup tells the next analyst what the baseline now is. The decision record is the compounding asset of experimentation.

## Common Traps

### Reading Significance As Truth

A p-value under 0.05 feels like proof, but it is a probability about the data under the null, not a guarantee the effect is real or important. The trap is shipping on a single significant result without checking effect size, interval width, and whether the result replicates.

### Ignoring The Confidence Interval

Reporting only the point estimate hides the range of plausible outcomes. The trap is presenting a 2 percent lift as settled when the interval runs from -0.5 percent to 4.5 percent, leaving real downside risk invisible to the decision maker.

### Mining Segments For A Winner

When the overall result is flat, slicing by platform, region, tenure, or plan often surfaces a segment that looks significant. The trap is that with enough slices, one will appear significant by chance, and shipping on that slice is shipping on noise.

### Confusing Novelty Spike With Lasting Value

A first-week lift can look decisive. The trap is extrapolating a curiosity-driven spike into a permanent improvement and shipping a feature whose benefit fades before the next quarter's review.

### Killing On An Underpowered Non-Result

An inconclusive result from an underpowered test is read as "the feature does not work." The trap is killing a genuinely valuable feature because the experiment could not have detected its effect in the first place.

### Running Until Significant

Extending the experiment day by day until the p-value crosses the threshold inflates false positives, because each extra day is another chance to cross by chance. The trap is that the result looks rigorous while the stopping rule invalidates it.

### Ignoring Concurrent Experiments

Treating each experiment as independent when several overlapped attributes combined behavior to one feature. The trap is a clean-looking lift that is partly or wholly caused by a different change the analyst never accounted for.

### Recording A Verdict Without The Learning

A decision to ship or kill with no documented rationale forces the next team to rediscover the same answer. The trap is treating the experiment as closed rather than as an asset that compounds only if its learning is written down.

## Self-Check

- [ ] Statistical significance, practical significance, and power are each stated separately before the decision.
- [ ] The confidence interval is reported alongside the point estimate, including whether it crosses zero.
- [ ] Segment analysis is treated as exploratory, with pre-specified segments and awareness of the multiple-comparisons problem.
- [ ] Novelty and primacy effects are considered, with the trend over the window examined rather than only the aggregate.
- [ ] Concurrent experiments are identified and their potential interaction with this result is assessed.
- [ ] An inconclusive result is interpreted as a range of plausible effects, not as proof of no effect.
- [ ] The decision is explicitly categorized as ship, iterate, kill, or investigate, with the evidence supporting that bucket.
- [ ] Guardrails are checked and any breach is addressed before shipping, regardless of the primary metric.
- [ ] The stopping rule was fixed in advance and the experiment was not extended until significance appeared.
- [ ] The decision and the underlying learning are documented for future teams, including hypothesis, interval, segments, and rationale.
