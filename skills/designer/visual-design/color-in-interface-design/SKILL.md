---
name: color_in_interface_design.md
description: Use when the agent is applying color to a user interface, choosing semantic and functional colors for success, warning, error, and informational states, building accessible color combinations, using color to encode data and status, deciding when color must be reinforced by a non-color cue, managing color across themes such as dark mode, or ensuring color carries meaning and hierarchy without excluding users who perceive color differently.
---

# Color In Interface Design

Color in an interface is not decoration; it is a functional system that encodes meaning, directs attention, and signals state. Used well, it lets a user grasp status at a glance: green for success, red for error, a highlighted row for selection. Used poorly, it excludes users who perceive color differently, hides meaning behind a signal many cannot see, and creates a product that fails in dark mode, in sunlight, and on dim screens. The central mistake is treating color as a visual preference rather than as a semantic and accessibility-critical system that must work for every user in every context.

Agents tend to fail interface color in predictable ways. They use color as the only signal for state, so a selected item differs from an unselected one only by a tint invisible to color-blind users. They choose semantic colors that look pleasant but fail contrast minimums, so error text becomes unreadable. They build a palette for light mode and watch it break in dark mode, where the same hues produce glare or vanish. Or they encode data in color alone, so a chart becomes meaningless to anyone who cannot distinguish the hues.

Use this skill before applying color to any interface, when defining semantic colors, and when reviewing whether color carries meaning accessibly. The goal is a color system that is semantic, accessible, robust across themes and contexts, and never the sole carrier of critical meaning.

## Core Rules

### Treat Color As A Semantic System, Not Decoration

Color in an interface should mean something. A consistent semantic mapping, where red always signals error and green always signals success, lets users learn the language of the product and respond without reading. Arbitrary or inconsistent color, where the same hue means different things in different places, breaks that language and forces users to decode each instance.

Build a semantic system:

- define semantic colors for success, warning, error, and informational states, and use them consistently;
- reserve specific hues for specific meanings, so users can learn and predict;
- separate semantic color from brand and decorative color, so meaning is never ambiguous;
- document the mapping so that additions follow the system rather than fragmenting it.

### Never Use Color As The Only Carrier Of Meaning

Roughly one in twelve men and many users under poor lighting, on dim screens, or with high-contrast needs cannot distinguish some color differences. A state, status, or data value that depends on color alone is invisible to those users and fragile in screenshot reviews and printed materials. Color must always be reinforced by a non-color cue.

Reinforce color with a second cue:

- pair status colors with icons, such as a checkmark for success or an alert for error;
- use text labels alongside color-coded states;
- add borders, weight changes, or position shifts to distinguish selected, active, or emphasized items;
- in data visualizations, pair color with shape, pattern, or direct labeling.

This is both an accessibility requirement and a robustness practice: a design that survives the loss of color is a design that survives many real-world conditions.

### Meet Contrast Requirements For All Text And Meaningful Graphics

Color combinations that look pleasant often fail contrast minimums, especially when light text sits on a tinted background or when semantic colors are used for text. Low contrast excludes users with low vision and degrades readability for everyone in adverse lighting. Contrast is a measurable requirement, not a matter of taste.

Ensure contrast:

- meet the established contrast ratios for normal and large text, and for meaningful non-text elements such as icons and boundaries;
- test semantic colors used as text or on text backgrounds, because pleasant hues often fail when used this way;
- account for text over images and gradients, where contrast varies across the surface;
- re-check contrast in every theme, because colors that pass in light mode often fail in dark mode.

### Design Semantic Colors That Survive Theme Changes

A palette built for light mode rarely translates directly to dark mode. The same hues that read clearly on white can glare or vanish on black, and semantic colors that passed contrast in light mode often fail in dark. Theme changes are not a simple inversion; they require a re-derived palette.

Design for themes:

- derive separate palettes for light and dark, adjusting hue, saturation, and lightness rather than inverting values;
- re-validate contrast and meaning in every theme, because the same semantic color can become ambiguous or unreadable;
- avoid pure black or pure white backgrounds, which produce harsh contrast and exaggerate perceived differences;
- test semantic colors against themed surfaces, not just neutral backgrounds.

### Use Color To Direct Attention Deliberately

Attention follows color, especially saturated and warm color. A single saturated element in an otherwise muted interface will draw the eye, whether the team intends it to or not. Color is a tool for directing attention, and using it carelessly scatters focus.

Direct attention with color:

- reserve saturated, high-contrast color for the primary call to action and critical state;
- desaturate or mute secondary and tertiary elements so they do not compete;
- avoid multiple equally saturated elements, which create competing focal points;
- use color consistently to signal the same kind of importance everywhere.

### Encode Data In Color With Care

Color is a common encoding in data visualization, but it is also the most fragile, because color perception varies widely and many palettes are not distinguishable to color-blind users. Encoding continuous or categorical data in color requires a palette chosen for perception, not for aesthetics.

Encode data carefully:

- use color-blind-safe palettes for categorical data, and pair color with shape or label;
- for continuous data, use perceptually uniform sequential palettes, not rainbows, which imply false divisions;
- limit the number of categories encoded by color alone, because distinguishability drops sharply with count;
- provide direct labels, tooltips, or a legend that does not depend on matching hues.

### Account For Cultural And Contextual Meaning

Color carries cultural meaning that varies across audiences, and a hue that signals danger in one culture may signal prosperity in another. Interface color is consumed globally, and assumptions based on one culture's color associations can mislead or offend others.

Consider cultural context:

- be cautious with color meanings that vary across cultures, especially for red, green, white, and yellow;
- prefer functional consistency, where red means error everywhere, over culturally specific symbolism;
- test with users from the target cultures where color meaning is consequential;
- avoid relying on color symbolism to carry critical meaning that could be misread.

### Manage Color Across Brand, Semantic, And Surface Roles

A mature interface separates color into roles: brand color for identity, semantic color for meaning, surface color for backgrounds and structure, and text color for content. Conflating these roles produces ambiguity, where a brand color used for a surface is mistaken for a semantic state.

Separate color roles:

- define distinct tokens for brand, semantic, surface, border, and text colors;
- avoid using brand color for semantic meaning unless the mapping is intentional and consistent;
- keep surface and text colors structured so that themes can re-derive them together;
- document the roles so that additions respect the separation.

## Common Traps

### Color As The Only State Signal

A selected row or error state that differs only by tint is invisible to color-blind users and fragile in screenshots.

### Pleasant Colors That Fail Contrast

Semantic hues chosen for looks often fail contrast minimums when used as text or on tinted backgrounds.

### Light-Mode Palette Forced Into Dark Mode

Direct inversion produces glare, ambiguity, and failed contrast; dark mode needs a re-derived palette.

### Saturated Everything

Multiple equally saturated elements create competing focal points and scatter attention.

### Rainbow Encodings For Continuous Data

Rainbow palettes imply false categorical divisions and fail for color-blind users; use perceptually uniform sequences.

### Brand Color Used For Semantic Meaning

Conflating brand and semantic roles makes state ambiguous when the brand color appears as a surface.

### Cultural Color Assumptions

Color meanings vary across cultures; relying on one culture's symbolism can mislead global users.

### Untested Real Conditions

Colors chosen in ideal conditions fail in sunlight, on dim screens, and under user accessibility settings.

## Self-Check

- [ ] Color is treated as a semantic system, with consistent meaning for success, warning, error, and informational states.
- [ ] No critical meaning depends on color alone; every color-coded state is reinforced by icon, text, border, weight, or position.
- [ ] All text and meaningful non-text elements meet established contrast ratios in every theme.
- [ ] Separate palettes are derived for light and dark themes, with hue, saturation, and lightness adjusted and re-validated.
- [ ] Saturated, high-contrast color is reserved for the primary call to action and critical state, with secondary elements muted.
- [ ] Data encoded in color uses color-blind-safe categorical palettes or perceptually uniform sequential palettes, paired with labels or shape.
- [ ] Brand, semantic, surface, border, and text colors are separated into distinct roles and tokens.
- [ ] Cultural variation in color meaning was considered where consequential, and functional consistency preferred over culturally specific symbolism.
- [ ] Color was tested under real conditions: sunlight, dim screens, user accessibility settings, and across themes.
- [ ] The color system is documented so that additions follow the semantic mapping rather than fragmenting it.
