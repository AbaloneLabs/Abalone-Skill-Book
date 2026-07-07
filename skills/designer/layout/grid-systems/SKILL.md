---
name: grid_systems.md
description: Use when the agent is selecting a grid type for a layout, choosing between manuscript, multicolumn, modular, baseline, and hierarchical grids, designing fluid and responsive grid behavior with container queries, building asymmetric or deconstructed grids, deciding column counts and grid regions, or applying grid theory to organize complex editorial, dashboard, or application layouts.
---

# Grid Systems

A grid system is the structural framework that organizes a layout into a deliberate arrangement of columns, rows, and regions. It looks like a set of lines, but it is really a theory of order: what kind of grid suits the content, how the grid adapts across viewports, and whether the structure imposes discipline or flexibility. Agents tend to default to a twelve-column web grid for everything, ignore the rich typology of grid types, or treat the grid as a fixed scaffold rather than a responsive system. The harm is layouts that fight their content: a data dashboard forced into an editorial grid, or a marketing page locked into rigid symmetry that should breathe.

Use this skill before choosing a grid type, defining its columns and regions, or planning how it behaves responsively. The goal is to prevent the agent from applying one grid pattern to every problem, from locking the grid to a fixed width, or from building a structure that cannot accommodate the content's real variety.

## Core Rules

### Match The Grid Type To The Content's Structure

Grids are not one thing. There is a typology of grid types, each suited to different content structures, and choosing the wrong type produces a layout that fights its material. A manuscript grid (single column) suits continuous reading; a multicolumn grid suits editorial with varied content; a modular grid suits repeated units like cards; a baseline grid suits text rhythm; a hierarchical grid suits irregular, asymmetric compositions.

Choose the grid type deliberately:

- manuscript grid (single column): for long-form reading where measure and flow matter most;
- multicolumn grid: for editorial content with varied lengths and sidebars;
- modular grid (columns and rows): for repeated units like image galleries, card grids, dashboards;
- baseline grid: for vertical text rhythm, especially in text-heavy layouts;
- hierarchical grid: for asymmetric, content-driven compositions where strict structure is too rigid.

A modular grid imposed on editorial prose, or a manuscript grid imposed on a dashboard, is the wrong tool. The content's structure should dictate the grid type, not the reverse.

### Let The Grid Serve The Content, Not The Other Way Around

A grid is a tool for organization, not a cage. The most common failure is forcing content into a grid that does not fit, producing awkward gaps, cramped columns, or truncated elements. A mature grid system knows when to follow the structure and when content legitimately breaks out of it.

Balance discipline and flexibility:

- use the grid to create consistent alignment and rhythm where it helps;
- allow specific content types (full-bleed media, wide tables, feature images) to break the grid deliberately;
- define the rules for breaking: which content may break, how far, and how it relates to the grid;
- avoid breaking the grid ad hoc, which produces inconsistency; make exceptions a documented pattern.

A grid that never bends produces rigid layouts; a grid that always bends is not a grid. The discipline is knowing where each applies.

### Design The Grid As Responsive, Not Fixed

A grid defined only at one width fails the moment the viewport changes. Modern grids must be fluid and responsive, adapting their column structure, gutters, and regions across breakpoints. Treating the grid as a fixed desktop scaffold produces broken mobile layouts and awkward tablet intermediates.

Build responsive grids:

- define how the column structure changes at each breakpoint (twelve columns may collapse to one on mobile);
- let gutters and margins scale, since fixed gutters break on narrow and ultrawide screens;
- use fluid units (percentages, fr units, viewport-based) for grid tracks rather than fixed pixels where appropriate;
- define a maximum content width beyond which extra space becomes margin, not wider content.

A grid that only exists at desktop width is half a grid. The responsive behavior is part of the grid's definition, not an afterthought.

### Use Container Queries For Component-Level Grid Adaptation

Traditional media queries adapt layout to the viewport, but components often appear in containers of different widths (a sidebar, a main column, a modal). A component grid that adapts to the viewport breaks when the component is placed in a narrow container on a wide screen. Container queries let the grid adapt to its actual available space.

Leverage container queries:

- design component grids that respond to their container's width, not just the viewport;
- use container queries for reusable components that appear in different contexts;
- combine viewport-based breakpoints for page structure with container queries for component structure;
- test components in all the containers they may inhabit, not just the default.

A component whose grid only works in one context is fragile. Container queries make grids context-aware, which is essential for component-driven products.

### Define Grid Regions For Complex Layouts

Simple column grids work for simple content, but complex layouts (dashboards, application shells, editorial spreads) need defined regions: a header, a sidebar, a main content area, an aside. These regions are not just columns; they are named areas with distinct behavior, and defining them explicitly makes the layout comprehensible and maintainable.

Define regions deliberately:

- name and size the major regions (header, nav, main, aside, footer) using grid template areas;
- decide which regions are fixed, fluid, or collapsible;
- define how regions reflow at breakpoints (a sidebar may move below content on mobile);
- document the region structure so the layout's organization is explicit, not implicit.

Undefined regions produce layouts where the structure is inferred from spacing alone, which is fragile and hard to maintain. Named regions make the architecture legible.

### Consider Asymmetric And Deconstructed Grids For Editorial Work

Not every grid is symmetric and regular. Editorial and marketing layouts often benefit from asymmetry, where elements are placed off-center, at varied sizes, and in deliberate tension. A deconstructed grid intentionally breaks regularity for expressive effect. These approaches are powerful but require more skill, because without discipline they read as random rather than intentional.

Use asymmetry with intention:

- use asymmetric grids where the content benefits from dynamic, expressive composition;
- balance asymmetry deliberately: offset a large element with a smaller, visually heavier one;
- ensure the asymmetry serves communication, not just novelty;
- avoid deconstructed grids in functional interfaces where predictability matters more than expression.

Asymmetry that is accidental reads as sloppy; asymmetry that is deliberate reads as designed. The difference is whether the imbalance serves a purpose.

### Align The Grid To A Baseline For Text Rhythm

Horizontal grids get attention; vertical rhythm is often neglected. A baseline grid, where text lines snap to a consistent horizontal rhythm, creates calm, scannable layouts, especially in text-heavy editorial work. Aligning the grid to a baseline ensures that vertical spacing is as disciplined as horizontal alignment.

Establish baseline rhythm:

- decide whether to use a strict baseline grid (every line snaps) or a soft rhythm (spacing steps only);
- align component heights, section gaps, and line-height to the baseline where feasible;
- reserve strict baselines for text-heavy layouts where the payoff is highest;
- in component-driven products, a soft rhythm tied to the spacing scale is usually more practical.

A layout with perfect columns but inconsistent vertical spacing still feels off, because users scan vertically. The baseline completes the grid in both dimensions.

### Document The Grid So It Survives Implementation

A grid that lives only in a design file will not survive implementation. Engineers, working from screenshots, reinvent the grid differently, and the structure drifts. The grid must be documented and tokenized so design and code share one definition.

Document and tokenize:

- express grid values (columns, gutters, margins, breakpoints) as design tokens consumed by both design and code;
- document the grid rules: which type, how it responds, when content may break;
- provide reference layouts showing the grid in use across content types;
- version the grid so changes are deliberate, not accidental drift.

An undocumented grid fragments across implementations. Documentation and tokenization are what make the grid a system rather than a one-off design.

## Common Traps

### Defaulting To A Twelve-Column Web Grid For Everything

A single grid type cannot serve manuscript reading, dashboards, and editorial alike; match the grid type to the content's structure.

### Forcing Content Into An Ill-Fitting Grid

A grid that never bends produces cramped columns and truncated elements; allow defined, documented breaks for content that needs them.

### A Fixed Desktop-Only Grid

Grids defined at one width fail on mobile and tablet; design responsive behavior with fluid tracks and scaling gutters.

### Ignoring Container Width

Components placed in different containers break when their grid only responds to the viewport; use container queries for context-aware grids.

### Undefined Layout Regions

Complex layouts without named regions have implicit structure that is fragile and hard to maintain; define and document major regions.

### Accidental Asymmetry

Asymmetric grids without deliberate balance read as sloppy rather than expressive; use asymmetry with intention and purpose.

### Neglecting Vertical Rhythm

Perfect columns with inconsistent vertical spacing still feel broken; align the grid to a baseline for text rhythm.

### An Undocumented Grid

Grids that live only in design files drift when implemented; tokenize values and document the rules so design and code share one definition.

## Self-Check

- [ ] The grid type (manuscript, multicolumn, modular, baseline, hierarchical) was chosen to match the content's structure, not defaulted by convention.
- [ ] The grid serves the content, with documented rules for when and how specific content types may break out.
- [ ] The grid is responsive, with column structure, gutters, and margins that adapt across breakpoints and a defined maximum content width.
- [ ] Component grids use container queries to adapt to their actual available space, not just the viewport.
- [ ] Major layout regions are named, sized, and documented, with defined reflow behavior at breakpoints.
- [ ] Asymmetric or deconstructed grids, if used, are deliberate and balanced, serving communication rather than novelty.
- [ ] Vertical rhythm is established through a baseline or soft rhythm tied to the spacing scale.
- [ ] Grid values are tokenized and the rules documented so design and code share one versioned definition.
