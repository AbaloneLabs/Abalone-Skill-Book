---
name: solution_validation_before_build.md
description: Use when the agent is validating a chosen solution direction before committing to full build, choosing between prototypes, fake-door tests, concierge tests, or pretotypes, or deciding when discovery evidence is strong enough to start delivery.
---

# Solution Validation Before Build

Between choosing a solution direction and building it lies the most expensive decision in product work. Engineering investment is large, slow, and hard to reverse, so the purpose of pre-build validation is to reduce the risk that the team builds the wrong thing or builds the right thing the wrong way. Validation is not about proving the idea is good; it is about trying to kill the riskiest assumptions cheaply before the team is committed.

The recurring failure is running a test that produces a confident-looking signal but does not actually de-risk the assumption that matters. A polished prototype that wins praise in a demo, a fake-door test that collects clicks during a hype cycle, or a concierge test run on friendly users can all generate false positives. The product manager must match each method to the specific risk, define the assumption and the kill criteria before testing, and decide honestly whether the evidence crosses the threshold to build.

Use this skill before answering questions such as "how do we validate this idea before building it", "which prototype or test should we run", "is this evidence strong enough to start delivery", "how do we avoid a false positive", or "what is the cheapest way to test this assumption". The goal is to prevent the agent from recommending a validation method that is impressive but irrelevant, or from declaring readiness to build on evidence that does not survive scrutiny.

## Core Rules

### Name The Risk Before Choosing The Method

Every solution carries multiple risks, and each validation method de-risks a different one. Choosing a method before naming the risk is how teams end up with tests that feel conclusive but answer the wrong question.

The four risk families are:

- Desirability: do the target users want this, and will they use it in real conditions?
- Feasibility: can we build it within our technical, data, and operational constraints?
- Viability: does it work for the business across pricing, cost, compliance, and strategy?
- Usability: can users accomplish the task effectively in the actual interface?

Start by ranking the risks. If the biggest unknown is whether users care at all, a usability prototype is premature; if the biggest unknown is unit economics, a clickable demo will not answer it. The method follows the risk, not the other way around.

### Match Method To Risk And Fidelity

Different techniques trade fidelity for speed and cost, and each answers a different question.

Common methods and what they test:

- Pretotype: tests whether demand exists at all, with the thinnest possible representation, before any real behavior is built.
- Fake-door or painted-door test: measures real expressed interest by presenting an entry point that is not yet backed by the feature, useful for demand signaling.
- Concierge test: delivers the value manually to a few real users to learn whether the outcome is valued, hiding the manual work behind the scenes.
- Wizard-of-Oz: simulates automation with humans on the back end to test perceived behavior and value before building the system.
- Clickable prototype: tests usability, comprehension, and preference, but not real-world adoption or willingness.
- Functional prototype: tests feasibility and real behavior on a narrow path, at higher cost.

Fidelity should be as low as the risk allows. A high-fidelity build used to test a desirability question wastes time and adds commitment bias, because teams resist abandoning something they have already polished.

### Define The Assumption And Kill Criteria Before Testing

A test without a pre-registered assumption and kill criteria becomes a search for confirming evidence. Before running anything, write down the specific assumption, the signal that would justify continuing, and the threshold at which the team will stop.

For each test specify:

- the single assumption being tested, stated falsifiably;
- the metric or qualitative signal that maps to that assumption;
- the minimum signal required to continue investing;
- the kill criteria that would end the direction;
- the decision owner and the date the decision will be made.

If the team cannot state what would make them abandon the idea, the test is not yet ready to run. Pre-registration is what separates validation from advocacy.

### Judge Qualitative And Sample Sufficiency Honestly

A few interviews can reveal whether a concept is misunderstood or irrelevant, but they cannot prove adoption. Conversely, a large quantitative signal from the wrong population proves nothing about the target users. Match the evidence type to the claim being made.

Consider:

- whether participants represent the target segment, not convenient internal or friendly users;
- whether the behavior observed is stated preference or revealed behavior;
- whether the sample is large and varied enough to detect the signal you care about;
- whether the test runs long enough to separate novelty from sustained interest;
- whether incentives or framing are biasing responses toward positivity.

State the confidence level explicitly. "Five users said they would use it" is a desirability hint, not a viability proof.

### Guard Against False Positives From Hype And Novelty

Novelty, attention, and enthusiasm routinely masquerade as validation. Users react positively to anything new in a demo, and clicks on a fake door during a launch spike do not reflect steady-state demand.

Mitigations:

- separate first-impression enthusiasm from repeated, unsolicited behavior;
- watch for what users do, not only what they say;
- discount signals collected during hype windows, promotions, or pressured demos;
- look for evidence of cost or effort users are willing to bear, not just interest;
- test whether users return or complete the real task, not only whether they initially engage.

A signal that only appears under attention is fragile. The strongest validation is behavior that recurs when no one is watching.

### Respect Ethical And Trust Boundaries

Some methods present something that is not real, which can mislead users or erode trust if handled carelessly. Fake-door tests, in particular, show an entry point to a feature that does not exist.

Apply judgment:

- avoid collecting sensitive data through a pretense;
- keep fake-door exposure small and short, and provide a graceful response when the feature is unavailable;
- be transparent enough that users are not harmed or deceived about what they signed up for;
- consider brand and regulatory risk, especially in healthcare, finance, or regulated markets;
- do not run concierge or manual tests in a way that promises reliability you cannot sustain.

Cheap does not mean exempt from care. A test that damages trust has not reduced risk; it has relocated it.

### Decide The Threshold To Start Delivery

Validation ends with a build decision, and that decision should be made against an explicit evidence bar rather than a feeling of momentum. The bar differs by risk and reversibility: a reversible, low-cost feature may proceed on lighter evidence, while a large, hard-to-reverse investment needs stronger, multi-method confirmation.

Define readiness as:

- the riskiest assumption has been tested with a method that can actually falsify it;
- the surviving evidence is consistent across qualitative and quantitative signals;
- the team has looked for disconfirming evidence, not only supporting signals;
- feasibility and viability have been checked, not only desirability;
- the cost of being wrong is understood and acceptable.

Cross the threshold deliberately. "We have learned enough to build" is a decision, not a default that arrives once enough tests have been run.

## Common Traps

### Testing The Wrong Risk With A Convenient Method

Running a usability prototype when the real question is demand, or a fake-door test when the real question is unit economics, produces confident answers to questions no one needed to ask. The trap is choosing the method the team already knows how to do.

### Confusing Stated Preference With Revealed Behavior

What users say in an interview or demo is a weak predictor of what they will do unprompted. Treating enthusiastic interview reactions as adoption evidence is one of the most reliable sources of false positives.

### No Kill Criteria, So Every Test Confirms

Without a pre-registered threshold for abandonment, teams interpret any result as supportive. Ambiguous signals become "promising", and weak signals become "directionally correct", so the project never stops.

### Over-Fidelity And Commitment Bias

The more polished the artifact, the harder it is to abandon. A team that invests heavily in a beautiful prototype starts defending it instead of learning from it, and the test becomes a sales exercise.

### Reading Hype-Window Signals As Steady Demand

Clicks and sign-ups gathered during a launch, promotion, or press spike overstate real demand. Building to a peak that will not repeat locks in overinvestment.

### Ignoring Feasibility And Viability

Desirability validation that ignores whether the thing can be built or whether it pays off leads to a loved-but-impossible product. All three risk families must be addressed before build, not discovered after.

### Declaring Ready To Build On Momentum

When tests have run for a while, the temptation is to start building just to keep moving. Without an explicit evidence bar, the team proceeds on calendar pressure rather than validated confidence.

## Self-Check

- [ ] The riskiest assumption was named and ranked before any validation method was selected.
- [ ] The chosen method actually de-risks the specific risk family it is being used for.
- [ ] Fidelity was kept as low as the risk allows, avoiding premature polish.
- [ ] Each test has a pre-registered assumption, a minimum continuing signal, and explicit kill criteria.
- [ ] Participants represent the target segment, and stated preference was not confused with revealed behavior.
- [ ] Novelty, hype-window, and attention effects were discounted when interpreting signals.
- [ ] Ethical and trust boundaries for fake-door, concierge, and pretotype methods were considered.
- [ ] Desirability, feasibility, viability, and usability were each addressed before declaring readiness.
- [ ] Disconfirming evidence was actively sought, not only supporting signals.
- [ ] The decision to start delivery was made against an explicit evidence bar appropriate to cost and reversibility.
