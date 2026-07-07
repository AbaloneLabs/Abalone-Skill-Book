---
name: design_tokens.md
description: Use when the agent is defining design tokens, structuring a token hierarchy of global versus alias versus component tokens, naming tokens semantically, deciding token formats and transformation pipelines, managing token versioning, or ensuring design and code share a single source of truth for color, spacing, typography, and other design decisions.
---

# Design Tokens

Design tokens are the named variables that store design decisions as data. They look like key-value pairs, but they are really the mechanism that lets a single decision (the brand's primary color, the standard spacing unit) propagate consistently across design tools, code platforms, and documentation. Agents tend to treat tokens as a flat list of values, name them by appearance, or skip the hierarchy that makes tokens maintainable. The harm is invisible until a value must change: a brand color shift requires editing hundreds of references, aliases break, and the "single source of truth" becomes many sources of disagreement.

Use this skill before finalizing a token structure, naming scheme, or transformation pipeline. The goal is to prevent the agent from building tokens that cannot scale, from naming them in ways that block theming, or from creating a system where design and code drift apart because the token contract is undefined.

## Core Rules

### Build A Token Hierarchy, Not A Flat List

A flat list of tokens (every color and spacing value as a peer) does not scale. When the same raw value serves many purposes, changing it breaks unrelated surfaces. The solution is a layered hierarchy: global tokens hold raw values, alias tokens map those to purposes, and component tokens reference aliases. This separation lets one raw value change without rewriting every usage, and lets a purpose change without touching the raw palette.

Structure tokens in layers:

- global tokens: the raw palette and scale, such as the full color ramps and spacing steps, named without purpose (for example, blue-500, spacing-4);
- alias or semantic tokens: global tokens mapped to a purpose (color-action-primary references blue-500; spacing-component-padding references spacing-4);
- component tokens: alias tokens mapped to a specific component use (button-background references color-action-primary).

The hierarchy is what makes theming, rebranding, and dark mode tractable. A flat list forces every change to be a global find-and-replace with unpredictable consequences.

### Name Tokens By Purpose, Not By Appearance

Token names should describe what a value is for, not what it looks like. A token named blue-500 tells you the color but not its role; if the brand shifts to green, the name becomes a lie. A token named color-action-primary describes the role and survives a palette change. Purpose-based naming is what lets tokens remain valid when values change.

Name by purpose:

- semantic names describe role: color-text-primary, color-surface-background, spacing-section-gap;
- avoid appearance-based names for anything beyond the global layer: not button-blue or header-dark;
- keep names stable even when values change, so references do not break;
- use a consistent naming convention (category-property-role-scale) across all tokens.

Appearance-based names couple the token to a value that may change, defeating the purpose of indirection. Purpose-based names make tokens resilient.

### Keep The Global Layer Raw And Stable

The global token layer is the foundation: the raw color ramps, spacing scales, type sizes, and radii. This layer should be complete, stable, and rarely changed, because everything above it depends on these values. Incompleteness at the global layer forces ad-hoc values higher up, breaking consistency.

Build the global layer completely:

- provide full ramps for every color (not single swatches), so aliases have steps to choose from;
- define a complete spacing scale derived from a base unit, not a handful of arbitrary values;
- include type sizes, weights, line heights, border radii, shadows, and z-index scales;
- avoid gaps that force designers to invent off-token values.

A weak global layer is the root cause of most token system failures, because the missing values get filled with one-offs that fragment the system.

### Ensure Design And Code Consume The Same Tokens

Tokens only create a single source of truth if both design tools and code consume them. If designers reference tokens in Figma while engineers hardcode values, the two drift and what ships does not match what was designed. The token pipeline must deliver to both sides.

Unify consumption:

- define tokens in a neutral format (such as JSON or a token specification) that transforms into platform-specific outputs;
- generate design tool variables and code variables from the same source;
- version the token source so design and code reference the same release;
- audit code and design files for hardcoded values that bypass the tokens.

When the pipeline breaks or is ignored, the system fragments silently. Regular audits catch drift before it becomes systemic.

### Plan For Theming And Dark Mode At The Alias Layer

Theming (light, dark, brand variants) is the strongest argument for the alias layer. A raw value cannot change per theme without breaking its name, but an alias can point to different global values in different themes. Designing the alias layer with theming in mind makes dark mode and multi-brand support possible without duplicating the component layer.

Design aliases for theming:

- ensure every alias has a value defined for each theme (light and dark at minimum);
- let component tokens reference aliases, not global tokens, so themes propagate automatically;
- test that semantic meaning holds across themes (error color stays distinguishable in dark mode);
- define fallback behavior when a theme lacks a value for an alias.

A token system that cannot theme is locked to one appearance. The alias layer is where theme flexibility lives.

### Govern Token Naming With A Documented Convention

Inconsistent token naming destroys findability. If one team names a token color-primary and another names it brand-main, neither can find what they need and duplication proliferates. A documented naming convention, enforced in review, keeps the system navigable.

Enforce naming convention:

- define a structure (category, property, role, scale) and apply it universally;
- document the convention with examples and anti-examples;
- lint or validate token names against the convention so violations are caught;
- evolve the convention deliberately, with migration paths for existing tokens.

A naming convention is only valuable if enforced. Without enforcement, it degrades into suggestions.

### Version And Communicate Token Changes

Tokens are consumed by many teams and platforms, so changes cannot be silent. A token rename or value change that lands without warning breaks consumers and erodes trust. A mature token system versions its changes and communicates them.

Manage token changes:

- version the token source and use semantic versioning for releases;
- maintain a changelog documenting added, changed, renamed, and removed tokens;
- provide migration paths for breaking changes, including codemods where possible;
- communicate releases through channels consumers read.

Predictable, communicated changes build the trust that keeps teams consuming the system rather than forking it.

## Common Traps

### A Flat List Of Tokens

Tokens without a hierarchy force every value change to ripple unpredictably; build global, alias, and component layers.

### Naming Tokens By Appearance

Names like blue-500 or button-dark become lies when values change; name by purpose so names survive palette shifts.

### An Incomplete Global Layer

Gaps in the raw palette and scale force ad-hoc values higher up, fragmenting the system; build the global layer completely.

### Design And Code Using Different Sources

When designers and engineers reference different token definitions, what ships does not match what was designed; unify the pipeline.

### Theming At The Wrong Layer

Trying to theme by changing global or component tokens breaks names or duplicates work; theme at the alias layer.

### Inconsistent Naming Without Enforcement

A convention that is documented but not linted degrades into suggestions; validate token names against the convention.

### Silent Token Changes

Unversioned renames and value changes break consumers without warning; version, changelog, and communicate releases.

### Hardcoded Values Bypassing Tokens

When teams hardcode values instead of using tokens, the system fragments silently; audit for off-token values regularly.

## Self-Check

- [ ] Tokens are structured in a hierarchy: global (raw values), alias (semantic purpose), and component (specific use).
- [ ] Alias and component tokens are named by purpose, not appearance, so names survive value and palette changes.
- [ ] The global layer is complete, with full color ramps, spacing scales, type sizes, radii, and shadows.
- [ ] Design tools and code consume tokens from the same versioned source, with regular audits for hardcoded values.
- [ ] Theming and dark mode are handled at the alias layer, with every alias defined per theme and tested for semantic consistency.
- [ ] A documented naming convention is enforced through linting or validation, with migration paths for evolution.
- [ ] Token changes are versioned, with a changelog, migration paths, and communication through channels consumers read.
- [ ] The token pipeline transforms a neutral source into platform-specific outputs without manual duplication.
