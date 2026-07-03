---
name: hypothesis_framing_and_falsifiability.md
description: Use when the agent is writing a product hypothesis, making assumptions explicit and falsifiable, stating predicted outcomes with magnitude, or checking whether a belief can actually be proven wrong before investing in a build.
---

# Hypothesis Framing And Falsifiability

A hypothesis is not a goal dressed up in scientific language. It is a claim specific enough to be proven wrong, written down before the test, so that the test can actually resolve a question. Most product "hypotheses" fail this test, and the work built on them cannot learn anything because the claim was never at risk.

The judgment problem is that teams write hypotheses that cannot fail. "We believe this feature will improve engagement" can be claimed as success no matter what happens, because any movement can be reinterpreted as engagement and any absence can be explained away by external factors. A hypothesis that cannot fail produces no learning, because the test has no outcome that would change the team's mind. Agents tend to write aspirations instead of predictions, omit the magnitude that would make the claim testable, leave the population vague, and bury the rationale that connects the intervention to the effect. The harm is builds that consume resources on beliefs that were never actually subjected to risk, and readouts that confirm whatever the team already wanted to do. The deeper harm is cultural: when no hypothesis can ever be refuted, the organization stops learning and starts rationalizing.

Use this skill before writing a hypothesis, before approving a build that rests on one, and before designing a test. The goal is to force every belief that drives investment into a form where it can be proven wrong, so that the test, whatever it is, can actually change a decision.

## Core Rules

### Make The Hypothesis Falsifiable Above All

The first and most important property of a hypothesis is that it could be proven wrong. Before anything else, ask whether there is any result that would make the team abandon the belief. If the answer is no, the statement is an aspiration, not a hypothesis, and no test of it can produce learning.

Falsifiability comes from specificity. A claim that names a population, an intervention, a metric, a direction, and a magnitude can fail, because a result outside the predicted range refutes it. A claim that names none of these cannot fail, because any result can be reconciled with vague language. Force the question: what exact outcome would make you stop believing this? If you cannot answer, rewrite until you can.

### Include All Four Parts: Population, Intervention, Effect, Rationale

A complete hypothesis names four things. The population, who is affected. The intervention, what changes. The predicted effect, on which metric, in which direction, by what magnitude. And the rationale, why the intervention should produce that effect in that population. Omit any part and the hypothesis becomes untestable or unfalsifiable.

Write the hypothesis as a single sentence that makes each part explicit. "We believe that showing a progress indicator to first-time uploaders will increase 7-day upload completion by at least 3 percentage points, because users currently abandon when they cannot tell how long the upload will take." That sentence tells you who is tested, what changes, what should move and by how much, and why. If you cannot write this sentence, you are not ready to test; you are ready to do discovery.

### Pre-Commit To Direction And Magnitude

The direction and the magnitude must be chosen before the test, because choosing them after the data is visible is selection on the outcome. A hypothesis that predicts "an increase of at least 3 points" can be refuted by a 1-point gain or a decline. A hypothesis that predicts only "a change" cannot be refuted by any result and is worthless.

Set the magnitude to the smallest effect that would matter, the threshold below which the change is not worth shipping. This ties the hypothesis to a real decision and prevents the team from declaring victory on a trivial movement. The magnitude is the most contested and most valuable part of the hypothesis, because it forces an honest conversation about what counts.

### State The Rationale So The Mechanism Is Testable

The rationale is the causal story that connects the intervention to the effect, and it matters because it tells you what to measure if the primary metric does not move. If the rationale is "users abandon because they cannot see progress," then a lack of effect should send you looking at whether users saw the indicator, whether they understood it, and whether abandonment happens at that moment.

A hypothesis without a rationale is a black box: if it fails, you learn only that it failed, not why. The rationale makes the hypothesis diagnostic, because it predicts not just the outcome but the mechanism, and a test of the mechanism is what turns a failed hypothesis into a better next hypothesis rather than a dead end.

### Define The Metric And How It Will Be Measured

The effect must be tied to a specific, measurable metric, defined the same way before and after the test. "Engagement" is not a metric; "weekly active sessions per user" is. Vague metrics let the team switch definitions after the result to find a favorable one, which is p-hacking at the hypothesis level.

Write the metric definition precisely, including the window, the population, and the calculation. Confirm it can be measured with the available instrumentation before the test runs. A hypothesis whose metric cannot be measured is untestable in practice, regardless of how well it is written.

### Separate The Hypothesis From The Solution

The hypothesis is a belief about users and outcomes; the solution is one attempt to exploit that belief. Conflating them means a failed test kills the belief along with the solution, when the belief may be right and only the execution wrong. Keep them distinct so that a failed solution does not bury a valid insight.

Frame the hypothesis in terms of the user problem and the predicted outcome, and treat the feature as one test of that hypothesis. If the feature fails, the hypothesis may still hold, and a different solution may succeed. This separation preserves learning across iterations instead of forcing the team to start over each time.

### Write The Hypothesis Before Designing The Test

The hypothesis must exist before the test is designed, because the test is built to refute a specific claim. Designing the test first and writing the hypothesis to match it produces a hypothesis tailored to what the test can show, which is the reverse of the correct logic and invites post-hoc framing.

Lock the hypothesis in a shared, timestamped artifact before any test design begins. If the team wants to revise it after seeing early data or after designing the test, the revision must be documented as a new hypothesis, not a refinement of the original. Pre-commitment is what makes the eventual result meaningful.

## Common Traps

### Writing A Hypothesis That Cannot Fail

"We believe this will improve engagement" can be claimed as success under any result. The trap is that the hypothesis feels safe because it cannot be wrong, but it also cannot produce learning, because no outcome would change the team's mind. Force a predicted direction and magnitude.

### Omitting The Magnitude

A hypothesis that predicts "an increase" cannot be refuted by any size of increase. The trap is declaring victory on a trivial movement that no one would ship. Set the magnitude to the smallest effect that matters.

### Leaving The Population Vague

"This will help users" does not say which users, so the team can claim success on whichever segment moved. The trap is a hypothesis that shifts its population to fit the result. Name the population precisely.

### Choosing Direction And Magnitude After The Data

Selecting the predicted effect after seeing results is selection on the outcome. The trap is a hypothesis that is guaranteed to match whatever happened, making the test meaningless. Pre-commit direction and magnitude.

### Burying Or Omitting The Rationale

Without a rationale, a failed hypothesis teaches only that it failed. The trap is a dead end instead of a better next hypothesis, because there is no mechanism to investigate. State the causal story.

### Using A Vague Or Switchable Metric

"Engagement" lets the team redefine success after the fact. The trap is metric shopping disguised as hypothesis testing. Define the metric precisely and lock it before the test.

### Conflating Hypothesis With Solution

When the belief and the feature are fused, a failed feature kills the belief. The trap is losing a valid insight because the first execution was wrong. Keep the hypothesis about users, the solution separate.

## Self-Check

- [ ] The hypothesis could be proven wrong; a specific result exists that would make the team abandon the belief.
- [ ] All four parts are present and explicit: population, intervention, predicted effect with direction and magnitude, and rationale.
- [ ] The magnitude is set to the smallest effect that would matter, not to the effect the team hopes for.
- [ ] Direction and magnitude were chosen before any data was visible, and any later change is documented as a new hypothesis.
- [ ] The rationale states the causal mechanism, so a failed primary metric can be diagnosed rather than treated as a dead end.
- [ ] The metric is defined precisely with window, population, and calculation, and can be measured with available instrumentation.
- [ ] The hypothesis is about a user belief and outcome, separate from the specific solution being tested.
- [ ] The hypothesis was written and timestamped before the test was designed, not reverse-engineered to match the test.
