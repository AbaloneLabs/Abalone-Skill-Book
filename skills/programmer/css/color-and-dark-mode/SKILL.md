---
name: color_and_dark_mode.md
description: Use when the agent is building a color system or implementing dark mode — choosing between prefers-color-scheme and an explicit theme toggle, setting color-scheme for native controls, selecting modern color spaces (OKLCH/LCH/display-p3) for perceptually uniform palettes, using relative colors and color-mix for derived shades, designing dark-mode surfaces that are not simply inverted light mode, or debugging theming that breaks because of hardcoded colors. Also covers the trap of colors baked into components that cannot re-theme, contrast that collapses in dark mode, and system colors ignored in forced-color contexts. Distinct from color-contrast-and-visual accessibility (which is in accessibility/) and design-tokens-and-theming (token architecture).
---

# Color And Dark Mode

Color theming is where a working light-mode UI silently fails the moment a second theme is required, because most color decisions are made as isolated values rather than as a system that can re-resolve. An agent that hardcodes `color: #1F2937` for text, `background: #FFFFFF` for a surface, and `border: 1px solid #E5E7EB` for a divider has built a UI that looks correct and cannot be themed without rewriting every line. Dark mode is not "light mode with the colors flipped" — it is a different visual problem with different contrast relationships, different surface elevation rules, and different failure modes. The judgment problem is not "pick nice colors" but "build a color system whose values re-resolve per theme, whose relationships (contrast, elevation, emphasis) survive the re-resolution, and whose raw values never leak into components."

Agents tend to fail dark mode in two ways: they treat it as an inversion (swap light and dark, keep the same saturations and contrasts), which produces a dark UI that is harsh, low-contrast, or visually broken; or they hardcode colors that work in the theme they were testing and silently fail in the other. The deeper failure is invisible: a color system built in sRGB and HSL produces palettes that look uneven — steps that appear equal in lightness are not, hues shift as they lighten, and a "consistent" ramp has visible bands. This skill is the color-and-theming deep dive. It complements `design-tokens-and-theming` (the token *architecture* and tiers), `css-architecture-and-specificity` (which warns against hardcoded values broadly), and the accessibility skill `color-contrast-and-visual` (which covers WCAG ratios and color-blind encoding); here the focus is the craft of color systems and dark-mode mechanics — color spaces, derived colors, theme switching, surface design, and the system-colors dimension.

## Core Rules

### Build Color As A Theme-Aware System, Never As Hardcoded Values

The foundational rule: no color used in a component should be a raw value. Every color should be a reference to a semantic token (e.g., `var(--color-text-primary)`, `var(--color-surface-raised)`) whose *value* is defined per theme. This is the mechanism that makes dark mode possible at all — when the theme changes, the token values change, and every component that references the token re-resolves automatically. A single hardcoded `#FFFFFF` background is a theming defect: it will not flip in dark mode, and it cannot be found without auditing every file. The discipline is absolute: components consume semantic tokens; raw values live only in the token definitions. (The token *architecture* — primitive, semantic, component tiers — is covered in `design-tokens-and-theming`; here the point is that the consumption layer must be token-driven or theming is impossible.)

### Choose The Theme-Switching Mechananism By What You Owe The User

There are two ways to decide which theme is active, and they answer different needs:

- **`prefers-color-scheme` (a media query)** follows the user's OS setting automatically. Use it when the product should respect the system preference with no user action. It is the right default for "do what my OS does."
- **An explicit scope (`[data-theme="dark"]` or a class on the root element)** lets the user choose a theme independent of the OS, including options beyond light/dark (a brand variant, a high-contrast mode). Use it when the product owes the user an explicit toggle.

The two compose: default to `prefers-color-scheme`, and let an explicit user choice override it. Decide which is authoritative for your product and apply it consistently. The override is implemented by re-pointing semantic tokens under the scope selector, so the cascade re-resolves every `var()` — no reload, no re-fetch. If switching themes requires a reload, the architecture is wrong.

### Always Set `color-scheme` So Native Controls Follow The Theme

A common dark-mode bug: the custom UI is dark, but native form controls, scrollbars, the browser chrome, and `::selection` stay light. The fix is the `color-scheme` property. Setting `color-scheme: light dark` (or the specific scheme) on the root tells the browser to render native UI in the matching theme. Without it, dropdowns, date pickers, scrollbars, and form inputs appear in the wrong theme even when everything else is correct, producing a jarring mismatch. Set `color-scheme` to match the active theme, and set it per-scope if different regions use different schemes. This is distinct from `prefers-color-scheme` (which reads the OS) — `color-scheme` is the instruction to the browser about how to render its own native surfaces.

### Design Dark Mode As A Different Visual Problem, Not An Inversion

Dark mode is not light mode with values flipped. Naive inversion produces a broken result, because the visual rules differ:

- **Avoid pure black and pure white.** Pure `#000` backgrounds create harsh contrast and emphasize banding; dark surfaces are usually dark grays (`#121212`-ish) or subtly tinted. Pure white text on black is glaring; dark-mode text is often off-white or lightly tinted.
- **Surface elevation is conveyed by lighter surfaces on dark.** In light mode, elevation (a raised card) is often a shadow on white; in dark mode, shadows are invisible, so elevation is conveyed by making raised surfaces *slightly lighter* than the base. This is the inverse of the light-mode intuition.
- **Reduce saturation and contrast.** Saturated colors that pop on white become neon and vibrate on dark. Desaturate brand colors for dark mode, and reduce the overall contrast ratio target slightly (dark backgrounds with full-white text is fatiguing) while still meeting accessibility floors.
- **Re-check contrast in dark mode specifically.** A color pair that meets WCAG in light mode may fail in dark mode (and vice versa), because the perceived contrast depends on the surrounding luminance. Contrast is covered in the accessibility skill, but the dark-mode-specific point is: do not assume a passing light-mode pair passes in dark.

### Use Perceptually Uniform Color Spaces (OKLCH/LCH) For Palettes

Building color ramps in sRGB or HSL produces uneven results, because those spaces are not perceptually uniform: equal numeric steps do not look equal to the eye, and hue shifts as lightness changes (the "blue turns lavender" problem). OKLCH and LCH are perceptually uniform — a constant lightness ramp looks evenly light, and hue stays stable as you adjust lightness. The practical consequences:

- **Generate even ramps.** An OKLCH ramp from light to dark of a single hue produces steps that look evenly spaced, which HSL cannot. This makes "50/100/.../900" palette scales actually consistent.
- **Match lightness across hues.** Two brand colors at "the same lightness" only look equally light if defined in a perceptual space; in HSL they often do not.
- **Prefer OKLCH for new systems.** OKLCH corrects LCH's blue-shift issue and is the modern default. Use `oklch()` in CSS where supported; provide an sRGB fallback for older browsers via `@supports` or a fallback declaration, since OKLCH is not universally supported yet.

The trap is designing a palette in HSL, eyeballing it as "fine," and shipping visible banding or hue drift that a perceptual space would have prevented.

### Use Relative Colors And `color-mix` For Derived Shades, Not Hand-Picked Variants

Modern CSS lets you derive colors from other colors rather than hand-picking every variant, which keeps relationships consistent and reduces the token count:

- **Relative colors (`rgb(from var(--brand) r g b / ...)`, `oklch(from ...)`) derive a new color by adjusting channels of an existing one** — e.g., a hover state that is the base color at lower lightness, or a transparent overlay at a given alpha. The derived color tracks the base, so a brand change propagates to all derived states automatically.
- **`color-mix(in oklch, var(--brand), white 20%)`** mixes two colors in a specified space, useful for tints, shades, and overlays that must stay perceptually consistent.

The discipline: derive variants from base tokens where the relationship is systematic (hover, disabled, focus rings, scrims), rather than defining each as an independent hardcoded value that will drift. This keeps a rebrand or theme change localized to the base tokens.

### Use System Colors For Forced-Color And Native-Context Correctness

Some users run their OS in high-contrast or forced-color mode, which replaces authored colors with a limited system palette. Interfaces that depend on authored background tints or color-only borders can break — boundaries vanish, icons disappear. CSS system color keywords (`Canvas`, `CanvasText`, `Highlight`, `ButtonText`, `LinkText`) map to the OS palette and survive forced-color mode. Use them (or the `forced-color-adjust` property) where the design must adapt to the system palette, and never rely on a background tint alone to convey a boundary — pair it with a border or semantic structure. This overlaps with the accessibility skill's forced-color guidance; the color-systems point is that a theme built entirely on authored hex values has no forced-color story, while one that routes through system colors where it matters does.

## Common Traps

### Hardcoded Colors That Cannot Re-Theme

Scattering raw hex/rgb values (`#3B82F6`, `rgb(31,41,55)`) through components, so dark mode or a rebrand requires a find-and-replace across hundreds of files and is never complete. Route every color through a semantic token; raw values live only in token definitions.

### Dark Mode As Naive Inversion

Flipping light and dark values and keeping saturations and contrasts identical, producing a harsh, neon, or low-contrast dark UI. Design dark mode separately: dark gray (not pure black) surfaces, lighter-equals-raised elevation, desaturated accents, and re-checked contrast.

### Missing `color-scheme` Leaving Native Controls In The Wrong Theme

Custom UI is dark but dropdowns, scrollbars, date pickers, and form inputs stay light. Set `color-scheme` to match the active theme so native surfaces follow.

### Assuming Light-Mode Contrast Passes In Dark Mode

A color pair that meets WCAG on a white background failing on a dark one (or vice versa), because perceived contrast depends on surrounding luminance. Re-check contrast in each theme; do not inherit a light-mode pass.

### Building Palettes In HSL With Visible Banding And Hue Drift

Constructing color ramps in HSL/sRGB where equal steps look unequal and hues shift as they lighten. Use OKLCH/LCH for perceptually uniform ramps and stable hue across lightness.

### Hand-Picking Every Variant Instead Of Deriving

Defining hover, disabled, focus, and scrim colors as independent hardcoded values that drift from the base, so a brand change updates the base but not the variants. Derive systematic variants with relative colors or `color-mix` so relationships track the base token.

### Pure Black Backgrounds And Pure White Text

Using `#000` and `#FFF` for maximum contrast, which is harsh, fatiguing, and emphasizes banding. Use dark grays and off-whites; reduce contrast slightly while still meeting accessibility floors.

### Relying On A Background Tint Alone For Boundaries Under Forced-Color

A card distinguished only by a subtle background tint that vanishes when forced-color mode overrides the palette. Pair tints with borders or semantic structure; route critical surfaces through system colors.

## Self-Check

- [ ] No component uses a raw color value — every color is a reference to a semantic token whose value is defined per theme, so theming works by re-pointing tokens rather than restyling components.
- [ ] The theme-switching mechanism matches the product's obligation: `prefers-color-scheme` to respect the OS default, an explicit `[data-theme]`/class scope for a user toggle, composed deliberately with a clear decision on which is authoritative — and switching is instant without reload.
- [ ] `color-scheme` is set to match the active theme (per-scope where needed) so native controls, scrollbars, form inputs, and `::selection` follow the theme instead of staying in the wrong scheme.
- [ ] Dark mode is designed as a distinct visual problem: dark gray (not pure black) surfaces, off-white (not pure white) text, lighter-equals-raised elevation, desaturated accents, and reduced-but-accessible contrast — not a naive inversion of the light palette.
- [ ] Contrast was re-checked in dark mode specifically (not inherited from a light-mode pass), and color pairs meet WCAG floors in both themes.
- [ ] Color ramps and palettes are built in a perceptually uniform space (OKLCH preferred, LCH acceptable) so steps look even and hue stays stable across lightness, with an sRGB fallback provided where OKLCH support is uncertain.
- [ ] Systematic color variants (hover, disabled, focus, scrims, tints) are derived from base tokens via relative colors or `color-mix` rather than hand-picked, so a base change propagates to derived states.
- [ ] Critical surfaces route through CSS system color keywords or `forced-color-adjust` where forced-color/high-contrast mode must be supported, and no boundary relies on a background tint alone.
- [ ] The color system was verified across both themes and in forced-color mode — confirming no hardcoded leaks, no native-control mismatch, no collapsed contrast, and no vanishing boundaries — not only in the theme being actively designed.
