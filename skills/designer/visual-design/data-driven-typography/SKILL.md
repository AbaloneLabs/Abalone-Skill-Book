---
name: data_driven_typography.md
description: Use when the agent is designing typography for data-heavy or dynamic interfaces, setting type for tables, dashboards, metrics, and variable-length content, choosing type for numeric alignment and tabular figures, handling responsive type that adapts to content length and viewport, designing typographic systems that survive unknown or user-generated content, or ensuring type remains legible and structured when content is long, short, empty, or extreme.
---

# Data Driven Typography

Typography in a marketing layout is chosen for expression; typography in a data-driven interface is chosen for resilience. Dashboards, tables, metrics, and dynamic content confront the type system with conditions a static mockup never simulates: numbers of varying length, labels that overflow their columns, content that is empty or extremely long, values that update in real time, and viewports that stretch from a phone to an ultrawide monitor. A type system that looks beautiful in a fixed mockup collapses the moment real data arrives, with columns that misalign, numbers that jump as they change, and labels that truncate into meaninglessness. Data-driven typography is the discipline of designing type that survives the data it will actually carry.

Agents tend to fail data-driven typography in predictable ways. They choose proportional figures for numeric columns, so changing a value shifts every digit and the column jitters. They fix column widths to ideal content, so a slightly longer value truncates or wraps disastrously. They design for typical content and never test the extremes: the empty state, the longest possible value, the localized string twice as long as the English. Or they treat type as a static visual choice and hand the dynamic behavior to engineering without specification.

Use this skill before finalizing typography in any data-heavy or dynamic interface, when setting type for tables and metrics, and when designing type systems that must survive variable content. The goal is type that stays legible, aligned, and structured across the full range of real data and viewports.

## Core Rules

### Use Tabular Figures For Numeric Alignment

Numbers in columns must align by digit, so that a user scanning a column of values can compare them vertically without the digits dancing. Proportional figures, where each digit takes a different width, destroy this alignment, and a changing value shifts every digit, producing a jittering column that is hard to read and harder to trust.

Use tabular figures:

- enable tabular, monospaced figures for any numeric column where values are compared or change over time;
- align numbers by the decimal or by the right edge, depending on whether precision comparison matters;
- reserve proportional figures for standalone numbers in prose, where alignment is irrelevant;
- verify that the chosen typeface includes tabular figures, because not all do, and a fallback may be needed.

A column of currency or metrics in proportional figures is a common and avoidable failure; tabular figures fix it instantly.

### Design For The Full Range Of Content, Not The Typical

Mockups are built with ideal content, and real interfaces receive content that is shorter, longer, emptier, and more varied than the mockup ever showed. Designing type for the typical case guarantees failure at the extremes, which is where interfaces break most visibly.

Design for the range:

- test with the shortest, longest, and empty variants of every text container;
- define truncation, wrapping, and overflow behavior for each field, so engineering does not guess;
- account for localized content, which can be far longer or shorter than the source language;
- plan for user-generated content, which is unpredictable in length, character set, and formatting.

A label that truncates to an ellipsis may be acceptable; a label that truncates to a single character is a bug. Decide the threshold deliberately.

### Set Type For Scanning And Comparison

Data interfaces are read by scanning and comparison, not by sequential reading. Users move their eyes across rows and down columns, comparing values and finding anomalies. Type in this context must support rapid scanning: consistent vertical rhythm, clear column separation, and values that align for comparison.

Support scanning:

- use consistent line height and vertical rhythm so rows are easy to track across a wide table;
- separate columns with enough space or subtle dividers so values do not bleed into neighbors;
- align related values consistently, numbers to the right or decimal, text to the left;
- reserve weight and color for emphasis, so anomalies stand out without overwhelming the scan.

### Handle Variable-Length Labels And Values

Labels and values in a data interface vary in length, and a type system that assumes fixed widths breaks when content does not cooperate. The design must specify how each element behaves when content is shorter or longer than expected.

Handle variability:

- decide for each text element whether it truncates, wraps, shrinks, or overflows, and under what conditions;
- prefer truncation with a tooltip or accessible expansion over wrapping that breaks row height;
- define minimum and maximum widths so columns do not collapse to nothing or stretch without limit;
- test the interaction between variable content and the layout, because a long value can push neighbors aside.

### Choose Typefaces That Survive Data Conditions

Not every typeface suits a data interface. Expressive display faces, thin weights, and faces without tabular figures all fail under data conditions. The typeface must be legible at small sizes, must include the numeric features the interface needs, and must render consistently across the platforms and languages the product serves.

Choose typefaces for data:

- prioritize legibility at small sizes, because data interfaces often pack much into little space;
- confirm the typeface includes tabular figures, multiple weights, and the language scripts required;
- avoid thin or decorative weights for data, which lose legibility at small sizes and on low-resolution screens;
- test rendering across platforms and browsers, because the same typeface can render differently and break alignment.

### Make Type Responsive To Viewport And Density

Data interfaces appear across a wide range of viewports and density preferences, and fixed type sizes break at the extremes. Type that is comfortable on a desktop monitor can be too small on a high-density display or too large on a phone. A responsive type system adapts while preserving its structure.

Make type responsive:

- use relative units so type scales with user settings and viewport;
- define how type scales at breakpoints, preserving the hierarchy and rhythm;
- support user density preferences, where the user can choose compact or comfortable spacing;
- re-test alignment and truncation at each size, because behavior that works at one scale fails at another.

### Preserve Hierarchy And Emphasis Under Density

A data interface must communicate hierarchy even when dense: which value is the primary metric, which row is selected, which cell is an anomaly. Under density, hierarchy is easily lost, and the type system must retain the tools to signal emphasis without relying on space alone.

Preserve hierarchy:

- use weight, size, and color to signal primary metrics and anomalies, not only position;
- keep emphasis cues consistent, so the same treatment means the same thing everywhere;
- reserve the strongest emphasis for the single most important value, avoiding a wall of bold numbers;
- ensure emphasis survives theme changes and accessibility settings, so it does not depend on color alone.

### Account For Real-Time And Updating Values

Some data interfaces display values that update in real time, and updating values introduce typographic challenges that static type does not face. A value that changes width on every update causes layout shift, and rapid updates can produce a flickering, unreadable field.

Handle updating values:

- reserve fixed space for the maximum likely width, so updates do not shift the layout;
- use tabular figures so digit positions stay stable across updates;
- throttle or smooth rapid updates so the value remains readable rather than flickering;
- signal updates through subtle cues, such as a brief highlight, rather than through layout change.

## Common Traps

### Proportional Figures In Numeric Columns

Proportional digits destroy vertical alignment and cause columns to jitter as values change; use tabular figures.

### Designing For Typical Content Only

Mockups with ideal content hide failures at the extremes, where interfaces break most visibly.

### Undefined Overflow Behavior

Fields without specified truncation, wrapping, or overflow force engineering to guess, producing inconsistent results.

### Ignoring Localization Length

Localized strings can be far longer or shorter than the source; fixed widths break across languages.

### Decorative Or Thin Typefaces For Data

Expressive and thin faces lose legibility at small sizes and lack the numeric features data needs.

### Fixed Type Sizes Across Viewports

Fixed sizes break at the extremes of viewport and density; use relative units and breakpoint scaling.

### Lost Hierarchy Under Density

Dense interfaces lose emphasis when hierarchy relies on space alone; preserve weight, size, and color cues.

### Layout Shift From Updating Values

Real-time values that change width cause layout shift; reserve space and use tabular figures.

## Self-Check

- [ ] Tabular figures are enabled for numeric columns where values are compared or update over time, with consistent decimal or right alignment.
- [ ] Type was tested with the full range of content: shortest, longest, empty, localized, and user-generated variants.
- [ ] Truncation, wrapping, and overflow behavior is specified for every variable-length text element.
- [ ] Column alignment, line height, and spacing support scanning and comparison across rows and columns.
- [ ] The typeface is legible at small sizes, includes tabular figures and required scripts, and renders consistently across platforms.
- [ ] Type uses relative units and scales at breakpoints, preserving hierarchy and rhythm, and supports user density preferences.
- [ ] Hierarchy and emphasis survive density through weight, size, and color, without relying on space or color alone.
- [ ] Real-time and updating values reserve fixed space and use tabular figures to avoid layout shift and jitter.
- [ ] Minimum and maximum widths are defined so columns do not collapse or stretch without limit.
- [ ] The type system was validated with real data at real lengths, not placeholder content that hides failures.
