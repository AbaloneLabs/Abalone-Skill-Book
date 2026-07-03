---
name: responsive_composition.md
description: Use when the agent is designing responsive layouts, arranging text and images, adapting desktop screens to mobile, designing grids, preventing overflow, or ensuring visual composition works across viewport sizes.
---

# Responsive Composition

Responsive design is not shrinking a desktop screen until it fits. It is deciding which relationships must stay visible, which groups can stack, which controls need priority, and how content should remain readable and usable across viewport sizes, input methods, and content lengths.

Use this skill before designing or reviewing layouts with headers, sidebars, cards, forms, tables, dashboards, image/text combinations, product pages, mobile breakpoints, or any UI where content may overflow, collapse, wrap, or change priority. The goal is to prevent the agent from producing a layout that looks acceptable in one viewport but breaks under realistic conditions.

## Core Rules

### Define The Core Relationship

Before choosing a grid, ask what relationship the layout must preserve.

Examples:

- label and value;
- image and caption;
- item and action;
- chart and explanation;
- filter and result;
- navigation and current page;
- comparison between rows;
- primary object and supporting details.

Responsive behavior should protect these relationships. If stacking separates an action far from its item, or if a sidebar becomes a hidden drawer that blocks the main workflow, the adaptation may be wrong.

### Design Mobile As A Real Layout, Not A Collapse

Mobile users should not receive a leftover version of the desktop design. Decide mobile order, navigation, density, tap targets, media treatment, and action placement intentionally.

For mobile:

- put the primary object and state early;
- avoid horizontal overflow;
- keep primary actions reachable;
- reduce simultaneous columns;
- preserve labels when values stack;
- make filters and menus discoverable;
- avoid text over images unless contrast and space are reliable;
- ensure sticky elements do not cover content.

Do not assume every desktop section can simply stack in source order.

### Use Stable Constraints

Fixed-format elements need stable dimensions. Boards, grids, toolbars, counters, tiles, charts, image frames, media cards, and buttons should not resize unpredictably when labels, icons, hover states, loading text, or validation messages appear.

Use layout thinking such as:

- min and max widths;
- predictable column tracks;
- aspect ratios for media;
- consistent row heights where comparison matters;
- wrapping rules for labels;
- overflow handling for long text;
- reserved space for status and actions.

The design specification should make clear what happens when content is longer than expected.

### Prioritize Reading And Comparison

Some layouts are for reading, others for comparison, and others for action. Responsive choices should serve the dominant mode.

For reading:

- line length should remain comfortable;
- headings should not overwhelm body text;
- images should support comprehension;
- paragraphs should not become narrow columns.

For comparison:

- repeated items need aligned fields;
- key values should remain visible;
- sorting and filtering should be accessible;
- horizontal tables may need alternative mobile structures.

For action:

- controls should stay near affected content;
- confirmation and errors should stay visible;
- primary action placement should be predictable.

### Treat Images As Content

Images are not just decoration. Product, venue, person, portfolio, and object pages often require the actual subject to be visible and inspectable. Cropping, dark overlays, blur, or tiny thumbnails can destroy the user's ability to evaluate.

For images:

- preserve important focal points;
- choose aspect ratios that match the subject;
- avoid text overlays when content varies unpredictably;
- provide enough resolution;
- keep captions or labels close;
- ensure loading and missing-image states do not collapse layout;
- avoid using generic atmospheric media when the real subject matters.

### Prevent Text Breakage

Text is one of the most common responsive failure points. Long names, translations, numbers, URLs, button labels, and user-generated content can overflow or overlap.

Plan for:

- wrapping;
- truncation with accessible full value;
- multi-line buttons where acceptable;
- minimum container width;
- dynamic labels;
- localized strings;
- long unbroken tokens;
- content that expands after loading.

Do not scale font size with viewport width as a primary solution. Use typography and layout constraints that stay readable.

### Respect Navigation And Persistent UI

Headers, sidebars, tab bars, floating actions, cookie banners, chat widgets, and sticky filters can consume too much small-screen space. Persistent elements should support the workflow, not cover content.

Check whether sticky UI:

- overlaps headings or controls;
- reduces usable viewport too much;
- traps focus;
- hides validation or completion states;
- creates double scroll areas;
- conflicts with virtual keyboards.

## Common Traps

### Designing Only At One Width

A layout that works at 1440px and 390px may still break at tablet, narrow desktop, split-screen, zoomed browser, or large text settings. Think in ranges, not just named devices.

### Cropping Away The Subject

Image containers that look elegant with sample assets may crop the product, face, venue, chart, or document the user needs to inspect.

### Letting Cards Become Too Narrow

Cards with actions, labels, status, and metadata can become unreadable when squeezed into many columns. Reduce columns before content collapses.

### Overusing Hidden Drawers

Hiding filters, navigation, or actions can be appropriate, but it can also make repeated workflows slow. High-frequency controls may need visible compact treatment instead.

### Ignoring Real Data Length

Placeholder text is usually short. Real names, translations, addresses, legal labels, and financial values are longer. Design with worst plausible content.

### Sticky Elements That Cover End States

Bottom bars and sticky footers often cover final form fields, totals, errors, or submit confirmations on mobile.

## Self-Check

- [ ] The layout protects the core relationship between content, action, state, image, or comparison target.
- [ ] Mobile order, action placement, navigation, and density are intentionally designed rather than inherited from desktop source order.
- [ ] Fixed-format elements have stable constraints for dimensions, aspect ratio, wrapping, hover, loading, and dynamic states.
- [ ] The layout supports its dominant mode: reading, comparison, action, exploration, or inspection.
- [ ] Images preserve important subject matter and include loading, missing, caption, and cropping behavior.
- [ ] Long text, translations, URLs, numbers, labels, and user-generated content do not overflow or overlap.
- [ ] Tables, grids, cards, sidebars, headers, and filters have clear behavior across viewport ranges.
- [ ] Sticky or persistent UI does not cover content, trap focus, or block form completion.
- [ ] The design considers tablet, split-screen, zoom, large text, and realistic data length, not only one desktop and one phone size.
- [ ] Responsive changes preserve usability rather than merely fitting pixels.
