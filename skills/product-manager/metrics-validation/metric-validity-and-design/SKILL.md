---
name: metric_validity_and_design.md
description: Use when the agent is choosing or designing a product metric, checking whether a metric actually measures the outcome it claims to, selecting proxies, or reviewing whether a proposed metric is valid, reliable, and sensitive enough to guide decisions.
---

# Metric Validity And Design

Choosing a metric is choosing a theory of what matters, compressed into a number. A product manager rarely lacks metrics; the danger is using a number that does not actually track the outcome it is supposed to represent. A metric that looks precise can be a poor proxy for value, a metric that is easy to measure can be insensitive to real change, and a metric that is reliable today can drift as the product grows. The math is trivial; the judgment about what the number means is hard.

The harm this skill prevents is optimizing the wrong signal with full confidence. Teams will move toward whatever is measured, so a poorly chosen metric does not merely misreport progress, it actively steers effort away from the outcome that matters. A login-count metric that rewards re-logins over reach, a session-length metric that rewards slow pages over engagement, or a signup metric that rewards low-intent users all produce work that looks successful and is not.

Use this skill before adopting a new metric, replacing an old one, choosing a proxy for an outcome that cannot be measured directly, or defending why a number should drive decisions. The central question is always whether the metric is a true signal of the outcome, not merely a number that is easy to compute.

## Core Rules

### Start From The Outcome, Not The Available Data

Define the outcome you care about before looking at what is easy to measure. Outcomes are things like retained value, successful task completion, trust, revenue durability, or reduced friction. Data is the raw event stream. A valid metric is one chosen to represent the outcome, then checked against available data, not the reverse.

Write the outcome in plain language first: what should be true for the user and the business if the product is working. Only then ask which observable signal could stand in for that outcome. If you begin from the event log, you will inherit whatever the instrumentation happens to capture, which is rarely the thing that matters.

### Test Construct Validity Before Trusting The Number

Construct validity asks whether the metric actually represents the concept it names. "Engagement" measured as session count is not engagement; it is frequency of app opens, which may include accidental launches, background wakes, or low-value scrolling. Before adopting a metric, describe what higher or lower values should imply about user reality, then check whether that implication holds.

A useful test is to imagine two products with identical metric values and ask whether they are genuinely equivalent. If one could have a great score while delivering poor user value, the construct is weak and the metric needs sharpening or replacing.

### Separate Reliability From Validity

Reliability is whether the metric gives a stable, repeatable answer; validity is whether that answer is the right one. A metric can be highly reliable and completely invalid, like weighing the same object repeatedly on a miscalibrated scale. Teams often defend a metric because it is stable, but stability only proves consistency, not truth.

Check reliability by seeing whether the metric moves only when something real changes, and stays flat when nothing does. Check validity by comparing it against independent evidence such as qualitative research, support signals, or long-run business outcomes. Both must hold before the metric deserves to drive decisions.

### Evaluate Sensitivity To Real Change

A metric that does not move when the product improves is useless as a feedback signal, even if it is valid on average. Sensitivity is whether the metric responds to changes that should matter: a meaningful feature launch, a fix to a painful flow, a pricing change. If the metric is dominated by noise or by users unaffected by the change, it will mask real movement.

Ask how large a real improvement would need to be before the metric reflects it, and how long that lag would be. A metric with a months-long lag and high variance may be valid in principle but operationally blind. For fast feedback, pair a slow but valid outcome metric with a faster, more sensitive leading proxy.

### Choose Proxies Deliberately And Name Their Gap

Many important outcomes cannot be measured directly, so proxies are unavoidable. The rule is not to avoid proxies but to choose them deliberately and keep their gap visible. A proxy is a bet that a measurable behavior tracks an unmeasurable outcome, and that bet has a shelf life.

For each proxy, record what outcome it stands for, why it was chosen, and what would make it stop working. When the product, audience, or instrumentation changes, revisit the proxy. A proxy that was valid at small scale often decays as the user base shifts, because the relationship between the proxy and the outcome is not stable.

### Check Directionality And Floor And Ceiling Effects

Confirm the metric moves in the direction you intend and that it has room to move. A metric that has already saturated near its ceiling cannot reward further improvement and will flatten even as value grows. A metric bounded at zero can only show decline, never recovery, which biases the team toward pessimism.

Map the plausible range of the metric against real conditions. If the current value already sits at an edge, the metric cannot serve as a steering signal and should be replaced or complemented. Direction errors are subtler: a metric where higher is sometimes good and sometimes bad is dangerous because it invites misinterpretation under pressure.

### Define The Metric Precisely Enough To Reproduce

A metric that cannot be defined precisely cannot be trusted, because two analysts will compute two different numbers. Specify the population, the event definitions, the time window, the aggregation, and the exclusions. Document edge cases such as bot traffic, internal users, test accounts, and partial events.

Precision also protects against silent drift. When the definition lives only in someone's head, a query change or schema update alters the metric without anyone noticing. A written definition lets the team detect when the number changed because reality changed versus because the computation changed.

## Common Traps

### Confusing Activity With Value

Metrics that count actions, such as clicks, sessions, or logins, reward activity rather than value. A product can drive more activity while delivering less usefulness, for example by adding friction that forces extra steps. Always ask whether more of this metric means users are better off or merely busier.

### Picking The Metric That Is Easiest To Query

The metric closest to an existing event table is often the weakest signal, because instrumentation is built for debugging, not for measuring outcomes. Convenience in computation is not evidence of validity. Prefer a metric that requires more work to define but tracks the real outcome.

### Trusting Stability As Proof Of Correctness

A metric that barely moves over time feels trustworthy, but it may simply be insensitive. A broken thermometer reads a constant temperature. Distinguish a metric that is stable because the product is stable from one that is stable because it cannot detect change.

### Letting One Metric Stand For A Whole Outcome

Complex outcomes like trust, satisfaction, or engagement rarely fit in a single number. Forcing one metric to carry the full meaning invites over-optimization of whatever that metric happens to reward. Use a small set of complementary metrics that cover different facets, and resist collapsing them into one score without a clear rationale.

### Ignoring Population Composition

A metric computed across all users can move purely because the mix of users changed, not because any user's experience improved. A shift toward higher-intent acquisition channels lifts average metrics without any product change. Always segment by population before interpreting a movement as improvement.

### Defining The Metric Around The Instrumentation

When the metric is built from whatever events happen to exist, it inherits the assumptions of the logging code. Re-derive the metric from the outcome and then check whether the instrumentation can support it, rather than letting the logging dictate what is measured.

## Self-Check

- [ ] The outcome was written in plain language before any metric was chosen, and the metric was selected to represent that outcome.
- [ ] Construct validity was tested by imagining two products with identical metric values and checking whether they are genuinely equivalent in user value.
- [ ] Reliability and validity were checked separately, and stability was not mistaken for correctness.
- [ ] Sensitivity was evaluated, including how large a real change must be and how long the lag is before the metric reflects it.
- [ ] Any proxy has a recorded rationale, the outcome it stands for, and the conditions under which it would stop being valid.
- [ ] Directionality, floor, and ceiling were checked, and the metric has room to move in the intended direction.
- [ ] The metric is defined precisely enough to reproduce, including population, events, window, aggregation, and exclusions.
- [ ] The metric distinguishes activity from value, and more of it genuinely means users are better off.
- [ ] Population composition was checked so a movement is attributed to product change only when the mix is stable.
- [ ] The metric is not carrying a complex outcome alone but is paired with complementary measures where the outcome is multi-faceted.
