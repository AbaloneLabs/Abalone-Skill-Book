---
name: release_scoping_and_mvp_definition.md
description: Use when the agent is defining an MVP or minimum lovable release, cutting scope to the smallest valuable slice, deciding release boundaries, or evaluating whether a planned scope is small enough to learn from quickly.
---

# Release Scoping And MVP Definition

Scoping is where strategy meets delivery, and it fails in both directions. Over-scoping produces slow, expensive releases that delay learning and accumulate risk; under-scoping produces releases so thin they teach nothing or damage trust. The purpose of an MVP is not to ship a minimal pile of features. It is to ship the smallest slice that delivers real user value and answers a defined learning question, so the next decision is better informed than the last.

The persistent failure is treating the MVP as a stripped-down version of the eventual product rather than as a learning vehicle. Teams then cut arbitrarily, removing the very pieces that make the slice valuable or learnable, while keeping complexity that does not serve the learning goal. The product manager must cut by value and learning, not by ease of removal, and must keep each release independently shippable and coherent.

Use this skill before answering questions such as "what should be in the MVP", "is this scope small enough", "how do we cut this release", "should we split this into phases", or "what is the smallest valuable slice". The goal is to prevent the agent from defining a release that is too large to learn from, too thin to deliver value, or fragmented in a way that teaches nothing.

## Core Rules

### Treat The MVP As A Learning Vehicle

An MVP is defined by the question it is meant to answer, not by the features it omits. Before deciding scope, state what the team needs to learn and which behavior or signal will provide that learning. The scope is then whatever is necessary to deliver value while generating that signal.

Define the learning question first:

- What assumption about value, behavior, or willingness does this release test?
- What metric or qualitative signal will indicate the answer?
- How quickly do we need the answer to inform the next investment?
- What would make us pivot, persevere, or stop?

A release without a learning question is just a small product, not an MVP. The learning goal is what justifies cutting everything that does not serve it, and it is what makes the eventual scope defensible to stakeholders who want more.

### Slice By User Value, Not By Layer

A valuable slice delivers an end-to-end outcome the user can actually complete, not a horizontal layer like "the data model" or "the settings page". The useful unit is a thin vertical slice that lets a real user achieve a real result, however narrow.

Compare:

- Thin vertical slice: a user can complete one core task end to end, with real data and a real result.
- Horizontal layer: the backend exists, or the UI exists, but no user can yet accomplish anything.

A walking skeleton that exercises the full path, even crudely, is more learnable than a polished fragment. When cutting, preserve the path from entry to value and cut breadth, not depth. A slice that stops short of the outcome teaches nothing about whether the outcome is valued.

### Apply Must-Have Versus Nice-To-Have Discipline

Cutting requires distinguishing what the release must contain to deliver its value and learning from what would merely be better. The trap is treating every requested element as must-have, which inflates scope and delays the learning the release was meant to provide.

For each candidate element ask:

- Does the user outcome fail without it?
- Does the learning question become unanswerable without it?
- Is it required for safety, trust, compliance, or correctness?
- Is it a dependency for another must-have, or genuinely standalone?
- Would its absence damage the brand or erode user trust?

Must-have means the slice breaks without it. Everything else is a candidate for a later phase, and naming it as such is a decision, not a deferment. The discipline is to move items out as deliberately as they were added in.

### Keep Each Slice Independently Shippable And Learnable

A release is only useful if it can ship and generate signal on its own. If it depends on a later phase to deliver value, or if its results cannot be interpreted without the next release, it is not a valid slice.

A shippable, learnable slice satisfies:

- a user can reach a real outcome without waiting for a future phase;
- the slice is coherent enough that users understand what it does;
- instrumentation and feedback channels are included, not deferred;
- the slice is safe and correct within its narrow scope;
- the result can be measured against the learning question.

If shipping the slice would embarrass the brand or break trust, it is too thin or shaped wrong, not "lean". Independently shippable means genuinely valuable in the user's hands, not merely deployable.

### Weigh The Cost Of Over-Scoping Against Under-Scoping

Both errors are expensive, and they fail differently. Over-scoping delays learning, increases build risk, and makes the release harder to reverse; under-scoping ships something that teaches nothing or undermines confidence in the product.

Consider the tradeoffs:

- Over-scoping cost: slower feedback, larger blast radius, more integration risk, sunk-cost pressure to keep going.
- Under-scoping cost: no usable signal, user confusion, wasted release, damaged trust if the slice feels broken.
- Reversibility: how hard is it to add or remove scope after launch?
- Urgency: how much does delay itself cost, versus the cost of being wrong?

The right scope is the smallest one that still answers the learning question and delivers real value. It is rarely the largest the team can fit in the timeline, and rarely the absolute minimum that can deploy.

### Decide When To Split A Release Into Phases

Some scopes are too large to learn from quickly but too coherent to abandon. The answer is phasing: sequencing releases so each phase ships value and generates signal that informs the next.

Split into phases when:

- the learning question can be answered earlier by a subset;
- one part carries most of the risk and should be tested first;
- dependencies allow a meaningful earlier release;
- stakeholders need a visible milestone before funding the rest;
- the full scope would delay learning past the point of usefulness.

Each phase should have its own learning question and its own definition of value, not merely be a fragment of a larger whole. A phase that ships nothing learnable is sequencing, not phasing.

### Avoid MVPs That Damage Trust Or Brand

A release that feels broken, careless, or misleading can cost more trust than the learning is worth. The "minimum" in MVP refers to scope, not to quality of the experience within that scope.

Guard against:

- shipping flows that fail silently or lose user data;
- presenting a slice as finished when it is obviously incomplete in a way that erodes confidence;
- collecting sign-ups or data through a pretense the slice cannot honor;
- ignoring accessibility, safety, or correctness in the name of speed;
- releasing to populations who will judge the whole product by the fragment.

A minimum lovable release keeps the narrow scope excellent rather than making the whole experience mediocre. Trust is part of the viability calculation, not an obstacle to speed.

## Common Traps

### Defining The MVP As A Stripped-Down Final Product

When the MVP is just the eventual product with features removed, it inherits assumptions it was meant to test and usually fails to deliver value or learning. The MVP should be designed for its question, not carved from a wishlist.

### Cutting The Wrong Things To Hit A Date

Removing the elements that create value or enable measurement, while keeping incidental complexity, produces a release that is small but useless. Cutting must preserve the outcome and the learning path.

### Horizontal Slices That Teach Nothing

A backend, a settings page, or an API with no user outcome cannot answer whether the value proposition holds. These layers may be necessary, but they are not a release on their own.

### No Learning Question, So Success Is Undefined

Without a stated learning goal, the team cannot tell whether the release worked, and scope debates devolve into opinions. The learning question is what makes scope decisions objective.

### Over-Scoping Under Stakeholder Pressure

Adding scope to satisfy every stakeholder delays the release past the point where its learning is useful. The result is a large, late release that tests less than a smaller one would have.

### Shipping A Slice That Erodes Trust

Releasing something that feels broken or misleading to protect the timeline trades durable trust for a short-term milestone. The brand cost often exceeds the learning value.

### Phases That Are Fragments, Not Releases

Splitting work into phases where only the final phase delivers value defeats the purpose. Each phase must ship value and generate signal, or it is just sequencing disguised as phasing.

## Self-Check

- [ ] The release has an explicit learning question and a signal that will answer it.
- [ ] Scope is sliced vertically so a user can reach a real outcome end to end.
- [ ] Must-have elements are distinguished from nice-to-have, and cuts are justified against the learning goal.
- [ ] The slice is independently shippable and delivers real value without depending on a future phase.
- [ ] Instrumentation and feedback channels are included in the scope, not deferred.
- [ ] The cost of over-scoping and under-scoping was weighed, including reversibility and urgency.
- [ ] Any phasing gives each phase its own value and learning question, not just a fragment of the whole.
- [ ] The slice is safe, correct, and accessible within its scope, and does not damage trust or brand.
- [ ] Scope cuts preserved the outcome and the learning path rather than removing what made the slice valuable.
- [ ] The release is the smallest scope that still answers the learning question and delivers value, not the largest that fits the timeline.
