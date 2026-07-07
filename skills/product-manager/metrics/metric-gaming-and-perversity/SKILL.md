---
name: metric_gaming_and_perversity.md
description: Use when the agent is evaluating whether a metric can be gamed, designing metrics that resist manipulation, diagnosing perverse incentives and Goodhart's law effects, reviewing a team's behavior for metric-driven distortion, or choosing guardrail metrics to prevent optimization from harming the product.
---

# Metric Gaming And Perversity

When a measure becomes a target, it ceases to be a good measure. This is Goodhart's law, and it is the central danger of metric-driven product management. The moment a team is evaluated on a number, the team begins to optimize the number, and optimization does not distinguish between genuine improvement and manipulation. A metric that rewarded real value before it was a target will, once it is a target, reward whatever moves it fastest, including dark patterns, inflated counts, suppressed cancellations, and low-quality actions that pad the score. The product gets worse while the dashboard gets better, and because the dashboard is what leadership watches, the decline is invisible until users leave.

The harm this skill prevents is the slow, quiet destruction of product value through metrics that reward the wrong behavior. Perverse incentives rarely look perverse at the moment they are set. A target to increase engagement sounds reasonable until it rewards notification spam. A target to reduce support contacts sounds reasonable until it rewards hiding the help center. A target to grow weekly active users sounds reasonable until it rewards counting accidental opens. The skill is anticipating how a metric will be gamed before it is adopted, detecting gaming once it begins, and designing metric systems that resist manipulation rather than inviting it.

Use this skill before adopting or revising a metric that will drive team behavior, before setting targets or quotas, when a metric is improving but the product feels worse, when reviewing whether a team's optimization is producing genuine value, or when designing guardrail and counter-metrics. The goal is to prevent the agent from recommending a metric that is easy to game, from ignoring the manipulation that follows, or from treating a rising number as evidence of success without checking what the team actually did to move it.

## Core Rules

### Anticipate Gaming Paths Before Adopting A Metric

Every metric that drives decisions will be gamed. The question is not whether gaming will happen but how, and whether the gaming paths cause harm. Before adopting a metric, enumerate the ways a team could move it without creating value.

For each candidate metric, ask:

- what is the fastest way to move this number;
- does that fast path create genuine value or merely inflate the count;
- could the number rise through a change users would experience as harmful, such as more notifications, harder cancellation, or default opt-ins;
- could the number rise by redefining what counts, such as counting accidental interactions or low-quality actions;
- could the number rise by shifting the mix toward lower-quality users or sessions.

If the fastest path to moving the metric is also a path that harms users, the metric will be gamed toward harm, because the team is under pressure to move it and the harmful path is the easiest. Anticipate this before adoption, not after the harm appears.

### Distinguish Value Creation From Metric Manipulation

A metric can rise for two reasons: the underlying value increased, or the measurement was manipulated. These look identical on the dashboard but are opposite in substance. The discipline is to separate them whenever a metric is improving.

Check what the team actually did to move the number:

- did user behavior genuinely change, or did the counting change;
- are the gains concentrated in low-quality actions that were reclassified to count;
- did the improvement come from a product change users would describe as beneficial;
- are there signs of dark patterns, such as harder opt-out, more aggressive prompts, or suppressed friction;
- does the improvement survive when measured by an independent metric the team does not control.

When a metric improves, ask what would have to be true for the improvement to be genuine, and check whether that is true. A rising number without a plausible value-creating mechanism is a signal to investigate, not to celebrate.

### Recognize The Selection And Mix-Shifting Failure

A common and subtle form of gaming is improving a metric by changing who is counted, rather than by improving outcomes for anyone. If a team increases the average by acquiring only high-value users, the average rises even though no individual user is better off. If a team reduces churn by redefining who counts as churned, the rate improves without anyone staying longer.

Guard against mix-shifting and redefinition:

- check whether the denominator or population changed alongside the numerator;
- decompose aggregate metrics into cohorts to see whether gains are real or compositional;
- watch for definitions that quietly narrowed, such as excluding the users most likely to churn;
- compare like-for-like cohorts rather than period-over-period aggregates that hide mix changes.

A metric that improves only because the counted population shifted is not improvement; it is measurement drift. The users who left the count are still leaving; they are just invisible.

### Design Metrics That Require Genuine User Value

The strongest metrics are those that can only be moved by creating real value for users, because they require a genuine action that produces a genuine outcome. A completed transaction, a return visit driven by choice, a document another person edits, a problem resolved without contacting support, these cannot be faked through a dark pattern, because they require the user to actually do something meaningful.

Favor metrics with these properties:

- they require a deliberate, non-accidental user action;
- they produce an outcome the user would describe as valuable;
- they cannot be moved by a single interface tweak or default change;
- they correlate with retention or other independent value signals;
- they are resistant to inflation through low-quality or accidental interactions.

When a candidate metric can be moved by a trick, pair it with a counter-metric that punishes the trick, or replace it with a metric that requires genuine value. The goal is to make the easy path and the valuable path the same path.

### Pair Every Target With Guardrail And Counter-Metrics

A single metric, once targeted, will be optimized at the expense of everything it does not capture. The defense is to define, before adoption, the metrics that must not degrade while the target improves. These counter-metrics define the boundary of acceptable optimization.

Choose counter-metrics that are the natural failure mode of the target:

- if the target is engagement, counter-metrics include churn, unsubscribes, complaints, and accidental actions;
- if the target is conversion, counter-metrics include refund rate, early cancellation, and support contacts;
- if the target is content creation, counter-metrics include quality signals, moderation load, and creator burnout;
- if the target is activation speed, counter-metrics include long-term retention of activated users.

Counter-metrics must be agreed before the target is set, not defined reactively after harm appears. Defining them under pressure turns them into excuses or blame rather than constraints. The counter-metrics are what make a target safe to optimize.

### Detect Quota-Driven Distortion

When a metric becomes a quota or a performance target tied to compensation or promotion, the gaming pressure intensifies dramatically. Quotas do not merely invite optimization; they demand it, because the consequences of missing the number are real and personal. Under quota pressure, even well-intentioned people will find the manipulation paths, because the alternative is personal cost.

Be especially vigilant when a metric is tied to:

- individual or team performance reviews and bonuses;
- promotion and compensation decisions;
- public leaderboards or rankings;
- contractual obligations to customers or partners.

Under quota pressure, increase the guardrails, audit the gaming paths more frequently, and watch for the subtle distortions that quotas produce, such as timing actions to hit the period boundary, reclassifying borderline cases, or suppressing the actions that would lower the score.

### Watch For The Metric That Rewards Harming The User

The most dangerous perverse incentive is a metric that rises when the user is harmed. Engagement that rises because users are confused and clicking around. Session count that rises because the app keeps logging people out. Support contacts that fall because the help is hidden. Revenue that rises because cancellation is obstructed. These metrics reward the exact behavior that drives users away, and because the dashboard looks good, the harm continues until the users are gone.

When a metric is improving, explicitly ask: would a user who contributed to this improvement describe their experience as better or worse? If the answer is worse, the metric is perverse, and improving it is destroying the product. This check is uncomfortable because it questions success, but it is the check that catches the most damaging gaming.

### Re-Examine Metrics After Context Changes

A metric that resisted gaming when adopted may become gameable after the product, audience, or competitive context changes. A metric validated against one user base may reward manipulation against another. Re-examine the gaming paths periodically, especially after changes to onboarding, pricing, acquisition mix, or competitive positioning.

Gaming paths emerge over time as the team learns the system. What resisted manipulation in year one may be thoroughly gamed by year three, because the team has had time to discover the shortcuts. Treat metric integrity as an ongoing maintenance task, not a one-time design decision.

## Common Traps

### Adopting A Metric Without Enumerating Gaming Paths

Choosing a metric because it measures something valuable, without asking how it could be moved without creating value, invites the exact manipulation that destroys the value the metric was meant to capture.

### Treating A Rising Number As Success

A metric that improves through manipulation looks identical to genuine improvement on the dashboard. Celebrating the number without checking the mechanism rewards the gaming and encourages more of it.

### Defining Counter-Metrics Only After Harm

Waiting until a metric has been gamed and users have complained to define the counter-metric turns the guardrail into a post-hoc excuse rather than a constraint. Counter-metrics must precede the target.

### Ignoring Mix-Shifting And Redefinition

Improving an average or rate by changing who is counted produces apparent improvement without any real gain. Aggregates that hide cohort changes are a common hiding place for gaming.

### Tying Metrics To Quotas Without Strengthening Guardrails

When a metric becomes a quota tied to compensation, the gaming pressure multiplies. Failing to increase guardrails and auditing under quota pressure guarantees distortion.

### Keeping A Metric That Rewards Harming The User

A metric that rises when users are confused, trapped, or obstructed will, once targeted, drive the team to confuse, trap, and obstruct. Clinging to the metric because the dashboard looks good accelerates the decline.

### Assuming A Metric Stays Game-Resistant

A metric that resisted manipulation at adoption may become thoroughly gamed as the team learns the shortcuts and the context changes. Metric integrity requires ongoing re-examination.

### Optimizing The Proxy And Losing The Outcome

When the metric is a proxy for an outcome, optimizing the proxy can decouple from the outcome entirely. The team hits the target while the underlying value erodes, because the proxy stopped predicting the result.

## Self-Check

- [ ] Before adopting the metric, the fastest gaming paths were enumerated, and the harmful ones were assessed.
- [ ] When the metric improves, the mechanism is checked to distinguish genuine value creation from manipulation.
- [ ] Aggregate improvements are decomposed into cohorts to detect mix-shifting and redefinition.
- [ ] The metric requires a genuine user action that produces real value and cannot be moved by a single dark-pattern tweak.
- [ ] Counter-metrics were defined before the target was set, chosen as the natural failure mode of the target.
- [ ] Where the metric is tied to quotas or compensation, guardrails and auditing were strengthened accordingly.
- [ ] The metric was checked for the perverse case where improvement corresponds to user harm rather than benefit.
- [ ] Gaming paths are re-examined periodically, especially after changes to product, audience, or context.
- [ ] No rising number was celebrated without confirming that a user who contributed would describe their experience as better.
- [ ] The metric set includes independent signals the team does not control, to catch decoupling between proxy and outcome.
