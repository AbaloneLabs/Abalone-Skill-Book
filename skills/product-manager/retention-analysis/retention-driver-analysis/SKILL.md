---
name: retention_driver_analysis.md
description: Use when the agent is analyzing what drives retention, finding the behaviors and features correlated with sticking around, distinguishing causal drivers from correlated ones, or prioritizing which retention levers to invest in.
---

# Retention Driver Analysis

Knowing that retained users behave differently from churned users is easy. Knowing which of those behaviors actually cause retention, and which are merely symptoms of users who were going to stick around anyway, is the hard and valuable problem. Retention driver analysis is the work of separating the levers a product team can pull from the correlations that only describe who already succeeded.

Agents miss this because correlation is seductive. They observe that retained users touch a certain feature, send more messages, or complete onboarding, and conclude that pushing all users toward that behavior will improve retention. The harm is that the team invests in driving a behavior that was never the cause, the intervention fails to move retention, and the analysis is quietly blamed on execution rather than on a flawed causal assumption. The opposite failure is paralysis, refusing to act on any correlation until perfect causal proof exists and letting promising levers go untested.

Use this skill before answering broad questions such as "what drives our retention", "which feature should we push to improve retention", "why do some users stick around", "what is our aha moment", or "which retention lever should we invest in next". The goal is to prevent the agent from presenting correlated behaviors as actionable drivers without examining whether the relationship is causal.

## Core Rules

### Start With The Hypothesis, Not The Dashboard

Driver analysis should begin with a hypothesis about why retention happens, grounded in the product's value proposition, not with a scan of every user behavior looking for something that correlates. Scanning first produces significance-chasing and findings that do not survive replication.

State the proposed mechanism before testing it: "users who reach outcome X within their first week retain better because X delivers the core value." A hypothesis names a cause and a reason. A correlation hunt names only a pattern. Begin from value theory, then test whether the data supports it.

### Distinguish Drivers From Markers And Consequences

A driver is a behavior that, when increased, increases retention. A marker is a behavior that co-occurs with retention but does not cause it, often because both are driven by the same underlying user intent. A consequence is a behavior that happens because the user already decided to stay, not the other way around.

For any candidate driver, ask which it is. Users who invite teammates may retain better because inviting causes commitment, or because the kind of user who invites was already committed. Users who use advanced features may retain because the features create value, or because power users both use features and retain. The same correlation supports opposite conclusions, and only causal reasoning and experimentation can tell them apart.

### Use Cohort And Time-Ordering To Narrow Causation

While only experimentation proves causation, observational analysis can rule out obvious non-causal relationships by checking time ordering and comparing comparable cohorts. If the candidate behavior happens before the retention outcome, it can at least be a cause; if it happens after, it cannot.

Compare cohorts that differ in the candidate behavior but are otherwise similar, and check whether retention differs. Segment by acquisition channel, plan, or initial intent to reduce the chance that an unobserved difference is driving both the behavior and the retention. These methods do not replace experiments, but they prevent acting on relationships that are obviously consequential rather than causal.

### Validate Drivers With Experiments Before Investing

The decisive test of a driver is whether increasing it, holding other things equal, increases retention. That requires an experiment: push a randomly assigned group toward the candidate behavior and measure retention against a control.

Before the team invests in onboarding flows, nudges, or feature promotion built around a candidate driver, run the experiment. Many confidently reported drivers fail this test, and finding that out cheaply in an experiment is far better than finding it out expensively in a launched feature that does not move the metric. Treat the experiment as the validation step, not as an optional extra.

### Rank Drivers By Addressability, Not Just Correlation Strength

A behavior can be strongly correlated with retention and still be a poor investment if the product cannot realistically change it. The best driver to invest in is one that is both causal and addressable, meaning the team has a plausible lever to increase it.

For each candidate, assess the strength of the causal evidence, the size of the retention effect, and the feasibility and cost of moving the behavior. A moderately strong driver the team can actually influence beats a stronger one that resists intervention. Prioritize the intersection of evidence, impact, and addressability.

### Separate First-Experience Drivers From Ongoing Drivers

Some drivers act in the first hours or days and determine whether a user ever reaches value; others act over the long term and determine whether an established user stays. The two require different analysis windows and different interventions.

Identify which kind each candidate is. First-experience drivers are usually found in onboarding, time-to-value, and early feature adoption, and they are tested against early retention windows. Ongoing drivers are found in habit formation, integration depth, and switching costs, and they are tested against longer windows. Mixing the two produces muddled findings about "what drives retention" without saying for whom or when.

### Watch For Common-Cause Confounding

A frequent trap is that an unobserved third factor drives both the candidate behavior and retention. Users with strong intent retain better and also complete onboarding, use more features, and invite others. The behaviors all correlate with retention, but none of them cause it; intent does.

Look for the underlying variable. Segment by intent, by acquisition source, by use case, or by initial behavior, and check whether the candidate driver still predicts retention within a segment. If the effect disappears within homogeneous segments, it was likely a common-cause artifact rather than a true lever.

## Common Traps

### Treating Correlation As Causation

The single most common error. A behavior correlates with retention, so the team drives all users toward it, and retention does not move because the behavior was a marker, not a cause.

### Finding The "Aha Moment" By Data Mining

Scanning for the behavior most correlated with retention and labeling it the aha moment produces a statistically significant but often meaningless finding, because the same scan would have found something regardless of true structure.

### Acting On Power-User Behaviors

Behaviors that distinguish power users from casual users often correlate with retention but are not addressable in new users. Pushing new users to adopt power-user patterns rarely works and can frustrate them.

### Ignoring Common-Cause Confounding

Intent, motivation, or use case drives both behaviors and retention. Without segmenting on these, the analysis attributes to the behavior what actually belongs to the kind of user who performed it.

### Investing In A Driver Before Experimenting

Building onboarding, nudges, or features around an unvalidated driver wastes effort when the driver turns out not to be causal. The experiment is cheaper than the build.

### Mixing First-Experience And Ongoing Drivers

Analyzing all drivers in one window blurs the distinction between early value-reaching behaviors and long-term habit behaviors, producing recommendations that do not fit either context.

### Ranking By Correlation Strength Alone

The strongest correlation is often the least addressable. Prioritizing by statistical strength alone steers investment toward behaviors the team cannot realistically change.

## Self-Check

- [ ] The analysis began with a hypothesis about the mechanism, not a scan for any correlating behavior.
- [ ] Each candidate driver was examined as a potential driver, marker, or consequence, not assumed causal.
- [ ] Time ordering and cohort comparison were used to rule out obviously non-causal relationships before acting.
- [ ] Drivers were validated with experiments before the team invested in onboarding, nudges, or feature promotion around them.
- [ ] Candidate drivers were ranked by the intersection of causal evidence, impact size, and addressability, not by correlation alone.
- [ ] First-experience drivers and ongoing drivers were analyzed in separate windows rather than blended.
- [ ] Common-cause confounding from intent, acquisition source, or use case was checked by segmenting before concluding.
- [ ] Power-user behaviors were not promoted to new users without evidence they are addressable and causal in that population.
- [ ] The recommended levers are ones the team can plausibly move, not merely ones that predict retention.
- [ ] Findings distinguish what predicts retention from what a product change can actually improve.
