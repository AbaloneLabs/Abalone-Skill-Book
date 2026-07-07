---
name: table_and_list_design.md
description: Use when the agent is designing tables, data grids, lists, or repeated-row layouts, choosing columns, handling long content, deciding row density, supporting selection and bulk actions, or making dense tabular data scannable, comparable, and usable across data volumes and viewport sizes.
---

# Table And List Design

Tables and lists are how users compare, scan, and act on many items at once. Designing them is not drawing a grid of cells; it is deciding which columns matter, how rows stay scannable at scale, how users select and act on items, and how the layout survives ten rows and ten thousand. Agents tend to include every column because each seems important, to let content overflow or truncate unpredictably, and to treat mobile as an afterthought where the table becomes a horizontal-scroll trap. The harm is tables no one can scan, where the differentiating column is off-screen, where long text truncates into meaninglessness, and where bulk work requires clicking each row.

Use this skill before designing a table, grid, or repeated-row list, choosing columns, or defining row interactions. The goal is to prevent the agent from building a table around all available data rather than the user's comparisons, from leaving overflow and truncation undefined, or from making selection and bulk actions an afterthought.

## Core Rules

### Choose Columns By What Users Compare And Decide On

The columns in a table should be the ones users actually use to compare and choose, not every field the data has. Too many columns push the differentiators off-screen and force horizontal scrolling; too few force users to open each row to find what they need. Column selection is a decision about the user's task.

Select columns by asking:

- which fields do users scan to identify the right row?
- which fields do users compare across rows to decide?
- which fields trigger the action they take on a row?
- which fields are nice-to-have and can move into a detail view?

A table with twenty columns serves no one; a table with the four fields users actually compare serves everyone. Default to fewer, and let detail live in the row expansion or detail page.

### Order Columns To Support The Scan Path

Column order shapes how fast users find and compare. The identifying columns should lead, the comparing columns follow, and the action column sits where the user expects to act. Order should reflect the user's scan path, not the data's internal order.

Order for scanning:

- lead with identifiers (name, ID, status) so users locate the row;
- follow with the key comparison fields;
- place numeric comparison fields adjacent so the eye scans values together;
- put row actions consistently at the start or end, not floating.

When the action column sits in the middle, users hunt for it on every row; when comparison numbers are separated by text columns, the eye cannot scan them together.

### Handle Long And Variable Content Predictably

Real data has long names, multiline values, very large numbers, and empty cells. A table that assumes uniform short content breaks on the first real dataset. Long-content handling must be defined so the table stays scannable and no information is silently lost.

Define content handling:

- decide truncation with tooltips versus wrapping per column type;
- align numbers right for comparison, text left for reading;
- handle empty values explicitly (dash, "none") rather than blank cells;
- reserve width for columns that need it and let others flex.

Undefined truncation produces a column of "Acme Cor..." entries that the user cannot distinguish; aggressive wrapping turns one row into five and destroys scanability.

### Choose Row Density By Task And Device

Row density is a tradeoff between how many rows fit and how easily each is read. Dense rows fit more for experts who scan many items; spacious rows are easier to read and tap. Density should match the task and the device, and often should be user-controllable.

Set density deliberately:

- compact density for expert scanning of many rows on desktop;
- comfortable density for general use and mobile tap targets;
- offer a density toggle where users have strong preferences;
- ensure touch targets stay usable at every density.

A table locked to dense rows on mobile is unusable; a table locked to spacious rows forces an expert to scroll endlessly.

### Make Selection And Bulk Actions First-Class

When users act on many rows, per-row actions are punishing. Selection with bulk actions turns a hundred clicks into a few. Selection and bulk actions must be designed as core interactions, not added later, and they must work clearly across selection, filtering, and pagination.

Design selection and bulk actions by:

- providing clear row selection (checkboxes) with a select-all that respects the current filter;
- showing a bulk action bar when items are selected, with the count;
- making clear whether select-all selects the page or all matching results;
- preserving selection across filter and pagination changes where appropriate.

Ambiguity about what "select all" covers is a classic and serious table failure, especially before destructive bulk actions.

### Support Sorting On The Columns Users Decide By

Users reorganize tables to find what they need, and sorting is how they do it. Sortable columns should be the ones users actually use to prioritize: date, status, amount, name. Sorting must be clearly indicated and must compose predictably with filters.

Design sorting by:

- making sortable columns visibly interactive with clear current-sort indication;
- offering sort on the columns users prioritize, not on every column;
- indicating sort direction with more than an arrow color;
- composing sort with filters so the order applies to the filtered set.

A table that cannot be sorted, or that sorts unpredictably when filtered, forces users to scan the whole set every time.

### Design For The Full Range Of Data Volume

A table designed against typical data fails at the extremes. Zero rows, one row, hundreds, and thousands each need handling. The empty state must guide the user; the large state must stay performant and navigable.

Handle volume extremes:

- design an empty state that explains why and offers a next action;
- handle the single-row case without awkward layout;
- keep large datasets performant (virtualization, pagination, lazy load);
- provide search and filter so users can narrow large sets.

### Adapt The Table For Narrow Viewports

Tables that work on desktop collapse on mobile, where columns no longer fit. Horizontal scroll is a last resort, not a default. The table must transform for narrow viewports into something usable, often a card list that preserves the key fields and actions.

Adapt for mobile by:

- transforming rows into cards that show key fields and actions;
- keeping the identifying and action information primary;
- avoiding forcing horizontal scroll as the only option;
- preserving selection, sort, and filter capability in the mobile form.

A desktop table simply made horizontally scrollable on mobile is a common and near-unusable pattern.

## Common Traps

### Including Every Available Column

Tables that show all data fields push differentiators off-screen and force horizontal scrolling that hides what users need.

### Action Columns In Inconsistent Positions

Row actions placed in varying or middle positions force users to hunt for them on every row.

### Undefined Truncation And Overflow

Long content that truncates unpredictably turns distinguishable rows into identical "..." entries.

### Density Fixed For One Context

Tables locked to dense rows on mobile or spacious rows for experts serve neither audience.

### Ambiguous Select-All Scope

Select-all that does not make clear whether it covers the page or all filtered results causes serious errors before bulk destructive actions.

### Sorting That Breaks Under Filters

Sort that does not compose predictably with filters makes the order feel random and destroys trust.

### Ignoring Empty And Large-Volume States

Tables designed only for typical data collapse on empty, single, and very large datasets.

### Horizontal Scroll As The Only Mobile Option

Desktop tables made scroll-only on mobile are near-unusable; mobile needs a card transform.

## Self-Check

- [ ] Columns are selected based on what users identify, compare, and decide on, with non-essential fields moved to detail views.
- [ ] Column order follows the user's scan path: identifiers first, comparisons adjacent, actions consistently placed.
- [ ] Long and variable content has defined truncation, wrapping, alignment, and empty-value handling per column type.
- [ ] Row density matches the task and device, with a user toggle where preferences are strong and touch targets preserved.
- [ ] Selection and bulk actions are first-class, with clear select-all scope, a count-aware action bar, and selection preserved across filter and pagination.
- [ ] Sorting is offered on priority columns, clearly indicated with more than color, and composes predictably with filters.
- [ ] Empty, single-row, and large-volume states are designed, with performance and navigation support for large datasets.
- [ ] The table transforms for narrow viewports into a usable card form, not just horizontal scroll, preserving key fields, actions, selection, sort, and filter.
