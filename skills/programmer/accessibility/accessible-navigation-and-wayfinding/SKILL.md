---
name: accessible_navigation_and_wayfinding.md
description: Use when the agent is designing or reviewing site or application navigation — skip links, landmark regions, heading hierarchy, breadcrumbs, menu and submenu keyboard behavior, single-page-app route change announcements, in-page navigation and anchor scrolling, pagination and infinite scroll, or the overall wayfinding that lets a user understand where they are, where they can go, and how to get back. Also covers the failure modes of keyboard-trapped submenus, headings used for visual sizing, landmarks missing or duplicated, route changes that screen readers do not perceive, and the recurring mistake of treating navigation as a visual layout concern rather than a programmatic structure that assistive technology relies on to orient and move.
---

# Accessible Navigation And Wayfinding

Navigation is how a user answers three questions on every page: where am I, where can I go, and how do I get back. For a sighted user, a glance at the header, the sidebar, and the breadcrumb answers all three in a second. For a screen-reader user, a keyboard-only user, or a user with cognitive load, those answers exist only if the page exposes them programmatically — through landmarks, headings, skip links, and announced state. The judgment problem is that navigation is usually built as a visual artifact (a top bar with logos and links, a sidebar with icons) and then expected to work for assistive technology by osmosis. It does not. A menu that opens on hover is invisible and often unreachable to a keyboard user. A page with no landmark regions forces a screen-reader user to listen to the entire page linearly to find the main content. A single-page-app route change that swaps content without announcing it leaves the screen reader on stale content. Accessible navigation is the discipline of building the orientation and movement structure so that every user can perceive the page's structure, jump to regions, operate menus, and know when the content has changed.

Agents tend to build navigation visually first — matching the design comp — and add accessibility as attributes after, if at all. The harm appears as users who cannot use the product at all. A keyboard user tabs into a submenu and cannot get out (keyboard trap). A screen-reader user lands on a page and has no way to skip the 40-link navigation banner before the main content, so they tab through all of it every page load. A heading hierarchy that uses `<h1>` for the logo, `<h4>` for a section title because "it looked smaller," and skips `<h2>`/`<h3>` breaks the outline a screen-reader user navigates by. A "current page" indicator that is only a visual highlight is not announced, so the user does not know which page they are on. The judgment problem is to treat navigation as a programmatic structure — landmarks that define regions, headings that form an outline, skip links that bypass repetition, and announced state changes — and to design menus, routes, and wayfinding so they are operable and perceivable through every modality.

This skill covers landmarks, skip links, heading hierarchy, menu and submenu keyboard operation, route-change announcement, and current-state indication. It complements the aria-and-semantics skill (ARIA roles and properties) and the keyboard-and-screen-reader-support skill (focus management). Here the focus is the specific structure of orientation and movement: how a user knows where they are and gets where they are going.

## Core Rules

### Provide Landmark Regions So Users Can Jump To Structure

Landmarks (`<header>`, `<nav>`, `<main>`, `<aside>`, `<footer>`, plus `role="search"` and `role="region"` with labels) divide a page into regions that screen-reader users can jump between directly, instead of reading linearly. Without landmarks, every page is an undifferentiated stream of content.

- **Use semantic HTML landmarks, not generic `<div>`s.** `<main>`, `<nav>`, `<aside>`, `<header>`, `<footer>` are automatically exposed as landmarks; `<div class="main">` is not. Prefer the native elements.
- **Have exactly one `<main>` per page.** The main content region is the most important landmark; there should be one, and the skip link should target it.
- **Label navigations when there is more than one.** If a page has multiple `<nav>` regions (primary, footer, in-page table of contents), label each with `aria-label` or `aria-labelledby` so the user can distinguish "Primary navigation" from "Footer navigation." A single nav does not need a label.
- **Do not over-apply `role="banner"`/`role="contentinfo"`.** These are implied by `<header>`/`<footer>` at the top level; adding them redundantly or on nested elements creates noise.

The test: using a screen reader's landmark list, can you jump to main, nav, and footer? If the list is empty or a sea of unlabeled regions, the structure is missing.

### Provide A Skip Link To Bypass Repetitive Navigation

On every page with a repeating header/nav, the first focusable element should be a "Skip to main content" link that jumps focus past the navigation to the `<main>` region. This is a WCAG requirement and one of the highest-impact, lowest-effort accessibility features.

- **The skip link is the first focusable element.** It can be visually hidden until focused (the common pattern: off-screen until `:focus`, then visible). It must be reachable by the first Tab.
- **It targets the main content and moves focus.** The target (`<main>` or its `id`) must receive focus — set `tabindex="-1"` on the target if needed, because some browsers do not move focus to non-interactive elements on anchor click.
- **It actually bypasses the navigation.** A skip link that lands two links into the nav is broken; verify it jumps fully past the repeating chrome to the unique content.

### Build A Correct Heading Outline, Not A Visual Size Scale

Headings (`<h1>` through `<h6>`) form an outline that screen-reader users navigate by — jumping heading to heading to scan a page's structure the way a sighted user scans visually. The outline must reflect the content's structure, not the visual size the designer wanted.

- **One `<h1>` per page, representing the page's main topic.** Using multiple `<h1>`s or none breaks the outline; the `<h1>` should summarize the page (the page title, the main heading).
- **Do not skip heading levels.** Going `<h1>` → `<h4>` because "h4 looked the right size" breaks the outline; screen-reader users jumping by heading expect a logical hierarchy. Use the correct semantic level and control size with CSS.
- **Do not use headings for non-heading text.** A styled "IMPORTANT" callout that is an `<h3>` for its size pollutes the outline. Headings are for section titles; use styled `<p>` or `<div>` for emphasis.
- **Headings describe their section.** A heading like "Section 2" is unhelpful; "Payment methods" tells the user what the section contains. Write headings as content, not placeholders.

### Make Menus And Submenus Fully Keyboard-Operable Without Traps

Dropdown menus, mega-menus, and expandable submenus are common navigation patterns and a common source of keyboard inaccessibility. They must be operable by keyboard alone, with no traps and clear open/close behavior.

- **Do not rely on hover to open or close submenus.** Hover-only menus are unreachable by keyboard and by touch. Open submenus on click, on Enter/Space, or on Down-arrow when focused on the parent (the common accessible pattern).
- **Support arrow-key navigation within menus.** The ARIA Authoring Practices pattern for menus uses Up/Down arrows to move within a menu, Right/Left to enter/exit submenus, and Escape to close. Follow it for application-style menus; for site navigation, a simpler Tab-through-links model is often more usable than fake menu semantics.
- **No keyboard traps.** A user who opens a submenu must be able to close it and leave it (Escape, or Tab past it). A focus that cannot escape is a WCAG failure.
- **Manage focus on open and close.** When a submenu opens, move focus to it (or its first item); when it closes, return focus to the triggering control, so the user is not stranded.
- **Prefer the simplest semantics that work.** A nav of plain links is fully accessible with no ARIA. Reaching for `role="menu"` (which imposes arrow-key expectations and often traps users) when plain links would do makes the nav less accessible, not more.

### Announce Route Changes In Single-Page Applications

In a single-page app, clicking a link does not load a new page — it swaps content via JavaScript. A sighted user sees the content change; a screen-reader user, whose focus and reading position may be elsewhere, perceives nothing unless the change is announced. This is one of the most common and most invisible SPA accessibility failures.

- **Announce the new route's title or heading via a live region.** When the route changes, update an `aria-live="polite"` region (often visually hidden) with the new page's name, so the screen reader announces "Navigated to Account settings."
- **Move focus to the new content.** Move focus to the new page's `<main>` or `<h1>` (with `tabindex="-1"` so it is focusable), so the screen reader begins reading the new page rather than staying on the stale nav link the user clicked.
- **Do not rely on the URL change alone.** The URL changing is not announced; the content change must be explicitly communicated through focus and live region.
- **Update the document title.** The browser tab title should reflect the new route ("Account settings — App"); screen-reader users use it to confirm where they are.

### Indicate The Current Location Programmatically

A sighted user sees the current page highlighted in the nav; a screen-reader user must be told. "Current page" is a state that must be exposed through the accessibility tree.

- **Use `aria-current="page"` on the link to the current page.** This is announced as "current page" and is the correct mechanism for site navigation. Apply it to exactly one primary-nav link.
- **Use `aria-current="location"` for the current step in a breadcrumb or the current item in a set.** Breadcrumbs should mark the current (last) crumb, and the user should hear "current" on it.
- **Do not use `aria-selected` for navigation.** `aria-selected` is for tabs and listboxes with selection semantics; for page navigation, `aria-current` is correct. Misusing `aria-selected` confuses the announced state.

## Common Traps

### No Landmarks, Forcing Linear Reading

A page built from `<div>`s with no `<main>`/`<nav>`/`<aside>`, so a screen-reader user must read the entire page linearly with no way to jump to regions. Use semantic landmark elements and label multiple navs.

### Missing Or Broken Skip Link

No skip link, or a skip link that does not move focus (because the target lacks `tabindex="-1"`), so keyboard users tab through the entire nav on every page. Add a first-focusable skip link targeting focusable main content.

### Headings Used For Visual Sizing

Using `<h4>` for a sidebar title because it "looked right," skipping `<h2>`/`<h3>`, breaking the outline screen-reader users navigate by. Use headings for structure and control size with CSS; never skip levels.

### Hover-Only Submenus

A dropdown menu that opens only on `:hover`, unreachable by keyboard and touch, and a keyboard trap once a submenu is somehow entered. Open on click/Enter, support Escape to close, and return focus to the trigger.

### SPA Route Change Not Announced

A route change that swaps content with no announcement and no focus move, so a screen-reader user is stranded on stale content while the sighted user sees a new page. Announce via live region and move focus to the new content's heading.

### Current Page Indicated Only Visually

A nav link highlighted with color/background but no `aria-current="page"`, so the screen-reader user does not know which page is active. Mark the current link with `aria-current="page"`.

### Over-Applying `role="menu"` To Plain Navigation

Turning a simple link nav into `role="menubar"`/`role="menu"` with arrow-key expectations that trap or confuse keyboard users, when plain links would be fully accessible. Use the simplest semantics that work.

## Self-Check

- [ ] The page has semantic landmark regions (`<header>`, `<nav>`, `<main>`, `<aside>`, `<footer>`), exactly one `<main>`, and multiple `<nav>` regions are labeled with `aria-label`/`aria-labelledby` so they are distinguishable in a landmark list.
- [ ] A "Skip to main content" link is the first focusable element, targets the `<main>` region, moves focus to it (target has `tabindex="-1"` if needed), and fully bypasses the repeating navigation.
- [ ] The heading outline is correct: one `<h1>` representing the page topic, no skipped levels, headings used for section structure (not visual sizing or emphasis), and each heading describes its section.
- [ ] Menus and submenus are fully keyboard-operable: not hover-only, support open (click/Enter/arrow) and close (Escape), return focus to the trigger on close, have no keyboard traps, and use the simplest semantics (plain links where possible rather than `role="menu"`).
- [ ] SPA route changes are announced via a live region and accompanied by a focus move to the new content's heading/main, and the document title updates to reflect the new route.
- [ ] The current page/location is indicated programmatically with `aria-current="page"` (or `aria-current="location"` for breadcrumbs/steps), applied to exactly one item, and not conflated with `aria-selected`.
- [ ] The highest-risk cases were verified — a screen-reader user jumping by landmark and heading to find content, a keyboard user opening and closing a submenu without trapping, a SPA route change being announced and focused, and the current page being identified non-visually — not only the sighted, mouse-driven navigation.
