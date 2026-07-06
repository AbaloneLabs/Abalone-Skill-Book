---
name: keyboard_and_screen_reader_support.md
description: Use when the agent is building or reviewing interactive UI that must work without a mouse, managing keyboard focus and tab order, wiring skip links, avoiding keyboard traps, making dynamic content perceivable to screen readers, choosing what to announce via live regions, handling focus return after closing dialogs or menus, routing focus on single-page-app navigation, testing with NVDA, JAWS, VoiceOver, or TalkBack, or deciding how hidden content should behave for assistive technology. Also covers roving tabindex for composite widgets, focus visibility, reading order versus DOM order, the difference between visually-hidden and display-none, and the failure mode of an interface that is mouse-usable but strand keyboard and screen-reader users.
---

# Keyboard And Screen Reader Support

Keyboard and screen-reader support is the discipline of making an interface perceivable and operable for users who do not use a mouse and who do not see the screen. These are two overlapping but distinct populations: keyboard-only users (including switch-device and voice-control users, who both navigate by keyboard semantics) rely on a logical tab order and operable keys; screen-reader users rely on the accessibility tree, focus movement, and announced changes to know what is on the page and what just happened. An interface that works under a mouse often fails both groups silently, because the mouse user sees the whole screen at once and can point anywhere, while the keyboard and screen-reader user experiences the interface serially — one focusable element or one announced region at a time.

Agents tend to miss this because the default testing posture (sighted, mouse, fast browser) never exercises these paths. The result is an interface where a modal opens and focus stays at the top of the page, where a dropdown menu traps the keyboard, where a toast appears but is never announced, where a single-page-app route change is invisible to a screen reader, or where the tab order jumps around the visual layout. Each of these is a concrete exclusion: a user cannot complete the task, or completes it without knowing whether it succeeded. The judgment problem is deciding, for every dynamic change and every custom widget, what a keyboard user and a screen-reader user must experience — and then verifying it with a real screen reader, not by assuming the markup is correct.

## Core Rules

### Make The Tab Order Match The Visual Reading Order, And Make It Complete

Keyboard users move through an interface with Tab and Shift+Tab, and the order in which focus lands must match the order in which a sighted user would read. The browser derives tab order from DOM order by default, so the most reliable way to get a correct tab order is a correct DOM order. When visual layout diverges from DOM order (via CSS grid/flex reordering, absolute positioning, or `order`), a keyboard user experiences a jarring jump that breaks their mental model.

- **Prefer DOM order that matches the visual order.** If you must reorder visually with CSS, verify the tab order still reads logically by tabbing through. `tabindex` with positive values is almost always wrong — it forces a global ordering that fights the DOM and breaks as the page changes.
- **Every interactive element must be reachable.** A control that is visible and mouse-clickable but not in the tab order is inaccessible to keyboard users. Native interactive elements (`button`, `a`, `input`, `select`, `textarea`) enter the tab order by default; custom widgets built from `div`/`span` need `tabindex="0"` (or roving `tabindex`) to be reachable.
- **Remove non-interactive elements from the tab order.** Applying `tabindex="0"` to a `div` that is not actually interactive creates a stop that does nothing, confusing the keyboard user. Reserve tab stops for elements the user can operate.

The test is simple and mandatory: unplug the mouse and Tab through the entire interface. If you cannot reach and operate every control, in an order that makes sense, the keyboard access is incomplete.

### Implement Roving Tabindex For Composite Widgets

Composite widgets — menus, tablists, toolbars, radiogroups, tree views — should not put every child in the tab order. A menu with twenty items, each `tabindex="0"`, forces the keyboard user to Tab twenty times to get past it, which is exhausting and breaks the "one Tab, one region" model. The correct pattern is roving tabindex: one element in the composite has `tabindex="0"` (the active child), the rest have `tabindex="-1"`, and arrow keys move both focus and the `tabindex="0"` assignment among the children.

- **One Tab stop per composite.** Tab enters the widget at the active child; Tab again leaves the widget. This mirrors how native selects and menus behave.
- **Arrow keys move within the composite.** The widget handles arrow keys to move focus between children, following the ARIA Authoring Practices keyboard contract for that pattern (arrows for menus and tabs, arrow or tab for grids, etc.).
- **`tabindex="-1"` makes an element focusable programmatically but not via Tab.** This is how inactive children become arrow-key-focusable without entering the tab order.

Roving tabindex is the difference between a custom menu that behaves like a menu and one that behaves like a wall of links. If you build a composite widget, implement roving tabindex; do not default to `tabindex="0"` on every child.

### Never Create A Keyboard Trap, And Always Return Focus Deliberately

A keyboard trap is any state from which a keyboard user cannot escape using the keyboard. The classic case is a modal dialog that receives focus but has no Escape handler and no way to Tab out — the user is stuck. But traps also occur in embedded content (an iframe video player with no keyboard exit), in menus that open but cannot be closed by keyboard, and in components that capture focus and never release it.

- **Every component that captures focus must release it.** A modal must close on Escape and return focus to the element that opened it. A menu must close on Escape, Tab-out, or outside click and return focus to its trigger.
- **Trap focus *within* a modal while it is open, but not beyond it.** While a dialog is open, Tab should cycle among the dialog's focusable elements, not escape to the page behind (which is obscured). When the dialog closes, focus returns to the trigger — not to the top of the page, which strands the user.
- **Test for traps explicitly.** Tab through the interface and confirm you can always reach the browser address bar or the page content; if Tab cycles forever inside a component with no exit, it is a trap.

Focus return is the other half. When a dialog, menu, or panel closes, the user was somewhere before it opened; returning them to that element preserves their context. Leaving focus at the top of the page after a modal closes forces a screen-reader user to re-navigate to where they were.

### Announce Dynamic Changes Through Live Regions, Scoped Narrowly

When content changes without a page load — search results filter, a save succeeds, an error appears, a cart updates — a sighted user sees the change; a screen-reader user perceives nothing unless the change is announced. Live regions (`aria-live`) are the mechanism, and their misuse is one of the most common screen-reader defects.

- **Choose `polite` by default, `assertive` only for urgent interruptions.** `polite` announces when the screen reader is idle, so it does not interrupt the user; `assertive` interrupts immediately and should be reserved for errors or urgent status that must be heard now. Overusing `assertive` makes every minor update interrupt the user, which is disorienting.
- **Scope the live region to the specific message, not a large container.** Putting `aria-live="polite"` on a results container announces every minor DOM change inside it, producing noise. Point the live region at (or use a dedicated) small status element whose text changes to the message you want announced ("3 results found", "Saved", "Error: email is required").
- **Test what is actually announced.** Live region behavior differs across screen readers and browsers; an `aria-live` that works in NVDA+Firefox may behave differently in VoiceOver+Safari. Verify the announcement with a real screen reader, not by reading the markup.

The default question for every async update: if a sighted user would notice this change, what should the screen-reader user hear? If you have no answer, the change is invisible to them.

### Move Focus On SPA Navigation And Major View Changes

In a traditional multi-page app, navigation reloads the page and the screen reader announces the new page. In a single-page app, a route change updates the DOM but the screen reader has no signal that navigation occurred — it stays where it was, silently. The fix is deliberate focus management on view change.

- **Move focus to the new view's main heading or a dedicated announcement element** on route change. This tells the screen reader "you are somewhere new" and gives the user the new context. Focus a heading (`h1`) or an element with `tabindex="-1"` set as the announcement target.
- **Do not rely on the URL change alone.** The screen reader does not announce URL changes; it announces focus movement and live-region updates. Without one of those, navigation is invisible.
- **Announce the new page's purpose, not just that it changed.** Moving focus to an `h1` that says "Order History" tells the user where they are; moving focus to a generic container tells them nothing.

The same applies to major in-page view swaps (switching tabs in a tabbed interface that swaps content, opening a full-screen panel). If the user's context changed substantially, move focus so they can perceive it.

### Provide Skip Links And Logical Landmark Structure

Keyboard users navigating a large page Tab through every focusable element in order. On a site with a global nav of fifty links, that means fifty Tab presses to reach the main content on every page. A skip link — the first focusable element, typically "Skip to main content" — lets them jump past repetitive navigation.

- **Place the skip link first in the DOM**, visible on focus (it can be visually hidden until focused). It must be the first Tab stop.
- **Target the main content region**, typically a `<main>` element with `id` and `tabindex="-1"` (so it is focusable as a target).
- **Use landmarks (`header`, `nav`, `main`, `aside`, `footer`, `search`) to structure the page.** Screen-reader users navigate by landmarks; a page with proper landmarks lets them jump to "main" or "navigation" directly, which is the structural equivalent of a skip link for every region.

Landmarks and skip links together give keyboard and screen-reader users efficient navigation of a complex page. Without them, every page is a linear Tab-through of everything.

### Hide Content With The Right Technique For The Right Audience

"Hiding" content has three distinct meanings, and using the wrong technique breaks accessibility:

- **Hide from everyone (visual and AT).** Use `display: none` or `hidden`. The content is removed from the accessibility tree and not announced. Correct for content that is genuinely inactive.
- **Hide visually, keep for screen readers.** Use the visually-hidden pattern (clip/clip-path with fixed size and overflow hidden). The content is announced by screen readers but not shown. Correct for labels or context that sighted users get visually but screen-reader users need announced (e.g., an icon button whose meaning is clear visually but needs a text label for AT).
- **Never leave a focusable element visually hidden.** An element with `tabindex` or a native interactive role that is hidden via `opacity: 0` or `visibility` tricks but still focusable creates a "ghost" focusable — a keyboard user Tabs into invisible space. If it is focusable, it must be visible (or removed from the tab order with `tabindex="-1"` and `display:none`).

The trap is reaching for `display: none` for content that should be available to screen readers (it hides it from them too), or leaving visually-hidden-but-focusable elements that strand keyboard users.

## Common Traps

### Positive Tabindex To "Fix" Order

Using `tabindex="1"`, `tabindex="2"` etc. to force a specific tab order. Positive tabindex creates a global ordering that overrides DOM order, breaks as the page changes, and is almost always a sign the DOM order is wrong. Fix the DOM order; avoid positive tabindex entirely.

### Modal That Traps Focus And Never Returns It

A dialog that receives focus but has no Escape handler, or that closes and leaves focus at the top of the page. The keyboard user is either stuck or stranded. Every focus-capturing component needs a keyboard exit and deliberate focus return to the trigger.

### Live Region On A Huge Container

Putting `aria-live="polite"` on a results list or a large panel so every internal DOM change is announced, producing a stream of irrelevant interruptions. Scope live regions to a dedicated status element that carries exactly the message to announce.

### Assertive Everywhere

Using `aria-live="assertive"` for routine status because it "makes sure they hear it." Assertive interrupts the user mid-reading; reserve it for genuine errors or urgent interruptions. Default to polite.

### SPA Route Change With No Focus Movement

A single-page-app navigation that updates the DOM and the URL but moves no focus and announces nothing. The screen-reader user is stranded on the old context. Move focus to the new view's heading or an announcement target on every route change.

### Custom Widget With `tabindex="0"` On Every Child

A menu or tablist where every item is `tabindex="0"`, forcing the keyboard user to Tab through all of them. Implement roving tabindex so the composite is one Tab stop with arrow-key navigation inside.

### Visually-Hidden But Still Focusable

An element hidden with `opacity: 0` or a visibility trick that remains in the tab order, so a keyboard user Tabs into invisible space. If it is focusable it must be visible; if it is hidden, remove it from the tab order.

### Assuming The Markup Is Enough Without Testing and no Skip Link On A Nav-Heavy Page

Writing correct `aria-live`, `tabindex`, and focus management and concluding it works, without running a screen reader. Live-region timing, focus behavior, and announcement differ across screen-reader/browser combinations. Verify with at least one real screen reader through the real workflow.

A site with a large global navigation and no skip link, forcing keyboard users to Tab through dozens of links on every page to reach content. Add a skip link as the first focusable element targeting the main content.

## Self-Check

- [ ] The tab order matches the visual reading order (DOM order is correct, no positive `tabindex`), and every interactive element is reachable via Tab/Shift+Tab — verified by tabbing through the whole interface with the mouse unplugged.
- [ ] Composite widgets (menus, tablists, toolbars) use roving tabindex so they are one Tab stop with arrow-key navigation inside, not a wall of individual Tab stops.
- [ ] No keyboard traps exist: every component that captures focus (modals, menus, embedded players) has a keyboard exit (Escape, Tab-out) and returns focus to the triggering element on close — verified by attempting to escape every focus-capturing component by keyboard.
- [ ] Dynamic changes are announced via narrowly-scoped live regions (`polite` by default, `assertive` only for urgent interruptions) pointed at a specific status element, and the announcement was verified with a real screen reader rather than assumed from the markup.
- [ ] Single-page-app route changes and major view swaps move focus to the new view's heading or an announcement target, so screen-reader users perceive the navigation.
- [ ] A skip link is the first focusable element, targeting the main content, and the page uses landmarks (`main`, `nav`, etc.) to enable efficient navigation.
- [ ] Content is hidden with the correct technique for the audience: `display:none`/`hidden` for content removed from everyone, visually-hidden for screen-reader-only text, and no focusable element is left visually hidden (no ghost focus targets).
- [ ] Focus is visible at all times — no `:focus`/`:focus-visible` outline removed without an equally visible replacement — so a keyboard user can always see where they are.
- [ ] Verification included at least one major screen reader (NVDA or JAWS on Windows, VoiceOver on macOS/iOS, TalkBack on Android) through the core workflow, not only a markup review or an automated scan.
