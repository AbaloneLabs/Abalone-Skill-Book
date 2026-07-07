---
name: token_architecture.md
description: Use when the agent is structuring a design token system, deciding token tiers such as primitive, semantic, and component tokens, organizing color, spacing, typography, and elevation scales, or planning how tokens cascade across themes, brands, and platforms.
---

# Token Architecture

A design token system is a set of named decisions about color, space, type, motion, and elevation. Its value is not in the names themselves but in the layering: which tokens are raw material, which carry meaning, which are bound to components, and which can be overridden per theme or brand. Most token systems fail not because the colors are wrong but because the tiers are blurred, so a theme change ripples into component breakage, or a spacing tweak desyncs fifteen products. Architecture is the part that is expensive to fix later.

Use this skill before defining or restructuring a token system, before adding theming or multi-brand support, and before deciding how tokens map into code. The goal is to prevent the agent from building a flat list of hard-coded values, mixing primitive and semantic concerns, or creating a layering that cannot survive a theme or platform change without rewriting components.

## Core Rules

### Separate Tokens Into Tiers By Purpose, Not By Type

The single most important decision is the tier structure. Tokens that serve different purposes must live in different tiers, because they change for different reasons and at different rates.

A robust architecture uses three tiers:

- Primitive (also called global, base, raw, or alias-source): the raw palette and scales. `gray-900`, `blue-500`, `space-4`, `font-size-md`. These are inventory, not meaning. They rarely change.
- Semantic (also called contextual, purpose, or alias): tokens that assign meaning to primitives for a given context. `text-primary`, `surface-background`, `border-subtle`, `spacing-gap-medium`. These encode intent and are the primary override point for themes.
- Component (also called specific): tokens bound to a component. `button-primary-background`, `card-border-color`, `input-focus-ring`. These reference semantic tokens and let a component be tuned without touching its internals.

The discipline is that each tier references the tier above it, never skips, and never reaches back down. A component token references a semantic token; a semantic token references a primitive. When a component reaches straight to `blue-500`, the architecture is broken.

### Make Semantic Tokens The Override Point For Themes

Theming works cleanly only when themes override semantic tokens, not primitives and not components. If a dark theme redefines `text-primary` to point at a light primitive, every component that consumes `text-primary` updates correctly. If instead the dark theme redefines `button-primary-background` directly, you must theme every component individually, and new components silently break in dark mode.

Design semantic tokens around the dimensions that actually vary:

- light versus dark;
- brand versus co-brand versus white-label;
- dense versus comfortable;
- marketing versus product.

Each semantic token should answer a question of intent: "what is the background of a primary surface in this theme?" not "what is the hex value?".

### Decide What Is A Token And What Is A One-Off Value

Not every number in a design should be a token. Over-tokenizing creates noise and maintenance burden; under-tokenizing creates drift. The test is reuse and intent.

Promote a value to a token when:

- it is used in more than one place, or is meant to be consistent across places;
- it carries intent that should survive a theme change;
- it is part of a scale that should scale together;
- changing it in one place but not others would create inconsistency.

Leave a value as a literal when:

- it is a one-off adjustment tied to a specific layout;
- it is a mathematically derived value (a calc) rather than a decision;
- it is a temporary or experimental value not yet proven.

A token system with five hundred tokens is usually a sign that one-off values were promoted prematurely.

### Keep Scales Coherent Rather Than Exhaustive

A scale is a set of related tokens that should feel like steps on the same ladder. The goal is coherence, not completeness. A spacing scale of 4, 8, 12, 16, 24, 32, 48, 64 is coherent; a scale that also includes 5, 7, 10, 14, 18, 20, 28 is noise.

Coherent scales:

- use a consistent ratio or progression;
- have enough steps to cover real needs without gaps that force awkward choices;
- resist adding one-off steps that break the progression;
- are documented so contributors know which steps exist and why.

When a design needs a value between two scale steps, the right answer is usually to adjust the design to use the nearest step, not to add a new token.

### Plan The Cascade And Inheritance Explicitly

Tokens cascade: a component inherits from its context, which inherits from the theme, which inherits from the global defaults. The architecture must define this cascade so overrides are predictable.

Define:

- the default values at the root;
- which tokens a theme overrides and how (a dark theme object, a brand theme object);
- which tokens a component or section may override locally;
- what happens when overrides conflict (local beats theme beats global).

Without an explicit cascade, theming becomes a game of specificity and `!important`, and the system becomes impossible to reason about.

### Account For Platform And Code Representation

Tokens are authored once but consumed across web, iOS, Android, design tools, and sometimes email or documents. The architecture must define how tokens transform for each platform.

Consider:

- a single source of truth (often JSON or a token format like DTCG/W3C) that transforms per platform;
- naming that survives transformation (camelCase in JS, PascalCase or constants in Swift, snake_case elsewhere);
- units that translate (rem and px on web, dp and sp on Android, pt on iOS);
- which tokens are platform-agnostic and which are platform-specific (elevation and motion differ most).

A token system that only works in CSS will fragment the moment a native app needs the same brand.

### Document The Decision, Not Just The Value

A token named `space-4` with value `1rem` is half-documented. The other half is why it exists, when to use it, and what it should not be used for. Token documentation should capture intent so future contributors do not misuse or duplicate tokens.

Useful documentation includes:

- the intent of the token and a usage example;
- what it should not be used for;
- which tier it belongs to and what it references;
- which themes override it and how;
- deprecation notes when a token is being phased out.

## Common Traps

### Flat Lists Of Hard-Coded Values

A single layer of tokens with no semantic or component tier gives no theming leverage and drifts the moment two surfaces need different intent for the same color.

### Components Reaching Straight To Primitives

When a component consumes `blue-500` directly, a theme change cannot recolor it without editing the component, defeating the purpose of the system.

### Theming At The Wrong Tier

Overriding component tokens per theme, or redefining primitives per theme, multiplies the surface area of theming and guarantees that new components break silently in non-default themes.

### Over-Tokenizing One-Off Values

Promoting every literal to a token creates a bloated system where the meaningful decisions are buried among noise, and contributors cannot find what they need.

### Incoherent Scales

Scales with ad-hoc steps that break the progression force awkward choices and make the system feel arbitrary, which encourages contributors to add more one-off tokens.

### Implicit, Undocumented Cascade

When override precedence is governed by CSS specificity or import order rather than an explicit rule, theming becomes unpredictable and debugging turns into archaeology.

### Web-Only Token Systems

A system that assumes CSS units and naming fragments across native platforms, leading to parallel unofficial token sets that drift from the source of truth.

### Renaming Tokens Without Aliases

Renaming a token and removing the old name breaks every consumer. A deprecation alias period is essential for any rename in a system with real consumers.

## Self-Check

- [ ] Tokens are organized into clear primitive, semantic, and component tiers, and each tier references only the tier directly above it.
- [ ] Themes override semantic tokens rather than primitives or component tokens, so new components inherit the theme automatically.
- [ ] Each token's promotion to the system is justified by reuse or intent, and one-off values remain literals rather than bloating the token set.
- [ ] Scales for color, spacing, type, and motion are coherent, documented, and resist ad-hoc intermediate steps.
- [ ] The cascade from global default through theme to local override is explicit and predictable, not governed by specificity hacks.
- [ ] The token source of truth transforms cleanly to each target platform, with consistent naming and appropriate units.
- [ ] Each token documents its intent, usage, non-usage, tier, references, and theme overrides, not just its value.
- [ ] Renames and deprecations use aliases so existing consumers do not break during migration.
- [ ] The architecture was stress-tested against a real theme change (e.g., dark mode or a second brand) to confirm it recolors without component edits.
