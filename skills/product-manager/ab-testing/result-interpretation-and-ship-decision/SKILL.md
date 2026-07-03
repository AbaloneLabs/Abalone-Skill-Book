---
name: result_interpretation_and_ship_decision.md
description: Use when the agent is reading experiment results, distinguishing statistical from practical significance, guarding against peeking and multiple comparisons, or deciding whether to ship based on experiment data.
---

# Result Interpretation And Ship Decision

The readout is where experiments are won or lost in practice, and it is where disciplined design is most easily undone by motivated reasoning. A clean design produces clean numbers, but those numbers still have to be interpreted honestly, and the ship decision has to rest on more than a single p-value.

The judgment problem is that at readout time the team already has a preference. Someone built the feature, someone wants the win, and the calendar wants the slot back. In that environment, a borderline result becomes a green light, a guardrail regression becomes a footnote, and a non-significant primary metric gets replaced by a significant secondary one that was never declared. Agents tend to conflate statistical significance with practical importance, to peek and stop early when results look good, to slice segments until something wins, and to report the point estimate while hiding the confidence interval. The harm is features that ship on noise, features that kill real value on inconclusive evidence, and a slow erosion of trust in experimentation itself.

Use this skill at readout time, when the team is about to decide ship, hold, or kill. The goal is to prevent motivated interpretation from overriding the pre-registered plan, and to make the ship decision a real judgment call that weighs effect size, guardrails, cost, and risk rather than a reflex triggered by a green dashboard.

## Core Rules

### Honor The Pre-Registered Primary Metric First

The ship decision rests on the primary metric that was declared before launch, not on whichever metric looks best now. If the primary moved as predicted and the guardrails held, the result supports the hypothesis. If the primary did not move, the result does not support it, even if a secondary metric did.

This rule exists because the primary was chosen when no one knew the answer. Switching to a favorable secondary after results are visible is selecting on the outcome, which inflates false positives. If the team wants to act on a secondary result, that is allowed, but it must be labeled as exploratory and treated as a hypothesis for the next experiment, not as validation of this one.

### Separate Statistical Significance From Practical Significance

A statistically significant result is one unlikely under the null hypothesis; a practically significant result is one large enough to matter. With enough traffic, a trivially small effect becomes statistically significant, and with too little traffic, a large effect is not. Confusing the two ships features that move a metric by an amount no one cares about, or kills features whose effect was real but unmeasurable.

Always report the effect size and its confidence interval alongside the p-value. Ask whether the lower bound of the interval exceeds the minimum effect that would justify the cost and risk of shipping. A result that is significant but whose entire confidence interval sits below the meaningful threshold is a technical win and a practical irrelevance. Conversely, a non-significant result whose confidence interval includes large effects is inconclusive, not negative.

### Guard Against Peeking And Early Stopping

Peeking at results and stopping the moment they look good biases toward false positives, because every look is another chance to cross the threshold by chance. Early data is volatile, and a promising first day often regresses. The discipline of a fixed runtime or a pre-specified sequential rule exists precisely to control this.

If the team must monitor continuously, use a sequential method such as always-valid p-values or alpha-spending that adjusts for multiple looks. If no such method is in place, the only honest rule is to run to the pre-registered sample and decide once. Stopping early because the result looks great, or extending because it looks close, both invalidate the significance calculation.

### Check Guardrails Before The Primary

A win on the primary metric that comes with a regression on retention, latency, revenue, support volume, or safety is not automatically a win. Guardrails exist to catch harm, and a guardrail breach should trigger escalation and a hold, not a footnote.

Review the full guardrail panel before celebrating the primary. Decide in advance which guardrails are hard blockers that stop the ship regardless of the primary, and which are soft signals that require investigation. A feature that lifts conversion by degrading page load time is shipping debt disguised as a win. The guardrail rule must be enforced even when the primary result is exciting, because that is exactly when it is most likely to be waived.

### Beware Multiple Comparisons In Segments And Metrics

Slicing results by country, device, tenure, or cohort after the fact multiplies the chance of finding a spurious significant segment. With enough slices, something will always appear significant by chance, and the team will gravitate to the slice that tells the story they want.

Pre-declare the segments you intend to report, and treat any segment that becomes interesting only after results are visible as hypothesis-generating. If you must explore many segments, apply a multiple-comparison correction and lower your confidence in any single finding. The honest framing is that segment findings from an exploratory analysis need confirmation in a follow-up test before they drive a ship decision.

### Read The Confidence Interval, Not Just The Point Estimate

The point estimate is the best guess; the confidence interval is the range of effects consistent with the data. A wide interval that straddles zero means the experiment could not distinguish a large positive from a large negative effect, regardless of where the point estimate sits. Reporting only the point estimate hides this uncertainty.

Look at both bounds. If the upper bound is a large gain and the lower bound is a large loss, the test was underpowered for this question and the result is inconclusive. If the entire interval is in positive territory but narrow and near zero, the effect is real but trivial. The interval tells you what the data can and cannot support, which is the actual input to the ship decision.

### Make The Ship Decision A Judgment Call, Not A Threshold

Shipping is not a mechanical consequence of p less than 0.05. It is a decision that weighs the effect size, the guardrail status, the implementation cost, the maintenance burden, the strategic fit, and the risk of reversal. A significant result on a low-stakes, easily reversible feature warrants a different decision than the same result on a hard-to-reverse platform change.

State the decision and the reasoning together. Include the effect size and interval, the guardrail review, the cost of shipping and of not shipping, and the conditions under which you would roll back. A ship decision that cannot articulate why the result justifies the cost is not yet a decision; it is a reflex.

## Common Traps

### Switching To The Winning Secondary

When the primary metric is flat, the team often finds a secondary that moved and reports that as the result. The trap is that this is selection on the outcome, which inflates false positives and lets every experiment claim a win somewhere. The primary is the decision metric; secondary findings are hypotheses for next time.

### Treating Significance As Importance

With large traffic, a 0.1 percent lift is statistically significant, and the team ships it as a victory. The trap is that the feature cost more to build and maintain than the lift is worth, and the practical impact is invisible to users. Always compare the effect size to the threshold that would justify the cost.

### Peeking And Stopping On A Good Day

Early results are volatile, and a strong first day invites early stopping. The trap is that stopping on a peak biases toward false positives, and the feature that looked great on day one often regresses by day seven. Without a sequential method, run to the planned sample.

### Waiving A Guardrail Because The Primary Won

A regression in latency or retention is easy to rationalize when the primary metric is green. The trap is that the harm is real and often compounds, while the primary win may be noise. Enforce the guardrail rule especially when the primary result is exciting.

### Fishing Segments Until Something Wins

With enough slices, a significant segment always appears. The trap is that the team reports the winning country or cohort as if it were the finding, when it is a chance pattern among many. Pre-declare segments or treat exploratory slices as unconfirmed.

### Reporting The Point Estimate Without The Interval

A point estimate of plus two percent sounds like a win; the interval of minus one to plus five percent tells a different story. The trap is that hiding the interval hides the real uncertainty and makes the result look more decisive than the data supports. Always show the interval.

### Killing A Feature On An Inconclusive Result

A non-significant result from an underpowered test is not evidence of no effect. The trap is reporting "no impact" and killing the feature, when the confidence interval may include a large positive effect. Report the interval and the MDE, and distinguish inconclusive from negative.

## Self-Check

- [ ] The ship decision rests on the pre-registered primary metric, and any secondary finding is labeled exploratory.
- [ ] The effect size and confidence interval are reported alongside the p-value, and the lower bound is compared to the meaningful threshold.
- [ ] Statistical significance is not confused with practical significance; a trivial significant effect is not shipped as a win.
- [ ] No peeking or early stopping occurred, or a valid sequential method was used throughout.
- [ ] The full guardrail panel was reviewed before the primary, and hard-blocker guardrails were enforced even when the primary won.
- [ ] Segments were pre-declared, and any exploratory segment finding is treated as unconfirmed and hypothesis-generating.
- [ ] The confidence interval was examined for width and direction, not just the point estimate.
- [ ] A non-significant result is reported as inconclusive or negative based on the interval and MDE, not as automatic proof of no effect.
- [ ] The ship decision weighs effect size, guardrails, cost, maintenance, strategic fit, and reversibility, and is not a reflex to a green dashboard.
- [ ] The decision write-up states the rollback conditions and the reasoning, not just the verdict.
