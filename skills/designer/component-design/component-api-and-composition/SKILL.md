---
name: component_api_and_composition.md
description: Use when the agent is designing a component's props, slots, events, API surface, composition model, default behavior, or deciding what a component accepts, exposes, and lets callers override versus keeps internal.
---

# Component API And Composition

A component's API is the contract between the people who build it and the people who use it. Once a component is adopted, its props, slots, events, and defaults become load-bearing: callers depend on them, tests assert them, and changing them costs real work. Most component pain, in fact, comes not from visual bugs but from APIs that were designed reactively, exposing whatever a caller happened to need and hiding whatever was inconvenient, until the surface became inconsistent, leaky, or impossible to extend without breaking everyone. Designing an API deliberately, before the first consumer, is far cheaper than repairing one after.

Use this skill before designing a new component, before adding props or slots to an existing one, and before deciding what callers may override versus what stays internal. The goal is to prevent the agent from creating APIs that grow by accretion, that expose implementation details, that force callers into fragile workarounds, or that cannot evolve without breaking changes.

## Core Rules

### Design The API Around Intent, Not Around Implementation

A good component API expresses what the component does and what decisions it leaves to the caller, not how it is built internally. Props should be named for the role they play in the caller's mental model.

Strong API choices:

- props named by intent: `variant`, `size`, `intent`, `isLoading`, `disabled`;
- slots named by content role: `leading`, `trailing`, `header`, `footer`, `label`;
- events named by what happened, not how: `change`, `select`, `submit`, not `internalStateUpdated`.

Weak choices leak implementation:

- props that expose internal structure (`showInnerDivider`, `useNewTooltip`);
- events that describe plumbing (`emitRowIndexFromRender`);
- slots that mirror DOM structure rather than meaning.

The test: could a caller use the component correctly without reading its source? If not, the API is leaking implementation.

### Make The Common Case The Easy Case

The most frequent use of a component should require the least configuration. If ninety percent of callers pass the same three props, those values should be the defaults, and the rare case should require the extra configuration.

Practices:

- choose defaults that match the most common, correct usage;
- make the simple case a single prop or none, not a required bundle;
- avoid forcing callers to specify things the component could infer;
- reserve required props for information the component genuinely cannot know.

When every caller overrides the same default, the default is wrong. When no caller can use the component without extensive configuration, the component is doing too much.

### Decide Explicitly What Is Configurable And What Is Fixed

A component should have a clear boundary between what callers may control and what is the component's responsibility. Blurring this boundary creates two failure modes: over-flexible components that callers must assemble correctly every time, and over-rigid components that force forks for legitimate variation.

For each aspect, decide:

- appearance variants (size, emphasis, tone): usually configurable via a small enum;
- structure and layout: usually fixed, with slots for content the caller owns;
- behavior (selection, dismissal, loading): configurable through events and props;
- internal implementation (which primitive renders, internal state machine): fixed and hidden.

Expose variation through designed props and slots, not through escape hatches like `className`, `style`, or `renderProp` that let callers reach into internals. Escape hatches are sometimes necessary, but each one is a future breaking change waiting to happen.

### Prefer Composition Over Configuration Explosion

When a component accumulates a prop for every possible variation, the API becomes a combinatorial explosion that is hard to use and impossible to test. Composition, combining smaller components, often handles the same needs more cleanly.

Use composition when:

- the variation is structural (different layouts, different content arrangements);
- the caller needs fine control over a region;
- the combinations would multiply into untestable permutations.

Use configuration when:

- the variation is a small, closed set (variants, sizes, tones);
- consistency across callers matters more than flexibility;
- the component enforces important constraints that composition would let callers violate.

The right answer is usually a designed component with a few well-chosen props and slots for the regions callers need to control.

### Define Slots For Content Callers Own

Callers often need to inject content, actions, or metadata into a component without taking over its structure. Slots (or render props, or children patterns) are the mechanism, and their design matters.

Design slots so that:

- each slot has a clear, named role rather than a generic `children` overload;
- the slot documents what kind of content belongs there and what does not;
- the component still owns the surrounding structure, spacing, and behavior;
- slots have sensible defaults so the component works without them;
- slot content respects the component's constraints (focus order, semantics, theming).

A slot that accepts anything, anywhere, with no contract is an escape hatch, not an API.

### Specify Events And Callbacks For What Callers Need To Know

Events are the other half of the contract: they tell callers what happened. Under-specified events force callers to infer state; over-specified events leak internals.

For each event:

- name it for the occurrence, not the mechanism;
- include the payload a caller realistically needs (the selected value, the submitted data, the dismissed item);
- avoid payloads that expose internal IDs, render cycles, or unstable references;
- document when the event fires and when it does not;
- decide whether the event is cancelable or observable, and document that contract.

A common failure is firing events for internal state changes that callers then depend on, locking the internal implementation forever.

### Plan For Evolution And Backward Compatibility

APIs grow. The question is not whether the component will need new capabilities, but whether adding them will break existing callers. Design for additive evolution.

Evolution-friendly practices:

- prefer props with closed, documented enums that can grow over adding free-form strings;
- avoid boolean props that cannot be extended (prefer `variant` over `isPrimary`, `isSecondary`, `isDestructive`);
- version or deprecate rather than silently change behavior;
- keep slot and event names stable; renaming is a breaking change;
- document the public contract clearly so internal refactors do not accidentally change it.

A boolean prop like `compact` becomes `compact` plus `spacious` plus `comfortable` and then collapses into confusion; a `size` enum absorbs the same growth cleanly.

### Document The Contract, Not Just The Props

Each prop, slot, and event should document its intent, valid values, defaults, and interactions with other parts of the API. Documentation that lists props without intent leaves callers to reverse-engineer behavior.

Useful documentation:

- what the prop is for and when to use it;
- valid values and the default;
- interactions and conflicts with other props;
- examples of common and edge cases;
- what the component does not support, so callers do not attempt it.

## Common Traps

### APIs That Grow By Accretion

Adding a prop every time a caller asks for something produces an inconsistent surface where similar needs are met in dissimilar ways and no one understands the whole.

### Exposing Implementation Through Props And Events

Props and events that mirror internal structure lock the implementation in place; any internal refactor becomes a breaking change.

### Defaults No One Uses

When every caller overrides the default, the default is wrong and the API is forcing redundant configuration.

### Boolean Props That Cannot Evolve

Boolean flags multiply into overlapping flags (`isPrimary`, `isDestructive`, `isGhost`) that conflict and cannot be extended without breaking callers.

### Escape Hatches As A Substitute For Design

Leaning on `className`, `style`, or generic render props to handle variation avoids design decisions and creates fragile, version-locked usage.

### Slots Without Contracts

A generic slot that accepts anything, anywhere, gives callers no guidance and lets them violate the component's semantics, focus order, and theming.

### Silent Behavior Changes

Changing what a prop does, when an event fires, or what a default is, without versioning or deprecation, breaks callers who depend on the old behavior.

### APIs Documented As Prop Lists

Documentation that enumerates props without intent, valid values, defaults, and interactions forces callers to read source or guess.

## Self-Check

- [ ] Props, slots, and events are named for intent and caller mental model, not for internal implementation, structure, or plumbing.
- [ ] The most common correct usage requires minimal configuration, and defaults match what most callers actually pass.
- [ ] There is a clear boundary between configurable appearance and behavior versus fixed internal structure and implementation.
- [ ] Variation is handled through a small set of designed props and named slots rather than combinatorial configuration or escape hatches.
- [ ] Each slot has a named role, a documented content contract, sensible defaults, and preserves the component's structure and constraints.
- [ ] Events are named for occurrences, carry only the payload callers need, avoid internal references, and document when they fire and whether they are cancelable.
- [ ] The API is designed for additive evolution: closed enums, avoid-boolean-flags, stable names, and deprecation over silent change.
- [ ] Documentation covers intent, valid values, defaults, interactions, examples, and explicitly what is not supported.
- [ ] The API was reviewed by imagining a new caller using it without reading the source, to confirm it is usable from the contract alone.
