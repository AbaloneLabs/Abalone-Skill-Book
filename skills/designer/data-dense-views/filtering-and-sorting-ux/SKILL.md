---
name: filtering_and_sorting_ux.md
description: Use when the agent is designing how users filter, sort, search within, and narrow large data sets in tables, lists, and grids, managing filter state, applied-filter visibility, sort composition, mobile filter patterns, and recovery from over-constraining or zero-result states without losing context or work.
---

# Filtering And Sorting UX

Filtering and sorting are how users turn a large data set into the few items they care about. Designing them is not adding a filter sidebar; it is deciding how users express what they want, how they see what is currently constraining the results, how sort and filter compose, and what happens when they narrow too far and reach nothing. Agents tend to add filters that mirror the data fields, to let active filters become invisible state, and to leave users stranded at zero results with no explanation. The harm is users who cannot explain why items are missing, who rebuild the same query repeatedly because state was lost, and who abandon a view because they over-filtered and cannot find their way back.

Use this skill before designing filter and sort controls, applied-filter display, filter state management, or recovery from over-constraining. The goal is to prevent the agent from building filters from database structure, from hiding active constraints, or from letting users reach unrecoverable zero-result states.

## Core Rules

### Choose Filter Dimensions By How Users Actually Decide

Not every data attribute is a useful filter. A good filter reflects a decision users genuinely make when narrowing: status, date, category, owner, type, amount. A bad filter mirrors an internal field no one thinks in. Filter selection should come from studying how users choose, not from the schema.

Select filters by asking:

- what do users narrow on first to find what they need?
- which attributes eliminate the most irrelevant items?
- which are non-negotiable versus nice-to-have?
- which are so rarely used they belong in advanced, not primary, filters?

Too many filters dilute the interface and hide the useful ones; too few force users to scroll instead of filter. Curate by user decision, not by data completeness.

### Make Active Filters Always Visible And Removable

The single most common filtering failure is hidden state. Users apply filters, forget, and then cannot explain why results look wrong. Active filters must be visible as a summary or chips near the results, individually removable, and reflected in the result count. A filter that silently shapes results is a defect.

Keep active filters visible by:

- showing applied filters as removable chips or a clear summary near the results;
- letting users remove a single filter without clearing all;
- reflecting all active constraints in the result count;
- never letting a filter shape results invisibly.

When a user cannot see what is constraining the results, they cannot trust the results or debug a confusing view.

### Match The Control Type To The Filter's Logic

Different filters behave differently, and the control must match. Category is often single-select within a level; tags are often multiselect; date and amount are ranges; recency is often a relative preset. Using the wrong control makes filtering feel broken.

Match control to logic:

- single-select for mutually exclusive choices;
- multiselect for additive narrowing;
- range inputs for continuous values like date, amount, score;
- relative presets ("last 7 days", "this quarter") for common temporal intents.

A single-select radio where users expect to pick several, or free text where a range is needed, turns a useful filter into a frustrating one.

### Prevent And Recover From Zero-Result States

When users combine filters, they can easily reach zero results without understanding why. This is the moment filtering either helps or abandons the user. Designing against silent zero results is essential, because a dead end with no explanation forces a full reset.

Design against zero results by:

- updating result counts live as filters are selected, before applying;
- disabling or de-emphasizing values that would return nothing;
- when zero results occur, identifying which filter combination caused it;
- offering to relax the most restrictive filter rather than presenting a dead end.

A zero-result state caused by filters should always be explainable and recoverable in one action.

### Make Sort Compose Predictably With Filters

Filters and sort interact, and users expect them to combine sensibly: a filtered set, sorted by the chosen column. When sort applies to the unfiltered set, or when changing the filter resets the sort, the results feel random. The composition must be predictable and communicated.

Ensure predictable composition by:

- applying sort to the currently filtered set, not the full set;
- preserving useful sort when filters change, or resetting it deliberately with notice;
- reflecting the combined filter-and-sort state in the interface;
- making the current sort visible and changeable from the results.

### Preserve Filter And Sort State Across Navigation

Users paginate, open a detail, change sort, and return. Losing filter and sort state on any of these movements forces them to rebuild the query, which is a major source of abandonment. State should persist across the movements users naturally make.

Preserve state by:

- maintaining filters and sort across pagination and detail navigation;
- restoring them when returning to the list via back navigation;
- offering saved filter sets for recurring workflows;
- avoiding silent resets on navigation or view switches.

A user who carefully filtered a list, opens a row, and returns to find the filter gone has lost real work.

### Provide Clear Paths To Reset And Broaden

Users overshoot when filtering. They narrow too far, reach too few results, and need to broaden. Without an easy path to reset or relax, they are stuck clearing everything and starting over. Reset and broaden paths must be obvious and granular.

Provide recovery paths by:

- offering a clear reset or clear-all action;
- allowing removal of a single filter without losing others;
- suggesting how to broaden when results are too narrow;
- giving a way back to the unfiltered set without restarting.

### Design Mobile Filtering Deliberately

Filters that work in a desktop sidebar fail on mobile, where space is constrained and a tiny filter column is unusable. Mobile needs a dedicated pattern: a filter trigger showing the active count, a full-screen or bottom-sheet panel, live counts, and clear apply and reset actions.

Design mobile filtering by:

- using a filter trigger that shows the active filter count;
- opening a full-screen or bottom-sheet panel for filter entry;
- updating result counts live within the panel;
- showing applied filters prominently after the panel closes.

Compressing a desktop sidebar into a narrow mobile column produces filters no one can use.

## Common Traps

### Filters Based On Database Structure

Offering filters that mirror internal data fields rather than user decisions adds options no one uses and hides the useful ones.

### Hidden Active Filters

Filters that shape results invisibly produce confusion when users cannot explain why expected items are missing.

### Wrong Control Type For The Logic

A single-select where multiselect is expected, or free text where a range is needed, makes filtering feel broken.

### Silent Zero Results From Combined Filters

Letting users reach an empty set without identifying the cause forces a full reset and abandons the task.

### Sort That Does Not Compose With Filters

Sort applied to the unfiltered set, or reset unpredictably when filters change, makes the order feel random.

### Lost State On Navigation

Resetting filters and sort on pagination, detail navigation, or back-navigation forces users to rebuild queries repeatedly.

### No Path To Broaden

Over-filtered results with no easy way to relax constraints or reset partially abandon users at a dead end.

### Desktop Sidebar Compressed Onto Mobile

Tiny filter columns on mobile are unusable; mobile needs a dedicated panel pattern with live counts.

## Self-Check

- [ ] Filters are selected based on how users actually decide and narrow, not on database structure or completeness.
- [ ] Active filters are always visible as removable chips or a summary, reflected in the result count, and individually removable.
- [ ] Each filter uses the correct control type: single-select, multiselect, range, or relative preset, matching its logic.
- [ ] Zero-result states identify the restrictive combination and offer to relax the most restrictive filter rather than presenting a dead end.
- [ ] Sort composes predictably with filters, applies to the filtered set, and the combined state is visible and changeable.
- [ ] Filter and sort state persists across pagination, detail navigation, and back-navigation, with saved sets for recurring workflows.
- [ ] Clear reset, single-filter removal, and broaden paths exist to recover from over-filtering without a full restart.
- [ ] Mobile filtering uses a dedicated panel with a count-aware trigger, live counts, and clear apply and reset actions, not a compressed sidebar.
