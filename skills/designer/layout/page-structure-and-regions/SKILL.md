---
name: page_structure_and_regions.md
description: Use when the agent is structuring a page into regions such as header, sidebar, main content, footer, action bars, and contextual panels, deciding what is persistent versus scrolling, allocating viewport space, and defining how regions compose across page types.
---

# Page Structure And Regions

Page structure is the decision of which regions exist, where they live, how much viewport they claim, and how they behave when the page changes. It is easy to think of structure as arranging boxes, but it is really the decision of what stays stable while everything else changes. A header that disappears on scroll, a sidebar that steals a third of a small screen, an action bar that covers the submit button, a main region that has no defined place for the primary action, these are structure failures, and they compound because every page inherits them. Agents tend to draw one full page, place regions where they look balanced, and leave the behavior undefined, so the structure breaks the moment content varies or the viewport shrinks.

Use this skill before assigning regions to a page, deciding persistence and stickiness, allocating viewport budget, or defining how a shell composes across page types. The goal is to prevent the agent from designing a structure around one ideal page, from leaving region behavior undefined, or from letting persistent regions consume the viewport and bury the content the user came for.

## Core Rules

### Allocate Viewport Budget Before Placing Regions

Viewport space is a finite budget, and persistent regions spend it continuously. A header, a sidebar, and a bottom action bar can together consume most of a small screen before any content appears. Before placing regions, account for the budget they claim at every supported viewport.

Calculate and constrain:

- the combined height of sticky headers, sub-headers, and banners on mobile;
- the combined footprint of sidebars and contextual panels at each breakpoint;
- the space persistent action bars and toolbars reserve at the bottom;
- the minimum remaining space for primary content to be usable.

If persistent regions leave less than a comfortable amount for content on the smallest supported device, the structure is wrong. Cut, collapse, or transform regions before sacrificing content.

### Decide Persistence Per Region Based On Need

Not every region needs to be persistent. Persistence costs viewport and attention, and it should be earned by how often the user needs the region while working in the content. Decide stickiness by function, not by default.

Guide persistence choices:

- keep global navigation and identity persistent only when users move between top-level areas often;
- make contextual toolbars sticky when the user acts on content while scrolling;
- let long content scroll freely rather than trapping it under a fixed header that steals height;
- collapse secondary navigation into a drawer or menu when its persistence is not justified.

Defaulting everything to sticky is the most common structural mistake. Each sticky region must justify the viewport it permanently claims.

### Give The Primary Object And Primary Action Predictable Places

Most pages exist to show a primary object (a record, document, person, order) and to offer a primary action. If the structure does not reserve predictable places for these, every page improvises and users hunt. The structure should make the important thing findable by position.

Reserve structural slots for:

- the primary object's identity, title, and key state, visible without scrolling;
- the primary action, reachable in a consistent region across pages of the same type;
- a clear separation between primary, secondary, and destructive actions;
- status and contextual messaging tied to the primary object.

A structure that buries the primary object below a tall header or scatters actions across regions forces users to relearn each page.

### Define Region Behavior Under Overflow And Change

Regions do not hold still. Content overflows, sidebars collapse, regions are absent on some pages, and the virtual keyboard appears. Structure must define what happens in each case, or implementations diverge.

For each region define:

- how it behaves when its content overflows (scroll internally, expand, truncate);
- what happens when the region is absent (a page with no sidebar, no footer actions);
- how it interacts with the virtual keyboard, modals, and toasts;
- its stacking and z-index relationship to overlays and other regions.

Undefined overflow behavior produces sidebars that push content, footers that overlap totals, and modals that sit behind sticky headers. Define the interactions once.

### Separate Global, Contextual, And Content Regions Explicitly

Regions serve different lifetimes and audiences, and conflating them creates structures that are hard to maintain. Global regions (app-wide navigation, identity) belong to the shell. Contextual regions (filters, toolbars, detail panels) belong to the page or view. Content regions belong to the specific object. Mixing them couples unrelated concerns.

Structure the hierarchy:

- global regions live in the shell and persist across areas;
- contextual regions live in the page and change with the view;
- content regions live in the object detail and change with the record;
- keep their lifetimes and ownership separate so changes do not ripple unexpectedly.

When global navigation and contextual toolbars share a region, every page change risks disturbing app-wide wayfinding.

### Compose Regions Across Page Types, Not Per Page

A structure designed for one page type fails the others. Lists, detail pages, dashboards, forms, and empty states have different region needs, and the structure must compose across them rather than be redrawn each time.

Define the structure as a set of composable rules:

- which regions are mandatory on every page, which are optional, and which are page-specific;
- how the main region adapts to list, detail, dashboard, form, and empty content;
- how navigation transforms across page types (sidebar on detail, tabs on dashboard);
- how empty, loading, and error states fill the regions without collapsing the layout.

A structure that cannot describe its page types is a structure that will be re-implemented differently on each page.

### Keep Navigation Reachable Without Dominating

Navigation is essential, but it is not the user's goal; the content is. Structure must keep navigation reachable without letting it dominate the viewport. The line between reachable and dominant shifts with viewport size and task.

Balance navigation by:

- keeping primary navigation reachable in one action at every viewport;
- collapsing deep or secondary navigation rather than displaying all of it;
- ensuring navigation never consumes so much width or height that content is cramped;
- providing a current-location indicator so users know where they are without expanding navigation.

Navigation that is always fully expanded on mobile leaves no room for content; navigation that is too deeply hidden makes wayfinding painful.

### Plan For State In The Structure

Pages are not always full and healthy. They are empty during onboarding, loading during fetch, partial when some regions fail, and errored when things break. The structure must hold these states without collapsing, or the page looks broken exactly when reassurance matters most.

Ensure the structure supports:

- empty states that guide the user to the next action;
- loading states that hold the layout to avoid shift;
- error states that explain and offer recovery;
- partial states where some regions loaded and others did not.

## Common Traps

### Defaulting Every Region To Sticky

Sticky headers, sidebars, and action bars that are sticky by default consume the viewport and bury content, especially on mobile.

### Designing Structure For One Ideal Page

A structure built around one representative page breaks on list, detail, dashboard, empty, and error pages that inherit it.

### Undefined Overflow And Absent-Region Behavior

Regions without defined overflow, absence, keyboard, and stacking behavior are implemented differently on every page.

### Burying The Primary Object Or Action

Structures without reserved places for the primary object and action force users to hunt on every page.

### Mixing Global, Contextual, And Content Regions

Coupling app-wide navigation with page-level toolbars in one region makes every page change risk disturbing global wayfinding.

### Navigation That Dominates The Viewport

Fully expanded navigation on small screens leaves too little room for the content the user actually came for.

### Ignoring The Virtual Keyboard And Overlays

Structures not tested against the virtual keyboard, modals, and toasts produce overlap, trapped focus, and hidden confirmations.

### Structures That Cannot Describe Their Page Types

When the structure has no composable rules for list, detail, form, dashboard, and empty pages, each page is structured ad hoc.

## Self-Check

- [ ] Viewport budget was calculated at every breakpoint, and persistent regions leave comfortable space for primary content on the smallest device.
- [ ] Each region's persistence is justified by how often users need it, not sticky by default.
- [ ] The structure reserves predictable, visible places for the primary object's identity, key state, and primary action across page types.
- [ ] Each region has defined overflow, absent-region, virtual-keyboard, modal, and stacking behavior.
- [ ] Global, contextual, and content regions are separated by lifetime and ownership so changes do not ripple unexpectedly.
- [ ] The structure is defined as composable rules across list, detail, dashboard, form, empty, loading, and error page types.
- [ ] Navigation is reachable in one action at every viewport without dominating the screen, and current location is indicated.
- [ ] Empty, loading, error, and partial states are accommodated in the structure without layout collapse.
- [ ] The structure was reviewed at every supported viewport size and with realistic, long, and edge-case content, not only the ideal page.
