---
name: interaction_consistency.md
description: Use when the agent is establishing or auditing consistency across an interface, including consistent controls, gestures, terminology, placement, icons, state treatments, interaction models, and behavioral patterns, so users can transfer learning between screens and surfaces.
---

# Interaction Consistency

Consistency is not visual repetition. It is the property that lets a user learn an interaction once and apply it everywhere. When the same action is triggered differently across screens, when the same icon means different things, or when similar controls behave in incompatible ways, the user must relearn the product on every surface. The cost is slower task completion, more errors, more support load, and lower trust. The hardest part of consistency is that each inconsistency looks reasonable in isolation; the damage only appears across the whole product.

Use this skill before finalizing a design system, reviewing a multi-screen flow, adding a new surface to an existing product, or merging patterns from different teams. The goal is to prevent the agent from introducing locally sensible but globally inconsistent interactions that fragment the user's mental model.

## Core Rules

### Define What Must Be Consistent

Consistency has several layers, and each must be decided deliberately. Before designing or auditing, enumerate the dimensions that should stay stable across the product:

- control choice: the same kind of action uses the same kind of control;
- terminology: the same concept uses the same word everywhere;
- placement: the same action appears in the same location;
- iconography: the same meaning uses the same icon;
- state treatment: the same state looks and behaves the same;
- gesture mapping: the same gesture produces the same effect;
- feedback: the same outcome produces the same confirmation;
- motion: the same transition uses the same timing and direction.

Decide which of these are strict rules and which allow contextual variation. Consistency without any flexibility becomes rigid; consistency without boundaries becomes meaningless.

### Distinguish Internal From External Consistency

Internal consistency means the product agrees with itself. External consistency means the product agrees with platform and industry conventions. Both matter, and they can conflict.

When a platform has a strong convention, following it usually serves users better than inventing a custom pattern. A back gesture, a pull-to-refresh, a tab bar, a context menu, or a standard form control carries expectations users already hold. Violating an external convention forces relearning and increases error rates.

Prefer external consistency for fundamental patterns users already know. Prefer internal consistency once a product-specific convention is established, so that new surfaces extend the model rather than breaking it.

### Make The Same Action Behave The Same Way

The most damaging inconsistency is behavioral. If deleting an item is a swipe in one list, a button in another, and a menu item in a third, users cannot predict how to delete anything. If saving is automatic in one editor and manual in another, users lose data or duplicate work.

For recurring actions, define a canonical interaction and apply it everywhere:

- create, edit, delete, duplicate, and archive;
- select, multi-select, and clear;
- filter, sort, and search;
- navigate forward, back, and up;
- submit, cancel, and undo;
- expand, collapse, and disclose.

When a surface genuinely needs a different interaction, make the deviation intentional, documented, and justified by a task-specific reason, not by who built it.

### Keep Terminology Stable

Words build the mental model. If the same object is called a project in one place, a workspace in another, and a folder in a third, users cannot form a reliable map of the product. If the same action is called save, apply, update, and confirm in different contexts, users hesitate before every commit.

Maintain a controlled vocabulary for core nouns and verbs. Each term should have one meaning, and each meaning should have one term. Avoid synonyms unless a genuine audience difference requires them, and even then, document the mapping.

### Align Icons With Meaning

An icon is a promise of meaning. When the same icon represents different actions, or when the same action uses several icons, the interface becomes a guessing game. A gear should not mean settings in one place and configuration in another. A trash icon should not mean delete in one place and archive in another.

Audit icons for one-to-one mapping between symbol and meaning. When an icon must be reused, ensure the contexts are close enough that users will not be misled, and prefer distinct symbols for distinct actions.

### Keep Placement Predictable

Users develop muscle memory for where things are. Primary actions, search, navigation, settings, help, and account controls should live in predictable locations. Moving them between surfaces forces users to hunt and breaks flow.

This does not mean every screen is identical. It means the location of a given function should be stable relative to the surface type. A primary action belongs in a consistent region; a settings entry belongs in a consistent menu; a back control belongs in a consistent corner.

### Document The Model So It Survives Team Turnover

Consistency decays when it lives only in one designer's head. Each new contributor reinvents patterns unless the model is written down. Maintain a living record of interaction conventions, terminology, icon mapping, and state treatments, and require new surfaces to reference it.

Documentation is what turns a one-time design effort into a durable system property. Without it, consistency erodes with every pull request.

### Allow Deliberate Variation, Not Accidental Drift

Not every surface should be identical. A marketing page, a dense admin tool, and a mobile app legitimately differ. The goal is not uniformity; it is coherence. Variation is acceptable when it serves a different task, audience, or context, and when it does not break the core model.

The test is whether a user who learned one surface can predict the next. If they can, the variation is coherent. If they must relearn basic actions, the variation is drift.

## Common Traps

### Local Optimization That Breaks The Global Model

A team optimizes one screen in isolation and introduces a pattern that contradicts the rest of the product. Locally it looks like an improvement; globally it fragments the model.

### Reusing Icons For Different Meanings

A familiar icon borrowed for convenience ends up representing a different action, misleading users who transfer the original meaning.

### Synonym Drift In Copy

Different writers use save, apply, update, and submit interchangeably, and users no longer know what each button actually does.

### Ignoring Platform Conventions

Inventing custom versions of standard controls, such as a custom select or a non-standard back gesture, increases learning cost and accessibility risk for marginal visual gain.

### Inconsistent Same-Action Behavior

The same action triggered by different controls, in different places, with different feedback, makes the product feel unreliable.

### Undocumented Conventions

Patterns that exist only in tribal knowledge disappear when their owners leave, and new work silently diverges.

### Confusing Visual Uniformity With Behavioral Consistency

Making everything look the same while it behaves differently is worse than honest variation, because it sets an expectation the product then breaks.

## Self-Check

- [ ] The dimensions of consistency (control, terminology, placement, icon, state, gesture, feedback, motion) are explicitly defined.
- [ ] The product follows platform and industry conventions for fundamental patterns unless a deliberate, justified reason exists to deviate.
- [ ] Recurring actions such as create, edit, delete, select, filter, navigate, and submit behave the same way across all surfaces.
- [ ] Core nouns and verbs use a controlled vocabulary with one term per meaning and one meaning per term.
- [ ] Icons have a one-to-one mapping with meaning, and reuse is deliberate and non-misleading.
- [ ] Placement of primary actions, search, navigation, settings, and help is predictable across surface types.
- [ ] Interaction conventions, terminology, and state treatments are documented and accessible to all contributors.
- [ ] Variations between surfaces are deliberate, task-driven, and coherent rather than accidental drift.
- [ ] A user who learned one surface could predict the core interactions on a new surface.
