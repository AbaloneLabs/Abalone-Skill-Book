---
name: opportunity_solution_tree.md
description: Use when the agent is mapping a desired business outcome to user opportunities and candidate solutions, structuring product discovery around a target result, or checking whether discovery work actually connects to a measurable outcome.
---

# Opportunity-Solution Tree

Discovery fails quietly when teams produce a pile of ideas, interviews, and prototypes that never connect to a result the business cares about. The opportunity-solution tree is a structure that forces a continuous chain from a desired outcome, to the user opportunities that could drive it, to the solutions that could realize those opportunities, to the experiments that test the riskiest assumptions. Its value is not the diagram itself but the discipline of making every discovery artifact traceable to an outcome and making coverage gaps visible.

The most common failure is using the tree after the fact to rationalize a solution that was already chosen. The tree then becomes decoration rather than a tool for prioritizing opportunities before solutions, generating genuine alternatives, and exposing where the team is guessing. A product manager should treat the tree as a decision instrument that surfaces what is missing, not as documentation of what was already decided.

Use this skill before answering questions such as "how should discovery be structured around an outcome", "are we solving the right opportunities", "how do we connect research to a result", "how do we know we have enough solutions", or "how do we decide what to test next". The goal is to prevent the agent from building a tree that is shallow, solution-locked, or disconnected from a measurable outcome.

## Core Rules

### Start From A Desired Outcome, Not A Feature

The root of the tree is a target result, not a deliverable. An outcome describes a change in user or business behavior that the team is trying to create, and it should be specific enough to tell whether it moved.

A useful outcome answers:

- What metric or behavior are we trying to move?
- Over what time horizon?
- For which user or customer segment?
- What is the current baseline and the target?
- Who owns this outcome?

Weak roots sound like "improve the product" or "build AI features". Strong roots sound like "increase weekly active collaboration in workspaces from 12% to 20% over two quarters for new team accounts". If the root is a feature or a theme, the whole tree below it inherits the solution lock. When leadership hands down a theme, the product manager's first job is to translate it into a behavioral outcome before branching.

### Harvest Opportunities From Real Evidence

Opportunities are unmet needs, pain points, friction, or desires that, if addressed, could move the outcome. They are not features and not solutions. The strength of the opportunity layer depends entirely on the breadth and honesty of the sources feeding it.

Gather opportunities from:

- generative user interviews;
- support tickets and churn interviews;
- sales and customer success calls;
- product analytics, funnels, and drop-off points;
- search and help-center logs;
- session recordings;
- competitive and adjacent-market behavior;
- internal operational pain.

Each opportunity should be written as a user need or pain, not as a feature. "Users abandon checkout because shipping cost appears too late" is an opportunity; "add shipping cost earlier" is a solution that has collapsed the opportunity into one answer. Capture the opportunity richly enough that multiple solutions remain possible.

### Prioritize Opportunities Before Generating Solutions

The most valuable discipline the tree enforces is choosing which opportunities matter before falling in love with solutions. If the team jumps straight to ideas, it optimizes for cleverness instead of impact.

Prioritize opportunities against the outcome using factors such as:

- how directly the opportunity plausibly moves the target metric;
- reach, frequency, and severity of the pain;
- population affected and whether it is a strategically important segment;
- whether the opportunity is currently served by a workaround that masks the pain;
- strategic fit and dependency on other work;
- confidence in the evidence.

Force a ranking or a shortlist. A tree with thirty equally weighted opportunities has not made a decision. The prioritized opportunities are the ones worth branching solutions from; the rest are parked, not deleted.

### Generate Multiple Solutions Per Opportunity

For each prioritized opportunity, generate several distinct candidate solutions rather than one. The point is to make the choice visible and to avoid the first plausible idea becoming the plan. Distinct means genuinely different mechanisms, not three variations of the same button.

For example, for an opportunity about late shipping-cost surprise, candidates might include showing cost on the product page, a shipping estimator, a membership with free shipping, a price-lock promise, or a cart total preview. Each candidate should be traced back to the same opportunity so the tradeoff between them is explicit.

Resist stopping at one solution per opportunity. A single-child branch is a signal that the team has solution-locked and is using the tree to document the decision rather than to make it.

### Connect Experiments To Assumptions, Not To Solutions

Below solutions sit experiments, but experiments should test the riskiest assumptions underlying a solution, not the solution as a whole. A solution rests on desirability, feasibility, viability, and usability assumptions; the experiment layer should name which assumption each test targets and what evidence would confirm or kill it.

For each experiment define:

- the specific assumption being tested;
- the type of risk it de-risks;
- the minimum signal that would justify continuing;
- the kill criteria that would stop the team from investing further;
- how long the test needs to be trustworthy.

This keeps the tree honest. If an experiment cannot be tied to a named assumption, it is activity rather than learning. The experiment layer is also where the tree hands off cleanly to solution validation work.

### Use The Tree To Make Coverage Gaps Visible

A well-built tree exposes where the team is over- and under-investing. If most opportunities cluster around one user segment, one part of the funnel, or one type of pain, the tree shows the imbalance. If one opportunity has ten solutions and another has none, it shows where discovery has stalled.

Review the tree for:

- whether the opportunity set actually spans the journey that drives the outcome;
- whether opportunities are concentrated in loud segments while quiet segments are ignored;
- whether solutions are diverse in mechanism or all variants of one approach;
- whether experiments target the riskiest assumptions or only the easy ones;
- whether any branch is unsupported by evidence and resting on assertion.

Coverage gaps are findings, not failures. Naming them redirects the next round of research.

### Keep The Tree Honest Against Pre-Chosen Solutions

The tree must be allowed to contradict a favored solution. If the evidence points away from the pet idea, the product manager's job is to reflect that, not to prune the tree until it justifies the plan. Update the tree as learning arrives; a tree that never changes after the first workshop is a static artifact, not a living decision tool.

## Common Traps

### Starting The Root At A Feature Or Theme

When the root is a feature, every branch below it can only justify that feature, so the tree cannot surface better alternatives. This trap feels productive because the diagram fills quickly, but it has already foreclosed discovery.

### Collapsing Opportunities Into Solutions

Writing opportunities as features ("add reminders", "redesign dashboard") destroys the branching power of the tree. Once an opportunity is written as a solution, the team stops looking for other ways to address the underlying need.

### One Solution Per Opportunity

A single-child solution branch looks complete but is actually a decision made without alternatives. It hides the fact that the team never compared options and makes later prioritization meaningless.

### Treating The Tree As Documentation Of A Decision Already Made

If the tree is drawn after the solution is chosen and only contains supporting branches, it provides no decision value and can mislead stakeholders into thinking the choice was evidence-driven.

### Weighting Loud Segments And Recency Bias

Opportunities harvested only from the most recent interviews or the most vocal accounts overrepresent narrow pain. The tree then optimizes for noise instead of for the outcome, and quiet but strategic segments stay invisible.

### Testing Easy Assumptions Instead Of Riskiest Ones

Teams gravitate toward experiments that are likely to succeed and easy to run. This produces a tree full of green checks that never addresses the assumption whose failure would sink the whole solution.

### Letting The Tree Go Stale

A tree built once and never revisited cannot absorb new evidence. As experiments run and research continues, branches should be pruned, merged, or expanded; a frozen tree becomes a relic rather than a guide.

## Self-Check

- [ ] The root is a measurable behavioral or business outcome, not a feature, theme, or deliverable.
- [ ] Opportunities are written as user needs or pains, with multiple solutions still possible for each.
- [ ] Opportunities were harvested from diverse evidence sources, not only recent or vocal inputs.
- [ ] Opportunities were prioritized against the outcome before solutions were generated.
- [ ] Each prioritized opportunity has multiple genuinely distinct candidate solutions, not variants of one idea.
- [ ] Each experiment is tied to a named assumption with explicit kill criteria and a minimum continuing signal.
- [ ] The tree was reviewed for coverage gaps across segments, funnel stages, and assumption types.
- [ ] The tree reflects current evidence and has been updated, not frozen after the first workshop.
- [ ] No branch exists solely to justify a pre-chosen solution; favored ideas are held to the same evidence bar.
- [ ] The prioritized opportunities and next experiments are clear enough to direct the next round of discovery.
