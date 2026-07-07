---
name: design_to_development_handoff.md
description: Use when the agent is preparing design work for engineering handoff, organizing Figma or design files for developers, writing handoff documentation, defining component specs, or translating visual design into instructions an engineering team can build accurately.
---

# Design To Development Handoff

Handoff is not the moment a designer drops a file into Slack. It is the structured transfer of intent from the people who decided what the product should be to the people who will build it. Done well, handoff makes the engineering team faster, reduces rework, and preserves the decisions that make the design work. Done poorly, it produces a build that looks almost right, behaves wrong in the cases no one specified, and forces weeks of clarification that could have been avoided.

The judgment problem is that designers and engineers reason about different things. A designer sees a button and thinks about hierarchy, state, and intent. An engineer sees a button and asks about its data source, its variants, its loading and error states, its accessibility role, and what happens when the text is three times longer. The gap between these views is where handoff fails. The agent's job is to anticipate what the engineer will need to know and provide it before it becomes a question, a guess, or a bug.

Use this skill before handing off designs, organizing files for developers, writing specs, or reviewing whether design work is ready to build. The goal is to prevent the agent from shipping handoff that is visually complete but operationally incomplete, forcing engineers to guess at the states, edge cases, and behaviors that determine whether the build matches the intent.

## Core Rules

### Treat Handoff As A Transfer Of Intent, Not A Transfer Of Pixels

The deliverable is not a picture. It is a shared understanding of what to build. If the engineer can reproduce the picture but not the reasoning behind it, the build will diverge as soon as reality differs from the mockup.

Transfer intent by:

- explaining why the design is the way it is, not just what it looks like;
- documenting the decisions that matter and the alternatives that were rejected;
- defining behavior, not just appearance;
- clarifying which details are fixed and which are flexible;
- identifying the user and business goals the design serves.

Engineers make dozens of micro-decisions during implementation. When they understand the intent, those decisions align with the design. When they only have pixels, they guess, and the guesses accumulate into drift.

### Specify Every State, Not Just The Happy Path

The most common handoff failure is showing only the default, populated, success state. Real products spend much of their life in loading, empty, error, partial, disabled, and edge states, and these are almost always under-specified.

For every screen and component, specify:

- loading and skeleton states;
- empty states, including first-use and after-clearing;
- error states, including network failure, validation failure, and permission denial;
- disabled and read-only states;
- partial and incomplete data states;
- long-content and overflow states;
- success and completion states.

An engineer who has to invent the empty state will invent a different one than the designer imagined. Show every state, or explicitly mark which states are deferred, so nothing is left to assumption.

### Define Component Variants And Reuse, Not One-Offs

Handoff should communicate structure, not just screens. When the same pattern appears in multiple places, define it once as a component with its variants, rather than letting engineers discover the duplication.

Define components by:

- naming them consistently with the design system and codebase;
- listing all variants, sizes, and states;
- specifying which props or parameters change;
- showing where each variant is used;
- documenting composition, how components nest and combine;
- clarifying the boundary between a variant and a new component.

This lets engineers build reusable code instead of re-implementing similar markup, and it keeps the built product consistent with the system the designer intended.

### Provide Interaction And Behavior Specifications

Visual specs answer what it looks like. Interaction specs answer what it does. Both are required, and the second is more often missing.

Specify behavior for:

- hover, focus, active, pressed, and selected states;
- transitions, timing, and easing;
- what opens, closes, navigates, or updates on each action;
- keyboard and screen-reader behavior;
- gestures and touch behavior where relevant;
- error recovery and retry flows;
- what happens on cancel, back, and interruption.

Do not assume the engineer knows the intended timing of a transition or the focus order of a form. State it. Ambiguous behavior is the source of most "this doesn't feel right" feedback late in a project.

### Annotate Edge Cases And Content Variability

Mockups are usually built with ideal content: short names, balanced text, available images, and typical data. Real content is longer, messier, and more variable. If the handoff does not address content variability, the build will break on real data.

Annotate:

- minimum and maximum text lengths and how they wrap or truncate;
- behavior for missing, loading, and failed images;
- number formatting, currency, date, and locale considerations;
- empty and single-item cases for lists;
- very long lists and pagination or virtualization expectations;
- right-to-left and localized layouts where relevant;
- dynamic content that appears or disappears after load.

Engineers cannot defend against content they have not been warned about. Show the worst plausible content, not just the best.

### Align Naming, Structure, And Tokens With Engineering

Friction between design and engineering often comes from mismatched naming and structure. A color called "blue-500" in the design tool but "primary" in code, a spacing token that does not exist in the stylesheet, a component named differently in Figma and in the repo, all create confusion and bugs.

Align by:

- using the same names for tokens, components, and states as the codebase;
- mapping design tokens to existing code tokens where they exist;
- structuring files and frames to mirror the component hierarchy;
- confirming measurement units, base font size, and grid assumptions;
- resolving naming collisions before handoff, not during implementation.

When design and code share a vocabulary, handoff becomes verification rather than translation.

### Make Handoff Discoverable And Navigable

An engineer opening a handoff file should be able to find what they need quickly. A sprawling, inconsistently organized file wastes time and causes missed specs.

Organize for discoverability by:

- structuring frames and pages by feature or component, not by designer;
- providing an index or table of contents for large handoffs;
- linking related specs, components, and prototypes together;
- keeping annotations close to the elements they describe;
- archiving or clearly marking outdated sections;
- using consistent naming for frames so they are searchable.

A handoff no one can navigate is a handoff no one uses. Treat the file itself as a designed artifact.

### Define What Is Final, What Is Flexible, And What Is Open

Not every detail in a handoff carries the same weight. Some are fixed and must be built exactly; others are suggestions the engineer can adjust; others are genuinely open questions. Conflating these creates either rigid over-specification or dangerous ambiguity.

Clarify for each area:

- which dimensions, tokens, and behaviors are fixed;
- which are flexible within stated constraints;
- which are open and need a conversation;
- who owns the decision for open items;
- what the acceptance criteria are.

This lets engineers proceed confidently on the fixed parts, exercise judgment on the flexible ones, and flag the open ones early instead of guessing.

## Common Traps

### Handing Off Only The Happy Path

Default, populated, success states are easy to design and useless alone. Loading, empty, error, disabled, and long-content states are where builds diverge.

### Pixels Without Behavior

A static mockup cannot convey timing, transitions, focus order, or error recovery. Behavior must be specified, not inferred.

### One-Off Screens Instead Of Components

Delivering unique screens hides shared structure and forces engineers to re-implement similar code. Define components and variants.

### Mismatched Naming And Tokens

When design and code use different names for the same things, handoff becomes error-prone translation. Align vocabulary before handoff.

### Ignoring Content Variability

Ideal content hides overflow, truncation, and missing-data problems. Specify behavior for long, empty, and dynamic content.

### Unorganized Files

A sprawling handoff file wastes engineering time and causes missed specs. Structure, index, and name for discoverability.

### Treating Every Detail As Fixed

Over-specifying rigid details that do not matter, while leaving genuinely important behavior ambiguous, wastes attention and hides risk. Distinguish fixed, flexible, and open.

## Self-Check

- [ ] The handoff communicates intent and reasoning, not just appearance, so engineers can make aligned micro-decisions.
- [ ] Every screen and component specifies loading, empty, error, disabled, partial, long-content, and success states, or explicitly defers them.
- [ ] Shared patterns are defined as components with variants, sizes, states, composition, and usage, rather than as one-off screens.
- [ ] Interaction behavior, including hover, focus, transitions, timing, keyboard, gestures, error recovery, and interruption, is specified.
- [ ] Edge cases and content variability, including text length, missing images, locale, empty and single-item cases, and dynamic content, are annotated.
- [ ] Naming, structure, tokens, units, and grid assumptions align with the codebase and design system.
- [ ] The handoff file is organized, indexed, navigable, consistently named, and free of outdated sections.
- [ ] Each area is clearly marked as fixed, flexible, or open, with owners and acceptance criteria for open items.
- [ ] The handoff has been reviewed from the engineer's perspective, asking what they would need to know that is not yet stated.
- [ ] Open questions are surfaced before implementation begins, not discovered during it.
