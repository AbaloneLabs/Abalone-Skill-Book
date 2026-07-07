---
name: responsive_and_adaptive_strategy.md
description: Use when the agent is choosing between responsive and adaptive design strategies, deciding breakpoints and layout reflow, determining when to serve different layouts versus reflowing one layout, planning for viewport ranges from phone to desktop, handling orientation changes, foldables, and multi-window, or deciding the boundary between fluid reflow and distinct adaptive experiences, and must avoid both a single stretched layout that breaks at the edges and an unmaintainable proliferation of bespoke layouts.
---

# Responsive And Adaptive Strategy

Responsive and adaptive are not competing labels; they are two ends of a strategy for handling the fact that one product must work across a wide range of viewports, orientations, and window configurations. The judgment problem is not picking "responsive" or "adaptive" as a team. It is deciding, for each part of the interface and each range of conditions, whether a single fluid layout can reflow gracefully or whether the differences are great enough to warrant distinct adaptive layouts, and where to place the boundaries between them. Agents tend to fail in two opposite directions: building one fluid layout and stretching it until it breaks at the extremes, producing an experience that is mediocre everywhere, or building a bespoke layout for every device category until the maintenance burden makes every change expensive and slow.

Use this skill when planning how a product adapts across phone, tablet, desktop, large desktop, landscape, portrait, foldable, and multi-window configurations, when choosing breakpoints, or when deciding between reflow and distinct layouts. The goal is a deliberate strategy that uses fluid reflow where it suffices and distinct adaptive layouts where the differences demand them, without over-stretching or over-proliferating.

## Core Rules

### Treat Responsive And Adaptive As A Spectrum, Not A Binary

A real cross-viewport strategy is almost always a mix. Some parts of the interface reflow fluidly across a wide range with no structural change: a text article, a simple form, a list. Other parts change structure enough that a single layout cannot serve both ends: a data-dense dashboard, a multi-column comparison, a complex creation tool. The discipline is identifying, for each surface, where fluid reflow is sufficient and where adaptive distinct layouts are warranted, and combining both in one product. Treating the choice as all-or-nothing for the whole product produces either brittle stretching or unmaintainable fragmentation.

For each surface, ask:

- can one fluid layout serve the full viewport range without breaking;
- at what viewport does the layout start to strain, and can reflow rules fix it;
- are the structural differences between the small and large experience large enough to justify distinct layouts;
- where is the boundary, and is it a clean breakpoint or a gradual reflow.

Map the strategy surface by surface rather than imposing one approach globally.

### Choose Breakpoints Based On Content Strain, Not Device Names

Breakpoints should be placed where the layout begins to strain, not at the pixel widths of popular devices. A layout does not break because the viewport hits 768px; it breaks when a column becomes too narrow, when targets collide, when text wraps badly, or when comparison becomes impossible. Naming breakpoints after devices, tablet, desktop, leads to designs that break on the many viewports that do not match those names: large phones, small laptops, split-screen windows, zoomed browsers. Choose breakpoints by observing where the content stops working and placing the adaptation there.

Identify breakpoints by:

- resizing the layout continuously and noting where it strains;
- checking the content at the smallest and largest plausible widths;
- testing orientations, zoom levels, and multi-window configurations;
- placing breakpoints at the points of strain, not at device widths.

Content-driven breakpoints survive the constant arrival of new device sizes; device-named breakpoints do not.

### Decide Reflow Versus Distinct Layout By Structural Difference

The decision to reflow one layout or build a distinct adaptive layout should turn on how structurally different the experience needs to be. Minor differences, column counts, stacking order, control density, are well served by reflow rules on one layout. Major differences, different primary actions, different navigation models, fundamentally different task structures, warrant distinct adaptive layouts that are designed and maintained separately. Building distinct layouts for minor differences wastes effort; forcing reflow through major differences produces a layout that is bad at both ends.

Reflow when the differences are:

- column count and stacking;
- density and spacing;
- show or hide of secondary elements;
- reordering within the same structure.

Build distinct adaptive layouts when the differences are:

- different navigation model or primary action;
- fundamentally different task structure;
- different content priorities that reflow cannot express;
- input method changes that reshape the interaction.

### Preserve Content And Task Parity Across Configurations

Whatever strategy is chosen, the user must be able to accomplish the same core tasks across configurations. Responsive reflow that drops content at small widths, or adaptive layouts where the mobile version lacks a feature the desktop has, breaks the user's expectation that the product is one thing. Ensure that across every breakpoint and configuration, the core content is present and the core tasks are completable, even if the presentation differs. Parity is about capability, not identical pixels.

Check parity by:

- listing core tasks and confirming each is completable at every configuration;
- confirming essential content is reachable, not hidden or omitted, at each width;
- verifying that adaptive distinct layouts do not silently drop features;
- testing the extremes, smallest phone and largest desktop, not just the middle.

### Account For Orientation, Foldables, And Multi-Window

Viewport width is not the only variable. Orientation changes the aspect ratio and reachable zones; foldables change the available area and create seams; multi-window and split-screen shrink the usable viewport on a large screen. A strategy that only considers full-screen width breaks in these configurations, which are increasingly common. Plan for orientation by ensuring layouts work in both portrait and landscape where relevant, for foldables by avoiding critical content across the seam, and for multi-window by treating a half-width desktop window as a real configuration, not an edge case.

Consider:

- portrait and landscape for tablets and large phones;
- foldable seam placement and continuity;
- multi-window and split-screen as legitimate viewport sizes;
- resizable browser windows on desktop, not just maximized.

### Plan The Maintenance Cost Of Distinct Layouts

Every distinct adaptive layout is an ongoing maintenance cost: each change must be made in multiple places, each bug can appear in one layout and not another, and divergence accumulates over time. This cost is justified when the layouts are genuinely different enough, but it must be entered into knowingly. Before adding a distinct layout, weigh the experience benefit against the long-term maintenance burden, and prefer reflow with well-chosen breakpoints when it can serve the need. Where distinct layouts are necessary, share as much as possible through a common design system and component contracts to reduce duplicated effort.

### Test At The Extremes And The Seams

The middle of the viewport range is easy; the extremes and the transitions are where strategies fail. A layout that looks fine at common widths may break at the smallest phone, the largest monitor, the orientation flip, or the breakpoint itself, where the layout jumps. Test deliberately at the extremes, at every breakpoint boundary, and during the transition between configurations, because these are where strain and discontinuity appear.

## Common Traps

### Stretching One Layout Until It Breaks

A single fluid layout forced across the full range is mediocre at both ends. Use adaptive distinct layouts where structural differences demand them.

### Proliferating Bespoke Layouts

A distinct layout for every device category becomes unmaintainable. Prefer reflow with content-driven breakpoints where it suffices.

### Device-Named Breakpoints

Breakpoints tied to device widths break on the many viewports that do not match. Place breakpoints where content strains.

### Forcing Reflow Through Major Differences

When the small and large experiences need different structures, reflow produces a layout bad at both ends. Build distinct layouts for structural differences.

### Dropping Content Or Capability At Small Widths

Reflow or adaptive layouts that omit core content or tasks break parity. Ensure capability across every configuration.

### Ignoring Orientation, Foldables, And Multi-Window

Strategies that consider only full-screen width fail in landscape, across seams, and in split windows. Treat these as real configurations.

### Underestimating Maintenance Cost

Each distinct layout adds ongoing cost and divergence risk. Weigh the benefit and share components through a common system.

### Testing Only The Middle Of The Range

The extremes and the breakpoint transitions are where strategies fail. Test the smallest, largest, and seam configurations deliberately.

## Self-Check

- [ ] The strategy mixes fluid reflow and distinct adaptive layouts by surface, chosen by structural difference rather than imposed globally.
- [ ] Breakpoints are placed where content strains, not at device-named widths.
- [ ] Reflow is used for minor differences and distinct layouts for major structural or task differences.
- [ ] Core content and tasks remain available and completable across every configuration, not just the middle of the range.
- [ ] Orientation, foldable seams, and multi-window or split-screen are treated as real configurations, not edge cases.
- [ ] The maintenance cost of distinct layouts was weighed against the experience benefit before committing.
- [ ] Distinct layouts share as much as possible through a common design system to limit divergence.
- [ ] The layout was tested at the extremes, at every breakpoint boundary, and during configuration transitions.
- [ ] No configuration silently drops content or capability the user expects.
- [ ] The strategy survives the arrival of new device sizes because it is driven by content strain, not device names.
