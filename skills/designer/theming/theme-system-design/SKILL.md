---
name: theme_system_design.md
description: Use when the agent is designing or building a theme system, design tokens, color spacing typography scales, multi-theme support, white-labeling, CSS variables, token architecture, or a reusable theming layer across products and platforms.
---

# Theme System Design

A theme system is the contract between design intent and implementation reality. It decides how color, spacing, typography, elevation, and motion are named, stored, overridden, and consumed across components and platforms. A weak system forces every new feature to hardcode values, drifts silently from the brand, and makes a redesign or a new theme unbearably expensive. A strong system makes consistency the default and change cheap.

Use this skill before defining design tokens, structuring a theme layer, adding multi-brand or white-label support, migrating hardcoded styles to tokens, or building a theming architecture that must survive multiple products, teams, and platforms. The goal is to prevent the agent from shipping a token set that is too literal, too shallow, or too rigid to be maintained.

## Core Rules

### Separate Semantic Tokens From Primitive Tokens

The most important decision is the layering of tokens. Primitive tokens hold raw values: a hex color, a pixel size, a font weight. Semantic tokens hold intent: `surface-background`, `text-primary`, `border-subtle`, `spacing-content`. Components should consume semantic tokens, never primitives.

This separation is what makes theming possible. When a component references `#1A73E8` directly, recoloring requires editing every component. When it references `accent-primary`, recoloring is a single token reassignment. Primitive tokens still exist, but they feed semantic tokens, and only semantic tokens reach the UI.

### Name Tokens By Intent And Role, Not By Appearance

Names like `blue-500`, `large-text`, or `button-radius` age badly. When the brand changes from blue to green, `accent-primary` still makes sense; `blue-500` becomes a lie. When a radius is reused for cards and not buttons, `button-radius` misleads.

Prefer names that describe role and usage:

- `surface-raised` over `card-bg`;
- `text-secondary` over `gray-text`;
- `border-focus` over `blue-outline`;
- `spacing-section` over `large-gap`.

Role-based names survive redesigns, theme additions, and platform ports.

### Decide The Token Categories You Actually Need

A complete theme system usually covers more than color. Decide which categories the product needs and define each deliberately:

- color: background, surface, text, border, status, accent, overlay;
- typography: family, size, weight, line height, letter spacing;
- spacing: inset, stack, inline, section, component gaps;
- layout: width, breakpoint, grid, container;
- elevation: shadow, z-index, overlay opacity;
- motion: duration, easing, distance;
- radius: control, container, modal;
- border: width, style.

Do not create tokens for values that never vary. Over-tokenizing creates noise and maintenance cost without theming benefit.

### Design For Multiple Themes From The Start, Even With One Theme Now

If there is any chance of dark mode, a second brand, a white-label customer, or a seasonal theme, build the architecture for it immediately. Retrofitting multi-theme support into a single-theme system is far more expensive than adding a second theme to a layered system.

Concretely:

- keep primitive and semantic layers separate;
- make theme values swappable through a single source;
- avoid hardcoding values in components, even once;
- document which tokens are theme-dependent.

### Make Overrides Explicit And Traceable

Components sometimes need a value outside the system. The question is whether that exception is visible. Hidden overrides destroy consistency. Prefer mechanisms where an override is declared, named, and reviewable, rather than buried in a style attribute or a one-off constant.

Ask before allowing an override:

- is this a genuinely new role that should become a token?
- is this a one-off that should be flagged for cleanup?
- will this override survive a theme change correctly?

If overrides proliferate, the system is missing tokens, not too strict.

### Plan For Platform And Code Consumption

Tokens must be consumable where they are used: CSS variables, Tailwind config, a JS object, native iOS/Android resources, design tool variables. Decide the source of truth and the generation pipeline. A token defined only in Figma but absent from code, or only in CSS but missing from native, creates drift.

Document:

- where tokens are authored;
- how they are transformed for each platform;
- how naming maps across formats;
- how versioning and breaking changes are handled.

### Version And Communicate Breaking Changes

Theme systems evolve. A renamed or removed token breaks every consumer. Treat the token API like a public API: version it, deprecate before removing, and communicate changes. Maintain a changelog of token additions, renames, and removals so teams can migrate deliberately.

### Document The Decision Rules, Not Only The Tokens

A token list without rules degrades. Document when to use `text-primary` versus `text-secondary`, when to use `surface-raised` versus `surface-overlay`, and how status colors combine with text and icons. The rules preserve intent as teams grow.

## Common Traps

### Components Consuming Primitive Tokens

When components read raw color or size values, theming requires editing each component. This is the single most common reason theme systems fail to deliver value.

### Naming Tokens By Current Appearance

`blue-500` and `large-gap` become wrong after the first redesign. Intent-based names survive change.

### Over-Tokenizing Every Value

Creating a token for every pixel and shade produces a sprawling system that no one maintains. Token only what varies or carries shared intent.

### Single-Theme Architecture That Cannot Extend

Building for one theme with hardcoded values makes dark mode, white-labeling, or rebrand a rewrite. Layering is cheap early and expensive late.

### Hidden Overrides Scattered In Components

One-off values buried in styles erode consistency invisibly. If overrides are needed, make them explicit and reviewable.

### Drift Between Design Source And Code

Tokens defined in design tools but missing from code, or vice versa, produce interfaces that look right in mockups and wrong in production.

### No Deprecation Path For Token Changes

Renaming or removing tokens without migration breaks consumers silently. Treat tokens as a versioned API.

## Self-Check

- [ ] Primitive tokens (raw values) and semantic tokens (intent) are separate layers, and components consume only semantic tokens.
- [ ] Token names describe role and intent, not current appearance, so they survive redesigns and rebrands.
- [ ] The token categories cover the dimensions the product actually needs without over-tokenizing constants that never vary.
- [ ] The architecture supports multiple themes even if only one exists now, avoiding a future rewrite.
- [ ] Overrides are explicit, named, and reviewable rather than hidden in inline styles or one-off constants.
- [ ] Tokens are generated and consumable across every platform the product ships on, with a documented source of truth.
- [ ] Token additions, renames, and removals are versioned with a changelog and migration path.
- [ ] Decision rules for when to use each token are documented, not only the token values.
- [ ] Hardcoded values in components were audited and either converted to tokens or flagged as deliberate exceptions.
- [ ] The system was tested by adding a second theme or dark mode and confirming components adapt without per-component edits.
