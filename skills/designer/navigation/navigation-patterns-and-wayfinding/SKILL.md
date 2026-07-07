---
name: navigation_patterns_and_wayfinding.md
description: Use when the agent is choosing how users move through an application, selecting navigation patterns such as tabs, sidebars, drawers, bottom bars, command palettes, or nested menus, and must balance discoverability, reachability, current-location clarity, and the cost of getting lost across device sizes.
---

# Navigation Patterns And Wayfinding

Navigation is the system by which users understand where they are, where they can go, and how to return. It is not a menu component; it is the product's wayfinding model. Agents tend to pick a navigation pattern because it looks clean or because a reference product uses it, then discover too late that users cannot find features, cannot tell which section they are in, or cannot get back to where they were. The harm is users who feel lost in a product they otherwise understand, who abandon tasks because the path forward is invisible, and who never discover capabilities that were hidden one layer too deep.

Use this skill before choosing how users move between top-level areas, sub-sections, and deep views, and before deciding what is persistent, what is on demand, and how current location is communicated. The goal is to prevent the agent from choosing a navigation pattern by appearance, from hiding primary destinations behind gestures or menus, or from leaving users without a reliable sense of where they are and how to return.

## Core Rules

### Choose The Pattern By Depth, Frequency, And Platform

Navigation patterns have different strengths, and the right one depends on the structure of the product and how users move through it. Tabs suit a small number of peer destinations visited often. Sidebars suit many destinations with sub-structure. Drawers suit mobile where space is constrained. Command palettes suit expert tools with many destinations. Matching the wrong pattern to the structure produces either cramped tabs or a drawer that hides everything.

Decide by asking:

- how many top-level destinations exist, and is that number stable?
- do destinations have sub-levels, and how deep?
- how often do users switch between destinations within a session?
- what platform and input methods must be supported?

A product with two top-level areas does not need a sidebar; a product with fifteen does not work as tabs. Match the pattern to the real structure.

### Make Primary Destinations Discoverable Without Prior Knowledge

The most important navigation rule is that a user should be able to find the main areas of the product by looking, not by knowing. Primary destinations hidden behind a hamburger, a gesture, or an unlabeled icon are invisible to anyone who has not been told they exist. Discoverability is not optional for primary navigation.

Ensure primary destinations are:

- visible as labeled entries, not icon-only, on at least one common viewport;
- reachable without a gesture that a new user would not know to perform;
- present in a consistent location so users learn where to look;
- backed by an accessible, keyboard-reachable path.

Hamburger menus and icon-only bars are acceptable when space truly demands them, but they trade discoverability for space, and that tradeoff must be deliberate, especially for first-time users.

### Always Indicate Current Location

Users navigate more confidently when they can see where they are. A navigation system that does not mark the current section forces users to infer location from content, which fails the moment content is ambiguous. Current-location indication must be persistent and perceivable.

Indicate location so that:

- the active destination is marked persistently, not only on hover or tap;
- the indicator uses more than color (fill, weight, icon, underline, position);
- the indication survives across the active section's sub-pages;
- deep location is communicated when users are several levels in.

Color-only active states vanish in high-contrast and dark modes and are invisible to color-blind users. Location indication must be robust.

### Keep The Path Back Always Available

Getting somewhere is half of navigation; getting back is the other half. Users constantly move forward and return, and a navigation system that makes returning hard produces dead ends and lost context. The return path must be reliable and predictable.

Guarantee a return path by:

- keeping persistent global navigation reachable so users can jump back to any top-level area;
- providing back affordances that match platform convention (browser back, app back, in-product back);
- preserving context on return (scroll position, filters, selected item) where feasible;
- avoiding flows that trap users in a modal or wizard with no clear exit.

A user who cannot get back to a list after opening a detail item has lost their place, and often their work.

### Limit Depth And Flatten Where Possible

Every level of navigation depth adds a step and a chance to get lost. Deep hierarchies feel organized to the designer and labyrinthine to the user. The instinct to nest by category usually works against findability. Flatten structure wherever the content allows.

Reduce depth by:

- promoting frequently accessed deep content closer to the top;
- using cross-links instead of strict trees so users can jump laterally;
- combining thin intermediate pages into their parents;
- reserving depth for genuinely nested relationships, not for categorization.

If a user must pass through three levels to reach a common task, the structure is too deep. Depth should reflect genuine containment, not organizational tidiness.

### Keep Navigation Consistent Across The Product

Users build a mental model of how to move, and that model depends on consistency. If navigation lives in a sidebar on one page and a top bar on another, or if the same kind of action is reached differently in different areas, users must relearn each surface. Consistency is what makes navigation feel reliable.

Maintain consistency by:

- keeping primary navigation in the same place and form across areas;
- using the same patterns for the same intents (back, switch section, open detail);
- documenting navigation conventions and deviating only deliberately;
- ensuring transitions between areas do not reset the user's spatial model.

### Coordinate Global, Section, And In-Content Navigation

Navigation operates at multiple levels: global (between top areas), section (within an area), and in-content (within a page, like tabs or anchors). These levels must coordinate, or they compete and confuse. Each level should have a clear role and not duplicate the others.

Separate the levels:

- global navigation moves between top-level areas;
- section navigation moves within an area's sub-views;
- in-content navigation moves within a single page;
- avoid mixing levels in one component, which blurs the model.

When a sidebar shows both global areas and section sub-pages with no visual separation, users cannot tell which level they are acting on.

### Support Keyboard, Search, And Direct Paths

Point-and-click navigation serves most users, but efficient products also support faster paths. Keyboard navigation, search, and direct links let expert and assistive-technology users move without scanning menus. These paths should exist alongside the visual navigation.

Provide alternative paths:

- keyboard shortcuts or a command palette for frequent destinations in expert tools;
- search as a first-class way to reach deep content;
- direct, shareable URLs for every navigable view;
- screen-reader landmarks and skip links for assistive navigation.

## Common Traps

### Hiding Primary Navigation Behind A Hamburger On Desktop

Hamburger menus that hide all navigation trade discoverability for space on viewports where space is not constrained, hurting first-time users.

### Color-Only Current-Location Indication

Active states that rely on color alone vanish in dark and high-contrast modes and are invisible to color-blind users.

### Nesting By Category Instead Of By Need

Deep hierarchies organized for the designer's tidiness bury common tasks under levels of categorization users must learn.

### Inconsistent Navigation Across Areas

When navigation moves location or changes form between areas, users lose their spatial model and feel lost.

### Dead Ends With No Return Path

Flows, modals, or wizards that trap users without a clear back affordance lose their context and sometimes their work.

### Mixing Navigation Levels In One Component

Global, section, and in-content navigation crammed into one menu blurs which level the user is acting on.

### Icon-Only Navigation Without Labels

Icons without text labels are ambiguous to new users and inaccessible to screen readers unless perfectly labeled, which they rarely are.

### Forgetting Keyboard, Search, And Direct Paths

Products that offer only point-and-click navigation frustrate expert and assistive-technology users who need faster or alternative routes.

## Self-Check

- [ ] The navigation pattern was chosen by the product's depth, destination count, switch frequency, and platform, not by appearance.
- [ ] Primary destinations are discoverable by looking, with labeled entries and a visible location, not hidden behind gestures or icon-only menus.
- [ ] Current location is indicated persistently using more than color, surviving across sub-pages and in dark and high-contrast modes.
- [ ] A reliable return path exists at all times, preserving context and avoiding dead ends or trapped flows.
- [ ] Navigation depth is minimized, with frequent deep content promoted and cross-links used instead of strict trees.
- [ ] Navigation is consistent across areas, with the same patterns for the same intents and deviations only deliberate.
- [ ] Global, section, and in-content navigation are separated by role and not mixed in one component.
- [ ] Keyboard, search, and direct shareable-URL paths exist alongside visual navigation, with screen-reader landmarks and skip links.
