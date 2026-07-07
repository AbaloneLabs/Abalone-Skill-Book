---
name: screen_reader_design.md
description: Use when the agent is designing interfaces for screen reader compatibility, structuring semantic HTML and landmarks, writing accessible names and labels, managing focus and dynamic content announcements, handling ARIA roles and live regions, designing heading hierarchy and reading order, or ensuring dynamic interfaces work with assistive technology.
---

# Screen Reader Design

Screen reader design is the practice of building interfaces that work when the visual layer is absent and the content is read aloud or output to a braille display. It looks like adding labels, but it is really a structural discipline: the semantic order, the roles, the names, and the dynamic announcements must convey the same meaning and operability that sighted users get from the visual design. Agents tend to treat screen reader support as a labeling afterthought, assume the visual order is the reading order, or add ARIA without understanding it. The harm is interfaces that look complete but are mazes of unnamed buttons, illogical reading order, and silent dynamic changes for the users who depend on assistive technology.

Use this skill before finalizing structure, semantics, or dynamic behavior for screen reader access. The goal is to prevent the agent from building interfaces that are visually rich but semantically empty, from breaking the reading order, or from announcing dynamic changes in ways that confuse rather than inform.

## Core Rules

### Build With Semantics, Not Visuals, As The Foundation

A screen reader does not interpret pixels; it consumes the semantic structure of the page. A div styled to look like a button is not a button to a screen reader unless it has the right role, name, and operability. The foundation of screen reader access is semantic HTML and ARIA that convey what each element is, not how it looks.

Build semantically:

- use native HTML elements (button, nav, main, headings) that carry built-in semantics before reaching for ARIA;
- use ARIA only to fill gaps native HTML cannot express, since the first rule of ARIA is not to use it when not needed;
- ensure every interactive element has a correct role (button, link, checkbox, tab) that matches its function;
- never rely on visual cues (color, shape, position) to convey role or state without semantic backing.

An interface built visually first and semantically patched later is fragile and incomplete. Semantics must be part of the structure, not decoration applied afterward.

### Establish A Logical Reading Order Independent Of Visual Layout

Screen readers read content in the order it appears in the markup, which may differ from the visual layout when CSS repositions elements. A visually logical layout can have a reading order that jumps confusingly if the source order does not match. The reading order must be logical on its own, independent of how CSS arranges things visually.

Ensure logical reading order:

- structure the source order to match the intended reading sequence, not the visual positioning;
- avoid using CSS order or grid placement to reorder content in ways that break the source sequence;
- test the reading order with a screen reader, not just by sight;
- ensure related content reads together and unrelated content is clearly separated.

A layout where the visual order and source order diverge produces a reading experience that makes no sense. The source order is the reading order; make it intentional.

### Provide Accessible Names For Every Interactive Element

An interactive element without an accessible name is a mystery to a screen reader user: "button" with no label tells them nothing. Every interactive element needs an accessible name that describes its purpose, derived from visible labels, aria-label, aria-labelledby, or text content. Unnamed elements are unusable.

Name everything meaningfully:

- prefer visible text labels, which serve both sighted and screen reader users;
- use aria-label or aria-labelledby for icon-only buttons and controls with no visible text;
- ensure names are descriptive and distinguishable ("Close menu" not just "X"; "Search" not just an icon);
- avoid duplicate or generic names when elements do different things.

A screen reader announcing a string of unnamed "button" elements is an unusable interface. Names are how screen reader users navigate and understand.

### Use Headings And Landmarks To Create Navigable Structure

Screen reader users do not read pages linearly; they navigate by headings, landmarks, and regions to find what they need. A page with no heading hierarchy or landmarks forces linear reading, which is slow and disorienting. Headings and landmarks are the map that lets users jump to structure.

Build navigable structure:

- use a logical heading hierarchy (one h1, then h2s for major sections, h3s for subsections) without skipping levels;
- use landmark roles (header, nav, main, aside, footer) to define page regions users can jump to;
- ensure headings are descriptive of their section's content, not generic;
- do not use headings for visual styling alone, which pollutes the navigation structure.

A page with no headings is a wall of text to a screen reader user. Headings and landmarks are the table of contents that make content navigable.

### Manage Focus Deliberately In Dynamic Interfaces

Dynamic interfaces (single-page apps, modals, menus, route changes) change content without a page reload, and screen readers follow focus. If focus is not managed, the screen reader stays where it was while the interface changes around it, leaving the user lost. Focus management is what makes dynamic interfaces comprehensible.

Manage focus explicitly:

- move focus to new content when it appears (a modal opening should move focus into it);
- trap focus within modals and dialogs so the user cannot tab out into hidden content;
- return focus to the triggering element when dynamic content closes (closing a modal returns focus to the button that opened it);
- handle route changes in single-page apps by moving focus to the new page's main content and announcing the change.

Unmanaged focus in a dynamic interface is the most common cause of screen reader disorientation. Every dynamic change must include a focus decision.

### Announce Dynamic Changes Through Live Regions

Some dynamic changes (a new item added to a list, an error appearing, a status update) happen without focus moving to them. Without an announcement, a screen reader user does not know the change occurred. Live regions (aria-live) tell the screen reader to announce content changes, but they must be used precisely or they produce noise or silence.

Use live regions carefully:

- use polite live regions for non-urgent updates that should announce when the user pauses;
- use assertive live regions only for urgent changes that must interrupt;
- ensure the live region exists in the DOM before the change, so the announcement fires;
- avoid over-announcing, which floods the user with noise and teaches them to ignore alerts.

A live region that never announces leaves users unaware of changes; one that announces everything overwhelms them. Calibrate announcements to importance.

### Make Complex Widgets Operable With Established Patterns

Custom widgets (tabs, accordions, menus, comboboxes, data grids) have no native semantics, and screen reader users have expectations for how they behave based on established patterns. Building a custom widget without following the ARIA authoring patterns produces something neither sighted nor screen reader users can operate predictably.

Follow established widget patterns:

- implement the ARIA authoring practices for tabs, menus, accordions, comboboxes, and grids;
- ensure full keyboard operability: arrow keys, home, end, and the expected shortcuts for each widget;
- convey state (expanded, selected, current) through aria attributes, not visual cues alone;
- test the widget with screen readers to confirm it behaves as the pattern specifies.

A custom widget that reinvents interaction is unusable by screen reader users who rely on consistent patterns. Established patterns are the shared contract.

### Test With Real Screen Readers And Real Users

Screen reader behavior cannot be fully predicted from specifications; different screen readers (NVDA, JAWS, VoiceOver, TalkBack) interpret semantics differently, and bugs abound. Testing only in a browser's accessibility inspector, or only with one screen reader, hides failures. Real testing requires actual screen readers and, ideally, real users.

Test authentically:

- test with the screen readers your users use (NVDA and JAWS on Windows, VoiceOver on Mac and iOS, TalkBack on Android);
- learn the basic navigation commands of each, or test with someone proficient;
- verify reading order, names, focus management, and announcements behave as intended;
- include screen reader users in usability testing, as they reveal problems no simulator catches.

An interface "tested" only with an automated tool or a sighted tester tabbing through is not verified for screen reader users. Authentic testing is the only reliable validation.

## Common Traps

### Semantics As An Afterthought

Interfaces built visually first and semantically patched are incomplete; build with semantic HTML and ARIA as the foundation.

### Reading Order That Follows Visual Layout

CSS reordering that breaks the source order produces a confusing reading sequence; structure source order to match the intended reading.

### Unnamed Interactive Elements

Icon-only buttons and controls without accessible names are unusable mysteries; provide descriptive names for every element.

### No Heading Hierarchy Or Landmarks

Pages without navigable structure force slow linear reading; use logical headings and landmarks as a map.

### Unmanaged Focus In Dynamic Interfaces

Single-page apps and modals that do not move focus leave screen reader users lost; manage focus on every dynamic change.

### Silent Or Noisy Live Regions

Changes that go unannounced or live regions that over-announce both fail; calibrate announcements to importance.

### Custom Widgets Without Established Patterns

Reinvented interaction breaks screen reader users' expectations; implement ARIA authoring practices for complex widgets.

### Testing Without Real Screen Readers

Browser inspectors and one screen reader hide failures; test with the actual tools and users who depend on them.

## Self-Check

- [ ] The interface is built on semantic HTML and ARIA, with correct roles for every interactive element, not visuals patched with semantics.
- [ ] The source reading order is logical and matches the intended sequence, independent of CSS visual positioning.
- [ ] Every interactive element has a descriptive, distinguishable accessible name, with visible labels preferred.
- [ ] A logical heading hierarchy (one h1, descending without skipping) and landmarks define navigable page regions.
- [ ] Focus is managed on every dynamic change: moved to new content, trapped in modals, returned on close, and moved on route changes.
- [ ] Live regions announce dynamic changes with calibrated politeness, existing in the DOM before the change, without over-announcing.
- [ ] Complex custom widgets follow established ARIA authoring patterns with full keyboard operability and state communication.
- [ ] The interface was tested with real screen readers (NVDA, JAWS, VoiceOver, TalkBack) and ideally with screen reader users.
