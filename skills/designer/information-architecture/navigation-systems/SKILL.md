---
name: navigation_systems.md
description: Use when the agent is designing a navigation system, deciding between global and local navigation, choosing primary menus versus secondary menus, structuring breadcrumbs, faceted navigation, mega menus, hamburger menus, tabs, or in-page navigation, and must choose patterns that scale with content depth, user tasks, and platform constraints.
---

# Navigation Systems

A navigation system is the contract between how content is organized and how users reach it. It looks like a menu, but it is really a set of decisions about what is findable, what is primary, how deep users are expected to drill, and how the structure reveals itself as people move. Agents tend to treat navigation as a styling problem, copy a competitor's menu structure, or assume a single global navigation can serve every task. The harm shows up as users who cannot find content that exists, who bounce because the path is unclear, or who rely on search because browsing has failed them.

Use this skill before finalizing a menu structure, a set of navigation patterns, or the relationship between global, local, and contextual navigation. The goal is to prevent the agent from choosing navigation patterns that look clean but do not match how users actually seek content, from burying high-value destinations, or from building a structure that collapses under future content growth.

## Core Rules

### Design Navigation From User Tasks, Not From The Org Chart

The most common navigation failure is mirroring internal structure instead of user mental models. Departments, product teams, and content buckets mean something to the organization; they rarely mean the same thing to the user. Navigation organized by how the company is structured forces users to learn the company before they can find what they need.

Build navigation from task research:

- identify the top tasks users arrive to accomplish, not the top content buckets the team owns;
- name destinations in the user's vocabulary, validated through search logs, card sorts, or tree testing;
- group by user goals and mental models, then reconcile with organizational ownership;
- accept that a destination may need to appear under multiple labels if different user groups look for it differently.

When navigation and org chart align by coincidence, that is fine. When they are forced to align, users pay the cost.

### Separate Global, Local, And Contextual Navigation Deliberately

A single navigation bar cannot serve everything. Mature systems layer navigation by scope: global navigation persists across the product, local navigation reflects the current section, and contextual navigation appears inline where it is relevant. Collapsing these into one menu produces either an overwhelming global bar or a useless local one.

Define each layer's responsibility:

- global navigation holds the few top-level destinations users need from anywhere, usually between three and seven items;
- local navigation exposes the structure within the current section, adapting to where the user is;
- contextual navigation (inline links, related items, breadcrumbs) connects content laterally based on relevance.

If global navigation tries to hold everything, it becomes unusable. If local navigation is absent, users lose their place in deep sections. Decide the layering before choosing visual patterns.

### Choose Menu Patterns By Content Depth And Breadth, Not By Trend

Mega menus, hamburger drawers, accordions, tabs, and breadcrumbs each fit different structural realities. Choosing a pattern because it is fashionable, or because it worked on another product, produces navigation that fights the content.

Match pattern to structure:

- shallow, broad structures (many top-level items, little depth) suit horizontal bars or simple dropdowns;
- deep, hierarchical structures suit mega menus or expandable trees that reveal sub-levels without forcing a page load;
- narrow, peer-based structures suit tabs where all options are visible;
- mobile and space-constrained contexts suit drawers and accordions, but require careful handling of discoverability.

A mega menu on a shallow site is overkill and confusing. A hamburger hiding the only path to core content on desktop hurts findability. Let the information architecture's shape drive the pattern.

### Make The Current Location Always Visible

Users navigate more confidently when they can see where they are, where they came from, and where they can go. Navigation that does not indicate the current location forces users to re-orient from scratch on every page, increasing cognitive load and errors.

Ensure location feedback:

- highlight the active global and local navigation item;
- use breadcrumbs that reflect the actual path or hierarchy, not a fabricated one;
- maintain navigation state across filters and search so users do not lose their place;
- preserve context (selected filters, scroll position, expanded sections) when users navigate deeper and return.

Location indicators are cheap to implement and expensive to omit. Their absence is a leading cause of "lost in navigation" complaints.

### Design For Scalability, Not Just The Current Content

Navigation designed only for today's content breaks the moment a new section, product line, or content type is added. Agents often optimize for the existing sitemap and forget that the structure must absorb growth without reorganization.

Build for growth:

- leave headroom in global navigation for future top-level destinations;
- choose labels broad enough to accommodate sub-items not yet conceived, but specific enough to be clear;
- define the rules for when a section earns promotion to global navigation versus staying local;
- document the taxonomy so new content has a defined home rather than creating ad-hoc buckets.

A navigation system that requires a full redesign every time content grows is fragile. Plan the seams where expansion will happen.

### Decide How Search And Browse Cooperate

Navigation and search are not competitors; they are complementary paths that serve different seeking behaviors. Some users browse because they do not know the exact term; others search because browsing has failed them. A system that invests only in one leaves the other cohort stranded.

Plan the cooperation:

- ensure search is accessible from every page, not buried;
- design "no results" and poor-match states that offer browse paths back into the structure;
- use search analytics to find content users cannot reach by browsing, then fix the navigation rather than relying on search as a crutch;
- consider faceted navigation for content that users approach from multiple attributes.

If search is the only way users find key content, the navigation has failed, even if the search works well.

### Handle Faceted And Filtered Navigation Carefully

Faceted navigation (filtering by attributes like category, price, date) is powerful for large content sets but creates structural complexity that is easy to get wrong. Poorly designed facets produce duplicate or empty pages, confuse search engines, and overwhelm users with too many simultaneous filters.

For faceted navigation:

- limit the number of facets visible at once to what aids the task;
- show active filters clearly and make them removable individually;
- prevent combinations that yield zero results, or handle empty states gracefully;
- consider the SEO and URL implications of facet combinations, since each permutation can become an indexable page.

Facets are a tool for narrowing, not a substitute for a clear primary structure. Users should be able to reach most content without relying on filters.

## Common Traps

### Mirroring The Org Chart In The Menu

Structuring navigation by internal teams or departments forces users to learn the organization; structure by user tasks and mental models instead.

### Overloading Global Navigation

Putting every destination in the global bar makes it unusable; reserve global navigation for the few destinations needed from anywhere and push the rest to local or contextual layers.

### Choosing Patterns By Trend

Mega menus, hamburger drawers, and tabs each fit specific structures; copying a pattern without matching it to content depth and breadth produces navigation that fights the content.

### Hiding Core Paths On Mobile

Using a hamburger drawer to hide the primary navigation on mobile can destroy discoverability for first-time users who do not know to open it.

### Ignoring Location Feedback

Navigation that does not show the current location forces users to re-orient on every page and increases the sense of being lost.

### Designing Only For Current Content

Navigation optimized for today's sitemap breaks when content grows; plan headroom and expansion rules from the start.

### Treating Search As A Replacement For Navigation

If users can only find content through search, the navigation has failed; design browse and search to cooperate and use search analytics to fix structural gaps.

## Self-Check

- [ ] Navigation destinations are named in user vocabulary and grouped by user tasks, not by internal org structure.
- [ ] Global, local, and contextual navigation layers are defined with distinct responsibilities, not collapsed into one menu.
- [ ] Menu patterns (mega menu, tabs, drawer, accordion) were chosen to match content depth and breadth, not copied from another product.
- [ ] The current location is always indicated through active states, breadcrumbs, and preserved context.
- [ ] The system has headroom for future content growth, with documented rules for when sections earn global promotion.
- [ ] Search and browse are designed to cooperate, with search accessible everywhere and browse paths offered from poor search results.
- [ ] Faceted navigation, if used, limits visible facets, shows active filters clearly, and handles empty and duplicate-result cases.
- [ ] Mobile navigation preserves discoverability of core paths and does not hide everything behind an unlabeled drawer.
- [ ] Labels were validated through search logs, card sorts, or tree testing rather than chosen by internal preference.
