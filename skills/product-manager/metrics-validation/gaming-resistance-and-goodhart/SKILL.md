---
name: gaming_resistance_and_goodhart.md
description: Use when the agent is evaluating whether a metric can be gamed or manipulated, anticipating unintended behaviors a metric will incentivize, applying Goodhart's law, or hardening a metric against perverse incentives before it drives decisions.
---

# Gaming Resistance And Goodhart

When a measure becomes a target, it stops being a good measure. This is Goodhart's law, and it is the single most important idea in metric design for a product manager. A metric that is observed casually reveals behavior; the same metric tied to rewards, roadmaps, or performance reviews changes behavior, often in directions nobody intended. The number does not improve because the product improved, it improves because people learned how to move the number.

The harm this skill prevents is building incentives that look aligned but are not. A support team measured on ticket closure time will close tickets fast and poorly. An onboarding team measured on completion rate will push users through steps that do not help them. A growth team measured on signups will acquire low-intent users who churn. Each team looks successful on its metric while the product gets worse, because the metric rewarded a shortcut to the number rather than the outcome the number was meant to represent.

Use this skill before tying a metric to goals, compensation, team scorecards, or public dashboards, and whenever a metric is about to become a target rather than an observation. The work is to anticipate how the metric could be gamed, harden it against the easiest manipulations, and pair it with guardrails that catch perverse optimization.

## Core Rules

### Treat Any Tied Metric As Already Gamed

The moment a metric influences decisions, rewards, or status, assume someone will find the shortest path to improving it. This is not cynicism about people; it is realism about incentives. A metric observed passively and a metric used as a target behave like different metrics, because the target creates pressure to optimize the proxy rather than the underlying outcome.

Before adopting a metric as a target, run a gaming pre-mortem: ask how a person under pressure to improve this number could do so without improving the product. List the concrete shortcuts. If the shortcuts are easy and undetectable, the metric is not ready to be a target.

### Separate The Proxy From The Outcome It Serves

Every target metric is a proxy for a deeper outcome. Signups proxy for valuable users, completion rate proxies for successful onboarding, response time proxies for helpful support. Gaming happens at the gap between proxy and outcome, because the proxy can be moved without moving the outcome.

For each target metric, name the outcome it is meant to serve, and keep both visible. When the proxy and the outcome start to diverge, the proxy has been gamed or has decayed. A target that reports only the proxy hides this divergence, so always track the outcome alongside it even if the outcome is slower or noisier.

### Add Guardrail Metrics To Catch Perverse Optimization

A single target metric invites optimization at the expense of everything not measured. The defense is a small set of guardrail metrics that monitor the dimensions a team could sacrifice to hit the target. If the target is signup volume, guardrails might be signup quality, cost per valuable user, and early churn. If the target is response time, guardrails might be resolution quality and reopen rate.

Choose guardrails that capture the most likely collateral damage, and treat a target hit accompanied by a guardrail breach as a failure, not a success. Without guardrails, a team can hit every goal while the product deteriorates, because the deterioration shows up only in the unmeasured dimensions.

### Anticipate The Easiest Manipulation First

Gaming follows the path of least resistance. The first manipulation to appear is the one that is cheapest, safest, and most invisible. Changing definitions, reclassifying users, excluding failures, shifting time windows, and targeting easy subpopulations are all low-effort, low-risk ways to move a number. Anticipate these before worrying about sophisticated cheating.

For each metric, ask which exclusions or redefinitions would most easily inflate it, and lock those down in the definition. A metric whose denominator or population can be quietly shrunk is a metric that will be quietly shrunk. Make the boundaries explicit and versioned so silent changes are visible.

### Prefer Composite And Paired Targets Over Single Numbers

A single number is the easiest to game because there is only one thing to optimize. Pairing two metrics that pull against each other makes gaming harder, because improving one at the expense of the other becomes visible. Volume paired with quality, speed paired with accuracy, and reach paired with retention each force tradeoffs into the open.

Composite targets, such as requiring both a floor and a ceiling to hold, are more resistant than a single directional goal. The goal is not to make gaming impossible but to make the cheapest manipulation stop paying off, so that effort flows toward the real outcome.

### Watch For Goodhart Decay Over Time

A metric that was hard to game at first becomes easier as teams learn its weaknesses. Goodhart's law is dynamic: the longer a target is in place, the more the proxy and the outcome diverge, because optimization pressure accumulates. A metric that tracked the outcome well in year one may be mostly gamed by year three.

Schedule periodic re-validation of target metrics against their outcomes. If the relationship has weakened, either redefine the metric or retire it. Holding a stale target in place because it is familiar is itself a form of gaming, by the organization against its own goals.

### Distinguish Measurement From Incentive

Not every metric needs to be a target. Many metrics are valuable as observations that inform judgment without driving rewards. The gaming risk applies specifically to metrics tied to consequences. Keeping a metric as an observation preserves its diagnostic value while avoiding the incentive distortion.

Before promoting an observation metric to a target, ask whether the outcome it tracks is important enough to bear the distortion that targeting introduces. Some metrics are better left as signals than made into goals.

## Common Traps

### Optimizing The Proxy And Declaring Victory

When a team hits the target metric but the underlying outcome has not improved, the proxy has been gamed. Declaring victory on the proxy while the product is unchanged or worse is the classic Goodhart failure. Always confirm the outcome moved, not just the number.

### Shrinking The Denominator To Grow The Rate

Rate metrics are especially easy to game by removing users from the denominator. A conversion rate improves when low-intent users are excluded, even if no user's experience improved. Lock the population definition so the denominator cannot be quietly edited.

### Rewarding Activity That Looks Like Progress

Metrics that count actions reward motion rather than results. More tickets closed, more steps completed, more messages sent can all rise while quality falls. Pair activity metrics with outcome metrics so motion without result is visible.

### Ignoring Guardrails Until They Are Breached

Guardrails only work if they are checked and enforced. A team that hits its target while a guardrail slips often gets credit for the win and silence on the slip. Define in advance what a guardrail breach means for the target, so enforcement is not negotiated after the fact.

### Assuming People Will Not Game A Metric They Control

The closer a person is to the lever that moves a metric, the more pressure they feel to pull it. Self-reported metrics, self-graded quality, and team-owned definitions are the most vulnerable. Independent measurement and shared definitions reduce this risk.

### Keeping A Stale Target Because It Is Familiar

A metric that once tracked the outcome can drift into pure gaming territory over time, but it is often kept because changing targets is politically costly. Familiarity is not validity. Retire or redefine targets whose relationship to the outcome has decayed.

## Self-Check

- [ ] A gaming pre-mortem was run, listing concrete shortcuts that could move the metric without improving the product.
- [ ] The outcome the target serves is named and tracked alongside the proxy, so divergence is visible.
- [ ] Guardrail metrics monitor the most likely collateral damage, and a guardrail breach is defined as a failure of the target.
- [ ] The cheapest, most invisible manipulations, such as denominator shrinking and definition shifts, were anticipated and locked down.
- [ ] The target is paired or composite rather than a single number, so optimizing one dimension at the expense of another is exposed.
- [ ] The metric is scheduled for periodic re-validation against its outcome to catch Goodhart decay over time.
- [ ] A deliberate choice was made about whether the metric should be a target or remain an observation, based on whether the outcome is important enough to bear the distortion.
- [ ] Population, denominator, window, and exclusions are explicit and versioned so silent changes are detectable.
- [ ] Activity metrics are paired with outcome metrics so motion without result is not rewarded.
- [ ] A target hit was confirmed by checking that the underlying outcome moved, not only the proxy.
