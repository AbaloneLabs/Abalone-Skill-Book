---
name: search_results_presentation.md
description: Use when the agent is designing search results layout, result cards, ranking and sort presentation, result snippets, result density, pagination versus infinite scroll, empty and loading states, or how users evaluate and choose from a list of search results.
---

# Search Results Presentation

The results page is where the user decides whether search succeeded. A correct ranking buried in a confusing layout, with weak snippets, unclear sort, and no way to compare, fails even when the right item is technically present. Results presentation is the bridge between the system's relevance and the user's ability to recognize what they want. The design problem is making the best results obvious, scannable, and comparable, while supporting the different ways users evaluate results across content types.

Use this skill before designing results layouts, result cards, snippet presentation, sort controls, density choices, pagination or infinite scroll, or the states around results. The goal is to prevent the agent from presenting results in a way that hides relevance, wastes the viewport, or forces users to open items just to tell them apart.

## Core Rules

### Lead With The Information That Drives The Decision

Each result should front-load the attributes users use to choose. These differ by content type:

- products: image, title, price, availability, rating;
- documents: title, source, date, relevant snippet;
- people: name, role, avatar, affiliation;
- messages: sender, time, matched text;
- media: thumbnail, title, duration.

Identify the decision attributes for your content and make them dominant. Secondary details support but do not compete with the primary signals.

### Use Snippets To Show Why A Result Matches

For text-based search, the snippet is often more important than the title. It shows the user why this result matched their query. Design snippets to:

- highlight the matched terms in context;
- show enough surrounding text to judge relevance;
- avoid truncating mid-sentence where possible;
- distinguish the match clearly from the rest of the text.

A result with a generic description forces the user to open it to judge relevance; a good snippet lets them decide without leaving the page.

### Make Sort And Ranking Logic Transparent

Users need to understand why results are in this order and how to change it. Provide:

- a visible, changeable sort control;
- a clear default sort and why it is the default;
- options that match real user intents, relevance, date, price, rating;
- indication when results are personalized or sponsored.

Opaque ranking erodes trust, especially when a user expects a specific item and cannot find it.

### Distinguish Sponsored, Personalized, And Organic Results

When results include ads, recommendations, or personalization, label them clearly. Users who discover a result was sponsored after trusting it feel manipulated. Clearly demarcated sponsored slots preserve trust while still allowing monetization. Apply consistent, honest labeling rather than blending paid and organic results.

### Match Density To The Evaluation Task

Different tasks need different density:

- comparison-heavy tasks, like shopping or data lookup, benefit from compact rows so users can scan many options;
- exploration or visual tasks, like browsing media or destinations, benefit from larger cards with imagery;
- reading-heavy tasks, like document search, benefit from snippets and source context.

Do not apply one density everywhere. A results page that is too sparse for comparison or too dense for visual browsing fails its task.

### Choose Pagination, Infinite Scroll, Or Load-More Deliberately

Each pattern has tradeoffs:

- pagination gives stable position and a sense of extent, good for task-oriented search;
- infinite scroll supports browsing and exploration but loses position and footer access;
- load-more offers a middle ground with controlled loading.

Match the pattern to the intent. Infinite scroll on a results page where users need the footer, filters, or a sense of how many results exist creates navigation problems.

### Design Loading, Empty, And Error States As Part Of Results

Results pages have states beyond success:

- loading: show that search is running, with skeleton results or progress, not a blank screen;
- empty: explain why and offer recovery, not a dead end;
- partial: show what was found even if some sources failed;
- error: explain the failure and offer retry.

These states are part of the results experience, not edge cases to ignore.

### Support Comparison And Return Visits

Users compare results and return to ones they have evaluated. Support this:

- mark visited or viewed results;
- allow saving, pinning, or shortlisting;
- preserve scroll position when returning from a detail view;
- let users open results in new tabs or panes for comparison.

Forcing users to re-find results after viewing one detail page breaks comparison workflows.

### Handle Result Count And Extent Honestly

Show how many results exist so users can judge the scope of their search. But avoid false precision: "about 1,200 results" is more honest than an exact count that is actually an estimate. If results are capped, sampled, or paginated from a larger set, do not imply completeness that does not exist.

### Adapt Presentation To Zero, Few, And Many Results

The layout should respond to result volume:

- zero: recovery and suggestions, not a blank list;
- few: enough context to evaluate each without wasted space;
- many: strong scannability, clear sort, and filtering to narrow.

A single layout for all volumes serves none of them well.

## Common Traps

### Generic Descriptions Instead Of Match Snippets

Results that show the same description regardless of the query force users to open each item to judge relevance.

### Opaque Ranking

When users cannot see or change the sort, they lose trust, especially if an expected item is missing or buried.

### Blended Sponsored And Organic Results

Hiding paid placement erodes trust the moment users realize what happened.

### One Density For All Content

Applying sparse cards to comparison tasks or dense rows to visual browsing serves neither.

### Infinite Scroll Where Position Matters

Endless loading on pages where users need footers, filters, or a sense of extent creates navigation dead ends.

### Blank Or Missing Loading And Empty States

A blank screen during search or a dead-end empty state makes the product feel broken.

### No Support For Comparison

Forgetting visited state or losing scroll position on return forces users to re-evaluate from scratch.

### False Precision In Result Counts

Presenting estimates as exact counts, or implying completeness for sampled results, misleads users about scope.

## Self-Check

- [ ] Each result front-loads the attributes users actually use to decide for that content type.
- [ ] Text results show match snippets with highlighted terms and enough context to judge relevance without opening the item.
- [ ] Sort is visible, changeable, and the default is explained, with options matching real user intents.
- [ ] Sponsored, personalized, and organic results are clearly and honestly labeled.
- [ ] Density matches the evaluation task: compact for comparison, larger for visual or exploratory browsing.
- [ ] Pagination, infinite scroll, or load-more is chosen based on whether position, extent, or browsing matters most.
- [ ] Loading, empty, partial, and error states are designed as part of the results experience with recovery paths.
- [ ] Visited state, saving, pinning, and scroll preservation support comparison and return visits.
- [ ] Result counts are honest about estimates, caps, and sampling rather than implying false precision.
- [ ] The layout adapts to zero, few, and many results rather than using a single fixed presentation.
