---
name: visual_balance_and_alignment.md
description: Use when the agent is arranging elements on a screen, page, card, or section and must decide visual balance, alignment, symmetry, weight distribution, optical adjustment, and how composition reads as a whole rather than as scattered parts.
---

# Visual Balance And Alignment

A layout can contain all the right elements and still feel wrong. The cause is usually not the content itself but the composition: elements that are technically aligned but optically off, weight that piles up on one side, edges that almost meet, and groups that float without a relationship to anything around them. Balance and alignment are what make a design feel intentional and stable instead of assembled.

Use this skill before finishing any screen, section, card, hero, table row, toolbar, sidebar, or marketing block. The goal is to prevent the agent from producing a layout where every element is placed by implementation convenience — left to right, top to bottom, in source order — without considering how the whole composition reads, where weight accumulates, and whether relationships between elements are visually expressed.

## Core Rules

### Decide The Balance Strategy

Balance is the distribution of visual weight across a composition. Weight comes from size, color, density, contrast, imagery, and position. Before placing elements, decide which balance strategy the surface needs.

Common strategies:

- **Symmetrical balance**: weight mirrored around a center axis. Conveys formality, stability, calm, and trust. Useful for hero moments, login screens, certificates, and brand pages.
- **Asymmetrical balance**: unequal elements balanced by contrasting weight, position, or negative space. Conveys energy, sophistication, and modernity. Useful for editorial pages, dashboards, and product showcases.
- **Radial balance**: weight radiating from a center. Rare in product UI but useful for diagrams, hubs, and feature wheels.

The wrong strategy for the context is a common failure. A formal banking dashboard that uses loose asymmetry can feel unstable. A creative portfolio that uses rigid symmetry can feel stiff. Match the balance to the message.

### Align To A Shared Structure

Alignment is the skeleton that makes scattered elements feel related. Elements that share an edge or center axis read as a group even without a visible container.

Establish alignment through:

- a consistent left or center text edge within a column;
- baselines that keep text rows on a shared line;
- icon and label pairs aligned on a common center or edge;
- card grids whose tops, bottoms, or centers line up across rows;
- a clear relationship between a section heading and the content it governs.

When alignment is broken — a label nudged a few pixels off, an icon center that does not match its text baseline, a grid where one card hangs lower — the eye registers the error as sloppiness even when the user cannot name it.

### Use Optical Alignment Over Mathematical Alignment

Mathematical alignment treats bounding boxes as the truth. Optical alignment treats what the eye perceives as the truth. The two often disagree.

Cases where optical adjustment matters:

- icons inside buttons or tiles often need to sit slightly above the mathematical center to look centered, because rounded shapes read lower than flat shapes;
- text that is vertically centered in a box by its cap height can look too low;
- a left border or accent that aligns to the text's x-height rather than its bounding box looks more connected;
- shadows and rounded corners shift the perceived edge inward, so adjacent spacing may need compensation;
- a chevron or arrow next to text often needs to align to the text's optical center, not the text block's mathematical center.

Design systems that rely only on mathematical bounding boxes produce interfaces that feel almost right but subtly wrong. The difference between amateur and professional craft is often a few pixels of optical correction.

### Distribute Weight Intentionally

Visual weight is not evenly distributed by accident. Decide where the eye should land first and arrange weight to guide it there.

Weight accumulates from:

- large headings and hero imagery;
- saturated or dark colors;
- dense clusters of text or data;
- primary buttons and high-contrast calls to action;
- faces, figures, and prominent subjects in imagery.

If the heaviest elements cluster on one side with nothing to balance them, the composition tips. Balance does not require equal size — a large light element can balance a small dense one. The point is that the distribution is a decision, not an accident of source order.

### Use Negative Space As A Structural Element

Negative space is not leftover room. It is an active part of the composition that defines grouping, breathing room, and emphasis.

Negative space should:

- separate distinct groups more than it separates items within a group;
- increase around the primary focal point to give it emphasis;
- remain consistent within a rhythm so spacing reads as intentional;
- not be so large that related elements lose their relationship, nor so small that the layout feels cramped.

A common failure is filling every available pixel. Density without structure produces clutter. Structure with appropriate negative space produces clarity.

### Keep Edges And Gutters Consistent

The relationships between elements are expressed through consistent edges and gutters. When the gap between a heading and its body changes from section to section, or when the left edge of content drifts, the interface loses its grid feel.

Check:

- the left and right edges of content align across the page or screen;
- vertical rhythm between headings, body, and controls is consistent;
- gutters between grid columns are uniform;
- the spacing inside a card matches the spacing system used elsewhere;
- sticky headers, sidebars, and footers respect the same content edge.

Inconsistency in edges and gutters is one of the most common signs of a design assembled piece by piece rather than composed as a whole.

### Balance Density Across The Composition

A composition where one region is dense and packed while another is sparse and empty can feel unbalanced unless the emptiness is intentional emphasis. Review the whole surface, not just each element in isolation.

For dense regions, consider whether breaking content into a second column, a tab, or a progressive disclosure pattern would distribute weight better. For sparse regions, consider whether supporting imagery, a related action, or adjusted spacing would reduce the feeling of a hole.

## Common Traps

### Centering Everything By Default

Center alignment feels safe and balanced, but centering every element destroys alignment relationships and makes scanning harder. Center alignment is a deliberate choice for specific moments, not a default.

### Ignoring Optical Illusions

Rounded shapes, triangles, shadows, and letterforms all create optical effects that mathematical alignment cannot capture. Treating bounding boxes as truth produces layouts that look off by a few pixels.

### Balancing Only Left To Right

Balance is two-dimensional. A composition can be balanced horizontally but top-heavy or bottom-heavy. Review weight distribution in both axes.

### Inconsistent Baselines

When text rows do not share a baseline rhythm, columns that should align drift apart. This is especially visible in tables, dashboards, and side-by-side comparisons.

### Filling Space Instead Of Composing It

The instinct to fill empty regions often produces filler content, decorative elements, or oversized imagery that adds weight without adding meaning. Empty space used well is more valuable than empty space filled poorly.

### Aligning To The Wrong Reference

Aligning a label to a container edge when it should align to related content, or aligning an icon to a box when it should align to text, breaks the perceived relationship. Always align to the element the user relates the target to.

## Self-Check

- [ ] The balance strategy (symmetrical, asymmetrical, or radial) is a deliberate choice that matches the surface's intent.
- [ ] Elements that should read as a group share a clear alignment edge, center axis, or baseline.
- [ ] Icons, text in containers, and adjacent shapes use optical alignment, not only mathematical bounding-box alignment.
- [ ] Visual weight is distributed intentionally, with the primary focal point receiving appropriate emphasis.
- [ ] Negative space is used as structure to define grouping and emphasis, not merely as leftover room.
- [ ] Content edges, gutters, and vertical rhythm are consistent across the surface.
- [ ] Density is balanced across the composition, with no unintentionally top-heavy, bottom-heavy, or lopsided regions.
- [ ] Center alignment is used deliberately for specific moments, not as a default for everything.
- [ ] The composition was reviewed as a whole, not element by element, for how weight and relationships read together.
