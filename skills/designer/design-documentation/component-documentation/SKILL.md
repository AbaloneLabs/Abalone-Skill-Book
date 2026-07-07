---
name: component_documentation.md
description: Use when the agent is documenting a UI component, writing component READMEs or spec pages, defining props, variants, states, usage rules, do-and-don't examples, accessibility notes, or component handoff for designers and engineers, and must decide what a consumer needs to know to use the component correctly without inspecting its source.
---

# Component Documentation

A component is not finished when it is built. It is finished when another designer or engineer can use it correctly without reading its source, guessing its props, or reverse-engineering its states. Component documentation is the contract between the component and every team that consumes it, and weak documentation is the main reason design systems fragment: consumers cannot tell what a component supports, so they fork it, wrap it, or rebuild it. The judgment problem is deciding what a consumer actually needs to know, in what order, and how to make the limits of the component visible before it is misused.

Agents tend to fail by documenting what is easy to describe, the happy path, rather than what causes misuse: the edge cases, the forbidden combinations, the states that exist but are not obvious, and the accessibility obligations that are invisible in a screenshot.

## Core Rules

### Document The Contract, Not The Implementation

Consumers need to know what the component does, what it accepts, and what it guarantees, not how it is built internally.

Document the public interface:

- the purpose;
- the inputs or props;
- the outputs or events;
- the visual variants;
- the behavioral contract.

Avoid leaking implementation details that consumers might come to depend on and that then become impossible to change. If an internal detail is observable, decide deliberately whether to document it as supported or to mark it as unstable. The boundary between contract and implementation is the most important decision in component documentation, because once consumers rely on an undocumented behavior, it becomes part of the de facto contract.

### Enumerate Variants, States, And Compositions Explicitly

A component is not one thing; it is a matrix of variants and states, and undocumented states are the source of most bugs.

List every dimension the component exposes:

- variants, sizes, tones, and emphasis levels;
- every state: default, hover, focus, active, selected, disabled, loading, error, empty, read-only, and any component-specific state such as expanded or filtering;
- how the component composes with others: what it can contain, what can contain it, and what combinations are unsupported.

Where a combination is technically possible but unsupported, say so explicitly rather than letting consumers discover the failure. A state that exists in code but not in documentation will be used inconsistently or forgotten entirely.

### Lead With Purpose And When Not To Use

The first thing a consumer needs is not the prop table; it is a one-sentence statement of what problem the component solves and when it is the wrong choice. Document the intended use cases and, equally important, the cases where a different component is correct.

For example: "Use a dialog for a blocking task that must be completed or dismissed; do not use a dialog for non-urgent information, where a toast or inline message is correct." This guidance prevents the most common adoption error: grabbing the closest component instead of the right one. Without a "when not to use" section, components get stretched into roles they were not designed for.

### Make The Do And Don't Examples Concrete

Abstract usage rules are interpreted loosely. Pair each important usage rule with a concrete do example and a concrete don't example, with a short note on why the don't is wrong.

When building examples:

- show real content, not placeholder text, because placeholder content hides real problems like overflow, truncation, and long labels;
- make don'ts explicit, because they are often more instructive than dos, naming the failure mode the rule exists to prevent;
- keep examples aligned with the current component API, since a don't that references an old prop is worse than none.

### Document Accessibility As A First-Class Concern

Accessibility is not an appendix. For each component, document the keyboard behavior, the expected focus management, the required labels and roles, the screen reader experience, and any consumer obligations.

State explicitly:

- what the component guarantees;
- what the consumer is responsible for;
- any known limitations, such as imperfect screen reader support in a certain mode.

Accessibility failures often come from the seam between component and consumer, so the split of responsibility must be visible. An accessibility section that only says "this component is accessible" is not documentation.

### Specify Responsive, Internationalization, And Theming Behavior

A component that works at one breakpoint in English in light mode is not fully documented. These dimensions are where components break in production, and they are almost always under-documented.

Document each of the following:

- responsive behavior at different viewport sizes;
- long-content handling: truncation, wrapping, and overflow;
- internationalization: right-to-left behavior, text expansion under translation, date and number formatting, and any locale-dependent logic;
- theming: which design tokens the component consumes, how it responds to dark mode or custom themes, and what a consumer must override to re-skin it.

### Keep Documentation Versioned And Co-Located

Component documentation drifts from the component faster than any other artifact. Keep the documentation co-located with the component where practical, version it with the component, and record a changelog of breaking changes to the public contract.

When the component API changes, update the documentation in the same change. Documentation that lives in a separate, manually maintained system will always be stale, and stale documentation is actively harmful because it misleads consumers with confidence.

## Common Traps

### Documenting Only The Happy Path

Showing the default state and the most common variant hides the states and combinations where misuse happens. Enumerate the full matrix.

### No "When Not To Use" Guidance

Without negative guidance, consumers reach for the closest component instead of the right one, and the component gets misapplied.

### Leaking Implementation Details As If They Were Contract

When internal structure, class names, or private props appear in docs, consumers depend on them and the component becomes unchangeable. Keep the contract explicit and minimal.

### Accessibility Treated As A Boilerplate Line

A single sentence claiming accessibility is not documentation. State the keyboard, focus, labeling, and consumer obligations concretely.

### Placeholder Content In Examples

"Lorem ipsum" and "Label" hide overflow, truncation, and real-world layout failures. Use realistic, long, and edge-case content.

### Undocumented States That Exist In Code

A state the code supports but the docs omit gets used inconsistently or forgotten. If it exists, document it or remove it.

### Documentation That Drifts From The Component

Docs maintained separately and updated by hand become wrong within a few releases. Co-locate and version them with the component.

## Self-Check

- [ ] The component's purpose is stated in one sentence, with explicit "when not to use" guidance pointing to the correct alternatives.
- [ ] The public contract, props, events, variants, and behavioral guarantees are documented separately from implementation details.
- [ ] All variants and states are enumerated, including disabled, loading, error, empty, focus, and component-specific states.
- [ ] Each significant usage rule has a concrete do and don't example with realistic content and a reason for the don't.
- [ ] Accessibility is documented as a first-class section: keyboard behavior, focus management, roles, labels, and consumer obligations.
- [ ] Responsive behavior, long-content handling, internationalization, and theming are documented, not assumed.
- [ ] Compositions and unsupported combinations are stated explicitly, not left for consumers to discover by failure.
- [ ] Documentation is co-located or versioned with the component and has a changelog for breaking contract changes.
- [ ] No internal implementation detail is documented as supported unless that was a deliberate decision.
- [ ] A consumer could adopt the component correctly without reading its source or asking its author.
