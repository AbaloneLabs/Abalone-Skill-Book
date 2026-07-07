---
name: navigation_patterns.md
description: Use when the agent is choosing a navigation pattern, deciding between global navigation, hub-and-spoke, nested doll, bento box, tabbed views, command palettes, or gesture-based navigation, structuring primary and secondary navigation, designing for wayfinding across a large product, or selecting the navigation model that matches the product's content shape, device constraints, and user mental model.
---

# Navigation Patterns

Navigation is the structure that lets users move through a product, and the pattern chosen for it shapes what users can find, how they understand where they are, and how much effort each destination costs. Navigation is not a menu component; it is a theory of how the product's content is organized and how a user is expected to travel through it. The same content organized under different navigation patterns produces radically different experiences, and a pattern chosen because it is fashionable rather than because it fits the content's shape produces a product where users are perpetually lost.

Agents tend to fail navigation design in predictable ways. They default to a top global nav with a left sidebar because that is the convention, regardless of whether the content is shallow and wide or deep and narrow. They choose a pattern that looks clean in a screenshot but hides the destinations users actually need. They optimize the primary navigation and neglect wayfinding: where the user is, how they got there, and how to get back. Or they layer multiple navigation systems without reconciling them, so the user faces a global nav, a contextual sidebar, breadcrumbs, and a command palette that all describe slightly different models of the product.

Use this skill before choosing or restructuring a navigation pattern. The goal is a navigation model whose shape matches the content's structure, whose cost matches the frequency of the destinations, and whose wayfinding keeps the user oriented at every step.

## Core Rules

### Match The Navigation Pattern To The Content's Shape

Navigation patterns are not interchangeable. Each pattern implies a particular shape of content, and forcing content into an ill-fitting pattern makes destinations hard to find and hard to remember.

Match pattern to shape:

- global navigation with a flat hierarchy suits products with a small number of peer top-level destinations that users switch between frequently;
- hub-and-spoke suits products where users return to a central home between distinct tasks, common on mobile where screen real estate forces focus;
- nested doll, drilling from overview to detail, suits hierarchical content where users explore depth;
- bento box or dashboard suits products where many independent panels coexist and users monitor rather than travel;
- tabbed views suit a small set of peer views within a single object or context;
- command palettes suit power-user products where keyboard access and direct destination entry matter more than browsing.

A hub-and-spoke imposed on a product whose users need to move laterally between tasks forces constant returns to home. A global nav imposed on a deep hierarchy buries the leaves. The content's shape should select the pattern.

### Distinguish Primary, Secondary, And Tertiary Navigation

Most products need more than one level of navigation, and confusing the levels produces clutter and disorientation. Primary navigation orients the user across the product; secondary navigation orients them within a section; tertiary navigation orients them within a page or object.

Structure navigation by level:

- primary navigation holds the top-level destinations and stays persistent and predictable;
- secondary navigation reflects the structure within a section and changes as the user moves;
- tertiary navigation, such as in-page tabs or anchors, handles detail within a single context;
- keep the levels visually and behaviorally distinct, so the user always knows which level they are acting on.

When primary and secondary navigation look the same, users cannot tell whether clicking will move them to a new section or within the current one.

### Design Wayfinding, Not Just Movement

Getting somewhere is half the problem; knowing where you are is the other half. Wayfinding is the set of signals that tell users their current location, how they arrived, and how to return. Navigation that moves users but does not orient them produces a product that feels like a maze even when it is efficient.

Build wayfinding into every level:

- show the current location in the navigation, with a clear active state on the current destination and its ancestors;
- preserve breadcrumbs or an equivalent location trail for deep hierarchies;
- maintain context across navigation, so returning to a section restores the user's place rather than resetting it;
- make the back path obvious and reliable, because users navigate by retreating as much as by advancing.

A user who can move but cannot locate themselves spends cognitive effort on orientation that should go to the task.

### Match Navigation Cost To Destination Frequency

Not all destinations are equally important, and navigation should make frequent destinations cheap and rare destinations possible. A flat structure that treats every destination equally makes the common case as expensive as the rare one, and a deep structure that buries a frequent task behind several levels makes the common case expensive.

Allocate cost by frequency:

- place the most frequent destinations at the cheapest access point, usually the primary navigation;
- push rare or advanced destinations to secondary or tertiary locations, search, or settings;
- consider power-user shortcuts, such as command palettes or quick actions, for destinations that are frequent for a subset of users;
- revisit the cost allocation as usage patterns emerge, because the assumed frequency is often wrong.

Navigation that makes the rare case as cheap as the common case is cluttered; navigation that makes the common case expensive is frustrating.

### Reconcile Multiple Navigation Systems

Large products accumulate navigation systems: a global nav, a contextual sidebar, breadcrumbs, search, a command palette, in-page tabs. When these systems describe different models of the product, users must reconcile the contradictions, which is exhausting and error-prone.

Reconcile the systems:

- ensure all navigation systems reflect one consistent model of the product's structure;
- define which system is authoritative for which kind of movement, so the user knows where to look;
- avoid redundant systems that compete, such as a global nav and a sidebar that both offer the same destinations;
- document the navigation model so that additions do not fragment it further.

### Account For Device And Platform Constraints

Navigation that works on desktop often fails on mobile, and navigation that works on mobile often underuses desktop space. The pattern must adapt to the device's constraints: screen size, input method, and the context in which the device is used.

Adapt to the platform:

- on mobile, favor focused patterns such as hub-and-spoke or bottom navigation, because screen space cannot support a dense global nav;
- on desktop, use the available space for persistent wayfinding and quick switching;
- account for input method, since touch targets, hover states, and keyboard access differ across platforms;
- preserve the underlying navigation model across platforms even when its expression changes, so users transfer their learning.

A navigation model that is consistent in structure but adaptive in expression serves users across devices without forcing them to relearn.

### Validate Navigation Against Real Tasks

Navigation that looks logical in a diagram can still fail real users, because the diagram reflects the team's model, not the user's. The only reliable test is whether users can complete real tasks using the navigation, and this should be checked before the pattern is locked.

Validate before committing:

- run tree tests or first-click studies to measure whether users can find key destinations under the proposed structure;
- test with tasks of varying frequency and depth, not only the obvious ones;
- check that users can recover when they go the wrong way, because recovery is where navigation often fails;
- treat the navigation as a hypothesis to be tested, not a decision to be declared.

## Common Traps

### Defaulting To Global Nav Plus Sidebar

The conventional top-nav-plus-sidebar fits some content shapes and fights others. Choose the pattern from the content's structure, not from convention.

### Choosing A Pattern For Aesthetics Over Fit

A clean-looking pattern that hides the destinations users need produces a beautiful product no one can navigate.

### Neglecting Wayfinding

Navigation that moves users without orienting them leaves users lost even when movement is efficient.

### Equal Cost For All Destinations

Treating rare and frequent destinations the same makes the common case as expensive as the rare one, cluttering the interface.

### Unreconciled Navigation Systems

Multiple navigation systems that describe different models force users to reconcile contradictions and erode trust in all of them.

### Ignoring Device Constraints

A navigation pattern that works on desktop may be unusable on mobile, and vice versa. Adapt the expression while preserving the model.

### Optimizing The Diagram, Not The Tasks

A navigation structure that looks logical to the team but fails real tasks is still wrong. Validate against actual user tasks before locking the pattern.

### Forgetting Recovery Paths

Users go wrong constantly. Navigation that does not support easy recovery makes every wrong turn expensive.

## Self-Check

- [ ] The navigation pattern was chosen to match the content's shape, not defaulted by convention.
- [ ] Primary, secondary, and tertiary navigation are distinct in structure, appearance, and behavior.
- [ ] Wayfinding signals show the user's current location, how they arrived, and how to return at every level.
- [ ] Navigation cost is allocated by destination frequency, with frequent destinations cheap and rare ones possible.
- [ ] Multiple navigation systems, if present, reflect one consistent model of the product and do not compete.
- [ ] The pattern adapts to device and platform constraints while preserving a consistent underlying model.
- [ ] The navigation was validated against real tasks of varying frequency and depth, including recovery from wrong turns.
- [ ] The active state and location trail are clear, so users always know where they are.
- [ ] Power-user shortcuts are provided for destinations frequent to a subset of users.
- [ ] The navigation model is documented so future additions do not fragment it.
