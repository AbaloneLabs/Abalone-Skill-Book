---
name: color_palette_construction.md
description: Use when the agent is building, expanding, or auditing a color palette for a product or design system, choosing brand and accent colors, generating tints and shades, defining neutral ramps, setting color relationships, balancing vibrancy and restraint, or deciding how many colors a palette should contain.
---

# Color Palette Construction

A color palette is not a set of pretty swatches. It is a structured system of roles and relationships that must hold together across hundreds of surfaces, states, themes, and contexts while still expressing brand identity. Most palette failures are not aesthetic; they are structural. A palette that looks beautiful in a brand guide collapses the moment it meets real interfaces, because the roles were never defined, the neutrals were chosen by eye instead of by function, or the accent colors were added without considering how they coexist.

Use this skill before constructing a palette, adding colors to an existing system, generating tints and shades, defining neutral ramps, or deciding brand versus functional color boundaries. The goal is to prevent the agent from picking colors in isolation, building ramps that look even on one monitor but fail on others, or shipping a palette whose roles collide under real use.

## Core Rules

### Define Roles Before Choosing Hues

A palette fails when colors are chosen for appearance rather than function. Before selecting any hue, define the roles the palette must fill: primary brand, secondary or supporting brand, neutral surfaces and text, borders and dividers, and semantic statuses such as success, warning, error, and information. Each role has constraints that appearance alone cannot satisfy.

Map roles first, then assign colors to them. A vivid brand color may be perfect for a primary button but wrong as a background; a neutral that reads as a clean divider may fail as body text. Role determines whether a color works, not the reverse.

### Build Neutral Ramps By Function, Not By Eye

Neutrals carry most of an interface, yet they are often chosen casually. A neutral ramp needs enough steps to express hierarchy, surface elevation, borders, disabled states, and text levels, and each step must be perceptually distinct from its neighbors. Ramps built by eye often crowd together in the midtones, where the eye is least sensitive to differences, leaving too few usable steps.

Construct neutrals with:

- enough steps, typically eight to twelve, to separate background, surface, border, muted text, and strong text;
- perceptually even spacing, so adjacent steps are distinguishable on real screens;
- a slight hue bias, pure gray often feels dead, so a warm or cool tint gives neutrals life without distraction;
- verified separation between text and background at each step, not only visual appeal.

### Generate Tints And Shades Systematically

Brand colors rarely work at full strength everywhere. A primary used on a button, a background, a border, and a text label needs lighter and darker variants, and those variants must feel like the same color family. Manually lightening or darkening a hue produces muddy or washed-out steps, because lightness and saturation shift together unpredictably.

Use a consistent method to generate variants: adjust lightness while managing saturation and chroma so each step remains recognizably the same hue. Verify the ramp has enough steps for tinted backgrounds, hover states, borders, and accessible text without forcing a designer to invent a new color each time.

### Limit The Number Of Accent Colors

Every accent color added to a palette increases the chance of collision, inconsistency, and visual noise. Palettes grow by accretion: one accent for marketing, another for a new feature, another to solve a contrast problem, until the system no longer coheres. Restraint is a structural decision, not a stylistic one.

Before adding a color, ask:

- can an existing color, used differently, solve the problem;
- does the new color fill a role no current color fills;
- will the new color coexist with the semantic statuses without confusion;
- is the team prepared to maintain the color across states and themes.

Prefer fewer, well-defined colors with clear roles over many colors chosen ad hoc.

### Test Color Relationships, Not Only Individual Swatches

A palette is a system of relationships. Two colors that each look strong can clash when adjacent; a background tint that seems subtle can wash out a text color placed on it; an accent that pops on white can disappear on a tinted surface. Individual swatches reveal nothing about how colors behave together.

Test colors in combination:

- primary on neutral background, and neutral text on primary background;
- accent colors adjacent to each other and to semantic statuses;
- tinted backgrounds under text of every weight;
- borders against both lighter and darker neighbors;
- the full palette on a real screen, not only in a design tool.

### Verify Perceptual Uniformity Across The Palette

Colors that appear equally vibrant in a design tool can feel uneven in an interface, because human perception is not uniform across the spectrum. Yellows appear lighter and more intense than blues at the same numeric value; some hues need more chroma to feel as saturated as others. A palette built on raw values rather than perceptual space produces uneven emphasis.

Use a perceptually uniform color space to evaluate and adjust the palette, so that colors intended to feel equally prominent actually do, and so that ramps step evenly to the eye.

### Account For Display And Environment Variation

A palette finalized on a calibrated monitor can fail on the devices and conditions where users actually see it. Laptop screens, phones in sunlight, low-quality displays, and night-shift color temperature all shift how colors appear. Vibrant colors can look washed out or, on OLED, unpleasantly glowing.

Stress-test the palette on real devices, under bright and dim conditions, and confirm that critical distinctions, such as between two statuses or between text and background, survive variation.

## Common Traps

### Choosing Colors In Isolation

Picking swatches for beauty, then discovering they clash or fail in context, produces a palette that looks good in a guide and breaks in the product.

### Neutrals Chosen By Eye

A gray ramp built visually often crowds the midtones, leaving too few distinguishable steps for real interface hierarchy.

### Manual Tints And Shades

Lightening or darkening a hue by hand produces muddy or washed variants that no longer read as the same color family.

### Palette Growth By Accretion

Adding colors per feature or per campaign, without retiring or consolidating, produces an incoherent system where roles collide.

### Ignoring Perceptual Non-Uniformity

Treating numeric color values as if they appear equal to the eye produces uneven emphasis and uneven ramps.

### Finalizing On A Calibrated Monitor Only

Colors that look perfect in a controlled environment can shift, wash out, or glow on the devices users actually own.

### Confusing Brand Expression With Interface Color

Marketing palette choices, optimized for impact, often fail as interface colors where restraint, hierarchy, and contrast matter more.

## Self-Check

- [ ] Roles, primary, neutral, border, status, and accent, were defined before any hue was chosen.
- [ ] The neutral ramp has enough perceptually distinct steps for surfaces, borders, disabled states, and text levels.
- [ ] Tints and shades were generated systematically so each variant remains recognizably the same hue.
- [ ] The number of accent colors is limited, and each fills a role no existing color fills.
- [ ] Colors were tested in combination, not only as individual swatches, including text on tinted backgrounds and adjacent accents.
- [ ] The palette was evaluated in a perceptually uniform space so emphasis and ramp spacing appear even to the eye.
- [ ] Brand colors were separated from interface colors where their goals conflict.
- [ ] The palette was reviewed on real devices and under varied lighting, not only on a calibrated monitor.
- [ ] Semantic statuses remain distinguishable from brand and accent colors under the palette's relationships.
- [ ] Adding any new color would require justifying its role against the existing system, not merely its appearance.