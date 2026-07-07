---
name: edge_case_and_stress_case_design.md
description: Use when the agent is designing flows, states, or components and must account for edge cases, stress cases, empty data, extreme content, long values, zero results, errors, concurrent actions, degraded conditions, or unusual but realistic user situations that break idealized designs.
---

# Edge Case And Stress Case Design

Most interface design is done against happy, average content: names of medium length, lists with several items, images that crop cleanly, networks that respond instantly. Real users generate the opposite. They have one item or ten thousand, names that are forty characters or a single letter, no profile photo or an enormous one, a connection that drops mid-action, a balance that is negative, a date range that spans years. A design that survives only average content fails the moment it meets reality.

Use this skill before finalizing layouts, components, flows, data displays, forms, or empty and error states. The goal is to prevent the agent from designing only the happy path, hardcoding space for typical content, or treating unusual cases as someone else's problem. Edge and stress cases are not exceptions to manage later; they are part of the design.

## Core Rules

### Design From The Extremes, Not The Average

Average content hides layout failures. A card designed for a typical product name breaks when the name is one word or three lines. Decide spacing, truncation, wrapping, and alignment using the shortest and longest realistic values, then verify the average still works.

For every text container, image frame, list, and data field, identify:

- the minimum: zero items, empty string, single character, no image;
- the maximum: very long string, hundreds of items, oversized image, extreme number;
- the unusual: special characters, right-to-left scripts, emoji, negative numbers, null values.

If the design holds at both extremes, it generally holds in between.

### Plan For Empty, Loading, And Error From The Start

These are not fallback screens added at the end; they are core states that most users encounter. Design each deliberately:

- empty: first run, no data yet, no results, no permission, nothing to show;
- loading: skeleton, spinner, progressive reveal, placeholder that matches final layout;
- error: network failure, validation failure, permission denied, server error, partial failure;
- partial: some data loaded, some failed, stale cache, degraded mode.

Each state should explain what happened, what the user can do, and avoid dead ends. An empty state that only says "nothing here" is a failed design.

### Handle Long And Variable Content Explicitly

Text and data rarely match the designer's example. Decide, for every field, what happens when content overflows:

- truncate with an accessible way to read the full value, such as a tooltip or expandable region;
- wrap to additional lines and confirm the layout absorbs the height change;
- shrink within a bounded range, but never below a readable size;
- reflow rather than overlap or clip.

Never assume content will fit. Never clip text without an affordance to see it. Test names, addresses, titles, descriptions, and user-generated content at their realistic extremes.

### Account For Concurrent, Interrupted, And Repeated Actions

Users do not complete flows in a clean sequence. They open multiple tabs, leave a form half-filled, lose connection mid-submit, double-click submit, navigate back, or return days later. Each of these can corrupt state, duplicate submissions, or lose data.

Design for:

- resumable forms that preserve entered data across refresh, navigation, and sessions;
- idempotent submissions that do not create duplicates on retry;
- clear feedback when an action is in progress, completed, or failed;
- recovery from interruption without silent data loss;
- conflict handling when the same data is edited in two places.

### Treat Zero And One As First-Class Counts

Lists designed for many items often collapse or look broken with zero or one. A grid of products with a single item, a table with one row, or a dashboard with no activity each need deliberate design. Zero should offer guidance and next steps; one should not look like a loading error or a broken grid.

### Respect Degraded And Asymmetric Conditions

Not every user has full capability at all times: slow networks, intermittent connectivity, reduced motion, low battery, backgrounded apps, small viewports, or assistive technology. A design that depends on fast responses, smooth animation, or a large viewport fails under stress.

Build graceful degradation:

- content usable before all assets load;
- animation that does not block comprehension when reduced or absent;
- layouts that reflow rather than break on unusual viewports;
- actions that succeed or fail clearly on poor networks rather than hanging silently.

### Make Failure Recoverable And Understandable

When something goes wrong, the user needs to understand what happened and what to do next, not just that an error occurred. Avoid generic messages, error codes without context, and states that trap the user. Prefer specific, human explanations with a clear recovery path, and preserve any work the user has already done.

## Common Traps

### Designing Only The Happy Path

Screens are built with ideal content and ideal flow, then edge cases are patched in late, often as compromises that hurt the core experience.

### Hardcoding Heights And Widths

Fixed dimensions assume fixed content. The first long name, tall image, or empty list breaks the layout or clips the content.

### Truncation Without Recovery

Cutting off text with no way to read the full value hides information from exactly the users who need it most, such as those with long or unusual values.

### Generic Empty And Error States

A blank screen or a "something went wrong" message gives the user no path forward and erodes trust, especially when it loses their work.

### Ignoring Double Submit And Retry

Without idempotent handling, a retry or double-click creates duplicate orders, duplicate messages, or corrupted data.

### Assuming A Fast, Stable Connection

Designs that block all interaction until everything loads, or that hang silently on failure, punish users on slow or intermittent networks.

### Treating Zero Results As A Dead End

"No results" with no suggestions, filters, or alternatives abandons the user at the moment they were looking for something.

## Self-Check

- [ ] Every text container, image frame, list, and data field was tested with the shortest and longest realistic values, not only average content.
- [ ] Empty, loading, error, and partial states are designed deliberately with explanation and a recovery path, not added as fallback screens.
- [ ] Overflowing content truncates, wraps, or reflows with an accessible way to read the full value rather than clipping silently.
- [ ] Forms preserve entered data across refresh, navigation, interruption, and return visits.
- [ ] Submissions are idempotent and give clear feedback, preventing duplicates on retry or double-click.
- [ ] Zero and one item counts are designed as first-class states, not broken-looking versions of the many-item layout.
- [ ] The design degrades gracefully under slow networks, reduced motion, small viewports, and backgrounded conditions.
- [ ] Error messages explain what happened and what to do next, and never silently discard the user's work.
- [ ] No layout depends on fixed heights, widths, or content lengths that real data will violate.
- [ ] Concurrent editing, navigation back, and session resumption were considered and handled rather than left to chance.