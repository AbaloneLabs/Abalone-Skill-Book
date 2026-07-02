---
name: accessibility_and_wcag_compliance.md
description: Use when the agent is building or reviewing HTML, web components, forms, interactive widgets, dynamic UI, media players, or navigation; adding ARIA roles, states, or properties; managing keyboard focus and tab order; choosing colors, contrast, or text sizing; writing alt text, captions, or transcripts; building modals, menus, tabs, accordions, carousels, or drag-and-drop; handling screen-reader announcements and live regions; designing for users with visual, motor, cognitive, or auditory needs; or aiming to meet WCAG, ADA, Section 508, EN 301 549, or similar accessibility obligations. Also covers semantic HTML, focus management, color-contrast and not-color-only information, reduced-motion and prefers-reduced-motion, form labeling and error association, automated-tool limitations, and the difference between passing an audit and being usable by real assistive-technology users.
---

# Accessibility And WCAG Compliance

Accessibility is the judgment of whether an interface can be used by everyone, including people who do not see it, cannot use a mouse, cannot hear it, cannot perceive fine motion, or need more time and clarity than the default design assumes. It is not a checklist of attributes added at the end. Every structural decision — which element carries a meaning, where focus goes after a dialog closes, whether an error is conveyed by color alone, whether a status change is announced — is an accessibility decision, whether or not it was made deliberately.

Agents tend to under-invest here for a concrete reason: the interface works for the agent's own assumptions (sighted, mouse, fast connection, modern browser), automated scanners report zero obvious errors, and the harm is invisible to the team because the people harmed are not in the room. The harm is real and often legal: users cannot complete purchases, fill prescriptions, submit applications, or use core features; organizations face WCAG / ADA / Section 508 / EN 301 549 obligations and, in many jurisdictions, lawsuits for public-facing barriers. The judgment problem is deciding, for each interactive element and each piece of information, what a user who interacts differently needs in order to perceive, operate, and understand it — and verifying that with real assistive technology, not only with a tool that checks HTML attributes.

## Core Rules

### Treat WCAG's Four Principles As The Frame, Not A Scorecard

WCAG organizes requirements under four principles, and each maps to a concrete failure class. Use them as the questions you ask about every component, not as labels to claim compliance:

- **Perceivable.** Can the information be perceived by someone who cannot see it, hear it, or perceive its visual encoding? This covers text alternatives, captions, contrast, resizable text, and not relying on a single sense.
- **Operable.** Can the interface be operated by someone who cannot use a mouse, cannot move precisely, or needs more time? This covers keyboard access, focus visibility, no keyboard traps, enough time, and no seizure-inducing motion.
- **Understandable.** Can a user understand both the content and how to operate it? This covers readable language, predictable navigation, consistent labeling, and error prevention/recovery on inputs.
- **Robust.** Can the interface be interpreted reliably by the assistive technologies and browsers users actually have? This covers valid, semantic markup and correct ARIA so screen readers and other tools can expose the right role, name, state, and value.

A component that is "technically compliant" but unusable with a real screen reader fails the principle even if it passes a scan. Optimize for the user outcome each principle names, then verify with WCAG success criteria as the measurable test.

### Use Semantic HTML As The Primary Accessibility API

The browser exposes an accessibility tree derived from the DOM, and assistive technology reads that tree. Native semantic elements (`button`, `a`, `nav`, `main`, `label`, `fieldset`, `dialog`, `table` with real headers) produce correct role, name, state, value, and keyboard behavior for free. A `<button>` is focusable, activatable with Enter and Space, announced as a button, and operable without any extra code. A `<div onclick>` is none of those things.

The rule that prevents most defects: reach for the element whose built-in semantics and behavior match the intent before reaching for ARIA. Ask, for each interactive thing, "what native element already does this?" If a native element exists (button, link, checkbox, radio, select, tablist via disclosure, dialog), use it. Custom widgets built from `div`/`span` require you to manually recreate focusability, key handling, role, state, and ARIA — and each recreated piece is a place to be wrong. The cost of a custom widget is not the markup; it is the obligation to match the full keyboard and AT contract of the native equivalent.

### Apply ARIA Only To Fill Gaps, And Never To Override Working Semantics

The first rule of ARIA is "no ARIA is better than bad ARIA." ARIA does not add behavior; it only relabels the accessibility tree. A `<div role="button">` is announced as a button but is still not focusable or keyboard-operable unless you add `tabindex` and key handlers yourself. Misused ARIA is worse than missing ARIA because it actively lies to assistive technology.

Use ARIA when:

- No native element expresses the pattern (tabs, combobox, treegrid, live regions, custom disclosure where a `<details>` is insufficient).
- You must convey a dynamic state a native element does not expose (`aria-expanded`, `aria-pressed`, `aria-selected`, `aria-busy`, `aria-invalid`).
- You must relate elements (`aria-labelledby`, `aria-describedby`, `aria-controls`, `aria-owns`).

Avoid ARIA when:

- A native element already provides the semantics (`role="button"` on a `<button>` is redundant and sometimes harmful).
- You are tempted to repurpose semantics (`role="tab"` on something that does not behave like a tab).
- The name or state could be conveyed by visible text or a real label instead.

When you do use ARIA, every role must come with the keyboard contract the ARIA Authoring Practices define for that pattern. Half a widget — correct roles but missing key handling — is a broken widget.

### Guarantee Keyboard Operability And Visible Focus

Many users cannot use a pointing device: keyboard-only users, switch-device users, voice-control users, and screen-reader users all navigate by keyboard. Operability has two parts that must both hold:

- **Logical, complete keyboard access.** Every interactive element must be reachable via Tab (and Shift+Tab), and operable via the expected keys (Enter/Space for buttons and links, arrow keys for composite widgets like menus and tabs, Escape to close dialogs and menus). Define the reading/focus order to follow the visual order; do not rely on DOM order that differs from what users see.
- **Visible focus.** A user navigating by keyboard must be able to see where focus is at all times. Never remove `:focus`/`:focus-visible` outlines without providing an equally visible alternative. Hiding focus "because it looks cleaner" removes the only indicator a keyboard user has.

Specifically avoid **keyboard traps**: a component (modal, menu, embedded video) must release focus back to the page when closed or escaped. Test by tabbing through the whole interface with a mouse unplugged. If you cannot reach and operate every control, the interface is not operable.

### Manage Focus Deliberately In Dynamic Interfaces

When content changes without a full page load (SPAs, modals, inline updates, route changes, toast notifications), focus and announcement do not happen automatically the way they do on navigation. You must decide, for each state change, what a screen-reader and keyboard user experiences:

- **Opening a modal/dialog.** Move focus into the dialog (usually to the first focusable element or the heading), trap focus within it while open, and return focus to the triggering element on close. A dialog that closes and leaves focus at the top of the page strands the user.
- **Single-page route changes.** Announce the new view by moving focus to its heading or an announcement region; without this, a screen-reader user perceives no navigation occurred.
- **Inline content updates** (search results, filtering, async data). Use an `aria-live` region to announce meaningful changes ("3 results found", "Saved", "Error: email is required") so the change is perceivable without focus moving.
- **Revealing/hiding content** (accordions, tabs, menus). Move focus only when it helps the user; for tabs, move focus to the new panel's content; for an accordion that the user just opened, keep focus on the trigger.

Default to "what would a sighted user perceive here?" and then provide the non-sighted equivalent through focus movement or live-region announcement.

### Never Convey Information By Color Alone

Color is one encoding channel. Roughly 1 in 12 men and 1 in 200 women have some form of color vision deficiency, and color-only signals are also invisible to users on monochrome displays, in high-contrast mode, or using screen readers. Any state that is shown only by a color change is invisible to some users.

For each status, error, or data encoding, provide a second channel:

- **Form errors.** Red border plus an icon, plus text ("Email is required"), plus `aria-invalid` and `aria-describedby` linking to the message.
- **Charts and data viz.** Color plus shape, pattern, or direct labels; do not rely on a color legend alone.
- **Status indicators.** Color plus an icon or text label (a green dot plus a checkmark, or "Active" text), not a green dot alone.
- **Links in text.** Underline or another differentiator, not color alone — many users cannot distinguish link color from body text color.

### Meet Contrast And Text-Resizability Requirements

Text must be legible at the sizes and conditions users actually encounter, including low vision, glare, and small screens. WCAG contrast ratios (4.5:1 for normal text, 3:1 for large text and for meaningful graphics/UI components) are the floor, not the goal. Check contrast for text over images and gradients, not only solid backgrounds, because the effective contrast varies across the image. When text sits over an image, add a scrim, overlay, or solid background to guarantee contrast everywhere the text appears.

Support text resizing: do not use fixed pixel font sizes that block zoom, do not disable pinch-zoom on mobile (`user-scalable=no` or `maximum-scale` is an accessibility barrier), and verify the layout does not break or clip content at 200% zoom (WCAG 1.4.10). Use relative units (`rem`) for font sizing so user preferences and browser zoom both work.

### Provide Text Alternatives For Non-Text Content

Anything that conveys information through an image, audio, or video needs a text or media alternative that a user who cannot perceive the original can use.

- **Images.** Informative images get descriptive `alt` text that conveys the same meaning. Decorative images that add no information get empty `alt=""` so they are ignored. Functional images (an icon button) get `alt` (or an `aria-label`) that describes the action ("Search"), not the picture ("magnifying glass"). Complex images (charts, diagrams) may need a longer description in nearby text or `aria-describedby`.
- **Audio.** Provide a transcript. For prerecorded audio with speech, captions or a transcript is required.
- **Video.** Synchronized captions for speech and meaningful sound; an audio description track when important visual information is not conveyed in the audio; a transcript as a fallback. Do not autoplay audio — it interferes with screen readers and is disorienting.

Write alternatives for the meaning, not for the pixels. "Logo" is a weak alt text; "Acme Corp home" conveys function. "Chart" is weak; "Sales rose 40% from Q1 to Q2" conveys the point.

### Make Forms Understandable And Recoverable

Forms are where most users with disabilities are excluded, because errors are often conveyed visually and recovery requires sight and mouse precision. For every input:

- **Every input has a programmatically associated label** (`<label for>` wrapping or pointing at the input, or `aria-label`/`aria-labelledby` when a visible label is impossible — but prefer a visible label). Placeholder text is not a label: it disappears on input, has poor contrast by default, and is not exposed as the input's accessible name.
- **Required fields, formats, and constraints are communicated** before submission, in text, not only by a red asterisk.
- **Errors are identified, described in text, and associated with the field** (`aria-invalid`, `aria-describedby` pointing at the error message, and the message announced via a live region or focus moved to the first error). Do not rely on color or border alone.
- **Error recovery suggests a correction** where possible ("Enter a valid email like name@example.com") rather than only stating the problem.
- **Autocomplete attributes** (`autocomplete="email"`, `"street-address"`) let browsers and password managers fill common fields, which is essential for users who cannot type easily.

### Design For Motion, Timing, And Cognitive Needs

Not every accessibility need is sensory. Some users need more time, less motion, or simpler language:

- **Time limits.** If a session or process times out, give the user a way to turn off, adjust, or extend the limit, and warn before it expires. Time-outs that log users out mid-task exclude users who read or type slowly.
- **Motion and animation.** Respect `prefers-reduced-motion`: provide reduced or no-animation alternatives for non-essential motion. Avoid auto-playing carousels, parallax, and infinite animations that distract or induce vestibular discomfort. Never make motion the only way to perceive something.
- **Auto-triggered content** (hover-only menus, focus-only tooltips) can be inaccessible or disorienting; make such content dismissable and hoverable.
- **Language and clarity.** Use clear, plain language; define abbreviations on first use; set the page `lang` so screen readers use the right pronunciation engine; mark language changes within content with `lang` attributes.

### Recognize The Limits Of Automated Tools And Verify With Real AT

Automated tools (axe, Lighthouse, WAVE, browser devtools issues) catch a useful subset of defects: missing alt, low contrast, duplicate IDs, some ARIA misuse, missing labels. They cannot catch the majority of real accessibility problems: whether a custom widget is keyboard-operable, whether focus order makes sense, whether a live region announces the right thing, whether the interface is understandable, or whether a screen-reader user can actually complete the task. A clean automated scan is not compliance.

Verification must include:

- **Keyboard-only testing** (unplug the mouse, complete the core task).
- **Screen-reader testing** with at least one major screen reader (NVDA or JAWS on Windows, VoiceOver on macOS/iOS, TalkBack on Android) against the real workflow, not just the homepage.
- **Zoom and resize testing** at 200% and reflow at 320px width.
- **High-contrast / forced-color mode** to catch icons and borders that disappear.

Report what real users experience, not what the scanner reports.

## Common Traps

### Adding ARIA Instead Of Using The Right Element

Reaching for `<div role="button" tabindex="0">` when a `<button>` would do. The ARIA version announces as a button but you must now reimplement focus, Enter/Space activation, disabled state, and form-submission behavior — and each omission is a defect. The trap is believing `role` is sufficient; it relabels the tree but adds no behavior.

### Removing Focus Outlines For Aesthetics

Setting `outline: none` globally or on `:focus` to make the design "clean," with no replacement. Keyboard users lose the only indicator of where they are. If an outline is removed, provide an equally visible `:focus-visible` style; never remove focus indication without a deliberate alternative.

### Placeholder-As-Label

Using placeholder text as the only label because it looks minimal. The placeholder disappears on input (so users lose context), has low contrast by default, is often mistaken for a filled value, and is not reliably exposed as the field's accessible name. Use a real, persistent `<label>`.

### Conveying Errors By Color Alone

Showing a red border on an invalid field with no text, icon, or `aria-invalid`. Users with color vision deficiency, high-contrast mode, or screen readers never learn the field is invalid. Always pair color with text and an accessible association.

### Keyboard Traps In Modals And Embedded Content

A modal or embedded video player that receives focus but has no way out (no Escape handler, no focus return). The keyboard user is stuck. Every component that captures focus must release it deliberately on close and return focus to a sensible target.

### Trusting The Automated Audit

Shipping because axe/Lighthouse reports zero issues. Automated tools miss most real barriers: broken keyboard interaction, wrong focus order, silent live regions, unusable custom widgets, and anything requiring judgment. A passing scan proves only that the easy-to-detect subset is absent.

### Announcing Everything (Or Nothing) With Live Regions

Putting `aria-live="assertive"` on a large container so every minor update interrupts the screen reader, or forgetting live regions entirely so important status changes (save success, async errors, result counts) are never announced. Scope live regions narrowly to the specific message, choose `polite` by default (assertive only for urgent interruptions), and test what is actually announced.

### Hiding Content With The Wrong Technique

Using `display: none` or `visibility: hidden` for content that should be available to screen readers (they hide it from AT too), or using `visibility`/`opacity` tricks that leave the element focusable but invisible (a "ghost" focusable element). Use `display:none`/`hidden` only when content should be fully removed from everyone; use visually-hidden (clip/clip-path) CSS for text meant for AT only; never leave a focusable element visually hidden.

### Assuming "It Works On My Machine/Browser"

Testing only in Chrome with a mouse and concluding the interface is accessible. Real users span Safari + VoiceOver, Firefox + NVDA, iOS Safari + VoiceOver, Android + TalkBack, and forced-color/high-contrast modes. Behavior differs across browser/AT combinations; verify on the combinations your users actually have.

### Treating Compliance As A One-Time Audit

Passing an audit at launch and assuming accessibility holds. Every new component, redesign, or third-party widget reintroduces barriers. Accessibility is a property maintained through every change, verified with each release, not a certificate earned once.

## Self-Check

- [ ] Every interactive element uses the native semantic element that matches its behavior, and ARIA is used only to fill gaps — no `role="button"` on a `<div>` where a `<button>` would work, and every ARIA role comes with its full keyboard contract.
- [ ] The entire interface is operable with the keyboard alone (mouse unplugged): every control is reachable via Tab/Shift+Tab, operable with expected keys, no keyboard traps, and focus is visible at all times (no removed `:focus` without an equally visible alternative).
- [ ] Focus is managed deliberately on dynamic changes: opening a dialog moves focus in and returns it on close; SPA route changes are announced; live regions announce meaningful status updates scoped to the specific message.
- [ ] No information is conveyed by color alone — errors, statuses, links, and data encodings all have a second channel (text, icon, shape, underline).
- [ ] Text meets WCAG contrast ratios (4.5:1 normal, 3:1 large/UI) including over images and gradients, the page supports 200% zoom and 320px reflow without clipping, and pinch-zoom is not disabled.
- [ ] All non-text content has appropriate alternatives: descriptive `alt` for informative images, `alt=""` for decorative, functional alt for icon buttons, captions/transcripts/audio descriptions for media, and no autoplaying audio.
- [ ] Every form input has a programmatically associated persistent label, required fields and constraints are stated in text, and errors are identified in text and associated with their fields (`aria-invalid`, `aria-describedby`) with suggested corrections.
- [ ] Time limits are adjustable or extendable, non-essential motion respects `prefers-reduced-motion`, and the page `lang` is set with inline language changes marked.
- [ ] Verification went beyond automated scans: keyboard-only testing, at least one major screen reader (NVDA/JAWS/VoiceOver/TalkBack) through the core workflow, 200% zoom and 320px reflow, and forced-color/high-contrast mode were all checked.
- [ ] The judgment applied was "can a user who interacts differently complete the core task," not "does the scanner pass" — and the highest-risk barriers (custom widgets, focus management, live announcements, color-only signals) were specifically verified.
