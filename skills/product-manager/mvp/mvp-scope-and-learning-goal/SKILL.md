---
name: mvp_scope_and_learning_goal.md
description: Use when the agent is defining a minimum viable product, stating the learning goal an MVP must achieve, deciding what the smallest testable version is, or resisting scope expansion during early product builds.
---

# MVP Scope And Learning Goal

A minimum viable product is not a small version of the final product. It is the smallest thing you can build to learn whether a specific belief about users is true. Confusing the two is the most common and most expensive mistake in early product work, because it produces a build that is too large to iterate quickly and too unfocused to answer any single question.

The judgment problem is that teams approach an MVP as a feature list to trim rather than as a question to answer. They start from the full vision, cut what seems hard, and ship what remains, calling it minimum because it is small. But an MVP defined by subtraction has no learning goal, so there is no criterion for what to cut and no definition of success beyond launch. The result is a build that consumes months, resolves none of the riskiest assumptions, and teaches the team nothing they could not have guessed beforehand. Agents tend to inherit a feature list, accept it as the scope, and then negotiate cuts one feature at a time instead of asking what single belief the build is meant to validate.

Use this skill before scoping an MVP, before defending a scope to stakeholders, and before approving a build plan. The goal is to force the MVP to be defined around a falsifiable question and a learning goal, so that scope decisions serve the question rather than the vision.

## Core Rules

### Start From The Question, Not The Feature List

An MVP exists to answer a question about users, markets, or behavior. Before listing any features, state the question in one sentence and identify the riskiest assumption it depends on. The question is the anchor; every scope decision is judged by whether it serves answering that question.

A strong MVP question names the belief, the population, and what would confirm or refute it. For example, "do freelance designers want to share reusable component libraries with their clients" is a question; "build a component library tool" is a feature. If you cannot state the question, you are not ready to scope an MVP, you are ready to do more discovery. The discipline of writing the question first prevents the scope from drifting toward the vision.

### Define The Learning Goal As A Measurable Outcome

The learning goal is the specific evidence that will confirm or refute the MVP's core assumption, stated before the build begins. It is not "launch and see what happens," because that produces no criterion for success and no signal that can kill the idea. A learning goal is a threshold on a behavior, a rate, or a qualitative signal.

Write the learning goal as a statement of what you expect to observe and what would falsify it. "We expect at least 40 percent of invited users to return within a week; below 15 percent we kill the idea." The threshold turns a vague hope into a decision rule, and it forces the team to instrument the right signal before launch rather than rationalizing the result afterward.

### Make It Minimum By Cutting Everything That Does Not Serve The Question

Minimum means removing everything that does not contribute to answering the learning question, no matter how valuable it seems for the eventual product. Polish, edge cases, settings, integrations, and nice-to-have features are all candidates for removal if they do not change the answer to the question.

Test each proposed feature against the learning goal. If removing it would not change whether you can answer the question, cut it. This is harder than it sounds, because every feature feels essential to someone. The discipline is to be ruthless about the question and generous only where the question genuinely requires it. A smaller MVP iterates faster, costs less, and reaches the learning sooner.

### Make It Viable By Preserving The Core Value Proposition

Viable means the MVP must still deliver enough of the core value that the learning signal is trustworthy. Cut too far and you test a broken experience that no one would use even if the idea were good, producing a false negative that kills a valid idea. Minimum and viable are in tension, and the art is finding the smallest scope that still delivers the essence.

Identify the core value proposition, the one thing the product must do for the user to experience the promised benefit, and protect it absolutely. Everything else is negotiable; the core is not. If you cannot deliver the core value at minimum scope, the MVP is not yet feasible, and you need a different approach, such as a concierge or manual version, rather than a thinner build.

### Separate The MVP From The Vision

The MVP is not version one of the product; it is a probe. Conflating the two causes scope creep, because every stakeholder maps their vision requirement onto the MVP and argues it cannot be cut. Keep the vision documented separately, and treat the MVP as a temporary artifact whose only job is to resolve uncertainty.

When a stakeholder asks for a feature, ask which learning question it serves. If it serves the vision rather than the MVP question, it belongs in the roadmap, not in this build. This reframing turns scope negotiations from a fight over what to cut into a conversation about what the MVP is for.

### Choose The Build To Match The Learning Goal

The form of the MVP should follow the learning goal, not precede it. A concierge or manual MVP, where you deliver the value by hand behind the scenes, can validate willingness and workflow without building software. A fake-door or painted-door test can validate demand before building the feature. A clickable prototype can validate comprehension before building the backend.

Match the cheapest build that can produce the evidence the learning goal requires. Building production software when a manual version would answer the question is a waste of time and a source of premature commitment. The build method is a variable, not a given.

### Pre-Commit To The Iterate-Versus-Kill Decision

Before launch, state what result will lead to iterating, what will lead to killing, and what will lead to pivoting. Without this pre-commitment, every result gets interpreted as "keep going," because killing feels like failure and the team is motivated to continue. Pre-commitment makes the kill a planned outcome rather than an admission of defeat.

Write the decision rule next to the learning goal. "If return rate exceeds 40 percent, we invest in a fuller build. If it is between 15 and 40, we iterate on the onboarding. If it is below 15, we kill." This rule, written before data, is what turns the MVP into a learning instrument rather than a sunk-cost trap.

## Common Traps

### Defining The MVP By Subtracting From The Vision

Starting from the full product and cutting what seems hard produces a scope with no learning goal. The trap is that the build is smaller but still unfocused, so it resolves no assumption and teaches nothing. Always start from the question, not the feature list.

### Launch And See What Happens

Treating launch as the learning goal gives no criterion for success or failure. The trap is that every result is interpreted as encouragement to continue, because there is no threshold that would trigger a kill. Define a measurable learning goal before building.

### Cutting Until The Core Value Breaks

In the pursuit of minimum, teams sometimes remove the feature that delivers the core value, then test a hollow experience. The trap is a false negative: users reject a broken product, and the team concludes the idea was bad when only the execution was. Protect the core value proposition absolutely.

### Conflating MVP With Version One

When the MVP is treated as the first release, every vision feature gets pulled in and scope balloons. The trap is that the build becomes too large to iterate and too slow to learn, defeating the entire purpose. Keep the MVP and the vision in separate documents.

### Building Software When A Manual Version Would Do

Production software commits the team to an architecture and a maintenance burden before the idea is validated. The trap is spending months on a build when a concierge or prototype version would have answered the question in days. Match the build to the learning goal.

### Continuing Regardless Of Result

Without a pre-committed kill rule, the team rationalizes weak results as "early days" and continues. The trap is that the MVP becomes a permanent zombie project that consumes resources without ever resolving the assumption. Pre-commit the iterate-versus-kill decision.

### Inheriting Scope Without Questioning It

Accepting a feature list from a stakeholder and then negotiating cuts one by one treats the list as given. The trap is that the scope reflects someone's preferences rather than the learning goal, and the cuts are arbitrary. Re-derive scope from the question every time.

## Self-Check

- [ ] The MVP is defined by a single falsifiable question about users or the market, not by a feature list.
- [ ] A measurable learning goal states what evidence will confirm or refute the core assumption, with thresholds written before the build.
- [ ] Every feature in scope was tested against the learning goal, and anything not serving the question was cut.
- [ ] The core value proposition is identified and protected, so the MVP remains viable and does not produce a false negative.
- [ ] The MVP is documented separately from the product vision, and vision features are deferred to the roadmap.
- [ ] The build method, whether manual, prototype, fake-door, or software, was chosen to match the learning goal at the lowest cost.
- [ ] An iterate-versus-kill decision rule was pre-committed before launch, with explicit thresholds for each outcome.
- [ ] The scope would still be defensible if the result came back opposite to what the team hoped.
