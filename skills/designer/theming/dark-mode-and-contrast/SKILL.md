---
name: dark_mode_and_contrast.md
description: Use when the agent is designing or reviewing dark mode, light/dark theme switching, color contrast, accessibility of theme palettes, status colors across themes, brand color adaptation to dark surfaces, or resolving contrast failures in a product theme.
---

# Dark Mode And Contrast

Dark mode is not a color invert of the light theme. It is a separate palette that must preserve hierarchy, legibility, status meaning, and brand recognition while reducing brightness and respecting user environment. A theme that looks elegant on a designer's calibrated monitor can fail on an OLED phone at night, on a glossy laptop in daylight, or for a user with low vision who depends on strong contrast.

Use this skill before finalizing dark mode palettes, defining theme tokens, adapting brand colors to dark surfaces, choosing surface elevation, resolving contrast failures, or shipping automatic theme switching. The goal is to prevent the agent from treating dark mode as a cosmetic flip, shipping status colors that lose meaning, or passing a contrast checker while still being unreadable in real conditions.

## Core Rules

### Treat Dark Mode As A Redesigned Palette, Not An Inversion

Inverting a light theme produces wrong results. Pure white text on pure black causes halation, vibration, and eye strain at length. The common strong practice is a dark gray base, not pure black, with off-white text, not pure white. Surfaces should lighten slightly as they elevate, so raised cards, dialogs, and modals read as distinct without harsh borders.

Define the dark palette from scratch using the same semantic intent as the light theme. Re-derive each role: background, surface, border, primary text, secondary text, disabled, focus, and status. Do not assume a token that works in light survives unchanged in dark.

### Preserve Semantic Meaning Across Both Themes

Status colors are the most common failure. A red error that reads as a clear warning on white can look like a decorative accent on near-black. Green success can become muddy. Blue links can lose their link affordance against a dark surface.

For each status token, verify the meaning is still obvious in dark mode. You may need to:

- lighten or desaturate the hue for dark surfaces;
- add a text label or icon so meaning does not depend on color alone;
- pair the color with a border or background tint that reinforces it;
- test color-blind safe pairings in both themes.

A user switching themes should never lose the ability to distinguish success, warning, error, and info.

### Meet Contrast For Real Content, Not Only Headlines

Contrast requirements apply to the text users actually read, not just marketing headlines. The riskiest elements are usually:

- secondary and tertiary text;
- placeholder text;
- disabled-looking active controls;
- subtle borders that define interactive regions;
- icons that carry meaning;
- text over images, gradients, or branded backgrounds;
- focus indicators.

Check small text, labels, captions, metadata, and timestamps, not only body copy. A theme can pass the body text ratio and still fail the labels that carry the most operational information.

### Handle Brand Colors Carefully On Dark Surfaces

Brand colors chosen for white backgrounds often fail on dark ones. A vivid brand blue may glow unpleasantly on near-black, or a brand yellow may be illegible as text. Decide deliberately:

- which brand colors are structural (used for buttons, links, focus) and need a dark-mode variant;
- which brand colors are decorative (used in illustrations, logos, marketing) and can stay unchanged;
- where the brand color appears as text versus as a fill, since the contrast requirement differs.

Keep brand recognizability without forcing an inaccessible palette. It is acceptable, and often correct, to define a slightly different brand accent for dark mode.

### Design For Automatic And Manual Switching

Users switch themes for different reasons: comfort, battery, ambient light, accessibility, or personal preference. Support predictable switching behavior:

- respect the system preference by default, but allow explicit override;
- persist the user's choice across sessions;
- switch without reloading or losing scroll position, form state, or focus;
- avoid flashing the wrong theme on load, which is both ugly and, for some users, uncomfortable;
- handle media query changes live when the OS theme changes.

If the product supports only one theme, decide consciously which one and why, rather than defaulting to light because it is easier.

### Account For Images, Media, And Third-Party Content

Dark mode breaks when embedded content does not adapt. User avatars, uploaded photos, charts, maps, code blocks, embedded documents, and third-party widgets often carry their own white backgrounds. These create bright rectangles inside a dark interface.

Plan for this:

- give media containers adaptive backgrounds or borders;
- request dark variants from embeds that support them;
- avoid forcing invert filters that distort photos and break brand logos;
- test data visualizations whose palettes were tuned for white backgrounds.

### Respect Reduced Brightness And Accessibility Needs

Dark mode overlaps with, but is not the same as, high-contrast or low-vision accessibility. Some users need dark mode to reduce brightness; others need stronger contrast than a soft dark palette provides. Provide enough contrast in both themes so that low-vision users are not forced into one mode. Avoid pure black plus pure white as the only option, since that combination causes discomfort for many readers.

## Common Traps

### Inverting The Palette And Calling It Dark Mode

Color inversion produces vibrating text, broken status colors, and brand colors that glow. It feels finished in a screenshot and fails in use.

### Pure Black And Pure White Everywhere

Maximum contrast is not always the most readable. Pure white on pure black causes halation and fatigue during long reading sessions, especially on OLED displays.

### Status Colors That Lose Meaning In Dark Mode

A green or red tuned for light backgrounds often becomes illegible or decorative on dark surfaces. Without re-derivation, users can no longer tell success from error.

### Passing The Body Text Ratio While Failing Labels

Contrast checkers applied only to body copy miss the small secondary text, placeholders, and borders that actually carry operational meaning.

### Bright Embedded Rectangles

White-background images, charts, and third-party widgets punch holes in a dark interface and signal that the theme was not fully considered.

### Flash Of Wrong Theme On Load

A light flash before dark mode applies is more than cosmetic; for light-sensitive users it is a real comfort and accessibility failure.

### Treating Dark Mode As Optional Polish

If a product supports dark mode at all, it must be complete. A half-dark theme with broken components is worse than no dark mode, because it erodes trust in the whole interface.

## Self-Check

- [ ] The dark palette was derived from scratch, not produced by inverting the light theme.
- [ ] Base surfaces use dark gray rather than pure black, and text uses off-white rather than pure white where appropriate.
- [ ] Success, warning, error, and info colors remain distinguishable and meaningful in both themes, with non-color reinforcement.
- [ ] Contrast was checked on secondary text, labels, placeholders, borders, icons, focus indicators, and text over media, not only body copy.
- [ ] Brand colors have deliberate dark-mode variants where they appear as text, fills, or interactive accents.
- [ ] Theme switching respects system preference, persists user choice, avoids flashes, and preserves scroll, form, and focus state.
- [ ] Images, charts, maps, code blocks, and embedded content have a plan for dark backgrounds rather than bright rectangles.
- [ ] Elevation is expressed through surface lightening or subtle borders, not harsh lines.
- [ ] The theme was reviewed on real devices under real lighting, not only on a calibrated design monitor.
- [ ] Both themes provide enough contrast for low-vision users without forcing a single mode.
