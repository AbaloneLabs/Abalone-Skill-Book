---
name: type_scale_and_hierarchy.md
description: Use when the agent is defining a type scale, setting font sizes and line heights for headings and body, establishing visual hierarchy, choosing modular ratios, building responsive type, relating type to spacing, or ensuring hierarchy reads correctly across viewports, densities, and content lengths.
---

# Type Scale And Hierarchy

A type scale is not a list of font sizes. It is the system by which an interface communicates structure, priority, and relationship through type alone. When the scale is well built, users scan a page and instantly sense what is title, what is section, what is label, and what is body, without reading a word. When the scale is poorly built, every level fights for attention, headings barely differ from body, and the interface feels noisy even when it is sparse. Most hierarchy problems are not taste failures; they are scale failures, where the steps, ratios, and relationships were never designed as a system.

Use this skill before defining a type scale, setting heading and body sizes, choosing the number of type levels, building responsive type, or reviewing whether hierarchy reads correctly. The goal is to prevent the agent from picking sizes by feel, creating too many similar levels, or building a scale that collapses the moment content, viewport, or density changes.

## Core Rules

### Build Hierarchy Through Clear, Deliberate Steps

Hierarchy works only when adjacent levels are distinguishable. Two heading sizes that differ by one pixel read as the same level; a body size and a label size that are too close blur the boundary between content and metadata. The eye needs a clear jump between levels to register a change in importance.

Choose steps large enough to read as distinct. A common strong approach is a modular ratio, where each level multiplies or divides by a consistent factor, producing even, predictable steps. The exact ratio matters less than that the steps are deliberate and perceptible. Avoid clustering several levels within a narrow range, which produces a scale that looks graduated in a design tool but reads as flat in an interface.

### Limit The Number Of Levels

Every level added to a scale increases the chance of ambiguity and the maintenance burden. A scale with nine heading sizes guarantees that two of them will be used inconsistently, because designers cannot reliably tell them apart. Most interfaces need fewer levels than teams assume: a display or hero size, a few heading levels, body, and a small set of secondary sizes for labels, captions, and overlines.

Define the minimum set of levels that expresses the real structure of the content, and resist adding levels to solve one-off layout problems. If two levels are nearly identical, merge them.

### Pair Size With Weight, Color, And Spacing

Hierarchy is rarely carried by size alone. A small label in a bold weight, an uppercase treatment, or a muted color can mark a section heading without making it large. Relying only on size forces every level to be bigger than the last, producing oversized headings and an inflated scale. Combining size with weight, color, case, and spacing lets the scale stay compact while the hierarchy stays clear.

Use these levers together:

- size, for the primary structural jump between levels;
- weight, to emphasize without enlarging;
- color or opacity, to de-emphasize secondary text without shrinking it;
- case and letter-spacing, for labels and overlines that mark structure;
- spacing, since a heading surrounded by white space reads as a new section even at a modest size.

### Set Line Height Relative To Size And Purpose

Line height is not a single value; it scales inversely with text size. Large headings need tight line height, because the extra space between tall lines opens ugly gaps. Small body text needs looser line height, so the eye can track across wrapped lines without losing place. A single line height applied across a scale produces headings that gap apart and body that crowds together.

Set line height per level:

- display and large headings, tight, around 1.0 to 1.15;
- subheadings, slightly looser;
- body text, comfortable for reading, around 1.4 to 1.6;
- short labels and single-line text, can be tighter since wrapping is not the concern.

### Make The Scale Responsive Without Breaking Hierarchy

Type that works on desktop can overwhelm a small screen or vanish on a large one. A responsive scale adjusts sizes across viewports while preserving the relative relationships between levels. If a heading shrinks on mobile but the body does not, the hierarchy flattens; if everything scales by the same factor, the proportions hold.

Use fluid or viewport-based sizing, or define explicit scale steps per breakpoint, but in either case verify that the hierarchy, the ratio between levels, survives at every size. The smallest viewport is where scales most often collapse, because levels crowd together when everything shrinks.

### Relate Type Scale To Spacing Scale

Type and spacing are one system, not two. A heading's surrounding space should relate to its size, so that larger headings get more breathing room and the rhythm of the page stays consistent. When type and spacing scales are designed independently, headings float at arbitrary distances and the page loses its vertical rhythm.

Align the spacing scale to the type scale, often through a shared base unit, so that padding, margins, and gaps reinforce the same structure the type expresses.

### Verify Hierarchy With Real Content, Not Placeholder Text

A scale built against lorem ipsum or short labels hides the failures that real content exposes. Long headings wrap to multiple lines and need tested line height; dense body text reveals whether the chosen size and measure are readable; metadata-heavy layouts show whether secondary levels stay distinct from body. Test the scale against the longest, densest, and most varied content the interface will actually carry.

## Common Traps

### Too Many Similar Levels

A scale with many near-identical sizes produces ambiguity, because designers cannot reliably choose between them and the hierarchy reads as flat.

### Hierarchy By Size Alone

Forcing every level to be larger inflates the scale and produces oversized headings, when weight, color, or spacing could carry the distinction.

### One Line Height For Everything

A single line height gaps apart large headings and crowds small body text, the opposite of what each needs.

### Scale That Collapses On Mobile

When all levels shrink by different factors, or only some shrink, the hierarchy flattens on small viewports where space is tightest.

### Type And Spacing Designed Separately

Headings float at arbitrary distances when spacing does not relate to the type scale, breaking the page's vertical rhythm.

### Building The Scale Against Placeholder Text

Lorem ipsum hides wrapping, density, and length failures that real content reveals immediately.

### Steps Too Small To Perceive

A modular ratio chosen too conservatively produces levels that look distinct in a tool but read as the same size in an interface.

## Self-Check

- [ ] Adjacent type levels differ by a clear, perceptible step, not a marginal one.
- [ ] The scale uses the minimum number of levels needed to express real content structure.
- [ ] Hierarchy is carried by a combination of size, weight, color, case, and spacing, not size alone.
- [ ] Line height is set per level, tight for large headings and looser for body text.
- [ ] The scale is responsive, and the ratio between levels survives at every viewport, especially the smallest.
- [ ] The spacing scale relates to the type scale through a shared base unit or rhythm.
- [ ] The scale was tested against long, dense, and varied real content, not placeholder text.
- [ ] No two levels are so close that designers would use them inconsistently.
- [ ] Display and heading sizes do not overwhelm, because secondary levers carry emphasis where possible.
- [ ] The hierarchy reads correctly at a glance, allowing users to sense structure before reading the words.