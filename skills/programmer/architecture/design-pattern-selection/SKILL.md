---
name: design_pattern_selection.md
description: Use when the agent is deciding whether to introduce or remove a design pattern (factory, strategy, state, observer, decorator, command, visitor, repository, adapter, chain of responsibility, plugin, etc.), judging whether a pattern fits the current problem or adds complexity, refactoring toward or away from a pattern, or reviewing code where a pattern may be over- or under-applied. Also covers GoF patterns, enterprise patterns, reactive patterns, and the tradeoff between explicit structure and over-engineering.
---

# Design Pattern Selection

A design pattern is a named answer to a recurring structural problem. Used well, it makes intent obvious and gives a team a shared vocabulary. Used carelessly, it adds indirection that obscures a solution that did not need the help. The judgment problem is not "which pattern exists for this" but "does this problem have the shape that makes the pattern worth its cost."

Agents tend to over-apply patterns because patterns are easy to recognize and satisfying to introduce. A factory, a strategy, or an observer can be added to almost any code without breaking it. The harm is delayed: more files, more indirection, more places to look for behavior, and a system where the simplest change requires navigating several abstractions. The reverse failure is also real — refusing a pattern that the problem clearly calls for, leaving a tangle of conditionals that a strategy or state machine would have clarified.

## Core Rules

### Diagnose The Problem Before Naming The Pattern

A pattern solves a specific structural problem. Reach for it only after you can state the problem in structural terms, not in pattern terms.

Before introducing a pattern, write down:

- What force is making the current code hard to change? (e.g., "every new payment method requires editing a switch in three places.")
- What varies independently, and what stays stable?
- What would the next change look like without the pattern, and what would it look like with it?
- Is the variation real and recurring, or hypothetical?

If the only variation is a single future case you imagine, the pattern is speculative. If the same kind of change has happened three times and each time required touching unrelated code, the pattern is responding to a real force.

### Weigh The Pattern's Cost Against The Problem's Cost

Every pattern charges a tax: more types, more indirection, more places to trace a call, more names to learn. The tax is worth paying when the problem's cost is higher — when the code is genuinely hard to extend, test, or reason about.

Ask:

- How many participants does the pattern add (interfaces, implementations, registries, contexts)?
- Will a reader have to follow how many hops to understand a single behavior?
- Does the pattern replace conditional logic with polymorphism, or does it wrap already-clear code in ceremony?
- Is the team familiar with the pattern, or will it need to be explained every time?

A pattern that costs three new classes to remove one `if` is usually a loss. A pattern that lets a new variant land without touching five unrelated modules is usually a win. Decide on the actual tradeoff, not on whether the pattern is "correct."

### Match The Pattern To The Real Variation Axis

Patterns differ by what varies. Misreading the variation axis produces a pattern that fights the code.

- **Strategy** — when the algorithm or policy varies and the caller stays the same. Good when each variant is a cohesive rule.
- **State** — when behavior varies by an object's lifecycle state and transitions matter. Good when invalid transitions must be prevented.
- **Factory / abstract factory** — when construction varies and the caller should not know which concrete type is built. Good when construction is genuinely polymorphic; overkill for "new with defaults."
- **Decorator** — when behavior varies by composition of orthogonal concerns. Good for cross-cutting wrappers; misleading when the "decorators" are actually alternatives.
- **Observer** — when an unknown number of unrelated parties need to react. Good for decoupled notification; dangerous when order, causality, or back-pressure matter.
- **Command** — when an operation needs to be parameterized, queued, undone, or replayed. Good when those needs are real, not anticipated.
- **Adapter** — when an existing interface must match a target without changing it. Good at seams; redundant when you own both sides.
- **Repository** — when persistence must be abstracted behind a collection-like interface. Good for testability and swapping stores; ceremony when there is one store and no test double.
- **Visitor** — when operations vary over a stable type hierarchy. Good when the hierarchy rarely changes and operations often do; poor when the hierarchy changes often.

Read the variation axis from the code, then pick the pattern whose axis matches.

### Introduce Patterns At Seams, Not Everywhere

A pattern is most valuable at a seam — a place where two parts of the system meet and one should not know the other's details. Ports, plugin points, persistence boundaries, and external integrations are natural seams. Introducing a pattern at a seam localizes the abstraction and keeps the rest of the code direct.

Introducing a pattern in the middle of cohesive logic, where there is no seam, wraps clear code in indirection. The same pattern that clarifies a boundary can obscure a single-purpose function. Reserve patterns for places where the abstraction earns its keep.

### Prefer Removing A Pattern When The Variation Disappears

Patterns are introduced to handle variation. When the variation goes away — a feature is cut, a provider is consolidated, a branch is deprecated — the pattern often becomes dead structure. Leaving it in place preserves indirection that no longer serves a purpose and misleads future readers into expecting variation that does not exist.

When a pattern collapses to one implementation, consider inlining it. When a strategy has one strategy, a factory builds one type, or an observer has one listener that is always the same, the abstraction is paying for nothing. Removing it is a legitimate refactor, not a regression.

### Treat Familiarity As A Real Constraint

A pattern only communicates if the reader recognizes it. A pattern the team has never used adds a learning cost on top of its structural cost. This does not mean avoiding unfamiliar patterns, but it means the justification must be stronger, and the introduction must come with a clear, local example.

Do not introduce several unfamiliar patterns in one change. Each one is a cognitive load, and stacked they make the code unreadable. If a pattern is genuinely the right tool, introduce it in isolation, name it explicitly, and let the team learn it before reaching for the next one.

### Distinguish Structural Patterns From Language Features

Many patterns exist because the languages they originated in lacked a feature. In modern languages, several patterns are better expressed as language constructs:

- Strategy is often a function or a closure, not a class hierarchy.
- Observer is often a signal, a stream, or an event channel.
- Command is often a callable or a record plus a handler.
- Builder is often named or default parameters.
- Visitor is sometimes better expressed with pattern matching or sum types.

Before adding a class-based pattern, check whether the language already has a lighter expression of the same idea. The goal is the structural benefit, not the pattern's classic form.

### Separate "Pattern" From "Idiom" And "Convention"

Not every repeated structure is a design pattern. Some are language idioms (e.g., `try/finally` for resource cleanup), some are project conventions (e.g., "all handlers return a result type"), and some are framework dictates. Treating idioms as patterns over-formalizes them; treating patterns as idioms under-communicates them. Name what you are actually introducing so the reader knows whether to expect a structural contract or a local habit.

## Common Traps

### Pattern-Driven Design

Starting from "we should use a strategy here" rather than "this switch is painful to extend" puts the cart before the horse. The code ends up shaped to fit the pattern rather than the pattern serving the code. Always start from the force, then evaluate whether a pattern addresses it.

### The Single-Implementation Abstraction

An interface with one implementation, a factory that returns one type, or a strategy with one strategy is a pattern waiting for a need that may never arrive. The cost is real (indirection, more files, harder tracing); the benefit is hypothetical. Defer the abstraction until the second real variant appears.

### Speculative Generality

Building extension points, plugin systems, or abstract factories "because we might need them" produces code that anticipates a future that rarely matches the prediction. The extension points then have to be maintained, documented, and worked around when the real future arrives differently. Build for the variation you have evidence of, and leave room to add abstraction when evidence appears.

### Using Observer Or Pub-Sub To Hide Coupling

Events and observers feel decoupled because there is no direct reference. But a system where behavior depends on an event that some unknown listener handles is a system where causality is invisible, ordering is accidental, and failures are silent. Use observer when the producer genuinely should not know the consumers; do not use it to avoid deciding who owns a responsibility.

### Factory Everywhere

Wrapping every construction in a factory does not make code more flexible; it makes it harder to find what is actually constructed. Factories earn their place when construction is polymorphic, conditional, or needs configuration that the caller should not supply. For straightforward construction, direct instantiation is clearer.

### Keeping A Pattern After The Reason Is Gone

A pattern introduced for a feature that was later removed often survives because "we might bring it back." Dead abstraction is still debt. If the variation is gone and unlikely to return, inline the pattern and document why. Future variation can be re-abstracted when it is real.

### Confusing "Enterprise Pattern" With "Good Architecture"

Repository, unit-of-work, specification, and similar enterprise patterns are tools, not virtues. Applied where the problem has the right shape, they improve testability and clarity. Applied as a default layer over simple CRUD, they add ceremony without benefit. Judge each by the problem it addresses, not by its presence in a book.

## Self-Check

- [ ] The structural problem the pattern addresses is stated in problem terms, not pattern terms, and the variation it handles is real and recurring.
- [ ] The pattern's added indirection (types, hops, files) is justified by the cost of the problem it solves.
- [ ] The chosen pattern's variation axis (algorithm, state, construction, composition, notification, operation) matches the actual variation in the code.
- [ ] The pattern is introduced at a genuine seam or extension point, not wrapped around cohesive single-purpose logic.
- [ ] No abstraction has a single implementation with no realistic second variant on the horizon.
- [ ] Existing patterns whose variation has disappeared were evaluated for inlining rather than preserved speculatively.
- [ ] The language's lighter constructs (functions, closures, sum types, pattern matching) were considered before adopting a class-based pattern form.
- [ ] The pattern is one the team can recognize, or it is introduced in isolation with a clear local example.
- [ ] Observer, event, or pub-sub usages were checked for hidden coupling, ordering assumptions, and silent-failure risk.
- [ ] Enterprise patterns (repository, unit-of-work, specification) are applied where their shape fits, not as a default architectural layer over simple operations.
