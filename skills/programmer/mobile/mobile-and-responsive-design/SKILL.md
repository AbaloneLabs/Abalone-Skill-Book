---
name: mobile_and_responsive_design.md
description: Use when the agent is building or adapting a web app, site, or hybrid application for phones, tablets, or small screens; choosing responsive versus adaptive layouts; setting viewport, breakpoints, or sizing units (px, rem, vw, clamp); designing touch targets, gestures, or thumb-zone layouts; handling virtual keyboards, input modes, and mobile forms; addressing mobile performance (network, CPU, battery); deciding between native, web, PWA, or hybrid; supporting offline, service workers, or app-install; or reviewing whether an interface is genuinely usable on a real mobile device rather than merely "responsive." Also covers mobile-first design, device diversity, safe-area insets, notches, dynamic type, reduced data, and the failure mode of designing on a desktop and assuming it works on a phone.
---

# Mobile And Responsive Design

Mobile design is the judgment of whether an interface is genuinely usable when the user's environment is nothing like the developer's: a small screen, a thumb, a flaky or slow network, a battery-conscious device, a virtual keyboard that covers half the screen, and a user who is walking, standing, or distracted. "Responsive" — making the layout reflow at breakpoints — is one technique inside this problem; it is not the goal. The goal is that a person on a phone can perceive, operate, and complete the task as well as a person on a laptop.

Agents tend to under-invest here because the design looks fine on the developer's large monitor and fast connection, the layout reflows at a breakpoint, and the emulator reports success. The harm appears only on a real device: tap targets too small for a thumb, text that requires pinching to read, forms where the keyboard covers the submit button, layouts that reflow but bury the primary action, pages that load slowly on cellular data and drain the battery, and interactions that assume a mouse hover or precise pointer the device does not have. The judgment problem is deciding, for each layout and interaction, what the mobile user's real constraints are and designing for those constraints first — not designing for desktop and hoping it shrinks.

## Core Rules

### Design Mobile-First, Not Desktop-Then-Shrink

The order of design determines the result. Starting from the desktop layout and removing or reflowing for small screens tends to produce cramped, secondary-feeling mobile experiences, because every decision was optimized for the large screen. Starting from the smallest meaningful screen forces you to identify the single primary action, the minimum content, and the simplest navigation — and then the larger screens become progressive enhancements.

Concretely:

- **Identify the one primary action** per screen on mobile. A phone shows one column and one focal point; a desktop can show many. If you design for the many first, the one is buried.
- **Author CSS mobile-first** (base styles for small screens, `min-width` media queries that add layout for larger screens). This keeps the mobile path the default and the fast path, rather than loading desktop styles and overriding them.
- **Order content by priority.** On a narrow screen, what is above the fold is what the user sees first; secondary content must not push the primary action below the fold or behind a scroll.

Mobile-first is a discipline of priority, not only a media-query order. The question is always: if a user sees only the top of this screen, can they do the most important thing?

### Size Touch Targets For Fingers, Not Cursors

A mouse pointer is precise to a pixel; a fingertip is not. The well-established guidance (and a common WCAG-adjacent expectation) is a touch target of roughly 44×44 CSS pixels minimum, with spacing between adjacent targets so a user does not trigger the wrong one. Smaller targets, or targets packed tightly together, produce mis-taps and frustration on real devices.

Apply this to every interactive element, including ones that look small:

- Icon-only buttons, close buttons, and link lists need adequate size and spacing, not just visual prominence.
- Inline links in dense text are hard to tap precisely; give tap regions room.
- Adjacent controls in a row (e.g., a list of small buttons) need gaps, not just borders.

Do not assume a visual size of 16px is tappable; the tappable area (padding, hit region) matters more than the visible glyph. A small icon with generous invisible padding around it is more usable than a large icon with no padding.

### Respect The Thumb Zone And One-Handed Use

Most phone interaction happens with one hand, often the thumb of the holding hand, which reaches the bottom and center of the screen easily and the top corners with strain. Layout decisions interact with this physical reality:

- **Primary actions** (main CTA, navigation, common controls) belong where the thumb reaches — typically the bottom of the screen or the bottom center.
- **Destructive or rare actions** can live higher or further from the thumb to avoid accidental triggers.
- **Sticky bottom bars and bottom sheets** match how the device is held; top-anchored toolbars require reach or a second hand.
- **Avoid requiring the top of the screen for the main flow** on large phones, where the top is genuinely hard to reach one-handed.

This is not a rigid rule — tablets, landscape, and two-handed use change the zones — but the default assumption should be one-handed, thumb-driven interaction unless you know otherwise.

### Choose Units And Viewport Deliberately

The unit you choose encodes an assumption about what should scale and what should stay fixed.

- **`px` (CSS pixels).** Predictable but does not respect user font-size preferences; fine for borders, spacing in some cases, and device-pixel-stable visuals, but problematic for font size.
- **`rem`.** Scales with the user's root font-size setting, so it respects accessibility and zoom. Prefer `rem` for font sizes and often for spacing tied to text.
- **`%`, `vw`/`vh`, `fr`.** Relative to container or viewport; good for fluid layouts but `vw`/`vh` can produce text that is too large on big screens or too small on phones — use `clamp()` to bound fluid typography between sensible min and max values.
- **`dvh`/`dvw` (dynamic viewport units).** On mobile, `vh` is often wrong because it does not account for the browser chrome appearing and disappearing as the user scrolls; prefer dynamic viewport units for full-height layouts.

Set the viewport meta (`width=device-width, initial-scale=1`) and **do not disable user zoom** (`maximum-scale=1` or `user-scalable=no` blocks pinch-zoom, which is an accessibility failure and offers little real benefit). Let the user scale the content.

### Account For The Virtual Keyboard And Mobile Inputs

On mobile, focusing an input summons a virtual keyboard that covers roughly half the screen, which changes the layout, hides content, and can cover the field the user is typing into or the submit button. Inputs also have input modes that change which keyboard appears.

- **Scroll the focused field into view** and ensure it is not hidden behind the keyboard; use the viewport resize/VisualViewport API or the browser's scroll-into-view on focus.
- **Place the primary submit action** so it remains reachable when the keyboard is up, or accept that the user scrolls to find it — do not trap it behind the keyboard.
- **Use the right input type and inputmode** (`type="email"`, `type="tel"`, `type="number"`, `inputmode="decimal"`, `autocapitalize`, `autocomplete`) so the correct keyboard appears and the browser/OS can autofill. `type="number"` changes semantics and validation; for numeric codes, `inputmode="numeric"` with `type="text"` is often safer.
- **Keep forms short.** Every field on mobile is friction: typing is slow, the keyboard covers context, and errors are harder to recover from. Ask only for what is needed, use sensible defaults and autocomplete, and split long forms into steps with clear progress.

### Treat Mobile Performance As A Design Constraint

A page that loads in 300ms on fiber and a desktop CPU may take many seconds on a mid-range phone on a congested cellular network, and the difference is not "slower but fine" — it is abandonment, battery drain, and exclusion of users on metered or weak connections. Performance is part of usability on mobile.

- **Measure on a real mid-range device on a throttled network**, not on the developer machine. DevTools throttling (Slow 3G, CPU slowdown) is a rough proxy; real devices differ.
- **Reduce what must load before use.** Defer non-critical scripts and assets, lazy-load below-the-fold images and components, and avoid loading large client-side bundles for content that could be server-rendered.
- **Watch images.** Serve appropriately sized, modern formats (WebP/AVIF), use `srcset` and `sizes` so the device downloads the right resolution, and set dimensions to prevent layout shift.
- **Respect the main thread.** Long JavaScript tasks block interaction on slow CPUs; break up work and avoid jank on scroll and input.
- **Consider data cost.** Users on metered plans abandon heavy pages; provide lighter paths where possible.

### Decide Native, Web, PWA, Or Hybrid Against Real Requirements

"Build an app" is not a decision; the choice between native, web, PWA, and hybrid trades off capabilities, reach, cost, and experience, and the right answer depends on what the product actually needs.

- **Native.** Best performance, full access to platform APIs, platform-native UX patterns, offline and background capability. Cost: separate codebases per platform, slower iteration, app-store distribution.
- **Web.** Fastest iteration, broadest reach, no install friction, linkable and indexable. Limited by browser APIs, sandbox restrictions, and (historically) weaker offline/background capability.
- **PWA (installable web).** Web with service-worker offline, push, home-screen install, and some device APIs. A strong middle ground when the needed capabilities are available and the experience does not require deep platform integration.
- **Hybrid / cross-platform (React Native, Flutter, Capacitor).** One codebase across platforms with near-native UX and broad API access. Trades some performance, platform fidelity, and ecosystem dependence for shared code.

Ask: which platform APIs are required? Is deep platform-native UX essential? How important is install friction and reach? What is the team's capacity to maintain multiple codebases? The answer is a tradeoff, not a default — and "responsive web" is often the correct answer for content and many workflows, while native is often correct for performance-critical, hardware-heavy, or deeply integrated experiences.

### Plan For Offline, Intermittent Connectivity, And Sync

Mobile connections are not reliable: users enter tunnels, ride elevators, switch between Wi-Fi and cellular, and sit on flaky networks. An interface that assumes the network is always present breaks the moment it is not.

- **Graceful degradation on network failure.** Show clear errors, allow retry, and avoid silent failures or stuck spinners. Cache previously loaded data so the user sees something useful offline rather than a blank screen.
- **Service workers / PWA offline.** For apps that should work offline, precache the app shell and cache or background-sync data. Be deliberate about cache invalidation and staleness (see the caching skill).
- **Optimistic UI and background sync.** For writes, consider accepting the action locally, showing success, and syncing when connectivity returns — but only where eventual consistency is acceptable and conflicts are handled.
- **Conflict and stale-data handling.** When data syncs later, decide how stale data and concurrent edits are reconciled; silent overwrites and duplicated writes are the common failures.

### Test On Real Devices And Real Conditions

Emulators and browser devtools are useful for quick checks but they lie about the things that matter most: actual touch accuracy, real network conditions, real CPU and battery behavior, OS-level interactions (notifications, keyboard, share sheets), and the feel of the interface in motion. The interface that "works" in Chrome devtools can be unusable on a real phone.

Verification must include:

- **Real devices** spanning at least a small phone, a large phone, and a tablet, including both major platforms.
- **Real networks** — test on throttled and actual cellular, not only office Wi-Fi.
- **Real input** — use touch, not a mouse, and try one-handed use.
- **OS features** — keyboard behavior, orientation change, dynamic type / font scaling, dark mode, and interruptions (calls, notifications).
- **Battery and background** — verify the app does not drain battery or misbehave when backgrounded/foregrounded.

## Common Traps

### Designing On Desktop And Assuming It Shrinks

Building the desktop layout fully, then adding a mobile breakpoint that reflows it. The result is a desktop experience squeezed onto a phone: primary actions buried, navigation collapsed awkwardly, and content ordered for the large screen. Design mobile-first so the small-screen experience is the primary one.

### Hover-Dependent Interactions

Menus, tooltips, or actions that appear only on `:hover`. Touch devices have no hover (or emulate it as a sticky first-tap), so hover-only content is invisible or behaves oddly. Provide tap-based alternatives and ensure nothing critical is reachable only by hover.

### Tiny Or Crowded Tap Targets

A row of small icon buttons with no spacing, or a 20px link in dense text. Real fingers mis-tap, triggering the wrong action or nothing. Give every interactive element an adequate hit area and spacing between adjacent targets.

### Fixed Units And Disabled Zoom

Font sizes in `px` that ignore user preferences, full-height layouts using `100vh` that break when the mobile browser chrome appears, and `user-scalable=no` that blocks pinch-zoom. Use `rem` for text, dynamic viewport units for height, and never disable user zoom.

### The Keyboard Covers The Action

A form where the submit button or the active field sits behind the virtual keyboard, with no scroll-into-view, so the user cannot see what they are typing or how to submit. Handle focus scroll and place primary actions where they survive the keyboard.

### Assuming Emulator Success Equals Device Success

Verifying only in browser devtools or an emulator and concluding the interface is mobile-ready. Emulators misrepresent touch accuracy, network conditions, CPU speed, and OS interactions. Real devices under real conditions are required.

### Heavy Bundle On A Metered Connection

Shipping a large JavaScript bundle and many high-resolution images because they load fine on the office network. On cellular, the page is slow, expensive, and abandoned. Measure and optimize for throttled networks and real device CPUs.

### One-Size Responsive That Ignores Context

A single responsive layout that merely reflows columns, without reconsidering what the mobile user actually needs (different primary action, simpler navigation, shorter forms). Responsive reflow is necessary but not sufficient; the mobile experience must be designed, not just allowed.

### Ignoring Safe Areas, Notches, And Device Diversity

Hardcoding edges to the screen without accounting for notches, home indicators, and safe-area insets, so content sits under hardware features. Use `env(safe-area-inset-*)` and test across device shapes, orientations, and dynamic-type settings rather than assuming one screen geometry.

### Native-Web-Or-Hybrid Chosen By Habit

Defaulting to "build a native app" because that is what the team knows, or to "responsive web" because it is cheapest, without evaluating the real requirements (offline, hardware, performance, distribution). The choice is a tradeoff against the product's actual needs; choosing by habit produces either wasted effort or an experience that cannot meet the requirements.

## Self-Check

- [ ] The design started from the smallest screen and the single primary action, and the desktop layout is a progressive enhancement — not a desktop design squeezed onto a phone.
- [ ] Every interactive element has an adequately sized touch target (~44×44 CSS px minimum) with spacing between adjacent targets, and tap regions were verified by touch, not by visual size.
- [ ] Primary actions and navigation are placed within the thumb zone (typically bottom/center) for one-handed use; nothing in the main flow requires reaching the top corners of a large phone.
- [ ] Units are chosen deliberately: `rem` for text, bounded fluid typography via `clamp()`, dynamic viewport units for full-height layouts, and the viewport meta is set without disabling pinch-zoom.
- [ ] Forms handle the virtual keyboard (focused field scrolled into view, submit reachable), use correct `type`/`inputmode`/`autocomplete`, and are kept to the minimum fields needed.
- [ ] Performance was measured on a real mid-range device over a throttled network: bundle size, image sizing (`srcset`/modern formats), main-thread blocking, and data cost were all addressed, not assumed.
- [ ] The native/web/PWA/hybrid choice was made against real requirements (required APIs, performance, distribution, maintenance cost), not by team habit.
- [ ] Offline and intermittent connectivity are handled: graceful errors and retry, cached/stale-but-useful data, deliberate optimistic-UI and background-sync only where consistency allows, and conflict handling for late syncs.
- [ ] Verification included real devices (small phone, large phone, tablet; both platforms), real cellular networks, real touch and one-handed use, orientation change, dynamic type / font scaling, dark mode, and OS interruptions.
- [ ] The judgment applied was "can a user on a real phone, on a real network, with one hand, complete the core task" — not "does the layout reflow at a breakpoint."
