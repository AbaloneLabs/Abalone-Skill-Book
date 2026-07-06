---
name: design_tokens_and_theming.md
description: Use when the agent is designing or scaling a design-token system, defining token tiers (primitive, semantic/alias, component), naming tokens by intent rather than raw value, architecting multi-theme support (light/dark mode, brand variants, density modes), implementing runtime theme switching via CSS custom properties and the cascade, handling SSR and FOUC during theme bootstrap, building a token pipeline across web/iOS/Android with Style Dictionary, consuming tokens in a component library or design system, driving component variant APIs from tokens, white-label or brand theming, theming third-party components, or governing tokens to prevent off-system one-off values. Use when a theming change is becoming a find-and-replace across many files, when dark mode looks wrong, or when tokens are fragmenting.
---

# Design Tokens And Theming

A design-token system is the bridge between design intent and implementation. Done well, it makes a rebrand a single config change, dark mode a swap of variable values, and a component library consistent across web, iOS, and Android. Done poorly — or not at all — it produces the slow-motion failure the `css-architecture-and-specificity` skill warns about: colors and spacings hardcoded in hundreds of places, theming impossible, and every "small" visual change a multi-day regression hunt. This skill goes deeper than that skill's token guidance: where that skill says "use tokens, not hardcoded values," this skill is about the *architecture* of a token system — the tiers, the naming, the theming mechanics, the cross-platform pipeline, and the governance that keeps the system from fragmenting the moment someone needs a value the system does not have.

The judgment problem in token work is that every shortcut feels reasonable in isolation and corrupts the system in aggregate. A developer adds `color: #4A90D9` because "it's just this one button," and now there is a color that is not in the palette, that will not update on rebrand, and that will not flip in dark mode. A team adds a `--color-primary` token but also a `--color-primary-hover` and a `--color-primary-disabled` and a `--button-primary-bg`, and now there are four competing sources of truth for "the primary color." A theming implementation reads the theme on the client after paint, and the page flashes the wrong theme on every load. The discipline is to treat tokens as a system with governance — tiers that separate raw values from intent, names that survive a rebrand, and a pipeline that keeps platforms in sync — rather than as a bag of variables.

## Core Rules

### Separate Tokens Into Tiers: Primitive, Semantic, And Component

A token system with one flat layer ("here are all the colors and sizes") fails to scale because it mixes raw values with intent. The strong structure is three tiers, each with a distinct role:

- **Primitive (or "global" / "reference") tokens** are raw, context-free values — the palette and scale itself. `color-blue-500: #3B82F6`, `spacing-4: 16px`, `radius-md: 8px`, `font-size-3: 1rem`. These are the atoms. They have no opinion about how they are used.
- **Semantic (or "alias") tokens** express intent by pointing at primitives. `color-text-primary: {color-blue-900}`, `color-surface-background: {color-gray-50}`, `color-action-primary: {color-blue-500}`, `spacing-card-padding: {spacing-4}`. These are what components should consume, because intent is stable across themes while raw values are not.
- **Component tokens** are scoped to a specific component and point at semantic tokens. `button-background-primary: {color-action-primary}`, `button-text-primary: {color-text-on-action}`. These let a component be themed independently and let a design system expose a controlled theming surface.

Why tiers matter: when a theme changes, primitive values change (dark mode swaps the palette), and semantic tokens are re-pointed (`color-text-primary` now points at a light primitive), but **components that consume semantic tokens do not change at all**. A component that consumed a primitive directly (`color-blue-900`) would break in dark mode because it baked in the assumption "text is dark blue." The tier separation is what makes theming a re-pointing exercise rather than a rewrite. The rule of thumb: components consume semantic (or component) tokens, never primitives; semantic tokens reference primitives, never hardcoded values.

### Name Tokens By Intent, Not By Value

A token name must survive a change to its value. `--color-text-primary` is a good name because it says *what the color is for*; `--color-dark-gray` is a bad name because it describes *the current value*, and the name becomes a lie the moment dark mode makes text light. The discipline:

- Name by role and intent: `text-primary`, `surface-raised`, `border-subtle`, `action-primary`, `status-danger`. These names remain correct across themes and rebrands.
- Avoid value-laden names: `blue-500` is fine *as a primitive* (it genuinely is blue-500), but `button-blue` as a semantic name is wrong — it breaks when the brand becomes green.
- Keep a consistent naming grammar (e.g., `category-property-scope-state`: `color-text-primary`, `color-action-primary-hover`) so tokens are discoverable and grep-able, and so the system does not accumulate three synonyms for the same concept.

### Implement Theming As CSS Custom Property Overrides On A Scope

The mechanism that makes multi-theme work at runtime is the CSS cascade over custom properties. Define default (light) tokens at `:root`, then override the semantic tokens under a scope selector such as `[data-theme="dark"]` or a `.theme-dark` class on a root element. Because custom properties participate in the cascade, every component that references `var(--color-text-primary)` automatically gets the overridden value when the scope applies — no component changes, no reload.

- **Choose the scope mechanism deliberately.** `prefers-color-scheme` (a media query) follows the OS preference automatically and is right for "respect the user's system setting." A `[data-theme]` attribute or class on the document element is right for an explicit user toggle (including options beyond light/dark, like a brand or density variant). The two compose: you can default to `prefers-color-scheme` and let an explicit `[data-theme]` override it. Decide which is authoritative for your product and apply it consistently.
- **Override semantic tokens, not primitives, when theming by attribute/class.** In a dark theme, re-point `--color-text-primary` to a light primitive; do not redefine every primitive. (Some systems do redefine primitives per theme; that is a valid alternative architecture, but pick one model and apply it everywhere — mixing "re-point semantic" in some themes and "redefine primitive" in others is incoherent.)
- **Apply the scope as early as possible** (see the SSR/FOUC rule below) so the first paint is already correct.

### Make Runtime Theme Switching Instant And Without Reload

A well-architected token system lets a user toggle theme and see the change immediately, with no network request and no reload, because the change is just swapping which custom-property values are in effect. The implementation sets the `[data-theme]` attribute (or class) on the document element; the cascade re-resolves every `var()` and the page re-paints. If your theming requires a reload or a re-fetch, the architecture is wrong — tokens are not actually driving the styles, or the theme is baked into generated CSS rather than into overridable variables.

### Handle SSR, FOUC, And Theme Bootstrap Correctly

The hardest theming bug is the flash of incorrect theme (FOUC) on load: the server renders with a default theme, the client reads the user's preference, and the page visibly flips. The fixes, in order of preference:

- **Set the theme before first paint.** For SSR, render the correct theme by reading the preference on the server (from a cookie or a header), so the HTML already carries the right `[data-theme]`. For client-only apps, inline a tiny blocking script in `<head>` that reads the stored/system preference and sets the attribute before the body renders. The goal is that the first painted frame is already correct.
- **Avoid layout shift from theme.** A theme change should not change layout (component sizes, spacing) — only colors and similar surface values. If switching themes shifts content, your tokens are leaking structural values into theme-specific overrides, which is a design error.
- **Prefer `color-scheme` for native form controls.** Set `color-scheme: light dark` (or the specific scheme) so native controls, scrollbars, and the browser chrome follow the theme without custom styling. Forgetting this leaves native dropdowns and scrollbars in the wrong theme even when the rest of the page is correct.

### Build A Token Pipeline When More Than One Platform Consumes Tokens

When tokens must reach web, iOS, Android, and design tools, hand-maintaining per-platform variables drifts immediately. A pipeline tool (Style Dictionary is the canonical choice) takes a single source of truth (JSON/JS token files) and *transforms* it into platform-appropriate outputs: CSS custom properties for web, Swift constants for iOS, XML resources or Kotlin objects for Android, and design-tool variables. The transformations handle format differences (e.g., color formats, spacing units, naming conventions per platform).

The discipline: maintain one source, generate everything else, and never edit generated files by hand. The pipeline is what keeps platforms in sync; the moment a team edits an Android color file directly, that platform drifts and the next rebrand misses it. Treat the source token files as the only editable layer, and make generation part of the build.

### Consume Tokens In Component Libraries Through A Stable Variant API

A design-system component library should expose theming through a controlled surface — variant props driven by tokens — rather than letting consumers pass arbitrary values. A `Button` with variants `primary | secondary | danger` that map to `color-action-*` tokens is themable and consistent; a `Button` that accepts an arbitrary `color` prop invites off-system values and defeats the token system. The rule: component variants resolve to tokens; consumers choose variants, not raw values. This is also what makes white-label/brand theming possible — a brand theme overrides the semantic tokens, and every component variant automatically reflects the brand without per-component theming code.

### Theme Third-Party Components Through Tokens, Not By Fighting Their Styles

Third-party component libraries (date pickers, rich editors, charting libraries) often ship their own styling. The clean path is a library that consumes your tokens via CSS variable overrides or a theming API; the messy path is overriding their classes with `!important`, which the `css-architecture-and-specificity` skill warns against. When integrating a third-party component, check whether it reads CSS custom properties (many modern ones do) and map its variables to your semantic tokens. If it does not support token theming, treat that as a real cost in the selection decision — you will be maintaining overrides that break on every library upgrade. Prefer token-aware libraries; isolate those that are not.

### Govern Tokens To Prevent Off-System Values

Token systems fragment not from big decisions but from a thousand "just this once" exceptions. Governance is what keeps the system coherent:

- **Make the token set the path of least resistance.** If using a token is easier than hardcoding a value (good editor autocomplete, clear documentation, lint rules that flag raw values), developers use tokens. If hardcoding is easier, they hardcode.
- **Lint for off-system values.** A stylelint rule (or equivalent) that flags hardcoded colors, spacings, and font sizes outside the token set catches fragmentation automatically. The exception path should be deliberate (an explicit allowlist or disable comment), not the default.
- **When the system lacks a needed value, extend the system, do not add a one-off.** A `padding: 13px` that "looked right" fragments the spacing scale. If 13px is genuinely needed, add it to the scale (and reconsider whether the scale is right); do not sprinkle one-offs. This is the same discipline `css-architecture-and-specificity` names, applied at the token-system level.
- **Review token additions as architecture changes.** Every new token is a long-lived commitment; treat adding one like adding a public API, not like a quick fix.

## Common Traps

### Components Consuming Primitive Tokens Directly

A component using `color-blue-900` for its text instead of `color-text-primary`. It works in light mode and breaks in dark mode because the primitive does not re-point. Components should consume semantic tokens; primitives are referenced only by semantic tokens.

### Value-Laden Token Names That Become Lies

Naming a token `--text-dark-gray` and then implementing dark mode where text is light. The name now contradicts the value. Name by intent (`--text-primary`); reserve value descriptions for primitives that genuinely are that value.

### Theming By Redefining Primitives In Some Themes And Re-Pointing Semantic In Others

An incoherent mix where the dark theme redefines primitives but the brand theme re-points semantic tokens. Pick one theming model and apply it to every theme, or the system becomes unpredictable and bugs multiply.

### Theme Flash (FOUC) Because The Theme Is Applied After First Paint

Rendering the default theme on the server and flipping on the client, so every load visibly flashes. Set the theme before first paint — server-side from a cookie/header, or a blocking inline script in `<head>` for client apps.

### Native Controls In The Wrong Theme

Custom UI is dark but native dropdowns, scrollbars, and form controls are light, because `color-scheme` was not set. Set `color-scheme` to match the active theme so the browser chrome and native controls follow.

### Editing Generated Platform Files By Hand and a Component API That Accepts Arbitrary Values

Manually editing the Android colors file or iOS constants that a pipeline generated, so that platform drifts from the source of truth and the next regeneration overwrites the edit (or, worse, the edit survives and silently diverges). Edit only the source token files; regenerate everything else.

A `Button` with a `color` prop, inviting consumers to pass off-system hex values that bypass tokens entirely. Expose variant props that resolve to tokens; do not accept raw values across the component boundary.

### Overriding Third-Party Component Styles With `!important` and one-Off Values That Fragment The Scale

Fighting a library's styles with escalating specificity and `!important`, which breaks on every library upgrade. Prefer libraries that consume CSS variables and map them to your tokens; treat non-tokenable libraries as a maintenance liability.

Adding `padding: 13px` because the spacing scale (which has 12 and 16) did not have what was needed. Each one-off makes the next person's decision harder and makes the system look optional. Extend the scale deliberately instead.

### Token Proliferation Without Governance and assuming `prefers-color-scheme` Is Enough When Users Expect A Toggle

Adding `--color-primary`, `--color-primary-2`, `--brand-blue`, and `--link-color` all meaning roughly the same thing, because there was no review of additions. Treat token creation as an architecture decision; deduplicate and keep one name per concept.

Relying only on the OS media query when the product needs an explicit light/dark/brand toggle. `prefers-color-scheme` follows the system; an explicit user choice needs a `[data-theme]`/class scope that can override it. Decide which is authoritative and compose them deliberately.

## Self-Check

- [ ] Tokens are organized into tiers — primitive (raw values like `color-blue-500`), semantic/alias (intent like `color-text-primary` referencing primitives), and component (scoped like `button-background-primary`) — and components consume semantic/component tokens, never primitives directly.
- [ ] Token names express intent (`text-primary`, `surface-raised`, `action-primary`) rather than current values (`dark-gray`, `blue-500` used semantically), so names remain correct across themes and rebrands.
- [ ] Theming is implemented as CSS custom-property overrides on a scope (`[data-theme]` attribute or class, composed with `prefers-color-scheme` where appropriate), overriding semantic tokens so components need no changes per theme.
- [ ] Runtime theme switching is instant with no reload (just an attribute/class change re-resolving the cascade); if a reload is required, tokens are not actually driving the styles.
- [ ] The theme is applied before first paint to avoid FOUC — server-rendered from a cookie/header for SSR, or set by a blocking inline script in `<head>` for client apps — and `color-scheme` is set so native controls and scrollbars follow the theme.
- [ ] Where tokens reach multiple platforms, a pipeline (e.g., Style Dictionary) generates web/iOS/Android/design-tool outputs from a single source, and generated files are never edited by hand.
- [ ] Component libraries expose variant APIs (e.g., `primary | secondary | danger`) that resolve to tokens, and do not accept arbitrary raw values across the component boundary.
- [ ] Third-party components are themed through their CSS-variable/theming API mapped to semantic tokens where possible; libraries that cannot be token-themed are recognized as a maintenance cost in the selection decision.
- [ ] Token governance is in place — lint rules flag off-system raw values, the token set is the path of least resistance, needed values extend the scale rather than fragmenting it with one-offs, and token additions are reviewed as architecture changes.
- [ ] A theme change affects only surface values (colors, etc.), not layout — if switching themes shifts content, structural values are leaking into theme-specific overrides and the design is wrong.
