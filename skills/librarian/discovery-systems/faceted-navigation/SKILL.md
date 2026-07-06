---
name: faceted_navigation.md
description: Use when the agent is designing or configuring faceted navigation for a discovery system, choosing which facets to expose, ordering and grouping facets, handling facet counts and multi-select, or evaluating whether facets help or hinder user search refinement.
---

# Faceted Navigation

Faceted navigation is the mechanism that lets users narrow a large result set along multiple dimensions, format, date, subject, author, location, and more. When designed well, facets transform an overwhelming flood of results into a navigable space where users can zero in on what they need without knowing specialized search syntax. When designed poorly, facets confuse, mislead, or frustrate, showing options that do not apply, hiding relevant materials behind unexpected groupings, or breaking when multiple selections combine. Facet design is a judgment problem because it requires understanding how users think about a collection, which dimensions matter for which tasks, and how to present options without overwhelming. It is not merely turning on all available metadata fields.

Use this skill when designing or configuring faceted navigation, choosing facets, ordering and grouping them, handling counts and selection behavior, or evaluating facet effectiveness. The goal is to prevent the agent from exposing every metadata field as a facet, ordering facets arbitrarily, hiding the effect of selections, or treating facets as a technical feature rather than a usability decision.

## Core Rules

### Choose Facets That Match User Mental Models

The most useful facets correspond to how users actually think about and filter materials. Expose dimensions that match common refinement tasks.

Strong facets typically include:

- format or resource type, book, article, video, dataset;
- date or year of publication;
- subject or topic;
- author or creator;
- language;
- location or library branch;
- availability, available online, on shelf.

Weak facets are those tied to internal metadata that users do not think in, such as internal record IDs, obscure classification subdivisions, or overly granular subject subdivisions. Choose facets by user value, not by what metadata happens to exist.

### Order Facets By Usefulness, Not Alphabetically

Facet order shapes which refinements users notice and apply. Place the most-used and most-useful facets first.

Ordering guidance:

- format and date typically among the most used;
- subject and author next;
- language and location as needed;
- specialized facets last or collapsed.

Alphabetical ordering buries high-value facets among low-value ones. Use analytics on facet usage to inform ordering and revisit it.

### Show Facet Counts

Facet counts tell users how many results each refinement will yield. Without counts, users select blindly and may land on empty sets.

Count display:

- show the result count next to each facet value;
- update counts dynamically as other facets are applied;
- indicate when a facet value would yield zero results;
- consider hiding or de-emphasizing zero-count values.

Counts manage expectations and guide efficient refinement. They are essential, not decorative.

### Support Multi-Select Within And Across Facets

Users often want to combine refinements, such as multiple formats or a format plus a date range. The system should support this intuitively.

Multi-select behavior:

- allow selecting multiple values within a facet, typically as OR;
- allow combining values across facets, typically as AND;
- show applied facets clearly with the ability to remove individual selections;
- preserve other refinements when one is removed;
- make the combined effect on results visible.

Single-select-only facets force users to repeat searches. Multi-select respects how users actually refine.

### Group And Collapse Specialized Facets

Too many visible facets overwhelm. Group related facets and collapse less-used ones to keep the interface manageable.

Grouping approaches:

- group facets into categories, content, creator, subject, location;
- collapse specialized or low-use facets by default with expand controls;
- show only the top values within each facet with a more option;
- use progressive disclosure to reveal complexity as needed.

The goal is a clean default that scales to power users. Do not show all facets and all values at once.

### Handle Facet Values Intelligently

Raw facet values from metadata are often inconsistent, duplicated, or overly granular. Facet value handling affects usability.

Handling:

- normalize and cluster equivalent values, e.g., merge author name variants;
- limit the number of values shown with a more option;
- sort values by count, not alphabetically, for long lists;
- handle hierarchical subjects with expandable levels;
- filter out empty or meaningless values.

Poor value handling produces facets full of duplicates and noise. Invest in normalization and curation.

### Make Applied Facets Visible And Removable

Users must always see which refinements are active and be able to remove them individually. Losing track of applied facets causes confusion.

Visibility practices:

- show applied facets in a persistent area, often above results;
- display each active refinement as a removable chip or tag;
- allow removing one facet without clearing all;
- provide a clear reset or clear-all option;
- reflect applied facets in the URL for shareable, bookmarkable searches.

Hidden or sticky facets trap users in over-narrowed result sets. Transparency and easy removal are essential.

### Evaluate Facet Effectiveness With Analytics

Facet design should be informed by data on how users actually use facets. Analytics reveal which facets are used, which are ignored, and where users struggle.

Track and analyze:

- which facets are applied most and least;
- which facet values are most selected;
- at what point users apply facets in a session;
- whether facet use correlates with successful finds;
- zero-result outcomes after facet application.

Use findings to reorder, add, remove, or regroup facets. Facet design is iterative, not one-time.

### Consider Facet Behavior For Different Content Types

Different collections and user tasks benefit from different facets. A single facet set may not serve all contexts.

Consider:

- article-heavy discovery benefits from peer-reviewed and date facets;
- catalog discovery benefits from format and location facets;
- digital collections benefit from collection and date-range facets;
- specialized databases may need domain-specific facets.

Configure facets contextually where the system supports it. One global facet set is a compromise, not an optimum.

### Test Facets With Real Users

Analytics show what users do but not why. Usability testing reveals confusion and unmet needs that data misses.

Test:

- whether users notice and understand facets;
- whether facet labels are clear;
- whether multi-select behavior is understood;
- whether zero-result situations are handled;
- whether users can recover from over-narrowing.

Combine analytics with testing for a complete picture. Either alone is insufficient.

## Common Traps

### Exposing Every Metadata Field As A Facet

Internal metadata does not equal user-facing facets. Choose facets by user value.

### Alphabetical Facet Ordering

Alphabetical buries high-value facets. Order by usefulness and usage data.

### No Facet Counts

Blind selections lead to empty sets and frustration. Show and update counts dynamically.

### Single-Select-Only Facets

Forcing one value per facet makes users repeat searches. Support multi-select within and across facets.

### Too Many Visible Facets At Once

Overwhelming interfaces cause abandonment. Group, collapse, and use progressive disclosure.

### Raw, Unnormalized Facet Values

Duplicates and variants create noise. Normalize and cluster equivalent values.

### Hidden Or Sticky Applied Facets

Users trapped by invisible refinements get confused. Show applied facets clearly with individual removal.

### Designing Facets Without Analytics Or Testing

Guessing at facet design fails. Use usage data and usability testing to guide decisions.

## Self-Check

- Are facets chosen to match user mental models and common refinement tasks, not just available metadata?
- Are facets ordered by usefulness informed by usage analytics, not alphabetically?
- Are facet counts shown and updated dynamically as refinements are applied?
- Does the system support multi-select within and across facets with clear applied-facet display?
- Are specialized or low-use facets grouped and collapsed to keep the default interface manageable?
- Are facet values normalized, clustered, and limited to reduce duplicates and noise?
- Are applied facets visible, individually removable, and reflected in shareable URLs?
- Is facet effectiveness evaluated with analytics on usage, selection, and outcomes?
- Are facets configured contextually for different content types and user tasks where possible?
- Has facet design been tested with real users to reveal confusion and unmet needs?
