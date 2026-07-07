---
name: token_naming_and_scaling.md
description: Use when the agent is naming design tokens, structuring color, spacing, typography, radius, and motion scales, choosing numeric versus semantic naming, deciding scale steps and ratios, or refactoring token names without breaking consumers.
---

# Token Naming And Scaling

Token names are read far more often than they are written, and they outlive the people who chose them. A name like `color-brand-1` or `spacing-medium` feels reasonable on day one and becomes opaque within a year, when no one remembers whether `medium` was 12px or 16px, or whether `brand-1` is the primary or the accent. Naming and scaling decisions determine whether a token system is usable, evolvable, and resistant to drift. They are also the decisions most often made casually, because choosing a name feels trivial until hundreds of components depend on it.

Use this skill before naming new tokens, restructuring a scale, choosing between numeric and semantic naming conventions, or renaming existing tokens. The goal is to prevent the agent from choosing names that encode the wrong information, building scales that invite off-step values, or renaming tokens in ways that silently break consumers.

## Core Rules

### Decide Explicitly Between Numeric And Semantic Naming

There are two broad naming philosophies, and each has a cost. The choice should be deliberate, not accidental.

Numeric (or scale-based) naming: `gray-100` through `gray-900`, `space-2` through `space-16`, `text-sm` / `text-md` / `text-lg`.

- Strengths: stable, theme-independent, easy to keep coherent, survives re-skinning.
- Weaknesses: encodes no intent; consumers must know which step to use for which role.

Semantic (or role-based) naming: `text-primary`, `surface-background`, `border-subtle`, `spacing-gap`.

- Strengths: encodes intent, self-documenting, survives scale changes.
- Weaknesses: more tokens to maintain, names can drift from meaning over time.

The robust pattern is to use both, in tiers: numeric names for primitives (the inventory) and semantic names for the roles that components consume. A consumer should almost never reference `gray-700` directly; it should reference `text-primary`, which the theme maps to `gray-700` or `gray-200` depending on mode.

### Encode Meaning, Not Implementation, In Semantic Names

A semantic token name should describe the role, not the value or the component that happens to use it. Names that leak implementation become wrong when the implementation changes.

Strong semantic names:

- `text-primary`, `text-muted` (role in the type hierarchy);
- `surface-raised`, `surface-overlay` (role in elevation);
- `border-focus`, `border-subtle` (role in separation);
- `fill-destructive`, `fill-success` (role in meaning).

Weak names that leak implementation:

- `blue-500-button` (couples role to a color and a component);
- `header-gray` (couples role to a location and a value);
- `modal-shadow` (couples role to one component);
- `light-gray-border` (couples role to a value that changes in dark mode).

A name is strong if it stays correct when the value, theme, or consuming component changes.

### Make Scale Steps Predictable And Discoverable

A scale is only useful if a contributor can predict what steps exist and choose among them. Numeric scales should follow a consistent progression; named scales should follow a consistent set of modifiers.

For numeric scales:

- use a consistent ratio or doubling pattern (4, 8, 16, 32 or a modular scale);
- keep step labels aligned across scales so `space-4` and `radius-4` feel related where appropriate;
- document the progression so contributors do not invent off-step values.

For named scales:

- use a small, consistent set of modifiers (`sm`, `md`, `lg`, `xl` or `subtle`, `default`, `strong`);
- apply the same modifiers across categories so the vocabulary is learnable;
- avoid mixing ordinal systems (`small`, `medium`, `large` alongside `100`, `200`, `300`).

When a contributor cannot guess the next step, they invent a new token, and the scale fragments.

### Resist Off-Step And One-Off Values

The greatest threat to a scale is the off-step value: a spacing of 14px added because 12 was too small and 16 was too big. Each off-step value weakens the coherence of the scale and invites the next one, until the scale is noise.

Handle requests for intermediate values by:

- adjusting the design to use the nearest existing step;
- examining whether the scale has a real gap that many designs hit (which justifies a new step for everyone);
- resisting the urge to add a token for a single use case.

A scale with a few well-chosen steps is more usable than one with many ad-hoc steps.

### Name For Stability And Future Evolution

Token names are expensive to change because they propagate into code, design files, documentation, and contributor habits. Name for the long term.

Practices:

- avoid names tied to transient branding ("startup-green") that will not survive a rebrand;
- avoid names tied to specific features ("dashboard-blue") that may be reused elsewhere;
- prefer intent-based names that survive reorganization;
- prefer plural categories that can grow ("color", "space", "radius", "shadow") over singular locked ones;
- leave room in numeric scales for future steps (do not max out at 5 if growth is likely).

A name that is correct for two years beats one that is perfect for two months.

### Plan Renames And Deprecations With Aliases

Renaming a token is rarely safe to do in one step. Consumers across code, design files, and documentation will break if the old name simply disappears. Plan renames as migrations.

Migration practices:

- introduce the new name and keep the old name as an alias;
- emit deprecation warnings or notes when the old name is used;
- provide a clear timeline and migration path;
- update documentation and examples to the new name;
- only remove the old alias after consumers have migrated, and communicate the removal.

A rename without an alias period is a breaking change disguised as a refactor.

### Keep Names Consistent In Structure And Casing

Inconsistency in structure (category-attribute-scale versus attribute-category-scale) and casing (camelCase versus kebab-case) makes a token system harder to learn and transform across platforms.

Choose conventions and enforce them:

- a consistent segment order, such as `category-property-modifier` (`color-text-primary`, `space-gap-md`);
- a consistent separator (usually hyphen);
- a consistent casing per platform, documented in the transformation rules;
- a consistent pluralization (decide whether categories are singular or plural and stick to it).

Inconsistency is not a style preference; it breaks searchability, autocomplete, and cross-platform transformation.

### Document The Scale's Logic, Not Just Its Values

A scale documented only by its values invites misuse. Document the logic so contributors understand when to use which step.

Useful documentation:

- the ratio or progression and why it was chosen;
- the intended use of each step with examples;
- the modifiers and their meaning across categories;
- which steps are encouraged, discouraged, or reserved;
- how to request a new step and the bar for adding one.

## Common Traps

### Numeric Names In Component Code

Consuming `gray-700` directly in a component couples the component to a value and breaks theming; numeric names belong in the primitive tier only.

### Names That Leak Implementation

Names like `blue-button` or `header-gray` become wrong the moment the color, location, or component changes, and they mislead anyone reading the code later.

### Mixed Naming Systems

Combining `small`/`medium`/`large` with `100`/`200`/`300` and `sm`/`md`/`lg` in one system makes the vocabulary unlearnable and forces contributors to guess.

### Off-Step Values That Fragment The Scale

Each intermediate value added for a single use case weakens the scale and lowers the bar for the next one, until coherence is lost.

### Names Tied To Transient Branding Or Features

Tokens named after a current brand color or feature become misleading after a rebrand or reorganization and resist reuse.

### Renaming Without Alias Migration

Removing an old token name without an alias period breaks every consumer silently and turns a refactor into an incident.

### Inconsistent Segment Order And Casing

Shuffling the order of category, property, and modifier, or mixing casings, breaks search, autocomplete, and cross-platform transformation.

### Scales Documented Only By Value

A scale whose documentation lists values without intent, examples, or the logic of the progression invites misuse and off-step additions.

## Self-Check

- [ ] The system uses numeric names for primitive tokens and semantic names for the roles components consume, and the choice between conventions is deliberate.
- [ ] Semantic token names encode role and intent, not value, color, component, or location that would become wrong when those change.
- [ ] Scale steps follow a consistent ratio or modifier set, are predictable, and are documented so contributors can choose without inventing new steps.
- [ ] Off-step and one-off values are resisted; intermediate needs are met by adjusting the design or, rarely, by adding a step for everyone.
- [ ] Token names avoid transient branding and feature-specific references so they survive rebranding and reorganization.
- [ ] Renames and deprecations use aliases with a migration path, deprecation signals, and a communicated removal timeline.
- [ ] Segment order, separators, casing, and pluralization are consistent across the system and documented for cross-platform transformation.
- [ ] Each scale documents its progression logic, intended use per step, modifier meanings, and the bar for adding new steps, not just its values.
- [ ] The naming conventions were reviewed by imagining a contributor encountering the system cold, to confirm the names and scales are learnable without tribal knowledge.
