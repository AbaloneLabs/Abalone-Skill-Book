---
name: page_template_design.md
description: Use when the agent is designing page templates, page shells, app chrome, layout regions, content max-width and gutter rules, persistent navigation, sticky headers, sidebars, and reusable page structures for dashboards, detail pages, lists, forms, and marketing surfaces.
---

# Page Template Design

A page template is the skeleton that dozens or hundreds of real pages inherit. Because it is inherited so widely, every decision in the template, the placement of navigation, the width of the content column, the behavior of sticky regions, the treatment of the primary action, is multiplied across the product. A template that is slightly wrong becomes universally wrong, and a template that cannot accommodate the real range of page content forces each page into awkward local overrides that erode consistency. Template design is high-leverage work, and it is where small oversights become systemic problems.

Use this skill before designing or restructuring a page template, app shell, or layout system, and before deciding how persistent regions, content widths, and responsive behavior are defined. The goal is to prevent the agent from designing a template around one ideal page, from leaving responsive and persistent-region behavior undefined, or from building a skeleton that real content cannot fill without breaking.

## Core Rules

### Design The Template Around The Range Of Pages, Not The Ideal Page

A template built to fit one representative page will fail the pages that differ: the long-form article, the dense data table, the empty onboarding state, the wide comparison view, the narrow form. Before fixing the template, enumerate the real pages that will inherit it and stress-test the skeleton against the extremes.

Consider the range across:

- content volume (sparse onboarding versus dense dashboard);
- content width (narrow text versus wide tables and media);
- primary action presence (pages with a main action versus read-only pages);
- navigation needs (pages that need context versus standalone pages);
- state variation (empty, loading, error, partial);
- localization and text length.

A template that survives the extremes will serve the typical pages comfortably; one optimized only for the typical page will break at the edges.

### Define Stable Layout Regions And Their Behavior

A template is a set of regions, header, sidebar, main content, footer, persistent action bars, and each region needs defined dimensions and behavior, especially under change. Ambiguity here produces inconsistent pages.

For each region define:

- whether it is fixed, sticky, or scrolling with content;
- its width or height constraints (min, max, preferred);
- its behavior when content overflows;
- its visibility and behavior across viewport sizes;
- its stacking and z-index relationship to other regions;
- what happens when it is absent (a page without a sidebar, without a header action).

Regions that are not defined become regions that are implemented differently on every page.

### Set Content Width And Reading Measure Deliberately

Content width is one of the most consequential and most casually handled template decisions. Lines that are too long exhaust readers; content columns that are too narrow waste space on data-dense pages; max-widths that are too tight force tables and media into cramped containers.

Set width by content type:

- reading text: a comfortable measure, roughly 60 to 80 characters, to aid comprehension;
- data and comparison: wider containers that let tables and grids breathe;
- media and imagery: aspect-ratio-driven containers that preserve the subject;
- forms: a width that keeps labels and inputs aligned and scannable.

Use a content max-width that serves the dominant content type, and allow specific regions to break wider when the content demands it. A single global max-width applied to everything is usually wrong for at least some pages.

### Make Navigation And Wayfinding Predictable

Persistent navigation is the user's map. Its placement, behavior, and indication of current location must be predictable across every page that inherits the template, or users lose their sense of where they are.

Design navigation so that:

- primary navigation is in a consistent location across pages;
- the current location is always indicated, by more than color where possible;
- navigation remains reachable and usable at every supported viewport size;
- deep or contextual navigation has a defined place rather than appearing ad hoc;
- navigation does not consume so much screen that content becomes cramped, especially on mobile.

### Handle Sticky And Persistent Regions Carefully

Sticky headers, persistent action bars, and fixed sidebars are useful but dangerous. They consume viewport space continuously, can cover content, and interact badly with virtual keyboards, modals, and each other.

For sticky and persistent regions:

- calculate their combined footprint on small screens, where they can eat most of the viewport;
- ensure they do not cover the ends of long content, final form fields, or confirmation messages;
- define their stacking order so modals, banners, and toasts layer correctly;
- ensure they do not trap focus or block scroll;
- decide whether they persist over modals and dialogs or yield to them.

The most common failure is a bottom action bar that covers the submit button or success message on mobile.

### Define Responsive Behavior As A First-Class Concern

A template's responsive behavior is not a downstream task; it is part of the template definition. Each region must declare how it adapts across viewport ranges, and the adaptations must preserve the page's core relationships.

Define for each region:

- at which breakpoints it appears, collapses, or moves;
- how navigation transforms (sidebar to drawer, tabs to menu);
- how the content column reflows;
- how persistent regions behave on mobile;
- how the primary action stays reachable;
- what happens to tables, media, and forms at narrow widths.

A template whose responsive behavior is left to each page produces a different mobile experience on every page.

### Preserve The Primary Object And Primary Action

Most pages have a primary object (the record, product, person, document) and a primary action (the next thing the user should do). The template must give these predictable, visible places, or pages bury what matters most.

Ensure the template supports:

- a clear place for the primary object's identity and key state;
- a clear place for the primary action, reachable without hunting;
- separation between primary, secondary, and destructive actions;
- consistent placement so users learn where to look across pages.

### Plan For Empty, Loading, And Error States In The Template

Templates are often designed full of content. Real pages are sometimes empty, loading, or broken, and the template must accommodate those states without collapsing.

Ensure the template handles:

- empty states that guide the user to the next action;
- loading states that hold the layout to avoid shift;
- error states that explain and offer recovery;
- partial states where some regions loaded and others did not.

A template that only looks right when every region is full will look broken on the pages that are not.

## Common Traps

### Designing Around One Ideal Page

A template optimized for a single representative page breaks on the long, wide, empty, or dense pages that also inherit it.

### Undefined Region Behavior

Regions without defined dimensions, stickiness, overflow, and stacking behavior are implemented differently on every page, eroding consistency.

### A Single Global Content Width

Applying one max-width to all content underserves data-dense pages and overserves reading text; width should follow content type.

### Sticky Regions That Cover Content

Persistent headers and action bars that are not checked against small viewports cover final fields, totals, errors, and confirmations.

### Responsive Behavior Left To Each Page

When each page decides its own mobile adaptation, the product offers a different mobile experience on every page.

### Burying The Primary Object Or Action

Templates without a predictable place for the primary object and action force users to hunt on every page.

### Templates Designed Only For Full Content

Templates that look right only when every region is populated collapse visually on empty, loading, and error pages.

### Navigation That Consumes Too Much Viewport

Persistent navigation that is too large on mobile leaves too little room for content and makes the page feel cramped.

## Self-Check

- [ ] The template was designed and stress-tested against the real range of inheriting pages, including sparse, dense, wide, narrow, empty, and localized content.
- [ ] Each layout region has defined dimensions, stickiness, overflow, stacking, responsive, and absent-region behavior.
- [ ] Content width and reading measure are set by content type, with regions allowed to break wider when content demands.
- [ ] Navigation placement, current-location indication, reachability, and mobile behavior are predictable across all inheriting pages.
- [ ] Sticky and persistent regions are checked for combined footprint, content coverage, focus trapping, and stacking against modals and toasts.
- [ ] Responsive behavior is defined as part of the template, with declared breakpoints and adaptations for every region.
- [ ] The template provides predictable, visible places for the primary object, primary action, and separation of primary, secondary, and destructive actions.
- [ ] Empty, loading, error, and partial states are accommodated in the template without layout collapse.
- [ ] The template was reviewed at every supported viewport size and with realistic, long, and edge-case content, not only the ideal page.
