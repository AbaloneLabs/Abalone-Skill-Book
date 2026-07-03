---
name: north_star_and_counter_metrics.md
description: Use when the agent is choosing a north star metric, defining a product health scorecard, selecting counter-metrics and guardrail frameworks, distinguishing leading from lagging indicators, or aligning a team around a single shared measure of product value.
---

# North Star And Counter Metrics

A north star metric is meant to focus an entire product team on one expression of customer value. Done well, it sharpens prioritization, aligns engineering and growth and design around a shared definition of success, and prevents teams from optimizing separate local numbers that do not add up to a healthier product. Done badly, it becomes a slogan that everyone repeats but no one uses to make decisions, or worse, a single number that rewards gaming and quietly damages the product.

The hardest part of strategic metric frameworks is not picking a number. It is picking a number that reflects genuine value, moves before the business outcome it predicts, resists manipulation, and can be decomposed into inputs that teams can actually influence. A north star that is too abstract becomes unfalsifiable; one that is too narrow becomes a target to be gamed.

Use this skill before choosing or revising a product-level north star metric, building a north-star-input tree, defining a product health scorecard, selecting counter-metrics to prevent gaming, aligning multiple teams to one shared measure, or deciding whether the current north star should change. Ask: what single metric best expresses the value users receive, what inputs drive it, what could go wrong if a team optimizes only that number, and how will we know when the metric itself is misleading us?

## Core Rules

### Choose A North Star That Reflects Customer Value, Not Activity

A north star should measure the value users get from the product, not the effort they expend or the friction they endure. The test is whether more of the metric means a user is better off, or merely more trapped.

Distinguish value expression from activity proxies. "Sessions per user" measures activity and can rise when users are confused or stuck. "Weekly active collaborators," "listings booked," "documents shared and acted on," or "tasks completed end to end" measure value received. Ask whether a user who increased this metric would describe themselves as more successful, or merely more engaged out of necessity.

Prefer metrics expressed as a ratio or rate over raw counts where possible, because raw counts grow with the user base without indicating improving health. A north star should reward making each user more successful, not just acquiring more users.

### Favor Leading Indicators Over Lagging Outcomes

A north star that lags the business outcome by quarters is useless for steering. By the time it moves, the decisions that caused the movement are ancient history, and the team cannot learn or correct in time.

Leading indicators move before the outcome they predict. For a retention-focused product, activation quality or core-action frequency in the first week leads churn months later. For a marketplace, supply quality or match rate leads take rate and gross merchandise value. For a subscription product, onboarding completion and habit formation lead renewal.

The tradeoff is that leading indicators are noisier and less directly tied to revenue, while lagging indicators are cleaner but too slow to steer by. Choose a leading indicator that has a demonstrated relationship to the lagging outcome, and verify that relationship periodically rather than assuming it forever.

### Make The Metric Hard To Game

A good north star is one that is difficult to improve through manipulation. If a team can move the number while making the product worse for users, the metric will eventually be gamed, intentionally or not.

Test each candidate against obvious gaming paths. Could the number rise from pushing more notifications, defaulting users into flows, inflating counts with low-quality actions, hiding cancellation, or counting accidental interactions? If yes, either redefine the metric to exclude those paths or pair it with a counter-metric that punishes them.

Hard-to-game metrics usually require a genuine user action that creates real value, such as a completed transaction, a return visit driven by choice, a document that another person edits, or a problem resolved without support contact. They cannot be moved by a single dark-pattern tweak.

### Build A North-Star-Input Tree

A north star by itself is not actionable for a feature team. Decompose it into a tree of inputs that teams can influence, so that local work connects to the strategic metric through a visible causal chain.

The tree typically starts from the north star, breaks it into a small number of driver components, and then into input metrics that specific teams own. For example, a north star of "weekly successful sessions" might decompose into new-user activation, returning-user frequency, and session quality, each of which breaks into inputs like onboarding completion, core-action discovery, or load time.

Keep the tree shallow and causal, not exhaustive. Every input should have a plausible mechanism by which it moves the node above it. If a team cannot articulate how their input affects the north star, the input is probably dashboard decoration, not a lever.

### Pair The North Star With Counter-Metrics

A single north star will be optimized, and optimization creates pressure toward gaming or local harm. Counter-metrics are the metrics that must not degrade while the north star improves; they define the boundary of acceptable optimization.

Choose counter-metrics that are the natural failure mode of the north star. If the north star is engagement, counter-metrics include churn, complaints, notification unsubscribes, and accidental actions. If the north star is transactions, counter-metrics include refund rate, dispute rate, support contact, and time-to-first-value. If the north star is content creation, counter-metrics include quality signals, moderation load, and creator burnout.

Counter-metrics differ from feature-level guardrails: they are strategic and persistent, watched alongside the north star at the product level, not just during an experiment. They should be agreed before the north star is rolled out, because defining them only after a number looks bad is rationalization.

### Balance A Product Health Scorecard Across Dimensions

A product health scorecard captures the dimensions a leadership team must hold in balance: activation, retention, engagement, monetization, and referral, plus trust and quality where relevant. No single dimension should be allowed to dominate at the expense of the others.

The temptation is to let the north star absorb the scorecard, but a north star is one expression of value while the scorecard is the system of constraints around it. A team can hit the north star while retention collapses, or grow monetization while trust erodes. The scorecard exists to make those tradeoffs visible rather than hidden inside one number.

Decide which dimensions are primary, which are counter-balancing, and which are monitoring-only. Assign owners and review cadence to each, and define what movement would trigger investigation. A scorecard that no one reviews on a cadence is not a scorecard; it is a slide.

### Align Multiple Teams Without Forcing False Precision

When several teams share one north star, the alignment is valuable but the risk of false precision is high. Teams may be assigned arbitrary sub-targets of the same metric, each claiming a slice of movement that cannot actually be attributed to them.

Avoid attributing shared outcome movement to individual teams without a causal mechanism. Instead, give each team ownership of an input in the north-star-input tree, where their work has a traceable effect, and measure the shared north star at the product level. Hold teams accountable for moving their inputs and for not harming counter-metrics, not for arbitrary fractions of the top-line number.

Reconcile inputs across teams so they do not double-count the same user behavior or work at cross-purposes. Two teams optimizing different inputs that both depend on the same underlying action will appear to conflict when the action moves only once.

### Know When A North Star Should Change

A north star is not permanent. It should change when the product strategy changes, when the metric has been gamed past usefulness, when the leading relationship to the outcome has broken, or when the business has matured past the phase the metric was designed for.

Changing a north star is expensive: it resets targets, dashboards, planning rituals, and team motivation. Do not change it for convenience or to escape a number that looks bad. Change it only when the old metric no longer reflects the value the product is trying to create, and document the reasoning so the change is legible rather than arbitrary.

When changing, retain the old metric as a monitored comparison for a transition period, and re-derive the input tree and counter-metrics for the new north star rather than reusing the old framework unchanged.

## Common Traps

### Picking A North Star That Is Easy To Measure Rather Than Meaningful

Teams often choose the metric that is already in the dashboard because it is available, even if it measures activity rather than value. The trap is that an available metric feels rigorous while a meaningful one requires new instrumentation. An easy-to-measure north star will be optimized, and optimizing the wrong thing is worse than having no north star.

### Choosing A Lagging Metric Because It Is Cleaner

Revenue, retention at twelve months, and annual churn are clean and defensible, which makes them attractive north stars. The trap is that they move too slowly to steer by; a team can destroy value for three quarters before the lagging metric reflects it. Cleanliness is not the same as usefulness for steering.

### Letting The North Star Absorb All Decision-Making

When a single metric becomes the only lens, teams stop weighing dimensions the north star does not capture, such as trust, quality, or fairness. The trap is that the scorecard atrophies and harm accumulates in the unmeasured dimensions until it becomes a crisis. A north star is one expression of value, not the whole definition of product health.

### Ignoring Counter-Metrics Until Harm Appears

Counter-metrics are often defined only after the north star has been gamed and users have complained. The trap is that defining them reactively turns them into excuses or blame rather than constraints. By the time a counter-metric is added under pressure, the behavior it would have prevented is already embedded in the product.

### Forcing Every Team To Own A Slice Of The Same Outcome

Assigning each team an arbitrary percentage of the north star feels like alignment but is false precision. The trap is that the slices cannot be attributed, so teams either claim credit they cannot prove or feel accountable for movement they cannot control. This erodes trust in the metric and in the planning process.

### Treating The Input Tree As A Fixed Diagram

A north-star-input tree is built from assumptions about causality that decay over time as the product and user base change. The trap is treating the tree as permanent and continuing to optimize inputs that no longer move the north star. Re-verify the causal links periodically; an input that stopped driving the outcome is a lever connected to nothing.

### Keeping A North Star Past Its Usefulness

Teams resist changing a north star because of sunk cost in dashboards, targets, and rituals. The trap is that a metric that no longer reflects strategy becomes a constraint that prevents the team from pursuing the right work. Clinging to a familiar but outdated north star is a form of metric inertia that quietly misdirects the whole organization.

### Confusing A Scorecard With A Dashboard

A scorecard is a small set of balanced dimensions with owners and review cadence; a dashboard is a collection of charts. The trap is building an ever-growing dashboard and calling it a scorecard, which diffuses accountability and ensures no dimension is actually held in balance. More charts do not equal more strategic clarity.

## Self-Check

- [ ] The north star measures value users receive, not activity or friction they endure, and more of it genuinely means users are better off.
- [ ] The metric is a leading indicator with a demonstrated relationship to the lagging business outcome, not a clean but slow number.
- [ ] The candidate north star has been tested against obvious gaming paths, and either resists them or is paired with counter-metrics that punish them.
- [ ] A north-star-input tree decomposes the metric into inputs that specific teams own, each with a traceable causal mechanism.
- [ ] Counter-metrics are defined at the product level, chosen as the natural failure mode of the north star, and agreed before launch rather than after harm.
- [ ] A product health scorecard balances activation, retention, engagement, monetization, referral, trust, and quality, with owners and a review cadence for each dimension.
- [ ] Multiple teams are aligned through owned inputs in the tree, not through arbitrary slices of the shared outcome metric.
- [ ] The leading-to-lagging relationship and the input-tree causal links are re-verified periodically rather than assumed permanent.
- [ ] Criteria for when the north star should change are documented, and any change retains the old metric for a transition comparison.
- [ ] The north star has not absorbed the entire definition of product health, and unmeasured dimensions are still actively monitored.
