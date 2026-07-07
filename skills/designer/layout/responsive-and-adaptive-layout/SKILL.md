---
name: responsive_and_adaptive_layout.md
description: Use when the agent is designing layouts that must work across viewport sizes and devices, choosing between responsive and adaptive strategies, defining breakpoints, planning reflow of regions and components, handling touch and pointer input differences, designing for mobile, tablet, and desktop simultaneously, avoiding fixed-width assumptions, or ensuring a layout's structure, hierarchy, and usability survive from the smallest to the largest screen.
---

# Responsive And Adaptive Layout

A layout that works on one viewport is half a layout. Real products are encountered on phones held one-handed on a train, on tablets propped on a kitchen counter, on laptops in a meeting, and on ultrawide monitors in an office, and the same design must serve all of them. Responsive and adaptive layout is the discipline of designing for that range deliberately, so that structure, hierarchy, and usability survive from the smallest to the largest screen. The failure mode is not that the layout breaks visibly; it is that it technically reflows while losing the qualities that made it work, so a hierarchy that sang on desktop collapses into an undifferentiated scroll on mobile.

Agents tend to fail responsive design in predictable ways. They design for desktop first and squeeze the result onto mobile, producing cramped, hard-to-use small screens. They pick breakpoints by device size rather than by content, so the layout reflows at arbitrary points and breaks at the widths between them. They reflow regions without preserving hierarchy or reading order, so the most important content gets buried. Or they treat responsiveness as a technical concern handed to engineering, never specifying how the design should behave across the range.

Use this skill before finalizing any layout that must work across devices, when defining breakpoints, and when planning reflow. The goal is a responsive or adaptive strategy whose breakpoints are content-driven, whose hierarchy and order survive reflow, and whose interaction model fits each device.

## Core Rules

### Choose Between Responsive And Adaptive Deliberately

Responsive and adaptive are different strategies, and the choice has real consequences. Responsive design uses a single fluid layout that continuously adapts to the viewport. Adaptive design serves a small number of distinct layouts, each optimized for a class of device. Neither is universally superior, and choosing without intent produces a layout that is neither efficient nor consistent.

Choose deliberately:

- responsive, fluid layouts suit content that reflows gracefully and products that want one codebase across all widths;
- adaptive layouts suit experiences where distinct device classes, such as mobile and desktop, benefit from genuinely different structures;
- hybrids are common, with a fluid layout that adopts distinct configurations at major breakpoints;
- document the choice, because it determines how the design is built and where it can break.

### Let Content, Not Devices, Define Breakpoints

Breakpoints defined by specific device widths rot quickly, because devices change every year. Breakpoints defined by content, the points at which the layout stops working well, are durable, because they reflect the design rather than the hardware.

Define content-driven breakpoints:

- identify the viewport widths at which the current layout breaks, where lines get too long, regions collide, or hierarchy inverts;
- set breakpoints at those points, not at the pixel width of a popular phone;
- add breakpoints only where the design actually needs them, because each one adds maintenance cost;
- test the widths between breakpoints, because the layout must work everywhere, not only at popular sizes.

A layout that works at 375px, 768px, and 1440px but fails at 500px and 1000px has not been designed for the range; it has been designed for three devices.

### Preserve Hierarchy And Reading Order Across Reflow

The most common responsive failure is a layout that reflows correctly but loses its meaning. A hero that dominated on desktop becomes a small image buried mid-page on mobile; a sidebar of primary actions drops below the content it supported; a reading order that flowed left-to-right becomes top-to-bottom in the wrong sequence. Reflow must preserve hierarchy and order, not just avoid overlap.

Preserve structure through reflow:

- decide explicitly how the focal hierarchy adapts, so the primary content remains primary at every width;
- sequence the DOM so the natural reading order is correct at every breakpoint, which also serves screen readers;
- choose which regions promote, demote, collapse, or hide at each breakpoint, and document the decisions;
- avoid burying primary actions or content simply because they were secondary visually on desktop.

### Design Mobile First When It Serves The Product

Designing mobile first, starting from the smallest screen and expanding, forces hard decisions about priority that desktop-first design avoids. When the product's center of gravity is mobile, or when constraint clarifies hierarchy, mobile first produces stronger layouts at every size.

Apply mobile first when it fits:

- start from the constraints of the smallest target screen, which forces ruthless prioritization;
- add complexity as the viewport grows, promoting secondary content and multi-column layouts at breakpoints;
- use mobile first when the majority of users are on mobile, or when the team tends to overfill desktop layouts;
- consider desktop first when the product is primarily a professional desktop tool, but still validate mobile explicitly.

### Handle Input Differences Across Devices

Layout is inseparable from input. A layout sized for a mouse pointer fails on touch, where targets must be larger and spaced apart. A layout that depends on hover fails on touch, where hover does not exist. A layout that assumes a keyboard may fail on devices that lack one. The interaction model must fit the device.

Adapt to input:

- size touch targets adequately and separate them so they do not conflict, especially on small screens;
- replace hover-dependent affordances with persistent or focus-revealed equivalents on touch devices;
- ensure the layout is fully operable by keyboard, because some users on every device navigate without a pointer;
- account for input modality changes within a session, such as a touch laptop that switches between touch and pointer.

### Scale Type, Spacing, And Imagery Fluidly

Fixed values break across viewports. Type set in fixed pixels becomes too large on mobile and too small on ultrawide monitors; fixed spacing becomes disproportionate at extremes; fixed-width images overflow or waste space. Fluid values keep the layout proportional across the range.

Scale fluidly:

- use relative units for type and spacing, such as rem, em, and viewport-based units, so they scale with the user's settings and the viewport;
- let spacing scale at breakpoints so gaps remain proportional rather than fixed;
- make imagery responsive, serving appropriately sized assets and using techniques that preserve aspect ratio without overflow;
- define minimum and maximum values so fluid scaling does not produce unusable extremes.

### Test Across The Full Range, Not Just Popular Widths

A layout validated only at a few popular widths fails at the widths between them and at the extremes. The discipline is to test the full range, including the awkward in-between sizes and the smallest and largest targets.

Test the range:

- test at the smallest and largest target viewports, where failures are most visible;
- test the widths between breakpoints, where the layout is most likely to break;
- test with real content at realistic lengths, because placeholder content hides overflow and truncation;
- test on real devices, because emulators miss performance, input, and rendering differences.

### Decide What Collapses, Hides, Or Promotes At Each Breakpoint

Responsive design requires explicit decisions about what happens to each region as the viewport changes. Leaving these decisions implicit forces engineering to guess, and the result rarely matches the design's intent.

Decide per region:

- which content promotes to a more prominent position on smaller screens;
- which secondary content collapses behind a toggle or into a menu;
- which content hides entirely, and whether that loss is acceptable for that device class;
- how navigation transforms, from tabs to a drawer to a bottom bar, and what each transformation costs.

## Common Traps

### Desktop-First Squeezed Onto Mobile

Designing for desktop and compressing the result produces cramped, hard-to-use small screens with buried hierarchy.

### Device-Based Breakpoints

Breakpoints tied to specific device widths rot as hardware changes. Define breakpoints by where the content breaks.

### Reflow That Loses Hierarchy Or Order

A layout that reflows without preserving priority and reading order is technically responsive but functionally broken.

### Hover-Dependent Affordances On Touch

Designs that reveal actions or information only on hover fail on touch devices where hover does not exist.

### Fixed Values Across Viewports

Fixed pixel type, spacing, and imagery break at the extremes of the viewport range.

### Testing Only Popular Widths

A layout that works at 375, 768, and 1440 but fails between them has not been designed for the range.

### Untested Real Content

Placeholder content hides overflow, truncation, and density problems that real content reveals.

### Implicit Region Decisions

Leaving collapse, hide, and promote decisions to engineering produces results that rarely match design intent.

## Self-Check

- [ ] A responsive or adaptive strategy was chosen deliberately, documented, and matched to the product's needs.
- [ ] Breakpoints are defined by where the content breaks, not by specific device widths, and tested between breakpoints.
- [ ] Reflow preserves the focal hierarchy and reading order at every breakpoint, with primary content remaining primary.
- [ ] The starting point, mobile-first or desktop-first, was chosen to fit the product's center of gravity.
- [ ] Touch targets are adequately sized and spaced, hover-dependent affordances have persistent equivalents, and the layout is fully keyboard-operable.
- [ ] Type, spacing, and imagery use relative and fluid units with defined minimums and maximums.
- [ ] The layout was tested across the full range, including extremes and in-between widths, with real content on real devices.
- [ ] Explicit decisions were made for each region about promotion, collapse, hiding, and navigation transformation at each breakpoint.
- [ ] The DOM order supports the correct reading order at every breakpoint, serving both sighted and screen reader users.
- [ ] No fixed-width assumption causes overflow, truncation, or wasted space at any target viewport.
