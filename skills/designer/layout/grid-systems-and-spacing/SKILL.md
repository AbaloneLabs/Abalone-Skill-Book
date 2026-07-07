---
name: grid_systems_and_spacing.md
description: Use when the agent is defining a grid system, spacing scale, column structure, gutters, margins, baseline grid, vertical rhythm, breakpoint behavior, or responsive layout math, and must choose units, proportions, and rhythm rules that hold up across dense, sparse, and localized content.
---

# Grid Systems And Spacing

A grid and spacing system is the measurement foundation that every layout inherits. It looks like a set of numbers, but it is really a set of promises: that related things align, that gaps communicate relationship, that spacing scales predictably, and that the layout does not break when content gets longer, wider, or translated. Agents tend to treat grids as decoration, pick a 12-column layout because it is conventional, or invent ad-hoc spacing values that feel right on one screen. The harm is invisible at first and systemic later: inconsistent alignment, spacing that communicates nothing, layouts that drift as content varies, and a system no one can extend without breaking the rhythm.

Use this skill before fixing a column count, a gutter value, a spacing scale, a baseline, or breakpoint math. The goal is to prevent the agent from choosing grid and spacing values for one ideal screen, from leaving the relationship between horizontal and vertical rhythm undefined, or from building a system that cannot survive dense data, sparse marketing, and translated text at the same time.

## Core Rules

### Choose A Spacing Scale Built On A Base Unit

Spacing should come from a small, closed scale derived from a single base unit, not from a drawer of arbitrary pixel values. A base unit (commonly 4px or 8px) multiplied into a predictable sequence gives spacing that composes: two small gaps equal one medium gap, and designers stop inventing new values for every layout.

Decide the base and then derive the scale deliberately:

- pick one base unit and forbid off-grid values in production;
- define a limited set of steps (for example base, 2x, 3x, 4x, 6x, 8x, 12x) rather than every multiple;
- name the steps semantically (xs, sm, md, lg, xl) so layouts reference intent, not raw numbers;
- document which steps are for intra-component, inter-component, and section-level separation.

A scale with too many steps is as bad as no scale: designers pick the closest value and the rhythm drifts. A scale with too few steps forces awkward compromises. Aim for a set that covers tight grouping, component padding, section separation, and page margins without gaps.

### Let Spacing Communicate Relationship, Not Just Separate

Spacing is information. Elements close together read as related; elements far apart read as separate. When spacing is chosen by eye or by habit, it stops carrying meaning and the layout becomes ambiguous. Decide consciously what each gap communicates.

Use the spacing scale to encode hierarchy:

- tight spacing (intra-component) groups fields, labels, and controls that belong together;
- medium spacing separates sibling components that are peers;
- large spacing separates sections and major regions;
- whitespace around a primary element signals its importance.

If two unrelated things sit as close as two related things, the user cannot tell the grouping. If every gap is the same, nothing reads as grouped. Spacing should be the loudest signal of structure, quieter than color and typography but more consistent.

### Match Column Count To Content Type, Not Convention

A 12-column grid is a default, not a law. The right column count depends on what the layout must hold. Marketing and editorial layouts benefit from many columns for flexible asymmetric compositions. Data-dense layouts often need fewer columns with stable widths. Form layouts need a predictable few.

Choose columns by content:

- editorial and marketing: 12 columns for flexible subdivision and asymmetry;
- dashboards and data: fewer columns (4 to 8) with stable, predictable widths;
- forms and settings: 2 to 4 columns with clear label and input lanes;
- full-bleed media and tables: a single fluid region that can exceed the column grid.

A grid that forces a data table into 12 narrow columns, or a marketing hero into 4 rigid columns, is the wrong grid for that content. Let content type drive the count, and allow specific regions to opt out.

### Define Gutters And Margins As Responsive, Not Fixed

Gutters and margins behave differently at different viewport sizes, and treating them as fixed constants breaks layouts on mobile and on ultrawide screens. The gutter that feels right on desktop is often too large on mobile, and the margin that frames a laptop screen vanishes on a phone.

Define gutter and margin behavior across breakpoints:

- gutters should scale down on narrow viewports, often halving at the mobile breakpoint;
- page margins should be larger on desktop for framing and smaller but non-zero on mobile;
- decide a max content width beyond which extra space becomes margin, not wider content;
- define whether gutters are fluid (percentage) or fixed (pixel) and stay consistent.

A layout whose gutters do not scale produces cramped mobile columns or absurdly wide desktop gutters. Define the scaling rule once and apply it everywhere.

### Establish Vertical Rhythm Alongside The Horizontal Grid

Horizontal grids get attention; vertical rhythm gets forgotten. But users scan vertically, and inconsistent vertical spacing makes a layout feel sloppy even when the columns are perfect. A baseline grid or a vertical spacing scale tied to the horizontal scale keeps the page calm and scannable.

For vertical rhythm:

- align the vertical spacing scale to the same base unit as the horizontal scale;
- decide whether to use a strict baseline grid (every text line snaps to a baseline) or a soft rhythm (spacing steps only);
- keep component heights, section gaps, and line-height on the rhythm where feasible;
- reserve the strict baseline for text-heavy editorial layouts where it pays off.

Strict baseline grids are powerful but expensive to maintain in component-driven products; a soft rhythm using the spacing scale is usually the pragmatic choice.

### Define Breakpoints By Content, Then Map To Devices

Breakpoints chosen only by device widths become obsolete and miss the real transitions. The right breakpoints are where the layout must change to stay usable, which is a function of content and density, then mapped to common device widths.

A strong approach:

- identify the viewport widths at which the current layout breaks (columns too narrow, text measure too long, tables overflow);
- set breakpoints just above those failure widths;
- verify each breakpoint against real device widths so common devices land inside a band, not on the edge;
- avoid so many breakpoints that the system becomes untestable.

Breakpoints that chase every device width multiply maintenance. Breakpoints that ignore content break at awkward sizes. Start from content failures, then reconcile with devices.

### Let Dense Content Break The Grid Deliberately

Data tables, wide media, comparison views, and code blocks often need more width than the content column allows. A grid that forces everything inside the content measure produces cramped tables and truncated data. Define when and how content may exceed the grid.

Decide:

- which content types may break to a wider region or full bleed;
- whether wide content breaks symmetrically or aligns to one edge;
- how wide content interacts with sticky sidebars and persistent navigation;
- how the same content reflows on narrow viewports (horizontal scroll, card transform, collapse).

Undefined break behavior leads to inconsistent implementations: some tables overflow, some get clipped, some shrink until unreadable. Make the exception a documented rule.

### Keep The System Token-Driven And Documented

A grid and spacing system that lives only in a designer's head or in one Figma file will not survive implementation. The values must become tokens that code and design share, and the rules for using them must be written down.

Ensure:

- spacing steps and grid values exist as design tokens consumed by both design and engineering;
- the rules (which step for what, when content may break, how gutters scale) are documented;
- off-grid values are detectable and discouraged in review;
- the system is versioned so changes are deliberate, not accidental drift.

## Common Traps

### Picking A 12-Column Grid Because It Is Conventional

A default column count that does not match the content forces awkward subdivisions and cramped regions; the count should follow content type.

### Inventing Spacing Values Per Screen

Ad-hoc pixel values that feel right on one layout destroy rhythm across the product and make the spacing scale meaningless.

### Treating Gutters And Margins As Fixed

Fixed gutters and margins produce cramped mobile layouts and absurd desktop gaps; they must scale across breakpoints.

### Forgetting Vertical Rhythm

Perfect horizontal columns with inconsistent vertical spacing still feel broken, because users scan vertically and notice the drift.

### Breakpoints Chasing Device Widths

Breakpoints set only by popular device sizes miss the real layout failure widths and become obsolete as devices change.

### One Content Width For Everything

A single max-width applied to text, data, and media underserves dense content and overserves reading text; width should follow content type.

### Letting Dense Content Overflow Undefinedly

When wide tables and media have no defined break rule, each implementation overflows, clips, or shrinks differently.

### Too Many Spacing Steps

A scale with fifteen steps gives the illusion of precision but in practice means designers pick the closest value and the rhythm drifts.

## Self-Check

- [ ] A single base unit drives a closed, documented spacing scale with named semantic steps, and off-grid values are discouraged in review.
- [ ] Spacing encodes relationship: tight gaps group related elements, medium gaps separate peers, and large gaps separate sections, consistently.
- [ ] Column count was chosen by content type (editorial, data, form, media), not by convention alone.
- [ ] Gutters and page margins scale across breakpoints, with a defined max content width beyond which extra space becomes margin.
- [ ] Vertical rhythm is established, either as a strict baseline for text-heavy layouts or a soft rhythm tied to the spacing scale.
- [ ] Breakpoints were chosen by where the layout actually breaks, then reconciled with common device widths.
- [ ] Dense content (tables, media, code) has a documented rule for when and how it may exceed the content column, and how it reflows on mobile.
- [ ] Grid and spacing values exist as shared design tokens, and the usage rules are documented and versioned.
- [ ] The system was tested against sparse, dense, wide, narrow, and localized (long translated) content, not only the ideal screen.
