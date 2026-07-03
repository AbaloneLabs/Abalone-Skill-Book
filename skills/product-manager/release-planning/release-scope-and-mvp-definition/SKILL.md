---
name: release_scope_and_mvp_definition.md
description: Use when the agent is defining what goes into a release, scoping a minimum viable product or release, deciding what to cut to hit a release, or balancing the desire for completeness against the value of shipping a smaller release sooner for learning and feedback.
---

# Release Scope And MVP Definition

Release scope is the decision about what set of functionality constitutes a shippable release, and the minimum viable product is the smallest version that delivers real value and generates real learning. Scoping is one of the hardest product decisions because it requires cutting beloved features, accepting imperfection, and shipping something smaller than the vision in order to learn and iterate. Done well, scope decisions produce releases that deliver value, generate feedback, and build toward the vision through iteration. Done poorly, they produce either bloated releases that delay endlessly chasing completeness or stripped releases that ship without delivering value or enabling learning. Agents often default to one extreme: either including everything to satisfy all stakeholders, or cutting so aggressively that the release cannot serve its purpose.

The harm this skill prevents is the release that never ships or the release that ships empty. The bloated release accumulates scope until the team is exhausted and the market has moved; the over-cut release delivers so little that it teaches nothing and satisfies no one. Both failures come from losing sight of what the release is for: delivering value and generating learning that informs the next iteration.

Use this skill before answering questions such as "what should be in this release", "what is our MVP", "what should we cut", or "is this release ready to ship". The goal is to prevent the agent from scoping on completeness or on minimalism rather than on value and learning.

## Core Rules

### Define The Release By The Value It Delivers And The Learning It Generates

A release is worth shipping if it delivers real value to users and generates learning that informs future direction. State both explicitly before scoping: what user outcome will this release enable, and what assumption will shipping it test. A release that delivers value but generates no learning is fine if the direction is certain, but most early releases should learn as much as they deliver. A release that generates learning but delivers no value struggles to attract the users whose behavior would generate the learning. The strongest releases do both.

This dual purpose also disciplines scope. Every candidate feature should be evaluated against whether it contributes to the release's value outcome or its learning goal. Features that contribute to neither are scope creep, no matter how desirable they seem. Features that contribute to one but not the other may belong in a later release. The value-and-learning frame provides criteria for cutting that go beyond stakeholder enthusiasm.

### Cut To The Minimum That Delivers The Core Outcome

The minimum viable release contains only what is necessary to deliver the core user outcome. Everything else, however desirable, is candidate for deferral. The discipline is to identify the core outcome, the smallest set of functionality that enables a user to achieve it, and to cut everything else. This is harder than it sounds, because every stakeholder has a feature they consider essential and every team member has a completeness instinct that resists shipping something visibly imperfect.

Apply the cut aggressively and then justify additions. The question is not "what should we include" but "what can we still cut while delivering the outcome". Each addition should be defended against the cost it adds in time, complexity, and risk. A release that ships sooner with less, but that delivers the core outcome and generates learning, is more valuable than a complete release that ships later having missed the learning window.

### Distinguish Must-Have From Nice-To-Have By Outcome Dependency

Not all features contribute equally to the release's outcome. Must-have features are those without which the core outcome cannot be achieved; the release cannot ship without them. Nice-to-have features enhance the outcome but are not required for it; the release would deliver value without them. The common scoping error is treating nice-to-haves as must-haves, often because of stakeholder pressure or completeness instinct, which inflates scope and delays the release.

For each feature, ask whether the core outcome is achievable without it. If yes, it is a nice-to-have and a candidate for deferral. If no, it is a must-have. This test, applied honestly, typically reveals that many features treated as essential are actually enhancements that could wait. The release that ships with true must-haves delivers the outcome sooner and learns faster.

### Resist Completeness As A Scope Driver

The instinct to ship a complete product, with all the features a user might want and all the rough edges polished, is the enemy of iterative delivery. Completeness is appropriate for a mature product serving a stable market; it is counterproductive for a new product or a new direction, where the goal is to learn what to build, not to build everything imagined. Resist the completeness instinct by reminding the team that later releases exist precisely to add what is deferred now.

Completeness-driven scoping also tends to produce symmetric investment, polishing every feature to the same level, when asymmetric investment, deeply serving the core outcome and lightly serving the rest, would deliver more value sooner. Accept imperfection in non-core areas as a deliberate choice that enables shipping the core sooner.

### Scope For Reversibility And Risk

Scope decisions should account for risk and reversibility. Features that are hard to reverse once shipped, because they establish user expectations, data models, or integration contracts, deserve more scrutiny and often more scope discipline than features that can be easily changed. A release that establishes a data model or an API contract that will be hard to change should be scoped carefully, because the decisions it locks in persist. A release of easily-reversible UI experimentation can be scoped more aggressively and shipped faster.

Risk also affects scope through what could go wrong. A release that includes a risky, complex feature alongside the core outcome ties the outcome's delivery to the risk's resolution. If the risky feature encounters problems, it delays the entire release, including the valuable core. Consider scoping risky work into a separate release so that risk does not jeopardize the delivery of certain value.

### Make Scope Tradeoffs Visible And Deliberate

Scope decisions are tradeoffs among features, timeline, quality, and risk, and they should be made visibly and deliberately rather than accumulating through stakeholder pressure. When a feature is added, name what it costs in timeline or what else is delayed. When a feature is cut, name the value or learning that the cut enables by shipping sooner. Visible tradeoffs produce deliberate scope; invisible tradeoffs produce accidental bloat or accidental emptiness.

This visibility also manages stakeholder expectations. Stakeholders who understand that each addition has a cost make more informed requests, and stakeholders who understand why a cut enables sooner delivery are more likely to accept it. The alternative, silently absorbing or cutting scope, produces stakeholder surprise and erodes trust in the planning process.

## Common Traps

### Scope Bloat From Stakeholder Pressure

Including every requested feature to avoid conflict. The trap is a release that delays endlessly and ships late having missed the learning window.

### Over-Cutting To Minimalism

Stripping so aggressively that the release delivers no value or enables no learning. The trap is a release that teaches nothing because no one uses it.

### Treating Nice-To-Haves As Must-Haves

Inflating scope with enhancements treated as essential. The trap is delayed delivery of the core outcome for features that could have waited.

### Completeness-Driven Scoping

Polishing every feature symmetrically before shipping. The trap is a complete product that arrives too late to inform direction.

### Mixing Risky And Certain Work In One Release

Tying core value delivery to risky feature resolution. The trap is certain value delayed by problems in uncertain work.

### Invisible Scope Tradeoffs

Adding and cutting scope without naming costs. The trap is accidental bloat or emptiness that no one deliberately chose.

## Self-Check

- [ ] The release is defined by the value it delivers and the learning it generates, both stated explicitly.
- [ ] Scope is cut to the minimum that delivers the core outcome, with additions justified against their cost.
- [ ] Must-have and nice-to-have features are distinguished by whether the core outcome depends on them.
- [ ] Completeness is resisted as a scope driver, with imperfection in non-core areas accepted deliberately.
- [ ] Reversibility and risk are considered, with hard-to-reverse decisions scoped carefully and risky work separated from certain value.
- [ ] Scope tradeoffs are made visible, with each addition and cut naming its cost or benefit.
- [ ] The release delivers real value and generates real learning, not merely minimal output.
