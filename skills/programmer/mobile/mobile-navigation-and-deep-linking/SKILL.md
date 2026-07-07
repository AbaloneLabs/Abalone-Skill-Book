---
name: mobile_navigation_and_deep_linking.md
description: Use when the agent is designing or implementing mobile navigation (navigation stacks, tabs, modals, nested graphs, back-stack management), wiring deep links, universal links, or app links, handling custom URL schemes, routing an incoming link to the right screen with the right back stack, restoring navigation state after process death or cold start, or implementing deferred deep links for first-install users. Also covers the failure mode of deep links that 404 or route to the wrong screen, back stacks that lose the user's place or trap them with no way back, navigation state lost across process death, ambiguous or hijackable URL schemes, and the gap between "the link opens the app" and "the link lands the user on the right content with a usable back stack."
---

# Mobile Navigation And Deep Linking

Mobile navigation is the judgment of how a user moves through the app, where they came from, and where they can go back to — and deep linking is the judgment of what happens when an external entry point (a link, a notification, a voice assistant) drops the user into the middle of that structure rather than at the front door. The judgment problem is that a deep link is not "open the app"; it is "construct a valid navigation state that lands the user on the right content with a back stack that makes sense." A link to a product detail screen is broken if it opens the screen with no way back to browse, if it discards the user's in-progress work, if it routes to the wrong variant of the screen, or if the navigation state is lost the moment the OS kills the process. The discipline is to treat navigation as an explicit, serializable state machine (not a side effect of which screen is pushed), to design deep links as constructors of valid back stacks, to make every deep-linkable destination reachable with a sensible return path, to restore navigation state across process death, and to handle the deferred case where a brand-new install must receive the link that brought the user in.

Agents tend to treat navigation as "push the next screen" and deep linking as "parse the URL and open the screen," which works for a demo and collapses in production. The harm appears as deep links that land on the home screen instead of the content (because the app cold-started and the link was lost), as back stacks that trap the user (back exits the app from a deep-linked screen) or that lose their place, as navigation state wiped on process death so the user returns to the root, as URL schemes that collide with other apps or are hijackable, and as the first-install user whose deferred deep link never arrives. The judgment is to model navigation explicitly, to construct back stacks deliberately for each entry path, to persist and restore navigation state, to validate and scope deep-link routes, and to design the deferred path before shipping. Deep linking is a state-construction problem disguised as a URL-parsing problem.

## Core Rules

### Model Navigation As Explicit, Serializable State

Navigation is not the current screen; it is the full back stack — the ordered set of destinations, their arguments, and the modal/tab context — that determines where the user is and where back goes. Treat this as explicit, serializable state that can be saved, restored, and reconstructed, not as an implicit consequence of which view controller or composable happens to be on top.

- **Represent the back stack explicitly.** Model navigation as a stack (or graph) of destinations with their arguments, not as a pile of pushed screens; this is what lets you save, restore, and reconstruct it.
- **Make navigation state serializable.** Destinations and arguments must be saveable so the state survives process death and can be restored on cold start; non-serializable arguments (object references, callbacks) force you to lose state or crash on restore.
- **Separate navigation destination from its arguments and from its presentation.** A destination, its parameters, and whether it is shown as a push, modal, or in a tab are distinct concerns; conflating them produces ambiguous back behavior.
- **Prefer the platform's navigation framework.** iOS, Android, and the major cross-platform frameworks provide navigation components with back-stack, deep-link, and state-restoration support; building a bespoke stack is where restore bugs and lost state are born.

### Construct A Valid Back Stack For Every Deep Link

A deep link does not just open a destination; it must construct a back stack that makes the destination reachable and returnable. A deep link to a product screen should land the user on the product with a back path to the category or home — not stranded on the product with back exiting the app, and not dumped onto the home screen because the app cold-started.

- **Build the synthetic back stack, not just the leaf.** When a deep link targets a nested destination, synthesize the intermediate destinations (home → category → product) so back navigation is sensible, rather than pushing only the leaf.
- **Decide merge-vs-replace for the existing stack.** When the app is already running, decide deliberately whether the deep link replaces the current task, pushes onto the existing stack, or clears to a fresh stack; the wrong choice loses the user's place or traps them.
- **Handle cold start explicitly.** When the app is not running, the deep link must be preserved through launch and used to construct the initial navigation state; a link lost during cold start lands the user on the home screen and is, to them, broken.
- **Make every deep-linkable destination returnable.** Before marking a screen deep-linkable, confirm there is a valid back path; a deep-linked screen with no parent is a dead end.

### Restore Navigation State Across Process Death

The OS can kill the app's process at any time (background, memory pressure) and relaunch it; any navigation state not explicitly persisted is lost, and the user returns to the root instead of where they were. Navigation state restoration is what makes the app feel stable across interruption.

- **Save the back stack on backgrounding.** Persist the current navigation state (destination, arguments, scroll position where relevant) when the app goes to the background, so a process kill does not lose the user's place.
- **Restore on cold start.** On launch, restore the saved navigation state so the user returns to where they were, not the root; this is the difference between "the app remembered me" and "the app forgot everything."
- **Handle arguments that may no longer be valid.** A restored destination's argument (an ID) may point to deleted or changed data; handle the missing/stale case gracefully rather than crashing or showing a blank screen.
- **Test process death and restore on real devices.** Process death and state restoration behave differently on emulators and under memory pressure; verify on hardware with the OS's don't-keep-activities or memory-pressure tools.

### Validate, Scope, And Own Deep-Link Routes

Deep links are an external input that can carry malformed, hostile, or ambiguous targets. Validate them, scope them to your own domains, and own your schemes so they are not hijacked or confused with other apps.

- **Prefer universal links / app links over custom URL schemes.** Associated-domain links (iOS universal links, Android app links) are verified against a domain you own, avoiding the ambiguity where multiple apps register the same custom scheme and the OS picks one unpredictably.
- **Validate and sanitize every incoming link.** Treat deep-link inputs as untrusted: validate paths, query parameters, and IDs; reject or ignore malformed or out-of-scope links rather than routing to a broken or wrong screen.
- **Scope custom schemes if you must use them.** If a custom scheme is unavoidable, make it specific and unique, and assume another app may still register it; do not pass sensitive data through a scheme that can be intercepted by another app.
- **Map one logical destination consistently.** Ensure the same content has one canonical link, not multiple competing routes, so analytics, sharing, and back behavior are consistent.

### Handle Deferred Deep Links For First-Install Users

A user who does not have the app installed clicks a deep link, is sent to the store, installs, and opens the app for the first time. The link that brought them in must survive that journey and route them to the content on first launch — this is deferred deep linking, and it is the most commonly broken path because it crosses the store boundary.

- **Preserve the link across the store install.** Use a deferred deep-link solution (or server-side attribution) to carry the intended destination through the install, so the first-open user lands on the content, not the home screen.
- **Handle the case where the deferred link is lost or stale.** The attribution may fail, arrive late, or point to content the new user cannot access; fall back gracefully to a sensible default rather than crashing or hanging on a missing link.
- **Distinguish deferred from direct.** A first-open from a deferred link may need onboarding or sign-in before the destination; sequence these deliberately so the user is not dropped onto a screen they cannot use.
- **Verify the full first-install journey.** Test the store→install→open→destination path, not only the direct open; this is the path most likely to be broken and least likely to be tested.

### Document the Basis and the Reasoning

Every conclusion should be traceable to its evidence, assumptions, and alternatives considered. Record not only the outcome but the reasoning path: what entry paths were designed for, what back-stack behavior was chosen and why, what was checked, what uncertainty remains, and what would change the conclusion. Documentation that captures the basis allows another practitioner to review, reproduce, or challenge the work, and it prevents confident conclusions from becoming unrepeatable assertions. A decision made without a recorded basis cannot be audited, improved, or safely handed off.

## Common Traps

### Deep Link That Opens The App But Lands On The Home Screen

A link that launches the app but, because the app cold-started and the link was not preserved through launch, drops the user on the home screen instead of the content. Preserve the link through cold start and use it to construct the initial navigation state.

### Back Stack That Traps The User Or Loses Their Place

A deep-linked screen pushed as a leaf with no parent, so back exits the app, or a deep link that replaces the stack and discards the user's in-progress navigation. Synthesize a valid back stack for deep links; decide merge-vs-replace deliberately.

### Navigation State Lost Across Process Death

Navigation state held only in memory, so when the OS kills and relaunches the process the user returns to the root. Save the back stack on backgrounding and restore it on cold start; make navigation state serializable.

### Ambiguous Or Hijackable URL Scheme

A generic custom URL scheme (`myapp://`) that multiple apps register, so the OS opens the wrong app, or a scheme used to pass sensitive data that another app can intercept. Prefer verified universal links / app links tied to a domain you own; scope and sanitize custom schemes.

### Malformed Or Out-Of-Scope Link Routed To A Broken Screen

A deep link with a bad or hostile path routed directly to a screen that crashes, shows blank, or exposes content it should not. Validate and sanitize every incoming link; reject malformed or out-of-scope routes.

### Deferred Deep Link Never Delivered To First-Install Users

The store→install→open journey losing the link that brought the user in, so a brand-new install lands on the home screen instead of the shared content. Use a deferred deep-link / attribution solution; handle the lost-or-stale case gracefully; test the full first-install journey.

### Bespoke Navigation That Skips State Restoration

A custom navigation implementation that does not save or restore state, producing lost-place bugs and restore crashes that the platform navigation component would have handled. Prefer the platform navigation framework; represent the back stack as serializable state.

## Self-Check

- [ ] Navigation is modeled as explicit, serializable state — an ordered back stack of destinations with arguments and presentation context — not as an implicit pile of pushed screens, and the platform navigation framework is used rather than a bespoke stack that skips state restoration.
- [ ] Every deep link constructs a valid back stack: the leaf destination is synthesized with its intermediate parents so back navigation is sensible, merge-vs-replace of the existing stack is a deliberate decision, cold start preserves and applies the link, and no deep-linkable destination is a dead end.
- [ ] Navigation state is saved on backgrounding and restored on cold start so process death does not return the user to the root, restored arguments that may be stale or deleted are handled gracefully, and restoration was tested on real devices under process-death conditions.
- [ ] Deep links use verified universal links / app links tied to a domain you own rather than generic custom schemes, every incoming link is validated and sanitized as untrusted input, out-of-scope or malformed routes are rejected, and each logical destination has one canonical route.
- [ ] Deferred deep links for first-install users are handled: the link survives the store→install→open journey via a deferred-link or attribution solution, the lost-or-stale case falls back gracefully, onboarding/sign-in is sequenced before the destination where needed, and the full first-install path was tested.
- [ ] Back behavior was verified from every entry path — direct launch, running-app deep link, cold-start deep link, and deferred first-install — confirming the user can always reach a sensible place and is never trapped or dropped on a dead-end screen.
- [ ] The highest-risk cases were verified — a cold-start deep link landing on the right content, a process-killed app restoring the user's place, a malformed link rejected safely, and a first-install deferred link delivered — not only the clean direct-open path.
