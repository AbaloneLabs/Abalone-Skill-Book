---
name: first_run_experience.md
description: Use when the agent is designing a first-run experience, app launch flow, initial setup, sign-up and onboarding sequence, welcome screens, permissions requests, account creation, or the very first moments a new user spends in a product before reaching core value.
---

# First-Run Experience

The first-run experience is where a user decides whether the product is worth their time. Every screen, delay, permission prompt, and form field before the user reaches real value is friction they are paying without yet seeing a return. A first-run flow that feels efficient to the team often feels like a wall to a new user who has no reason to trust the product yet. The judgment problem is not how to make onboarding look polished, but how to minimize the cost a user pays before they experience why the product matters.

Use this skill before designing sign-up flows, welcome sequences, initial setup, permission requests, account creation, or the first session of a product. The goal is to prevent the agent from front-loading commitment, demanding data and permissions before earning trust, or building a first-run flow that delays the moment of real value.

## Core Rules

### Earn Trust Before Asking For Commitment

A new user owes the product nothing. Every request, for an email, a password, a payment method, a permission, a profile, is a withdrawal from a trust account that starts near empty. Sequence requests so the product gives value or clarity before each ask:

- show what the product does before requiring an account;
- explain why a permission is needed at the moment it is needed, not up front;
- defer optional profile data until it has a clear purpose;
- let users try core functionality before committing.

Front-loading commitment converts curious users into bounce statistics.

### Delay Or Eliminate Account Creation Where Possible

Account creation is one of the highest-friction steps. If the product can deliver a taste of value without an account, do so. Patterns to consider:

- guest or trial mode before sign-up;
- progressive account creation, collecting only what is needed now;
- social or passwordless sign-in to reduce form fatigue;
- requiring an account only when persistence or personalization truly demands it.

The longer the user can use the product before hitting the account wall, the more likely they reach it.

### Ask For Permissions In Context, Not In A Block

Permission prompts requested in a batch at launch are refused at high rates, because the user has no reason to grant them. Request each permission at the moment it enables a feature the user already wants:

- notifications when the user opts into alerts;
- camera when the user starts a scan or photo;
- location when the user searches nearby;
- contacts when the user invites someone.

Explain the benefit in the in-app rationale before the system prompt appears. Contextual requests convert far better than launch-time blocks.

### Minimize Steps To The First Moment Of Value

Map the path from app open to the first genuinely useful moment, then cut every step that does not serve it. Common waste:

- marketing carousels that sell the product the user already installed;
- long value-prop videos before any interaction;
- mandatory tutorials before the user can try anything;
- profile completion walls before core access;
- preference questions whose answers are not yet used.

Each step must justify its place by moving the user closer to value or by being genuinely required.

### Preserve User Agency And Skip Paths

Not every user wants the guided path. Provide:

- a clear skip on optional steps;
- a way to defer profile or preference setup;
- the ability to reach core value without completing onboarding;
- later reminders for skipped but useful setup.

Forcing a single linear flow punishes users who already know what they want.

### Handle Returning And Re-Onboarding Users

First-run is not only for brand-new users. People return after long absences, get new devices, or need to reconfigure. Design for:

- re-engagement that does not repeat full onboarding;
- contextual refreshers for features that changed;
- migration of existing state to a new device or account;
- recognition of prior progress.

Treating every launch as a first run annoys experienced users.

### Design Empty States As Part Of First-Run

After onboarding, the user often lands on an empty product. That empty state is part of the first-run experience. A blank screen with no guidance feels broken. Design empty states to:

- explain what the user will see once they add content;
- offer a clear first action;
- provide sample or demo content where appropriate;
- reduce the intimidation of a fresh start.

### Make Setup Decisions Reversible And Discoverable

Choices made during first-run, notification preferences, default privacy, theme, language, are often hard to find later. Ensure every onboarding decision is reversible from settings, and tell users where to change it. Locking users into a rushed early choice breeds resentment.

### Localize And Adapt To Prior Knowledge

First-run should adapt to the user's context: language, region, prior experience with similar products, and whether they migrated from another tool. A first-run that assumes a blank-slate beginner wastes the time of an expert, and one that assumes expertise alienates a newcomer.

## Common Traps

### The Account Wall Before Any Value

Forcing sign-up before the user sees what the product does is the most common cause of first-run abandonment.

### Batch Permission Prompts At Launch

Asking for notifications, location, and contacts up front, before any context, produces refusal rates that cripple the features those permissions enable.

### Marketing Carousels Selling An Already-Installed Product

Slides explaining benefits to someone who already downloaded the app add delay without value.

### Mandatory Tutorials Before First Use

Forcing users to read or watch before touching the product breaks the natural desire to explore.

### Profile Completion Walls

Blocking core access behind optional profile fields trades short data collection for long-term drop-off.

### Single Linear Flow With No Skip

Users who know what they want are punished by flows that cannot be bypassed.

### Blank Empty State After Onboarding

A product that shows nothing after setup feels broken, even when it works.

### Irreversible Early Choices

Rushed onboarding decisions that users cannot find or change later create lasting frustration.

## Self-Check

- [ ] The product delivers value or clarity before asking for account creation, payment, or significant commitment.
- [ ] Account creation is delayed or minimized, with guest, trial, or passwordless paths where possible.
- [ ] Each permission is requested in context with an in-app rationale, not batched at launch.
- [ ] Every step between app open and first value was audited and justified or removed.
- [ ] Optional steps can be skipped or deferred, and core value is reachable without completing onboarding.
- [ ] Returning, re-engaging, and migrating users are handled without repeating full first-run.
- [ ] Empty states after onboarding explain the product and offer a clear first action.
- [ ] All onboarding choices are reversible and discoverable from settings.
- [ ] The flow adapts to language, region, and prior experience rather than assuming a blank-slate beginner.
- [ ] The first-run path was measured from open to first value, not from open to onboarding completion.
