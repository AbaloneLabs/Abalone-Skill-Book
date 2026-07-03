---
name: activation-event-definition.md
description: Use when the agent is defining or revising the activation event, identifying the behaviors that predict retention, distinguishing activation from adjacent metrics, choosing between single and composite activation definitions, or validating that an activation metric actually predicts long-term engagement.
---

# Activation Event Definition

The activation event is one of the most consequential definitions a product team makes, because it becomes the target that onboarding, growth, and feature work optimize toward. A well-defined activation event focuses the team on the behavior that actually creates retained users. A poorly defined one — chosen because it is easy to measure, flattering to the dashboard, or assumed rather than discovered — sends the team optimizing for a metric that moves while retention does not. The cost of a wrong activation definition is not just a bad metric; it is months or years of misdirected effort across growth, onboarding, and product, all chasing a number that does not predict the outcome the business needs.

This skill covers the judgment needed to define, validate, and revise the activation event so that it genuinely predicts retention and usefully guides the team.

## Core Rules

### Discover the activation event from data, not from intuition

The activation event is the behavior that, when performed, materially increases the probability of retention. This is an empirical question, and the answer is frequently counterintuitive. The action the team believes is the "aha moment" often turns out to predict retention weakly, while an unglamorous behavior no one focused on predicts it strongly.

- Compare the behaviors of users who retained over a meaningful horizon (30, 60, 90 days) with those who churned. Identify which early behaviors most strongly separate the two groups.
- Consider a wide range of candidate behaviors: feature usage, frequency, breadth, completion of workflows, social or collaborative actions, returning within a window. Do not restrict the search to the obvious.
- Rank candidate behaviors by their predictive power for retention, not by their intuitive appeal or their measurability.

The activation event must be discovered, then validated, then periodically rediscovered. An activation event defined once and never re-examined drifts out of alignment as the product and audience change.

### Validate that the activation event actually predicts retention

The defining test of an activation event is predictive validity: users who activate must retain at a materially higher rate than users who do not. If this is not true, the activation event is wrong, regardless of how meaningful it feels or how well it measures.

- Measure the retention rate of activated versus non-activated users over a meaningful horizon. The gap should be large and consistent.
- Check that the activation event predicts retention across segments, not just in aggregate. An activation event that predicts retention for one segment but not others is incomplete.
- Watch for reverse causation. It is possible that users who were going to retain anyway also happen to perform the activation behavior, rather than the behavior causing retention. Look for evidence of directionality: does performing the behavior precede and predict the retention, controlling for other factors?

If activated users do not retain better, the activation event is a vanity metric. Change it.

### Distinguish the activation event from adjacent and precursor metrics

Activation is frequently confused with metrics that are related but distinct, and the confusion misdirects work. Each of these metrics has a role, but they are not activation.

- **Sign-up or registration** is acquisition, not activation. A registered user has not necessarily received value.
- **Setup or configuration completion** is a precursor, not activation. A user can complete setup and never use the product meaningfully.
- **First action** may or may not be activation. The first action is often exploratory or trivial and may not correspond to value.
- **Engagement or usage metrics** describe ongoing behavior after activation, not the threshold crossing itself.

Conflating these with activation leads to optimizing metrics that do not move retention. Keep activation focused on the specific threshold behavior that predicts a user will return and derive ongoing value.

### Decide deliberately between single, composite, and threshold activation definitions

Activation can be defined as a single action, a combination of actions, or a threshold of usage. Each formulation has tradeoffs, and the choice should be deliberate based on what predicts retention and what is actionable.

- **Single-action activation** ("performed action X") is simple and clear, and works when one behavior strongly predicts retention. Its risk is oversimplification if no single action is sufficient.
- **Composite activation** ("performed X and Y within the first session") captures that retention may require experiencing multiple facets of value. Its risk is complexity and the temptation to add conditions until the metric becomes unactionable.
- **Threshold activation** ("used the core feature N times" or "returned within W days") captures that value may require repetition or a habit to form. Its risk is that the threshold becomes arbitrary.

Choose the simplest formulation that retains predictive validity. Complexity in the definition makes it harder to design onboarding toward and harder to communicate.

### Segment the activation definition when retention predictors diverge

A single activation event for all users works only if the behavior that predicts retention is the same across segments. In many products, it is not. Different user types, use cases, and acquisition sources may activate through different behaviors, and forcing a single definition underserves the segments for whom it does not fit.

- Test whether the activation event predicts retention equally across segments. Where it does not, investigate what behavior does predict retention for that segment.
- Define segment-specific activation events where the divergence is material. A B2B product may have different activation for admins and end users; a multi-use product may have different activation for different primary use cases.
- Keep the number of segments manageable. A proliferation of activation definitions becomes unworkable; group segments where the activation behavior is similar.

### Treat the activation event as a target that shapes behavior, and own its side effects

Once an activation event is defined and communicated, the team optimizes toward it. This is the point, but it also creates risk: the team may find ways to increase activation that do not increase retention, by pushing users through the activation behavior without delivering the value that made the behavior predictive.

- Monitor activation quality: as activation rate changes, does retention change in the same direction? If activation rises but retention does not, the metric is being gamed.
- Watch for designs that manipulate users into the activation behavior (forced actions, misleading prompts) without delivering value. These produce activations that do not stick and that may harm trust.
- The activation event is a proxy for value received. When the proxy and the underlying outcome diverge, the proxy is being exploited, and the definition or the design needs attention.

### Revalidate and revise the activation event periodically

The activation event is not permanent. As the product evolves, as the audience shifts, as the value proposition changes, the behavior that predicts retention can change. An activation event that was correct at one stage becomes wrong at another, and continuing to optimize a stale definition wastes effort.

- Re-examine the activation event on a regular cadence and after major changes (new features, new segments, pivots in value proposition).
- Be willing to revise the definition when the data shows it no longer predicts retention, even if the revision makes the dashboard look worse in the short term. A correct definition that shows a lower activation rate is more useful than a flattering wrong one.

## Common Traps

### Assuming the activation event from intuition

The team picks the "aha moment" based on what feels meaningful and optimizes it without checking predictive validity. The metric improves; retention does not. Discover the event from data comparing retained and churned users.

### Reverse causation mistaken for prediction

Users who were going to retain anyway happen to perform the activation behavior, but the behavior does not cause retention. The activation event then describes retainers rather than predicting them. Look for evidence of directionality.

### Conflating activation with sign-up, setup, or engagement

Optimizing a precursor or adjacent metric as if it were activation produces motion without retention progress. Keep activation distinct and focused on the value-threshold behavior.

### Over-complicating the activation definition

Adding conditions until the definition captures every nuance makes it unactionable and hard to design toward. Choose the simplest formulation that retains predictive validity.

### A single activation definition for divergent segments

Forcing all users through one activation definition underserves segments whose retention predictors differ. Segment the definition where the divergence is material.

### Gaming the activation metric

Pushing users through the activation behavior without delivering value raises the rate without raising retention, deceiving the team. Monitor activation quality by checking that retention moves with activation.

### Stale activation definitions

The activation event that fit the product a year ago no longer fits, but it is still being optimized. Revalidate periodically and after major changes.

### Choosing the activation event for measurability rather than validity

A behavior is chosen because it is easy to measure, even though a harder-to-measure behavior predicts retention better. Prioritize predictive validity; invest in the instrumentation to measure the right thing.

## Self-Check

- Was the activation event discovered from data comparing retained and churned users, or assumed from intuition?
- Have I validated that activated users retain at a materially higher rate, across segments, with attention to reverse causation?
- Is activation kept distinct from sign-up, setup, first action, and engagement, each measured separately?
- Did I choose the simplest formulation (single, composite, or threshold) that retains predictive validity, resisting over-complication?
- Have I tested whether the activation event predicts retention equally across segments, and defined segment-specific activation where it diverges?
- Am I monitoring activation quality, checking that retention moves with activation rate, so the metric is not being gamed?
- Have I revalidated the activation event recently and after major product or audience changes?
- Did I prioritize predictive validity over measurability when they conflicted, and invested in instrumentation for the right behavior?
- If activation rate rose but retention did not, would I detect it and revise the definition, or celebrate the metric?
- Could I explain, with evidence, why this specific behavior predicts retention rather than another plausible behavior?
