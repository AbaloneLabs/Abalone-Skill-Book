---
name: pattern_catalog_design.md
description: Use when the agent is structuring a pattern library or pattern catalog, deciding which patterns to include, organizing patterns by user intent rather than layout, writing pattern documentation, or curating a design system's reusable solutions across forms, navigation, data display, and flows.
---

# Pattern Catalog Design

A pattern catalog is not a gallery of layouts. It is a curated set of reusable solutions to recurring user problems, organized so that teams can find the right pattern for a real need, understand when to use it and when not to, and apply it consistently. Most pattern catalogs fail not because the patterns are bad but because they are organized by appearance (a "card grid" section, a "sidebar" section) rather than by the problem they solve, documented without usage boundaries, or stuffed with every variation anyone ever requested until the catalog becomes unsearchable noise. Curation and structure are what make a catalog usable; their absence is what makes one rot.

Use this skill before creating or restructuring a pattern catalog, before adding a new pattern, and before deciding how patterns are documented and discovered. The goal is to prevent the agent from building a pattern dump organized by visual similarity, from documenting patterns without usage boundaries, or from growing the catalog without curation until it stops being useful.

## Core Rules

### Organize Patterns By User Intent, Not By Visual Layout

The organizing principle of a catalog determines whether teams find the right pattern or the closest-looking one. Organizing by layout (cards, tables, sidebars) leads teams to pick a pattern because it resembles what they want, regardless of whether it fits the task. Organizing by intent leads them to the pattern that actually solves their problem.

Intent-based categories look like:

- entering and editing data (forms, inline edit, bulk edit);
- navigating and wayfinding (primary nav, breadcrumbs, pagination);
- finding and filtering (search, facets, saved filters);
- comparing and choosing (lists, tables, comparison views);
- reviewing status and activity (dashboards, timelines, activity feeds);
- completing a multi-step task (wizards, steppers, progress flows);
- communicating state and feedback (toasts, banners, empty states).

A team designing a "how do users filter a large product list" surface should land in the finding-and-filtering category, not have to guess whether the answer is under "cards" or "tables".

### Define The Problem A Pattern Solves, Not Just How It Looks

A pattern entry that shows the layout and lists its parts is half-documented. The other half, the part that prevents misuse, is the problem the pattern solves and the conditions under which it is the right choice.

Each pattern entry should answer:

- what user problem does this solve;
- when is this the right pattern;
- when is it the wrong pattern, and what to use instead;
- what variations exist and when each applies;
- what constraints or dependencies it carries (data shape, content length, frequency of use);
- examples of correct and incorrect application.

Without the "when not to use" guidance, teams adopt patterns for tasks they were not designed for, and the catalog gets blamed for the resulting failures.

### Curate Ruthlessly; Do Not Collect Everything

A catalog's value is proportional to the signal-to-noise ratio of its contents, not to the count of patterns. Every pattern added increases search cost, maintenance burden, and the chance of overlapping, competing patterns. Curation means choosing what belongs and refusing what does not.

Adopt a pattern when:

- it solves a problem that recurs across multiple teams or surfaces;
- it has a clear, documented intent and usage boundary;
- it is stable enough to standardize, not a one-off or experimental solution;
- maintaining it is sustainable alongside the rest of the catalog.

Decline or retire a pattern when:

- it serves a single team or one-off use case;
- it overlaps heavily with an existing pattern without distinct intent;
- it is rarely used, poorly documented, or drifting from its definition;
- it has been superseded by a better solution.

A catalog of thirty well-chosen, well-documented patterns outperforms one of two hundred vaguely defined ones.

### Document Patterns For Decisions, Not Just For Assembly

Pattern documentation often reads like assembly instructions: here are the parts, here is the layout. Useful documentation reads like decision support: here is when to choose this, here are the tradeoffs, here is what goes wrong if you misuse it.

Strong documentation includes:

- the problem statement and the intent;
- the anatomy and the role of each part;
- the variants and when each is appropriate;
- the content and data assumptions (length, volume, frequency);
- interaction and state behavior, including edge cases;
- accessibility and responsive considerations;
- anti-patterns and common misuses to avoid.

Documentation that omits tradeoffs and anti-patterns leaves teams to discover the limits by failing.

### Make Patterns Discoverable Through Multiple Paths

Teams arrive at a pattern from different starting points: a problem they need to solve, a layout they are considering, a component they want to compose, or a flow they are designing. The catalog should support discovery from each.

Provide multiple discovery paths:

- by intent or problem category (primary organization);
- by the components a pattern composes (cross-reference);
- by the flows or page types it serves;
- by search over names, intents, and synonyms;
- by related and alternative patterns, so a team that finds the wrong one can navigate to the right one.

A catalog discoverable only by browsing a flat list forces teams to know the pattern's name before they can find it, which defeats the purpose.

### Show Patterns In Realistic Context, Not Just Isolation

A pattern shown in isolation, on a clean canvas with placeholder content, hides the conditions under which it breaks. Realistic context reveals the content lengths, data volumes, states, and responsive behavior that determine whether the pattern fits.

Show each pattern with:

- realistic content, including long, localized, and edge-case values;
- the states it can be in (empty, loading, error, partial);
- its behavior at different viewport sizes;
- its place in a realistic page, not only on a blank canvas;
- examples of correct and incorrect nearby patterns for comparison.

Isolated examples are the leading cause of patterns that look right in the catalog and fail in production.

### Keep Patterns And Components Distinct But Linked

Patterns and components are different artifacts and conflating them confuses both. Components are the building blocks (buttons, inputs, cards); patterns are the composed solutions to user problems (a search experience, a data table with filtering, a multi-step form). The catalog should keep the distinction clear while linking them.

Link them so that:

- each pattern references the components it composes;
- each component references the patterns it appears in;
- the boundary between "configurable component" and "composed pattern" is documented;
- teams know whether to extend a component, use a pattern, or propose a new pattern.

### Plan For Maintenance From The Start

A catalog that is not maintained decays. Patterns drift from their documented definitions, components change underneath them, and examples go stale. Maintenance must be planned, not assumed.

Maintenance practices:

- assign ownership for each pattern or category;
- schedule periodic reviews for usage, accuracy, and relevance;
- track adoption and retire patterns that are unused or superseded;
- keep examples in sync with the live components and tokens;
- version pattern definitions when they change meaningfully.

## Common Traps

### Organizing By Visual Layout

Grouping patterns by appearance (cards, sidebars, grids) leads teams to choose by resemblance rather than fit, producing solutions that look right and work wrong.

### Documenting Without Usage Boundaries

Pattern entries that omit "when not to use" and anti-patterns invite misuse and leave teams to discover limits through failure.

### Collecting Instead Of Curating

Accepting every requested pattern bloats the catalog with overlapping, one-off, or experimental entries that bury the useful ones.

### Isolated Examples With Placeholder Content

Patterns shown only on blank canvases with short placeholder strings hide the content lengths, states, and responsive behavior that determine real fit.

### Single-Path Discovery

A catalog reachable only by browsing a flat list forces teams to know a pattern's name in advance and misses those who arrive from a problem or component.

### Conflating Patterns And Components

Treating composed solutions and building blocks as the same thing blurs the boundary and leaves teams unsure whether to extend a component or use a pattern.

### No Maintenance Plan

A catalog without ownership, review cadence, and retirement process drifts until its examples and definitions no longer match reality.

### Documenting Assembly Instead Of Decisions

Documentation that lists parts and layout without tradeoffs, variants, and anti-patterns fails to support the decisions teams actually need to make.

## Self-Check

- [ ] Patterns are organized by user intent and problem category, not by visual layout or component similarity.
- [ ] Each pattern entry defines the problem it solves, when it is the right choice, when it is wrong, and what to use instead.
- [ ] The catalog is curated: patterns are adopted only when they solve recurring problems with clear intent, and retired when unused or superseded.
- [ ] Documentation supports decisions, covering anatomy, variants, content and data assumptions, states, accessibility, responsive behavior, and anti-patterns.
- [ ] Patterns are discoverable through multiple paths: intent category, composed components, served flows, search, and related alternatives.
- [ ] Each pattern is shown in realistic context with long, localized, and edge-case content, across states and viewport sizes, not only in isolation.
- [ ] Patterns and components are kept distinct but cross-referenced, with the boundary between them documented.
- [ ] Ownership, review cadence, adoption tracking, example syncing, and versioning are planned so the catalog does not decay.
- [ ] The catalog was reviewed for signal-to-noise: every pattern earns its place through recurrence, clarity, and sustainability.
