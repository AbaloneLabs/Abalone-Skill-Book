---
name: color_theory.md
description: Use when the agent is building a color system, choosing a palette, defining primary and secondary colors, setting up semantic colors for success, warning, error, and info, managing color contrast and accessibility, designing dark mode and theme variants, or deciding how color communicates meaning, hierarchy, and state across a product.
---

# Color Theory

A color system is the set of decisions about what colors exist, what each one means, and how they combine. It looks like a palette, but it is really a semantic structure: which color signals danger, which signals success, which draws the eye, which recedes, and how the same hue behaves across light and dark surfaces. Agents tend to treat color as decoration, pick a palette by aesthetic preference, or add colors ad hoc until the system fragments. The harm is invisible at first and systemic later: meaning becomes inconsistent (red means error here, sale there), hierarchy flattens (everything competes for attention), and accessibility fails (text disappears against its background).

Use this skill before finalizing a palette, semantic color roles, or theme variants. The goal is to prevent the agent from choosing colors that look good in isolation but fail in combination, from assigning meaning inconsistently, or from building a system that cannot pass contrast requirements or survive dark mode.

## Core Rules

### Separate Color Roles From Specific Hues

The most important shift in a color system is thinking in roles before hues. A "primary" color is a role; the specific blue it takes is a decision that can change. A "danger" color is a role; whether it is red is a convention. Defining roles first keeps the system semantic and makes theming, rebranding, and dark mode tractable.

Define roles before values:

- primary: the dominant brand or action color, used for key calls to action and focus;
- secondary or accent: a supporting color for less prominent actions and highlights;
- semantic: success, warning, error, info, each with a consistent meaning across the product;
- neutral: grays for text, backgrounds, borders, and surfaces, usually a full ramp from white to black;
- surface and background: the canvas colors that content sits on.

If colors are chosen without assigned roles, the system becomes a drawer of swatches with no rules for use.

### Build A Full Ramp For Each Color, Not Single Swatches

A single hex value per color is not a system. Real interfaces need a color at multiple lightness levels: a primary button uses one step, a hover state a darker step, a disabled state a paler step, a tinted background a much lighter step. A color system must provide a ramp (often nine or ten steps) for each hue so every state has a defined value.

Construct ramps deliberately:

- generate each hue across a lightness range, for example steps 50 through 900, where 50 is near-white and 900 is near-black;
- ensure adjacent steps are distinguishable but not jarring;
- define which steps are for text, backgrounds, borders, and tints;
- verify that the steps used for text-on-background combinations meet contrast targets.

A palette of single swatches forces designers to invent tints and shades ad hoc, producing inconsistent states and contrast failures.

### Assign Semantic Meaning Consistently And Never Reuse It

Semantic colors carry meaning that users learn. Green means success; red means error; yellow means warning. The moment a semantic color is reused for a non-semantic purpose, its meaning degrades. A red "sale" badge next to a red "error" message confuses users about what red signals.

Govern semantic color use:

- reserve each semantic color exclusively for its meaning across the entire product;
- do not use the error red for branding, the success green for decoration, or the warning yellow for emphasis;
- if the brand color conflicts with a semantic color (a brand that is red), adjust the semantic hue so it remains distinguishable;
- document the meaning of each semantic color so teams apply it consistently.

Inconsistent semantic color is worse than no semantic color, because it actively misleads.

### Design Color For Hierarchy And Attention, Not Decoration

Color is one of the strongest tools for directing attention, but only if used sparingly. When everything is colorful, nothing stands out. A disciplined palette uses color to signal importance and state, while neutrals carry most of the content.

Use color to direct attention:

- reserve saturated color for primary actions and critical state;
- use neutral grays for the bulk of text, backgrounds, and structure;
- limit the number of colors competing on a single screen;
- let one dominant color lead, with others supporting.

A screen with six equally saturated colors has no hierarchy. The eye does not know where to go. Restraint is what makes color effective.

### Ensure Contrast For Accessibility From The Start

Color contrast is not optional. Text must be legible against its background for users with visual impairments, in bright ambient light, and on low-quality screens. A palette that looks elegant but fails contrast targets excludes users and violates accessibility standards.

Meet contrast requirements:

- target WCAG AA at minimum: 4.5:1 for normal text, 3:1 for large text and meaningful graphics;
- consider AAA (7:1 for normal text) for critical reading content;
- test every text-on-background combination in the system, not just the obvious ones;
- account for color blindness: never rely on color alone to convey meaning (pair color with text, icon, or pattern);
- verify that focus indicators have sufficient contrast against their surroundings.

Designing for contrast first, then aesthetics, produces a system that is both accessible and coherent. Retrofitting contrast into an aesthetic palette is painful and often compromises the design.

### Plan Dark Mode And Theme Variants Together

Dark mode is not inverting the light palette. Colors that work on white often fail on black: saturated blues glow, semantic greens become muddy, and text contrast shifts. A color system must define how each role behaves in both light and dark themes, not assume inversion handles it.

Design themes together:

- define light and dark variants for every role, adjusting lightness and saturation appropriately;
- reduce saturation in dark mode, since colors appear more vivid against dark surfaces;
- ensure semantic colors remain distinguishable and meet contrast in both themes;
- avoid pure black backgrounds, which can cause eye strain and lose elevation cues; near-black is usually better;
- test both themes against real content, not just empty screens.

A palette that only works in light mode is half a system. Theme variants are a first-class part of color design.

### Account For Cultural And Contextual Meaning

Color meaning is not universal. Red signals danger in many Western contexts but prosperity in parts of East Asia. White is associated with purity in some cultures and mourning in others. A product that ships globally must consider how its palette reads across markets.

Consider context:

- research color associations for target markets, especially for semantic and brand colors;
- be cautious with color combinations that carry political or cultural weight;
- test with users in target regions when meaning is ambiguous;
- allow for localization of semantic colors where a convention strongly differs.

Ignoring cultural meaning can make a product feel alien or offensive in markets it intends to serve.

## Common Traps

### Choosing Colors By Aesthetic Preference Alone

A palette that looks beautiful in isolation can fail in combination, lack semantic structure, and break accessibility; build from roles and ramps, not swatches.

### Reusing Semantic Colors For Decoration

Using error red for a sale badge or success green for branding degrades the meaning users rely on for state recognition.

### Single Swatches Instead Of Ramps

One hex value per color leaves designers inventing tints and shades ad hoc, producing inconsistent states and contrast failures.

### Making Everything Colorful

When every element is saturated, nothing stands out; reserve color for attention and let neutrals carry the structure.

### Skipping Contrast Checks

A palette that fails WCAG contrast excludes users with visual impairments and violates accessibility standards; test every combination.

### Treating Dark Mode As Inversion

Inverting a light palette produces muddy semantics, glowing colors, and lost elevation; design theme variants together.

### Relying On Color Alone To Convey Meaning

Color-only signals exclude color-blind users; pair color with text, icon, or pattern so meaning survives without color.

### Ignoring Cultural Color Meaning

Colors carry different associations across markets; a palette that reads well in one region may alienate another.

## Self-Check

- [ ] Color roles (primary, secondary, semantic, neutral, surface) are defined before specific hues are chosen.
- [ ] Each color has a full ramp of lightness steps, with defined usage for text, background, border, and tint states.
- [ ] Semantic colors are reserved exclusively for their meaning and never reused for decoration or branding.
- [ ] Color is used sparingly to direct attention, with neutrals carrying the bulk of content and structure.
- [ ] Every text-on-background combination meets WCAG AA contrast (4.5:1 normal, 3:1 large), with AAA considered for critical reading.
- [ ] Meaning is never conveyed by color alone; color is paired with text, icon, or pattern for color-blind users.
- [ ] Light and dark theme variants are designed together, with adjusted saturation and verified contrast in each.
- [ ] Cultural color associations were considered for target markets, especially for semantic and brand colors.
- [ ] Focus indicators have sufficient contrast against their surroundings in both themes.
