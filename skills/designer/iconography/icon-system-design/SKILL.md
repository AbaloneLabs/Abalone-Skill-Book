---
name: icon_system_design.md
description: Use when the agent is designing or specifying an icon system, icon library, or icon set for a product, deciding grid, stroke, keylines, sizing scale, naming, states, export formats, or how icons will be shared across teams, platforms, and components.
---

# Icon System Design

An icon system is not a folder of icons. It is a contract: a set of shared rules that make every icon drawn by different people, at different times, for different screens look like it belongs to one family. When that contract is missing or vague, teams accumulate icons that are almost consistent — slightly different stroke weights, slightly different corner radii, slightly different optical sizes — and the product reads as assembled from parts rather than designed as a whole.

Use this skill before creating an icon library, defining icon guidelines, or scaling icons across many surfaces. The goal is to prevent the agent from shipping a collection of individual icons that each look fine alone but fail to form a coherent, maintainable system. The hardest decisions are rarely about any single icon; they are about the rules that govern all of them.

## Core Rules

### Define The Grid And Keylines Before Drawing

Every icon system needs a construction grid that establishes the canvas, the optical center, and the safe margins. The grid is what allows icons of different shapes to share a perceived size and center.

A standard approach is a 24x24 unit grid with a roughly 2-unit padding safe area, plus keyline shapes (circle, square, horizontal and vertical rectangles) that represent common icon footprints. Icons are drawn within these keylines so that a round icon, a tall icon, and a wide icon all occupy the same optical space.

Decide and document:

- the base canvas size and whether it scales (for example 16, 20, 24, or multiples);
- the safe area inside the canvas that content should respect;
- the keyline shapes and how icons should align to them;
- whether the grid is pixel-snapped for crisp rendering at target sizes.

Without an enforced grid, a magnifier and a gear that should be the same size will not be, and rows of icons will look uneven even when their bounding boxes match.

### Choose A Stroke Language And Hold It

Stroke weight is one of the strongest signals of icon family identity. Mixing a 1.5px stroke icon with a 2px stroke icon in the same toolbar immediately breaks the system.

Decide:

- the primary stroke weight at the base size;
- whether stroke scales with icon size or stays constant in pixels;
- stroke endings (butt, round, square) and joins (miter, round, bevel);
- whether strokes are centered on the path or drawn as fills.

Round joins and caps tend to read friendlier and survive small sizes better. Sharp joins read more technical or precise. The choice should match the product's voice, and once chosen, it must be applied to every icon without exception.

### Establish A Sizing Scale And Optical Sizing Rules

Icons appear at many sizes: 16px in dense tables, 20px in lists, 24px in toolbars, 32px or larger in empty states and marketing. A robust system defines which sizes are supported and how each size is derived.

Two approaches exist, and the agent should choose deliberately:

- **Single source, scaled**: one master icon is drawn and rasterized or scaled to each size. Simpler to maintain, but strokes can become too thin or too heavy at extremes.
- **Optical, size-specific masters**: separate drawings optimized for small (often heavier strokes, simplified detail) and large (more detail, finer strokes) sizes. More work, but far better legibility at 16px and below.

For any icon used below roughly 20px, consider a simplified small-size variant. Detail that looks elegant at 48px turns into mud at 16px.

### Decide Fill Versus Outline And State Behavior

Icons often need to communicate state: default, hover, active, selected, disabled, and error. The system must define how state is expressed consistently.

Common patterns:

- outline for default, filled for active or selected;
- color change for state, keeping geometry constant;
- weight change (for example, thicker stroke on hover).

Document which pattern applies where, and never express state through geometry changes alone (such as redrawing an icon differently when selected), because users read that as a different icon rather than a state of the same one.

### Name And Organize For Discovery And Code Use

An icon system that cannot be found or referenced is not a system. Naming should be semantic, consistent, and predictable enough that engineers and designers can guess the name before searching.

Strong naming conventions:

- use descriptive object or action names, not metaphors only the original designer understood;
- use a consistent prefix or suffix for variants (for example `-filled`, `-outline`, `-small`);
- avoid encoding state or color in the file name if those are handled by code;
- group by category when the library is large (navigation, action, status, content).

Maintain a single source of truth, whether a Figma library, an SVG repository, or a versioned package. Duplicated sources inevitably drift.

### Specify Export And Delivery Requirements

Icons are consumed by engineering, marketing, and other platforms. The system must define how icons leave the design tool.

Specify:

- the export format (SVG for product, PNG or WebP for marketing, PDF for print);
- whether SVGs are optimized, cleaned of editor metadata, and use consistent attributes;
- whether icons inherit color via `currentColor` or ship with baked-in colors;
- how icons are imported and referenced in code (component name, asset path);
- accessibility requirements such as whether an icon ships with a title or is marked decorative by default.

An icon exported with hardcoded fills cannot be themed. An icon without `currentColor` cannot respond to dark mode. These are system decisions, not afterthoughts.

### Plan For Growth And Deprecation

Icon libraries grow. Decide in advance how new icons are requested, reviewed, and added, and how old or duplicate icons are retired. Without this, libraries accumulate near-duplicates (three slightly different bells, four settings gears) that confuse everyone.

Maintain a contribution process that checks new icons against the grid, stroke, naming, and export rules before they are merged.

## Common Traps

### Drawing Icons Individually Without Shared Rules

When each icon is drawn to look good on its own, the set looks inconsistent. The system must be designed before the icons, not reverse-engineered from them.

### Ignoring Optical Size At Small Render Sizes

A 24px master scaled down to 12px often loses legibility because strokes become sub-pixel and detail collapses. Small icons usually need dedicated masters, not naive scaling.

### Mixing Stroke Weights And Corner Radii Across The Set

Even small deviations — 1.75px next to 2px, 1px corners next to 2px corners — are visible in toolbars and navigation. Consistency at the rule level prevents this.

### Encoding State By Redrawing The Icon

Changing the geometry of an icon to show selection makes it look like a different icon. State should be expressed through fill, color, or weight, not through a new drawing.

### Hardcoding Colors In Exported SVGs

Icons that ship with fixed fills cannot adapt to themes, dark mode, or brand variants. Use `currentColor` or token-driven color unless there is a specific reason not to.

### Naming By Appearance Instead Of Function

Names like `icon-1`, `circle-arrow`, or `thing-final-v2` are unsearchable and meaningless to new team members. Names should describe what the icon represents or does.

### Forgetting That Icons Need Accessibility Treatment

Decorative icons should be hidden from assistive technology; meaningful icons need text labels or accessible names. Treating all icons as self-explanatory excludes screen reader users.

## Self-Check

- [ ] A construction grid with canvas size, safe area, and keyline shapes is defined and documented.
- [ ] A single stroke weight, cap, and join language is chosen and applied to every icon in the set.
- [ ] A sizing scale is defined, with explicit decisions about scaling versus size-specific masters for small sizes.
- [ ] State behavior (default, hover, active, selected, disabled, error) is expressed consistently without redrawing geometry.
- [ ] Naming is semantic, consistent, and predictable, with clear variant conventions.
- [ ] Export formats, color inheritance, and code integration are specified so icons can be themed and consumed.
- [ ] A single source of truth exists, with a process for adding and retiring icons.
- [ ] Each icon was reviewed at its smallest real render size, not only at its drawing size.
- [ ] Accessibility treatment is defined: which icons are decorative and which require accessible names.
- [ ] The set was reviewed as a family, confirming that icons of different shapes share optical size, weight, and centering.
