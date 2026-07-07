---
name: interaction_pattern_selection.md
description: Use when the agent is choosing how users will interact with a surface, selecting interaction patterns such as drag and drop, inline edit, modal, drawer, accordion, tabs, command palette, swipe, long-press, hover menus, or multi-step flows, and must weigh discoverability, efficiency, error cost, and platform fit.
---

# Interaction Pattern Selection

Selecting an interaction pattern is not choosing the trendiest gesture or the most compact component. It is deciding how a user will reach a decision, complete a task, recover from mistakes, and build a reliable expectation of how the product behaves. The same goal can often be served by several patterns, and the correct one depends on frequency, stakes, discoverability, device, and the cost of getting it wrong.

Use this skill before specifying how a user opens, edits, navigates, filters, selects, reorders, deletes, or moves through content. The goal is to prevent the agent from picking a pattern because it looks modern, fits a single screen, or worked in another product, without analyzing whether it matches the task, the user's familiarity, and the platform.

## Core Rules

### Match The Pattern To Frequency And Expertise

Interaction patterns have a learning cost. Patterns that reward repeated use, such as keyboard shortcuts, command palettes, drag-and-drop reordering, or multi-select gestures, are efficient for frequent expert workflows but can be invisible or intimidating to first-time users. Patterns that are immediately readable, such as visible buttons, clear menus, and explicit links, are better for infrequent or first-time use.

Before choosing, estimate:

- how often a typical user performs this action per session;
- whether users are trained, self-serve, or one-time;
- whether the action is primary, secondary, or exceptional;
- whether discoverability or speed matters more.

A command palette is excellent for a power tool used daily; it is a poor default for a public form used once. A drag-and-drop board is efficient for recurring organization; it is risky for a destructive action a user performs rarely.

### Choose Discoverable Over Clever For Important Actions

If an action is important, infrequent, or consequential, it should be discoverable without prior knowledge. Hidden gestures, long-press menus, hover-only controls, and obscure shortcuts are acceptable for accelerators but dangerous as the only path to a critical function.

Ask: can a new user find this action by looking? If the answer is no, the pattern needs a visible affordance, a hint, a menu entry, or a documented path. Hidden patterns are appropriate only when the action is secondary, when the gesture is a platform convention, or when an explicit alternative exists.

### Weigh The Cost Of Errors And Recovery

Different patterns produce different failure modes. Drag and drop can cause accidental moves. Inline edit can cause unintended saves. Modals can trap users. Swipe-to-delete can trigger destructive actions on the wrong row. Multi-step wizards can lose progress.

For each candidate pattern, identify:

- the most likely mistake;
- whether the mistake is reversible;
- how the user notices the mistake happened;
- how the user undoes it.

Prefer patterns with cheap recovery for destructive or hard-to-notice actions. Add confirmation, undo, review steps, or friction when the error is costly. Prefer low-friction patterns when the action is safe, reversible, and frequent.

### Respect Platform Conventions And Input Methods

A pattern that feels native on one platform can feel broken on another. Hover menus do not exist on touch. Right-click context menus are invisible on mobile. Swipe gestures conflict with screen readers and voice control. Drag and drop is difficult with a trackpad and often impossible with assistive technology.

For each pattern, confirm:

- it works with the primary input method, including touch, keyboard, mouse, and voice;
- it does not rely on a capability the target platform lacks;
- an equivalent path exists for users who cannot perform the gesture;
- the convention matches user expectation on that platform.

When a gesture is the primary path, provide a visible, keyboard-accessible, and assistive-tech-compatible alternative.

### Decide Between Modal, Inline, And Page-Level

Where an interaction happens changes its cognitive cost. Modals focus attention but interrupt flow and hide context. Inline edits keep context but can be missed and can collide with other content. Page-level flows give space and history but cost a navigation round-trip.

Use:

- modals for focused, short, blocking tasks such as confirmation, sign-in, or a constrained choice;
- inline patterns for quick edits where surrounding context matters;
- drawers and panels for tasks that need room but should not lose the current view;
- page-level flows for multi-step, reviewable, or shareable processes;
- accordions and expandable regions for optional detail, not for primary tasks.

Avoid modals for long forms, comparison, or anything the user may want to reference alongside other content.

### Account For Data Shape And Volume

The pattern should fit the data. A select menu suits a small fixed option set. A search-driven combobox suits hundreds of options. Tabs suit a small number of peer views. Infinite scroll suits browsing many items but harms targeted access. Tables suit comparison; cards suit independent objects.

Before locking a pattern, test it mentally against:

- the smallest plausible data set;
- the largest plausible data set;
- empty, loading, and error states;
- very long names, numbers, or translated strings;
- simultaneous selections or bulk operations.

A pattern that collapses under volume or edge cases is the wrong pattern.

### Keep The Interaction Model Coherent

Within a product, the same kind of action should behave the same way. If deletion is a swipe in one list and a button in another, users must relearn each surface. If editing opens inline in one place and a modal in another, the model feels arbitrary.

Establish interaction conventions for recurring actions: create, edit, delete, reorder, filter, search, select, navigate, and submit. Document them. When a surface must deviate, make the deviation deliberate and explainable, not accidental.

## Common Traps

### Picking A Pattern Because It Looks Modern

A trendy pattern that does not match the task, audience, or platform creates friction disguised as polish. Novelty is not a design rationale.

### Hidden Gestures As The Only Path

Swipe, long-press, and hover patterns that have no visible alternative exclude users who cannot perform or discover the gesture, including assistive technology users and anyone on a different device.

### Overusing Modals

Stacking modals, using modals for long forms, or opening modals from within modals disorients users, hides context, and breaks keyboard and screen reader flow.

### Ignoring Recovery Cost

A pattern that makes mistakes easy to make but hard to notice or undo turns a small interaction into a serious support problem.

### Forgetting Empty And Error States

A pattern designed only for the happy path fails the first time the list is empty, the network is slow, or a value is missing.

### Inconsistent Same-Action Patterns

When the same action behaves differently across surfaces, users lose trust in the product and make more errors.

### Optimizing Only For Expert Speed

Designing every interaction for the fastest expert path hides functionality from new and infrequent users and raises the learning curve unnecessarily.

## Self-Check

- [ ] The chosen pattern matches action frequency, user expertise, and whether discoverability or speed matters more.
- [ ] Important, infrequent, or consequential actions have a discoverable path and are not hidden behind a gesture or hover alone.
- [ ] The most likely error for the pattern is identified, and recovery cost is acceptable, with confirmation or undo where needed.
- [ ] The pattern works across the target input methods and platforms, with an accessible alternative for any gesture-based path.
- [ ] The location of the interaction (modal, inline, drawer, page-level) matches the task's length, focus, and context needs.
- [ ] The pattern survives empty, loading, error, bulk, and large-data edge cases.
- [ ] Recurring actions such as create, edit, delete, reorder, and filter behave consistently across the product.
- [ ] Expert accelerators exist alongside, not instead of, discoverable defaults.
- [ ] The choice is justified by task and audience, not by novelty or a single reference product.
