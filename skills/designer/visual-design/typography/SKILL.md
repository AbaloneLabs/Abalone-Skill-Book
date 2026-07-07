---
name: typography.md
description: Use when the agent is selecting typefaces, building a type scale, defining font weights and styles, setting line height and letter spacing, pairing fonts, handling responsive type, managing text overflow and truncation, or deciding how typography creates hierarchy, readability, and brand voice across a product interface.
---

# Typography

Typography is how language becomes visible. It looks like choosing fonts, but it is really a system of decisions about legibility, hierarchy, rhythm, and voice that users experience as clarity or confusion on every screen. Agents tend to treat typography as a styling preference, pick a typeface because it looks modern, or set sizes by eye until the scale drifts. The harm is invisible at first and cumulative: text becomes hard to read at length, hierarchy flattens so nothing leads, and the product loses a consistent voice across surfaces.

Use this skill before finalizing typefaces, a type scale, or text styling rules. The goal is to prevent the agent from choosing fonts that fail at real sizes and lengths, from building a scale that does not hold across breakpoints, or from ignoring the reading conditions under which users actually consume the text.

## Core Rules

### Choose Typefaces For Function Before Aesthetic

A typeface is a tool before it is a statement. Its primary job is to make text legible at the sizes and conditions users encounter: small labels, dense body copy, long-form reading, and quick scanning. A typeface chosen for personality that fails at twelve pixels or blurs at length has prioritized aesthetics over the user's ability to read.

Evaluate typefaces for function:

- test candidates at the actual sizes they will be used, including small UI labels and long body copy;
- check legibility on target devices and screens, not just high-resolution design canvases;
- prefer typefaces with multiple weights so hierarchy can be built without adding more families;
- consider the reading context: high-density data interfaces need different traits than editorial reading.

A beautiful typeface that users struggle to read is a failure, no matter how it looks in a mockup.

### Build A Closed Type Scale, Not Arbitrary Sizes

Typography needs a scale: a small set of defined sizes that relate by a consistent ratio, so hierarchy is predictable and the system composes. Arbitrary sizes chosen per screen produce drift, where headings vary subtly and the rhythm breaks. A closed scale forces every text element to a defined step.

Construct the scale deliberately:

- choose a base size for body text that is comfortable for reading (commonly sixteen pixels for web);
- derive heading and small-text sizes by a consistent ratio (such as a major third or perfect fourth);
- limit the scale to the steps actually needed, typically six to eight, to avoid decision fatigue;
- name the steps semantically (display, heading, subheading, body, caption) so usage is by intent, not raw numbers.

A scale with too many steps is as bad as none: designers pick the closest value and the hierarchy drifts. A scale with too few forces awkward compromises.

### Define Line Height And Measure For Readability

Size alone does not make text readable. Line height (leading) and measure (line length) determine whether the eye can track across lines without losing place or tiring. These are often set by default and then ignored, to the detriment of long-form reading.

Set reading parameters deliberately:

- body text line height is typically 1.4 to 1.6 times the font size for comfortable reading;
- headings need tighter line height (1.1 to 1.3) since they are short and bold;
- measure (line length) for body copy should be roughly forty-five to seventy-five characters; longer lines fatigue, shorter lines break rhythm;
- adjust line height up for small text and down for large text, since the ratio that aids reading shifts with size.

Text that runs edge to edge across a wide screen, or lines that are too tightly packed, degrades comprehension even when the typeface and size are fine.

### Establish Hierarchy Through Size, Weight, And Color Together

Hierarchy is not just bigger headings. It is the combined effect of size, weight, color, and spacing that tells the eye what leads, what supports, and what is secondary. Relying on size alone produces a flat ladder where every level shouts. A disciplined system uses all four levers.

Build hierarchy with multiple levers:

- use size for the primary hierarchy steps;
- use weight (regular, medium, semibold, bold) to differentiate within a size;
- use color (darker for primary text, lighter for secondary) to indicate prominence;
- use spacing (more space above a heading than below) to reinforce grouping.

A heading that is only slightly larger but bolder and darker reads as more important than a heading that is larger but the same weight and color as its body. Hierarchy is a system property.

### Handle Responsive Type Across Breakpoints

Type that looks right on desktop often breaks on mobile: headings become too large, line lengths too long, and body text too small relative to the viewport. A type system must define how the scale adapts across breakpoints, not assume one set of sizes serves all.

Plan responsive type:

- define how each scale step changes at mobile, tablet, and desktop breakpoints;
- consider fluid typography (using viewport units or clamp) for smooth scaling, but verify it does not produce unreadable extremes;
- reduce heading sizes more aggressively than body sizes on small screens, since large headings dominate mobile viewports;
- re-evaluate line height and measure at each breakpoint, since narrow screens need different ratios.

A type system that only works at one breakpoint fails the majority of users who read on phones.

### Manage Font Loading And Performance

Every typeface and weight added to a product increases load time, and slow font loading causes layout shift and flash of unstyled text. Agents often specify many weights and families without considering the performance cost, degrading the experience especially on slow connections.

Govern font performance:

- limit the number of typeface families, ideally one or two;
- load only the weights actually used, not the full family;
- use font-display strategies (swap, optional) that balance perceived performance with avoiding invisible text;
- consider variable fonts, which provide many weights in a single file;
- preload critical font files so they are available for first render.

A beautiful type system that takes five seconds to load harms users more than a simpler system that renders instantly.

### Define Text Overflow And Truncation Behavior

Real content does not always fit. Long names, translated text, and user-generated content overflow their containers. A type system must define what happens when text exceeds its space, or each implementation handles it differently, producing clipped, overlapping, or broken layouts.

Define overflow rules:

- decide where truncation with ellipsis is acceptable and where wrapping is preferred;
- define line clamping for multi-line truncation and how the ellipsis appears;
- ensure tooltips or expandable states reveal truncated content;
- account for text expansion in translation, which can increase length by twenty to forty percent;
- test long-string edge cases (no spaces, very long words) that break layouts.

Undefined overflow behavior leads to inconsistent implementations where some text clips, some overflows, and some breaks the layout entirely.

### Maintain A Consistent Voice Across Surfaces

Typography carries brand voice. A product that uses different typefaces or treatments across its surfaces feels disjointed, as if each screen were designed by a different team. A consistent typographic system is part of how a product feels coherent.

Enforce voice consistency:

- use the same typeface families and scale across all surfaces, including marketing, product, email, and help;
- define how the type system adapts to different contexts without changing its core identity;
- document the type system so new surfaces inherit it rather than inventing their own;
- review cross-surface consistency as part of design critique.

A type system that only lives in the product UI, while marketing uses different fonts, creates a fragmented brand experience.

## Common Traps

### Choosing A Typeface For Looks Over Legibility

A typeface that is stylish but hard to read at real sizes and lengths fails its primary function; test candidates under actual reading conditions.

### Arbitrary Sizes Instead Of A Scale

Sizes chosen per screen produce hierarchy drift; build a closed scale with named semantic steps and a consistent ratio.

### Ignoring Line Height And Measure

Text with default leading and edge-to-edge line lengths fatigues readers; set line height and measure for comfortable reading.

### Hierarchy By Size Alone

Relying only on size produces a flat ladder; combine size, weight, color, and spacing to build clear hierarchy.

### One Size Set For All Breakpoints

Type that works on desktop often breaks on mobile; define how the scale adapts across breakpoints.

### Loading Too Many Fonts And Weights

Each additional family and weight increases load time and causes layout shift; limit families, load only used weights, and use font-display strategies.

### Undefined Text Overflow Behavior

When truncation and wrapping are not defined, implementations clip, overflow, or break inconsistently; define rules and test edge cases.

### Inconsistent Type Across Surfaces

Different typefaces in product, marketing, and email fragment the brand voice; maintain one documented type system across all surfaces.

## Self-Check

- [ ] Typefaces were tested for legibility at actual sizes and reading conditions before selection.
- [ ] A closed type scale with named semantic steps and a consistent ratio drives all text sizes.
- [ ] Line height and measure are set for readability (1.4 to 1.6 leading, forty-five to seventy-five character measure for body).
- [ ] Hierarchy is built from size, weight, color, and spacing together, not size alone.
- [ ] The type scale adapts across breakpoints, with heading sizes reduced more than body on mobile.
- [ ] Font loading is performant: limited families and weights, font-display strategies, and preloading of critical files.
- [ ] Text overflow and truncation behavior is defined, with handling for translation expansion and long-string edge cases.
- [ ] The type system is consistent across product, marketing, email, and help surfaces, documented and enforced.
