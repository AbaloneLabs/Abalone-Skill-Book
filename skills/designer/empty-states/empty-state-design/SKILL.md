---
name: empty_state_design.md
description: Use when the agent is designing empty states, zero-results screens, no-content views, filtered-out states, not-yet-configured panels, or any interface state where expected content is absent and the user needs to understand why and what to do next.
---

# Empty State Design

An empty state is not a blank screen; it is a moment of explanation and direction. When content is absent, the user does not know whether the system is broken, the data is missing, they did something wrong, or there is simply nothing there yet. A good empty state resolves that uncertainty and moves the user forward. A bad one, or none at all, leaves the user assuming the product is broken.

Use this skill before designing or reviewing any view that can be empty: lists, search results, dashboards, inboxes, libraries, feeds, carts, settings panels, reports, and filtered views. The goal is to prevent the agent from treating emptiness as an edge case to ignore, and instead designing it as a first-class state that diagnoses the cause and offers a clear next action.

## Core Rules

### Distinguish The Type Of Emptiness

Not all empty states mean the same thing, and they should not look the same. The design must reflect why the screen is empty, because the cause determines the correct response.

Common empty-state causes:

- first run: the user has never added content, so the state is an opportunity to begin;
- no results: the user's filter or search returned nothing, so the state is about adjusting criteria;
- cleared: the user removed everything, so the state confirms the action and offers undo;
- not configured: a prerequisite is missing, such as connecting an account or enabling a feature;
- no permission: the user lacks access, so the state explains the limit;
- error or loading failure: data failed to load, so the state offers retry;
- nothing new: there is genuinely no recent activity, which may be a healthy state.

A single generic empty state applied to all these causes confuses the user and blocks recovery.

### Explain Why, Not Just That

The user's first question on an empty screen is "why is this empty?" The empty state should answer that immediately and honestly.

Provide:

- a clear headline that names the state, such as "No messages yet" or "No results match your filters";
- a short explanation of the cause when it is not obvious;
- reassurance when emptiness is normal or expected;
- avoidance of blame, especially when the cause is the system or a filter, not the user.

"No results found" with no explanation forces the user to guess whether the search is broken, the data is missing, or the query was wrong.

### Offer A Primary Next Action

An empty state that only explains is incomplete. The user needs a path forward. Every empty state should offer at least one clear, relevant action.

Match the action to the cause:

- first run: "Add your first item" or "Import data";
- no results: "Clear filters" or "Try a different search";
- cleared: "Undo" or "Restore";
- not configured: "Connect account" or "Enable feature";
- no permission: "Request access" or "Contact admin";
- error: "Try again" or "Reload";
- nothing new: "Refresh later" or a link to related content.

The primary action should be the most likely next step, not a generic "Learn more" that goes nowhere useful.

### Match Tone To The User's Situation

Emptiness can be neutral, disappointing, or alarming depending on context. A cart that is empty is normal; a security log that is empty may be suspicious; a backup list that is empty may be a crisis. Tone should fit.

Calibrate tone:

- first-run and nothing-new states can be warm and encouraging;
- no-results states should be helpful and neutral, not cheerful;
- error and permission states should be plain and direct;
- cleared states should reassure that the action was intentional and recoverable;
- high-stakes emptiness, such as missing backups or logs, should signal concern without panic.

Cheerful illustrations on an alarming empty state read as indifference.

### Design For Repeated Encounters

Some empty states are seen once; others are seen often, such as an inbox after clearing it or a search with no results. The design should not annoy on repeat.

For frequently seen empty states:

- keep them lightweight and fast;
- avoid heavy animations that slow repeated use;
- do not force a tutorial or onboarding every time;
- ensure the primary action stays quick to reach;
- consider whether a compact inline message serves better than a full-screen treatment.

An empty state that was delightful once can become an obstacle the tenth time.

### Preserve Context And Navigation

An empty state should not trap the user. Navigation, filters, and the ability to leave or adjust must remain available.

Ensure:

- global navigation and breadcrumbs still work;
- active filters remain visible and editable so the user can broaden criteria;
- the empty state does not replace the page chrome that lets the user navigate away;
- related views or tabs remain reachable;
- the user can return to a non-empty state without starting over.

### Use Illustration And Space Deliberately

Illustrations can humanize an empty state, but they can also waste space and distract from the action. Use them where they aid comprehension or tone, not as default decoration.

Guidance:

- illustrations should support the message, not dominate it;
- keep them lightweight so they do not slow the view;
- avoid generic stock-style art that signals nothing;
- ensure the headline, explanation, and action are more prominent than the art;
- consider omitting illustration for error and high-stakes states where clarity matters more.

## Common Traps

### One Generic Empty State For Everything

A single "Nothing here yet" screen applied to first run, no results, errors, and permissions fails to diagnose the cause and offers no relevant action.

### Explaining Without Acting

An empty state that says "You have no items" but offers no button to add one leaves the user stuck. Explanation without action is incomplete.

### Cheerful Tone On Alarming Emptiness

A playful illustration on an empty security log or missing backup list trivializes a serious situation. Tone must match stakes.

### Blaming The User

Phrasing like "You haven't added anything" sounds accusatory. Describe the state, not the user's failure.

### Heavy Illustrations That Block Action

Large decorative art that pushes the action button below the fold turns a helpful empty state into an obstacle.

### Trapping The User

An empty state that removes navigation or filters prevents the user from adjusting criteria or leaving, turning emptiness into a dead end.

### Ignoring Repeat Viewing

A full-screen animated onboarding empty state shown every time the inbox is cleared becomes irritating fast. Frequent empty states should be lightweight.

## Self-Check

- [ ] The empty state identifies and reflects its specific cause: first run, no results, cleared, not configured, no permission, error, or nothing new.
- [ ] A clear headline and short explanation answer why the screen is empty, without blaming the user.
- [ ] At least one primary next action is offered, matched to the cause of the emptiness.
- [ ] Tone fits the situation: encouraging for first run, neutral for no results, serious for errors and high-stakes emptiness.
- [ ] Frequently encountered empty states are lightweight, fast, and do not force repeated onboarding or heavy animation.
- [ ] Navigation, filters, breadcrumbs, and related views remain available so the user is not trapped.
- [ ] Illustrations support the message without dominating it or pushing the action out of reach.
- [ ] Error and loading-failure empty states name the problem and offer retry or recovery.
- [ ] Cleared and destructive empty states confirm the action and offer undo or restore where possible.
- [ ] The empty state was designed as a first-class state, not left as a default blank or broken render.