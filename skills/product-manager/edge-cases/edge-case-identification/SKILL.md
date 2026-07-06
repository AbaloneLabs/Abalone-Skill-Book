---
name: edge_case_identification.md
description: Use when the agent is identifying edge cases for a feature or system, systematically discovering boundary conditions empty inputs concurrent operations and failure modes, or ensuring a design accounts for the unusual and extreme situations that reveal whether behavior is truly specified and robust.
---

# Edge Case Identification

Edge cases are the situations at the boundaries of expected behavior: empty inputs, maximum values, concurrent operations, rare combinations, failure modes, and extreme conditions. They are where designs most often break, because they are easy to overlook and because the happy-path thinking that produces a design does not naturally consider them. Done well, edge case identification is a systematic practice that discovers the situations the design must handle, before they are discovered by users in production. Done poorly, it is an afterthought that catches only the obvious cases, leaving the subtle and dangerous ones to emerge as bugs, incidents, and user harm. Agents often identify only the edge cases that are easy to think of, missing the systematic categories that a disciplined approach reveals.

The harm this skill prevents is the design that works for the typical case and fails at the edges. A feature that handles normal inputs flawlessly but crashes on empty input, corrupts data under concurrent access, or behaves unpredictably at scale is a feature that will harm users and erode trust. Edge cases are not rare curiosities; they are the situations that real users and real systems encounter, and a design that does not account for them is incomplete. Systematic identification is what makes a design robust, by surfacing the edge cases while there is still time to design for them.

Use this skill before answering questions such as "what edge cases should we consider", "how do we find edge cases", "what could go wrong with this feature", or "how do we make sure this design is robust". The goal is to prevent the agent from identifying edge cases ad hoc rather than systematically, missing the categories that disciplined enumeration reveals.

## Core Rules

### Enumerate Edge Cases By Systematic Category, Not By Inspiration

Edge case identification should be systematic, enumerating cases by category rather than waiting for inspiration. The categories that tend to produce edge cases include: boundaries of inputs and values, emptiness and null, size and scale, concurrency and timing, state and lifecycle, permissions and access, failure and recovery, and combinations and interactions. For each category, deliberately consider what edge cases it contains for the feature being designed. This systematic approach surfaces cases that ad hoc thinking misses, because it forces consideration of categories that the designer might not naturally think of.

For each category, ask specific questions. Boundaries: what happens at the minimum, maximum, and just beyond? Emptiness: what if the input is empty, null, or missing? Scale: what if there is one item, millions of items, or the system is under heavy load? Concurrency: what if two operations happen at once, or in rapid succession? State: what if the entity is in an unusual state, or mid-transition? Permissions: what if the user lacks expected permissions? Failure: what if a dependency fails, or returns unexpected data? Combinations: what if multiple edge conditions occur together? Working through these questions systematically reveals the edge cases that matter.

### Pay Special Attention To Boundaries And Emptiness

Boundaries and emptiness are the edge cases that most consistently produce bugs, because they are where assumptions break down. A system that handles values in the middle of a range often fails at the ends: zero, one, the maximum, just beyond the maximum. A system that handles present data often fails when data is absent: empty strings, null values, missing fields, zero items. These cases are common in real usage and common in bugs, and they deserve special attention in edge case identification.

For boundaries, test the values at and around the boundary: the minimum allowed, the minimum minus one, the maximum allowed, the maximum plus one, and typical values in between. For emptiness, test every form of absence the system might encounter: empty collections, null references, missing optional fields, whitespace-only strings, and zero counts. These tests, applied systematically to each input and value, catch the boundary and emptiness failures that are among the most common sources of bugs.

### Consider Concurrency And Timing Edge Cases

Concurrency and timing produce some of the most subtle and dangerous edge cases, because they involve interactions between operations that are hard to predict and hard to reproduce. What happens if two users act on the same entity at once? What if an operation is initiated and then cancelled mid-flight? What if a dependency responds slowly or out of order? What if the system receives events in an unexpected sequence? These cases are easy to overlook in sequential thinking but common in real systems, and the failures they produce, from data corruption to race conditions, are among the hardest to diagnose and fix.

Identify concurrency and timing edge cases by considering the operations that can overlap and the sequences that can occur. For each pair of operations that touch the same data, consider what happens if they run simultaneously. For each workflow, consider what happens if steps occur out of order or if a step is interrupted. For each dependency, consider what happens if it is slow, unreliable, or returns unexpected results. These considerations reveal the concurrency edge cases that sequential happy-path thinking misses.

### Examine Failure Modes And Recovery Behavior

Edge cases include the situations where things go wrong: dependencies fail, errors occur, resources are exhausted, and the system must recover. For each component and dependency, consider how it can fail and what the system does in response. What if the database is unavailable? What if a third-party API returns an error or unexpected data? What if disk or memory is exhausted? What if the network is unreliable? For each failure mode, the design should specify detection, response, and recovery, so that the system degrades gracefully rather than catastrophically.

Failure mode analysis also includes partial failures, which are often harder than total failures. What if a dependency is slow but responsive, degrading performance without failing? What if some operations succeed and others fail, leaving the system in an inconsistent state? What if a failure occurs mid-operation, after some side effects have occurred? These partial failure cases require explicit design, because the system's behavior in them determines whether it recovers or corrupts. Identify them and specify the behavior, rather than assuming failures are all-or-nothing.

### Trace Edge Cases Through The Full Workflow

Edge cases should be traced through the full workflow, not just at the point where they originate, because their effects propagate. An edge case in input validation may affect downstream processing, storage, and reporting. An edge case in concurrency may affect not just the immediate operation but related operations and the system's overall state. Trace each identified edge case through the workflow, considering how it affects each subsequent step, to identify the full range of behaviors that must be specified and the full range of failures that could result.

This tracing often reveals that an edge case handled well at its origin produces problems downstream, because the downstream components were not designed for the edge case's effects. For example, an input that is valid but unusual may pass validation but break a downstream assumption, producing a failure far from the edge case's origin. Tracing catches these propagated effects, allowing the design to address them at the right point, whether at origin, downstream, or both.

### Prioritize Edge Cases By Likelihood And Severity

Not all edge cases deserve equal design attention. Prioritize them by the combination of likelihood, how often the case occurs in real usage, and severity, how much harm results if it is not handled. High-likelihood, high-severity cases demand robust handling and thorough specification. Low-likelihood, low-severity cases may warrant only minimal handling or explicit acceptance of the risk. Prioritization focuses design and specification effort where it matters most, preventing both the over-engineering that comes from treating all cases equally and the under-engineering that comes from ignoring important cases.

Prioritization should be explicit and documented. For each edge case, note its likelihood and severity, and the decision about how to handle it. This documentation serves two purposes: it ensures that the decision is deliberate rather than accidental, and it provides a reference if the case occurs in production, allowing the team to assess whether the outcome was anticipated and accepted. Explicit prioritization turns edge case handling from a reactive activity into a deliberate design decision.

## Common Traps

### Ad Hoc Identification Missing Systematic Categories

Thinking of edge cases as they come to mind. The trap is missing entire categories of cases that disciplined enumeration would reveal.

### Ignoring Boundaries And Emptiness

Overlooking the most common sources of bugs. The trap is failures on zero, one, maximum, and absent data that occur frequently in real usage.

### Sequential Thinking Missing Concurrency

Considering operations in isolation. The trap is race conditions and inconsistent state from concurrent operations.

### Assuming Failures Are All-Or-Nothing

Designing only for total failure. The trap is unhandled partial failures that corrupt state or degrade silently.

### Edge Cases Handled At Origin But Not Downstream

Fixing the case where it appears but missing propagated effects. The trap is failures far from the edge case's origin.

### All Edge Cases Treated Equally

Investing uniformly regardless of likelihood and severity. The trap is over-engineering trivial cases and under-engineering important ones.

## Self-Check

- [ ] Edge cases are enumerated by systematic category, covering boundaries, emptiness, scale, concurrency, state, permissions, failure, and combinations.
- [ ] Boundaries and emptiness receive special attention, with values at and around boundaries and all forms of absence tested.
- [ ] Concurrency and timing edge cases are identified by considering overlapping operations and unexpected sequences.
- [ ] Failure modes, including partial failures, are analyzed with detection, response, and recovery specified.
- [ ] Edge cases are traced through the full workflow to identify propagated effects and downstream impacts.
- [ ] Edge cases are prioritized by likelihood and severity, with handling decisions documented explicitly.
- [ ] The design accounts for the unusual and extreme situations, not just the typical case.
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
