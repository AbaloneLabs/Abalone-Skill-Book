---
name: filtering_and_faceted_search.md
description: Use when the agent is designing filters, faceted search, filter UI, faceted navigation, filter state management, mobile filter patterns, applied filter display, or helping users narrow large result sets through multiple attributes without over-constraining or losing context.
---

# Filtering And Faceted Search

Filters are how users turn a large result set into the few items they actually want. Done well, filtering feels like a conversation that narrows toward the goal. Done poorly, it produces empty results, hidden constraints, contradictory state, and users who give up and scroll instead. The design problem is giving users enough control to narrow precisely without overwhelming them with options, and never letting them reach a zero-results state they cannot explain.

Use this skill before designing filter panels, faceted navigation, mobile filter experiences, filter state, applied-filter display, or the interaction between search and filtering. The goal is to prevent the agent from building filters that over-constrain silently, lose state, hide active constraints, or present so many facets that users cannot find the ones that matter.

## Core Rules

### Choose Facets Based On How Users Actually Decide

Not every attribute is a useful facet. A good facet reflects a decision users genuinely make when narrowing: price, category, brand, date, location, availability, format. A bad facet reflects database structure no one thinks in. Select facets by studying how users choose:

- what do they compare on first?
- what eliminates the most irrelevant results?
- what is non-negotiable versus nice-to-have?

Too many facets dilute the interface; too few force users to scroll instead of filter.

### Show Facet Values With Counts And Relevance

Facet values should communicate how much they will narrow the results. Show counts where useful so users understand the effect of selecting a value. Hide or de-emphasize facets and values that would return zero results given the current state. A facet that offers options leading nowhere wastes attention and causes frustration.

Balance information density: counts help decision-making, but cluttered facet panels overwhelm. Show counts where the magnitude matters, and omit them where they add noise.

### Make Active Filters Always Visible

The most common filtering failure is hidden state. Users apply filters, forget, and then cannot explain why results look wrong. Active filters must be:

- visible as applied chips or a summary near the results;
- individually removable without clearing all filters;
- clearly labeled so the user knows what is constraining the results;
- reflected in the result count so changes are explainable.

Never let a filter silently shape results without the user being able to see and undo it.

### Prevent Silent Over-Constraining And Zero Results

When users combine multiple facets, they can easily reach zero results without understanding why. Design against this:

- update result counts live as filters are selected;
- disable or visually de-emphasize values that would return nothing;
- when zero results occur, identify which filter combination caused it;
- offer to relax the most restrictive filter rather than presenting a dead end.

A zero-results state caused by filters should always be recoverable and explainable.

### Decide Multiselect Versus Single-Select Per Facet

Different facets behave differently. Category is often single-select within a level; brand is often multiselect; price is a range; date is often a range or relative option. Match the interaction to the facet's logic:

- single-select for mutually exclusive choices;
- multiselect for additive narrowing;
- range inputs for continuous values like price or date;
- relative presets like "last 7 days" for common temporal intents.

Using the wrong control makes filtering feel broken.

### Preserve Filter State Across Navigation And Sessions

Users navigate in and out of results, change sort, paginate, and return. Filters should persist across these movements:

- maintain filters when paginating or changing sort;
- restore filters when returning to search;
- offer saved filter sets for recurring workflows;
- avoid silently resetting filters on navigation.

Lost filter state forces users to rebuild complex queries, which is a major source of abandonment.

### Design Mobile Filtering Deliberately

Filters that work in a sidebar fail on mobile, where space is constrained. Common strong patterns:

- a filter trigger showing the active count;
- a full-screen or bottom-sheet filter panel;
- live result count updates within the panel;
- clear apply and reset actions;
- prominent display of active filters after closing the panel.

Compressing a desktop sidebar into a tiny mobile column produces unusable filters.

### Coordinate Filters With Sort And Search

Filters, sort, and query interact. A user searching "shoes" with a "running" filter and a "price low to high" sort expects all three to combine. Ensure:

- filters and sort compose predictably;
- changing the query preserves useful filters or resets them deliberately;
- the result count reflects all active constraints;
- the interface communicates the combined state clearly.

Inconsistent composition makes the results feel random.

### Provide Clear Paths To Reset And Broaden

Users overshoot when filtering. Provide:

- a clear reset or clear-all action;
- the ability to remove a single filter without losing others;
- suggestions to broaden when results are too narrow;
- a way back to the unfiltered set without restarting.

Punishing over-filtering by forcing a full reset wastes the user's query work.

### Order Facets By Relevance To The Current Query

Static facet order ignores context. Surface the facets most relevant to the current results or query higher. For a search within electronics, specs matter; within apparel, size and color matter. Adaptive facet ordering reduces scrolling to find the facets that matter now.

## Common Traps

### Hidden Active Filters

Filters that shape results invisibly produce confusion when users cannot explain why items are missing.

### Silent Zero Results From Combined Facets

Letting users reach an empty set without identifying the cause forces them to clear everything and start over.

### Facets Based On Database Structure

Offering facets that mirror internal data fields rather than user decisions adds options no one uses.

### Wrong Control Type For The Facet Logic

A single-select radio where users expect multiselect, or free text where a range is needed, makes filtering feel broken.

### Lost Filter State On Navigation

Resetting filters on pagination, sort change, or back-navigation forces users to rebuild queries repeatedly.

### Desktop Sidebar Compressed Onto Mobile

Tiny filter columns on mobile are unusable; mobile needs a dedicated panel pattern.

### Inconsistent Filter, Sort, And Query Composition

When filters and sort combine unpredictably, results feel random and users lose trust.

### No Path To Broaden

Over-filtered results with no easy way to relax constraints abandon users at a dead end.

## Self-Check

- [ ] Facets are selected based on how users actually decide, not on database structure.
- [ ] Facet values show counts or relevance where useful and de-emphasize options that lead to zero results.
- [ ] Active filters are always visible, individually removable, and reflected in the result count.
- [ ] Zero-results states caused by filtering identify the restrictive combination and offer to relax it.
- [ ] Each facet uses the correct control type: single-select, multiselect, range, or relative preset.
- [ ] Filter state persists across pagination, sort changes, navigation, and return, with saved sets for recurring workflows.
- [ ] Mobile filtering uses a dedicated panel with live counts and clear apply and reset actions.
- [ ] Filters, sort, and query compose predictably and the combined state is communicated clearly.
- [ ] Clear reset, single-filter removal, and broaden paths exist to recover from over-filtering.
- [ ] Facet ordering adapts to the current query and results rather than being statically fixed.
