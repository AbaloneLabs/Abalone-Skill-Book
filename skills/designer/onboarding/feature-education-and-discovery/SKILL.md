---
name: feature_education_and_discovery.md
description: Use when the agent is designing feature discovery, progressive disclosure, in-context education, tooltips, coach marks, feature announcements, onboarding hints, or helping users learn capabilities over time without overwhelming them or blocking their workflow.
---

# Feature Education And Discovery

Users do not read manuals, and they rarely complete onboarding tutorials. They discover features by doing, by stumbling, or by being shown the right thing at the right moment. The design problem is how to make capabilities discoverable over time without interrupting workflow, without condescending, and without burying important features behind guesses. Education that interrupts too aggressively teaches users to dismiss it reflexively; education that is too subtle never lands at all.

Use this skill before designing tooltips, coach marks, feature announcements, empty-state guidance, progressive disclosure, help systems, or any in-context education that helps users find and understand capabilities. The goal is to prevent the agent from building education that blocks work, appears at the wrong moment, or treats discovery as a one-time event rather than an ongoing relationship with the user.

## Core Rules

### Teach At The Moment Of Need, Not The Moment Of Launch

Education lands when it answers a question the user currently has. A tooltip about sorting is useful when the user is looking at a list, not on a welcome screen. A feature announcement is useful when the feature is reachable, not weeks before it ships in the user's region.

Map each piece of education to the trigger that makes it relevant:

- the user reaches the relevant screen;
- the user performs a related action;
- the user hesitates or fails;
- the feature becomes available to that user.

Teaching out of context is forgotten before it can be used.

### Prefer Progressive Disclosure Over Front-Loaded Tours

Showing every feature at once overwhelms new users and is forgotten by the time it matters. Reveal capabilities as they become relevant:

- show basic controls first;
- surface advanced options when the user demonstrates readiness;
- offer deeper features contextually rather than in an initial dump.

Progressive disclosure respects the user's current task and cognitive load.

### Never Block The Workflow To Teach

Education that forces a dismissal before the user can work is resented. Prefer non-blocking forms:

- subtle hints that fade or can be ignored;
- inline suggestions adjacent to the action;
- persistent help entry points the user can open on demand;
- empty-state guidance that teaches by inviting action.

If education must interrupt, limit it to genuinely critical, one-time moments, and keep it short.

### Make Discovery Self-Service And Searchable

Users forget what they learned and need to re-find it. Provide durable discovery paths:

- a searchable help or feature index;
- consistent placement of help entry points;
- the ability to replay a hint or tour on demand;
- contextual "learn more" links near features.

Education that disappears after one view wastes its own content.

### Time Announcements To Relevance And Reachability

Feature announcements fail when they arrive too early or too late. Announce a feature when:

- it is available to this specific user;
- the user is in a context where it is useful;
- the change is meaningful enough to warrant attention.

Avoid announcing features the user cannot yet access, or piling multiple announcements into one session. Sequence announcements so they do not compete.

### Measure And Respect Engagement Signals

Education should adapt to how the user responds. If a user dismisses a tooltip, do not show it again immediately. If a user engages with a hint, offer the next relevant one. Track:

- dismissal versus engagement;
- repeated failures that suggest missing education;
- feature usage that indicates readiness for advanced tips.

Treating all users identically produces education that annoys experts and under-serves beginners.

### Distinguish Education From Marketing

Feature education explains how to use something the user already has. Marketing sells something they do not. Mixing them erodes trust: users start dismissing "tips" because they are actually upsells. Keep educational surfaces focused on capability, and separate promotion clearly.

### Teach Through The Interface Itself

The strongest education is an interface that explains itself. Clear labels, obvious affordances, predictable placement, and meaningful empty states teach without explicit instruction. Before adding a tooltip, ask whether the interface could instead be clearer. Explicit education is a supplement to good design, not a substitute for it.

### Support Different Learning Preferences

Users learn differently: some explore, some read, some watch, some ask. Offer multiple paths:

- inline hints for explorers;
- searchable docs for readers;
- short videos or demos for visual learners;
- in-app chat or help for those who ask.

A single education format serves only one type of user.

### Localize And Adapt To Expertise

Education that assumes expertise bores beginners; education that explains the obvious condescends to experts. Adapt depth to signals of experience, and localize examples and terminology to the user's language and region.

## Common Traps

### The Mandatory Welcome Tour

Forcing users through a multi-step tour before they can work teaches them to dismiss education reflexively.

### Teaching Out Of Context

A sorting tip shown on the home screen is forgotten by the time the user reaches the list where it applies.

### Repeating Dismissed Education

Showing the same tooltip after the user dismissed it trains the user that dismissal is futile and erodes trust.

### Announcing Unreachable Features

Promoting a feature the user cannot yet access creates frustration and undermines future announcements.

### Education As Hidden Upsell

Tips that are actually marketing make users distrust all educational surfaces.

### Over-Explaining An Already Clear Interface

Adding tooltips to controls that are self-evident adds noise and condescension without value.

### One-Time Education That Cannot Be Replayed

If education vanishes after one view and cannot be found again, users who need it later are stranded.

### Treating All Users The Same

Identical education for beginners and experts either bores or confuses half the audience.

## Self-Check

- [ ] Each piece of education is triggered by a moment of need, not by launch timing alone.
- [ ] Capabilities are revealed progressively rather than dumped in an initial tour.
- [ ] Education is non-blocking and does not force dismissal before the user can work.
- [ ] Help and feature guidance are searchable, replayable, and consistently placed for self-service rediscovery.
- [ ] Feature announcements are timed to when the feature is reachable and relevant to the user.
- [ ] Engagement signals, dismissals, failures, and usage, shape what education is shown next.
- [ ] Educational surfaces are clearly separated from marketing and upsell content.
- [ ] The interface itself was reviewed for clarity before adding explicit education.
- [ ] Multiple learning paths, inline, searchable, visual, and conversational, are available.
- [ ] Education adapts to expertise level, language, and region rather than assuming a single user type.
