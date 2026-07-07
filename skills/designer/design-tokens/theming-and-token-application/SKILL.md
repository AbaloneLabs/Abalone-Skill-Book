---
name: theming_and_token_application.md
description: Use when the agent is applying design tokens to components and pages, implementing light and dark themes, multi-brand or white-label theming, high-contrast or accessibility themes, density modes, or deciding which token a given surface should consume.
---

# Theming And Token Application

Token architecture defines the layers; theming and application decide which token a given surface actually consumes, and how those tokens swap when the context changes. This is where good token systems are won or lost in practice. A perfectly tiered token set still produces broken dark modes, mismatched brands, and inaccessible contrast if components consume the wrong semantic token, if a theme swap leaves some surfaces un-themed, or if a component hard-codes a value "just this once." Application discipline is what makes tokens actually deliver the theming they promise.

Use this skill before applying tokens to a component or page, before implementing a new theme, before shipping dark mode or a second brand, and before auditing why a surface looks wrong in a non-default theme. The goal is to prevent the agent from consuming tokens by proximity rather than intent, from leaving theme gaps, or from treating theming as a color-swap exercise that ignores contrast, density, and semantic meaning.

## Core Rules

### Consume Tokens By Intent, Not By Proximity

When applying a token to a surface, choose the token whose intent matches the surface's role, not the token whose current value happens to look right. A border that separates content should consume a `border-subtle` or `border-default` semantic token, not `gray-200`, even if `gray-200` is the value `border-subtle` resolves to today.

Consuming by intent delivers three guarantees that consuming by value cannot:

- the surface re-themes correctly when the semantic token is overridden;
- the surface's role is self-documenting in the code;
- a future designer adjusting `border-subtle` updates every surface that shares that intent.

If you find yourself asking "which gray is this border?", you are consuming by value. The right question is "what is this border for?".

### Map Every Visual Role To A Semantic Token Before Theming

Before writing a theme, enumerate the semantic roles your surfaces actually use, so theming becomes a complete mapping rather than a series of patches. Typical roles include:

- text: primary, secondary, muted, disabled, inverse;
- surface: background, raised, sunken, overlay;
- border: subtle, default, strong, focus;
- fill: primary action, destructive, success, warning, info;
- state: hover, active, focus, selected, error.

A theme is then a complete assignment of primitives to these roles. Gaps in the role map are the most common cause of un-themed surfaces: a role that was never named gets no value in the dark theme and falls back to a light default.

### Treat Theming As More Than Color Swapping

A naive theme swap recolors surfaces and assumes everything else holds. Real themes differ in more than hue. Dark mode in particular changes contrast relationships, elevation perception, and which borders are visible.

Account for:

- contrast: a border that is subtle in light mode may vanish in dark mode, or become too harsh;
- elevation: shadows are often invisible or wrong in dark mode; surfaces may need lighter fills to imply depth;
- imagery and icons: logos, illustrations, and icons designed for light backgrounds may need dark variants;
- charts and data visualization: palettes that work on white often fail on dark and need re-tuning;
- density and focus: focus rings and selection states may need different treatment to remain visible.

A theme that only swaps text and background colors will leave borders, shadows, charts, and media looking broken.

### Guarantee No Surface Is Left Un-Themed

The most common dark-mode bug is a surface that was never connected to a semantic token and so retains its light value. These gaps are usually invisible in the default theme and glaring in the alternate one.

Prevent gaps by:

- forbidding hard-coded color values in component code (enforce via lint or review);
- auditing every surface in the alternate theme, not just the primary ones;
- treating any literal color as a defect to be tokenized or justified;
- providing a theme-agnostic fallback so an un-themed surface degrades gracefully rather than glaring.

A single hard-coded white background in a modal can ruin an otherwise complete dark theme.

### Design Theme Switching That Respects User Context

How and when a theme applies matters as much as the theme itself. Users have preferences, environments, and accessibility needs that the switching mechanism should respect.

Design switching so that:

- the system respects `prefers-color-scheme` and OS-level settings by default;
- the user's explicit choice overrides the system default and persists;
- theme does not reset on navigation, reload, or login;
- embedded contexts (widgets, emails, print) choose an appropriate theme explicitly rather than inheriting unpredictably;
- the transition between themes does not flash the wrong theme on load (flash of incorrect theme).

A theme switcher that resets, flashes, or ignores the OS preference feels broken even when the themes themselves are correct.

### Handle High-Contrast And Accessibility Themes Deliberately

Beyond light and dark, some users need high-contrast themes, often required by operating systems and accessibility regulations. These themes are not recolorings of the standard palette; they demand stronger contrast, clearer focus indicators, and removal of subtle decoration.

For accessibility themes:

- ensure text and UI meet the required contrast ratios at the required sizes;
- replace subtle borders and shadows with solid, high-contrast equivalents;
- make focus indicators unmistakable, not dependent on color alone;
- preserve all semantic meaning (status, error, success) through text or icon, since color nuance is reduced;
- test with the OS high-contrast mode actually enabled, not by approximation.

### Apply Density And Scale Modes As Separate Axes

Density (compact versus comfortable) and scale (regular versus large text) are independent of color theme and should be applied as separate token axes. Conflating them couples changes that users want independently.

Separate axes for:

- color theme (light, dark, brand);
- density (compact, comfortable, spacious);
- text scale (default, large, extra-large for accessibility);
- platform (web, iOS, Android).

A user who wants dark mode and comfortable density should not be forced to choose one.

### Validate Token Application Against Real Content

Tokens that look correct with placeholder content often fail with real data. Long text, localized strings, user-generated content, and edge-case values stress the application in ways the design tools do not.

Validate by checking surfaces with:

- the longest plausible strings and localized variants;
- empty, single-item, and overflow states;
- high-contrast and dark themes simultaneously;
- the largest text scale setting;
- real images and media, not placeholder assets.

## Common Traps

### Consuming The Token That Looks Right Today

Choosing a token by its current value rather than its intent works in the default theme and breaks the moment the semantic token is overridden.

### Incomplete Role Maps

If a semantic role was never named, it receives no value in alternate themes and falls back to a light default, producing glaring un-themed surfaces.

### Color-Only Theme Swaps

Swapping text and background while ignoring borders, shadows, charts, icons, and elevation leaves dark mode looking half-finished and often inaccessible.

### Hard-Coded Values "Just This Once"

A single literal color in a component evades the token system, cannot be themed, and becomes the surface that ruins the dark theme.

### Flash Of Incorrect Theme On Load

Loading the default theme and then swapping after render causes a visible flash that is jarring and, for photosensitive users, potentially harmful.

### Ignoring OS And Accessibility Preferences

Forcing a theme or ignoring `prefers-color-scheme` and high-contrast modes disregards user needs and often violates accessibility expectations.

### Coupling Density, Scale, And Color

Tying compact density to a specific color theme, or large text to a specific brand, prevents users from combining the settings they actually want.

### Testing Only With Placeholder Content

Tokens validated against short placeholder strings and idealized images fail against real, long, localized, and user-generated content.

## Self-Check

- [ ] Each surface consumes the semantic token whose intent matches its role, not the token whose current value looks correct.
- [ ] The full set of semantic roles (text, surface, border, fill, state) is enumerated and mapped in every theme, with no unnamed roles left to default.
- [ ] Themes account for contrast, elevation, borders, imagery, charts, and focus, not only text and background color swaps.
- [ ] No hard-coded color values exist in component code, and any literal is treated as a defect to tokenize or justify.
- [ ] Theme switching respects OS preferences, persists the user's explicit choice, avoids reload or navigation resets, and prevents flash of incorrect theme.
- [ ] High-contrast and accessibility themes meet required contrast ratios, use unmistakable focus indicators, and preserve semantic meaning without relying on color nuance.
- [ ] Density, text scale, color theme, and platform are applied as independent axes so users can combine them freely.
- [ ] Token application was validated with real, long, localized, and user-generated content across light, dark, high-contrast, and large-text contexts.
- [ ] Every surface was audited in the alternate theme, not only the primary ones, to catch un-themed gaps.
