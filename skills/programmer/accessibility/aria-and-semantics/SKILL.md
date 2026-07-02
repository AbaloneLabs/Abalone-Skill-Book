---
name: aria_and_semantics.md
description: Use when the agent is choosing between native semantic HTML and ARIA for a widget, adding or reviewing ARIA roles, states, or properties, building custom interactive components (tabs, comboboxes, menus, treegrids, accordions), exposing dynamic state to assistive technology, naming interactive elements, relating controls to the regions they affect, structuring landmarks, or deciding whether a given ARIA attribute helps or actively harms screen-reader users. Also covers the first rule of ARIA, landmark roles, accessible names, aria-label versus aria-labelledby versus visible text, aria-live and dynamic announcements, and the failure mode of relabeling the accessibility tree without providing the matching behavior.
---

# ARIA And Semantics

ARIA (Accessible Rich Internet Applications) is a set of attributes that let you adjust the *names, roles, states, and properties* the browser exposes to assistive technology through the accessibility tree. It does not add behavior, focus, or keyboard handling. This is the single most misunderstood fact about ARIA, and it is the source of most ARIA defects: a developer adds `role="button"` to a `<div>` and concludes it is now accessible, when in fact it is merely *announced* as a button while remaining non-focusable, non-keyboard-operable, and missing every behavior a real button provides. The accessibility tree now lies to the screen-reader user. The first rule of ARIA exists for this reason: no ARIA is better than bad ARIA.

Agents tend to reach for ARIA too readily because it feels like the "accessibility thing to do," and because adding attributes is easier than restructuring markup. The harm is that misused ARIA actively degrades the experience: a `role="tab"` on an element that does not behave like a tab tells the screen-reader user to expect arrow-key navigation that never comes; an `aria-label` that overrides visible text removes the visible label from the accessible name; a redundant `role="button"` on a `<button>` can conflict with the native semantics. The judgment problem is knowing when native semantics already solve the problem (almost always), when ARIA is genuinely needed to fill a gap (custom widgets, dynamic state, relationships), and how to apply it so the accessibility tree reflects the truth rather than a hopeful relabeling.

## Core Rules

### Use Semantic HTML First, And Reach For ARIA Only To Fill Gaps

The browser builds the accessibility tree primarily from native semantic elements. A `<button>` is exposed with role `button`, a name derived from its content, focusable by default, operable with Enter and Space, and announced correctly — all for free. A `<nav>` is a navigation landmark; a `<main>` is the main landmark; a `<label for>` associates with its input; a `<th scope>` relates a header cell to its row or column. These native semantics are correct, complete, and require no ARIA.

The decision process for every interactive element:

- **Ask first: what native element already does this?** Button, link, checkbox, radio, select, textarea, dialog, details/summary, fieldset/legend, table with real headers. If a native element matches the intent, use it. Its semantics, keyboard behavior, and AT exposure are all correct by default.
- **Reach for ARIA only when no native element expresses the pattern.** Tabs, comboboxes, treegrids, menus, live regions, and complex disclosure are cases where native elements do not yet cover the pattern completely. Here ARIA is necessary.
- **Never use ARIA to override working native semantics.** `role="button"` on a `<button>` is redundant and can conflict; `role="link"` on a `<button>` lies about the behavior (links navigate, buttons trigger actions). If the native semantics are correct, leave them alone.

The cost of a custom widget built from `div`/`span` is not the markup — it is the obligation to recreate, via ARIA and JavaScript, the full role/name/state/keyboard contract of the native equivalent. Each piece you recreate is a place to be wrong. The element whose built-in behavior matches the intent is almost always the right choice.

### Every ARIA Role Must Come With Its Full Keyboard Contract

ARIA roles describe what an element *is* to assistive technology, and screen readers communicate the expected interaction model for that role to the user. A `role="tab"` tells the user "use arrow keys to move between tabs"; a `role="menuitem"` tells the user "this is part of a menu navigated with arrows"; a `role="button"` tells the user "activate with Enter or Space." If the element does not actually implement that keyboard behavior, the screen-reader user follows the announced contract and nothing works.

- **Consult the ARIA Authoring Practices (APG) for each pattern.** The APG defines the keyboard interaction, focus management, and ARIA attributes for every common widget pattern (tabs, combobox, menu, tree, grid, accordion). Treat it as the contract.
- **Implement the full contract, not just the roles.** A tablist needs arrow-key navigation between tabs, the active tab in the tab order (roving tabindex), `aria-selected` on the active tab, `aria-controls`/`aria-labelledby` relating tabs to panels, and the panel visibility actually following the selected tab. Roles alone are half a widget.
- **Half a widget is a broken widget.** Correct roles with missing key handling is worse than a native element, because it promises behavior it does not deliver. If you cannot implement the full contract, use a simpler native pattern (e.g., disclosure via `<details>` instead of a custom tabset) or a tested library.

### Expose Dynamic State Through ARIA, Not Through Visual-Only Signals

Native elements expose some state automatically (a checked checkbox announces as checked; a disabled button is announced as disabled and excluded from the tab order). Custom widgets and dynamic interfaces need ARIA to expose state that the screen-reader user cannot see:

- **`aria-expanded`** on a trigger that toggles a panel (accordion, dropdown, disclosure): announces whether the controlled content is currently open or closed.
- **`aria-pressed`** on a toggle button: announces whether it is currently in the pressed/active state.
- **`aria-selected`** on a selectable item (tab, option, tree node): announces the current selection.
- **`aria-checked`** on a custom checkbox or menuitemcheckbox: announces the checked state.
- **`aria-invalid`** on a form field with an error: announces the field has an error (paired with `aria-describedby` pointing at the error message).
- **`aria-busy`** on a region actively loading: tells the AT to wait, reducing confusing intermediate announcements.
- **`aria-hidden`** on content that should be removed from the accessibility tree (decorative duplicates, off-screen content) — but never on a focusable element.

For each dynamic state change a sighted user would perceive visually, ask: how does the screen-reader user know this changed? If the answer is "they don't," add the ARIA state attribute and ensure it updates when the state changes.

### Name Every Interactive Element With A Meaningful Accessible Name

Every interactive element needs an accessible name — the text the screen reader announces as the element's label. Native elements derive their name from their content (button text, link text), an associated `<label>`, or `alt` text. Custom elements and icon buttons need a name provided deliberately, or they are announced as generic "button" with no indication of what they do.

- **Prefer a visible text label.** A button that says "Save" is clearest for everyone. An icon-only button needs its purpose conveyed through an accessible name.
- **`aria-label`** provides a name string directly (`aria-label="Search"`). Use it when a visible label is impossible (icon buttons) and no visible text exists to reference.
- **`aria-labelledby`** references the id of visible text that labels the element. Prefer this over `aria-label` when visible text exists, because it keeps the name in sync with the visible label automatically and avoids duplication that drifts.
- **`aria-describedby`** references supplemental description (a hint, an error message, formatting instructions) that follows the name in announcement.
- **Do not override a meaningful visible name with a worse `aria-label`.** If a button contains visible text "Submit order," an `aria-label="Button"` replaces the good name with a useless one. Only add `aria-label` when it improves on what the element would otherwise announce.

The test: with a screen reader, navigate to each interactive element and confirm the announced name tells the user what the element does. An icon button announced as "button" has failed.

### Relate Elements With ARIA When The Relationship Is Not In The DOM

Native HTML expresses some relationships structurally: a `<label for>` relates a label to its input; `<th scope>` relates a header to its cells; a `<fieldset>` groups related inputs. When the relationship is real but not expressible in the DOM structure (a trigger controls a panel elsewhere in the DOM, a heading labels a region, an input is described by text in a different container), ARIA provides the relationship:

- **`aria-controls`** — this element controls the element with the given id (a tab controls its panel, a button toggles a region). Helps the AT user understand the connection.
- **`aria-labelledby`** — the element with the given id labels this element (a section labelled by its heading).
- **`aria-describedby`** — the element with the given id describes this element (supplemental description, error text).
- **`aria-owns`** — this element owns the element with the given id as a child for the accessibility tree, when the DOM structure cannot express the parent-child relationship. Use sparingly; restructuring the DOM is usually better.

Use these when the relationship is real and the DOM does not express it. Do not add them decoratively — `aria-controls` pointing at an unrelated element confuses more than it helps.

### Use Landmarks To Structure The Page For Navigation

Screen-reader users navigate pages by landmarks — regions like header, navigation, main, complementary (aside), search, form, contentinfo (footer). Native HTML elements (`<header>`, `<<nav>`, `<main>`, `<aside>`, `<footer>`, `<search>`) create these landmarks automatically. ARIA landmark roles (`role="banner"`, `role="navigation"`, etc.) achieve the same when native elements cannot be used.

- **Prefer native landmark elements.** They provide the landmark and the semantics without ARIA.
- **Label multiple landmarks of the same type.** If a page has two `<nav>` elements (primary navigation and footer navigation), give each an `aria-label` or `aria-labelledby` so the screen reader distinguishes "Primary navigation" from "Footer navigation." Without labels, the user hears "navigation" twice and cannot tell them apart.
- **Have exactly one `main` landmark** containing the page's primary content. This is the target of "skip to main content" and the primary navigation destination.

Landmarks let screen-reader users jump between regions instead of reading linearly. A page with no landmarks is an undifferentiated wall of content to a screen-reader user.

### Respect The First Rule Of ARIA: No ARIA Is Better Than Bad ARIA

The governing principle: ARIA that misrepresents the element is worse than no ARIA, because it actively lies to the assistive technology. Before adding any ARIA, confirm it reflects the truth:

- Does the element actually behave as the `role` claims? (A `role="button"` must be focusable, operable with Enter/Space, and trigger an action.)
- Is the `aria-label` more accurate than what the element would announce without it?
- Does the `aria-expanded`/`aria-pressed`/`aria-selected` actually update when the state changes?
- Does the `aria-controls` target actually get controlled?

If any answer is no, the ARIA is harming the user. Remove it or fix the underlying behavior. A native element with no ARIA is correct by default; a custom element with lying ARIA is a defect.

## Common Traps

### `role="button"` On A `<div>` Without The Rest Of The Contract

Adding `role="button"` and `tabindex="0"` to a `<div onclick>` and concluding it is accessible. It announces as a button but is not operable with Space, does not handle disabled state, and is not exposed as a button to other AT behaviors. Use a `<button>`; if you truly cannot, implement the full button contract (focus, Enter+Space, disabled handling).

### Redundant Or Conflicting ARIA On Native Elements

`role="button"` on a `<button>`, or `role="link"` on a `<button>`. The native semantics are already correct; the ARIA is redundant at best and conflicting at worst (overriding the native role). Leave native semantics alone.

### `aria-label` Overriding A Good Visible Name

A button with visible text "Submit" given `aria-label="btn"`, replacing the clear name with a useless one. Only add `aria-label` when it improves on the element's existing name; never override a meaningful visible label with a worse string.

### Roles Without The Keyboard Behavior

A `role="tab"` without arrow-key navigation, or `role="menu"` without the menu keyboard model. The screen-reader user is told to use arrows and nothing happens. Every ARIA role carries a keyboard contract from the APG; implement it fully or do not use the role.

### Dynamic State Attributes That Never Update

`aria-expanded="false"` on a toggle that opens a panel, but the attribute is never set to `true` when the panel opens. The screen-reader user is told the panel is always closed. Wire state attributes to update on every state change.

### `aria-hidden` On A Focusable Element

Applying `aria-hidden="true"` to an element that contains focusable children (or is itself focusable), which removes it from the accessibility tree but leaves it in the tab order — a keyboard user Tabs into content the screen reader says does not exist. Never `aria-hide` a focusable element; remove it from the tab order or do not hide it.

### Unlabelled Same-Type Landmarks

Two `<nav>` elements with no distinguishing `aria-label`, so the screen-reader user hears "navigation" twice and cannot tell which is which. Label landmarks when there is more than one of the same type.

### Trusting ARIA Without Screen-Reader Verification

Writing correct-looking ARIA and concluding the widget is accessible. ARIA support and behavior differ across screen-reader/browser combinations; a `aria-live` region that works in one may be silent in another. Verify with a real screen reader through the real workflow.

### Overusing `aria-live` As A Catch-All

Slapping `aria-live="polite"` on large containers to "announce changes," producing a stream of irrelevant updates. Scope live regions narrowly to the specific message element.

## Self-Check

- [ ] Native semantic HTML is used wherever a native element matches the intent; ARIA is used only to fill gaps (custom widgets, dynamic state, relationships not expressible in the DOM) — no `role="button"`/`role="link"` on `<div>` where a native element works, and no redundant ARIA overriding correct native semantics.
- [ ] Every ARIA role comes with its full keyboard and state contract per the APG (arrow-key navigation for tablists/menus, Enter/Space for buttons, focus management for dialogs) — no half-widgets with roles but missing behavior.
- [ ] Dynamic state is exposed via ARIA (`aria-expanded`, `aria-pressed`, `aria-selected`, `aria-checked`, `aria-invalid`, `aria-busy`) and the attributes update on every state change so the screen reader reflects the current state.
- [ ] Every interactive element has a meaningful accessible name (visible text preferred, `aria-labelledby` referencing visible text next, `aria-label` only when no visible text exists), and no good visible name is overridden by a worse `aria-label`.
- [ ] Relationships not in the DOM are expressed with ARIA (`aria-controls`, `aria-labelledby`, `aria-describedby`) only when the relationship is real, and the targets exist and are correct.
- [ ] The page uses native landmark elements (`<header>`, `<nav>`, `<main>`, `<aside>`, `<footer>`, `<search>`) with exactly one `<main>`, and same-type landmarks are distinguished with `aria-label`/`aria-labelledby`.
- [ ] No `aria-hidden` is applied to a focusable element or a container of focusable elements — hidden content is either fully removed from the tab order or not `aria-hidden`.
- [ ] The first rule of ARIA was respected: every ARIA attribute reflects the truth (the role matches behavior, the state updates, the controls target is real), and ARIA that misrepresents the element was removed rather than shipped.
- [ ] Verification included a real screen reader (NVDA/JAWS/VoiceOver/TalkBack) confirming the announced names, roles, states, and relationships match the actual behavior, not only a markup review.
