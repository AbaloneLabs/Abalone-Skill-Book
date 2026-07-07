---
name: settings_and_preferences_design.md
description: Use when the agent is designing settings screens, preference panels, account configuration surfaces, options menus, control panels, toggles for feature behavior, user-configurable rules, or reviewing how people customize product behavior.
---

# Settings And Preferences Design

Settings are not a dumping ground for every option the team could not resolve through good defaults. A settings surface is where users take ownership of the product: adjusting it to their context, constraints, privacy tolerance, workflow, and taste. When settings are designed carelessly, users either never find the option they need, change something they do not understand, or leave defaults that quietly harm them.

Use this skill before designing or reviewing settings screens, preference centers, account configuration, privacy controls, notification preferences, workspace or team settings, feature toggles, accessibility options, billing preferences, or any surface where users configure ongoing product behavior. The goal is to prevent the agent from producing a long flat list of switches, burying consequential choices, or exposing implementation knobs as if they were user-facing decisions.

## Core Rules

### Decide Whether A Setting Should Exist At All

Before designing where a setting lives, decide whether it should be a setting. Many options exist because two stakeholders disagreed and the team punted the decision to the user. Every setting has a cost: it adds cognitive load, creates more states to test, and produces configurations no one anticipated.

Prefer a good default over a setting. Prefer automatic behavior over a setting. Prefer a one-time choice embedded in a flow over a persistent preference when the choice is contextual. Expose a setting only when:

- reasonable people genuinely differ in their needs;
- the product cannot reliably infer the right behavior;
- the user needs ongoing control, not a one-time answer;
- the cost of a wrong default is higher than the cost of exposing the choice.

If you cannot explain who would change this setting and why, do not add it.

### Separate Preferences From Configuration From Administration

Settings surfaces blur three different intents, and conflating them creates confusion.

- Personal preferences adjust the product for one user: theme, language, density, notification timing.
- Configuration adjusts how a feature works for a context: default currency, working hours, data retention, integrations.
- Administration governs others or shared state: team members, roles, billing, workspace policies.

Mixing these in one flat list forces users to scan past irrelevant controls. Personal preferences belong near the feature they affect or in a clearly personal area. Administrative controls belong behind clear authority boundaries and often in a separate surface, because they carry consequences for other people.

### Organize By User Mental Model, Not Engineering Modules

Users do not think in terms of your backend services. They think in terms of outcomes: "how do I stop getting emails," "how do I change what others see about me," "how do I connect my calendar," "how do I make this work on my slow connection."

Group settings by the goal the user is trying to achieve. Avoid labels like "Account module," "Notification service," "Auth provider." Prefer goal-oriented groupings such as "Privacy and visibility," "Notifications," "Connected accounts," "Appearance and accessibility." When a single feature has many knobs, consider a dedicated detail page rather than dumping all of them at the top level.

### Make The Consequence Of Each Setting Visible

A toggle is a decision, and decisions have consequences. Users should understand what changes when they flip a control, not just what the label says.

For consequential settings, show:

- what behavior changes, in plain language;
- who is affected, especially in shared or team contexts;
- whether the change is immediate or delayed;
- whether it is reversible, and how;
- any side effects, such as data loss, disconnection, or visibility changes.

"Do not disturb" is not self-explanatory: does it silence calls, mute only some apps, override repeated callers, or pause scheduled summaries? A privacy toggle that says "limit who can find me" must explain who can still find me and who cannot.

### Preserve Discoverability Without Overwhelming

A settings surface that hides everything behind search fails users who do not know the right word. A surface that shows everything at once overwhelms users who only need one thing.

Combine strategies:

- clear top-level categories with predictable names;
- progressive disclosure for detailed or advanced controls;
- search for users who arrive with a specific question;
- contextual entry points from the feature the setting affects;
- recently changed or recommended adjustments surfaced where helpful.

Avoid hiding the only path to a setting inside an unrelated category. If a user lands on a feature and wants to adjust it, offer a direct route to its settings.

### Design For Reversibility And Safety

Users change settings to experiment. Many changes feel risky because users cannot predict the outcome. Reduce that risk.

Support reversibility where feasible: undo, reset to default, compare before and after, or a clear record of what changed. For destructive or hard-to-reverse settings, require explicit confirmation and explain what cannot be undone. For settings that affect other people, surface the blast radius before the change is applied.

Never let a single toggle silently delete data, revoke access for a whole team, or change what is visible publicly without making that consequence explicit.

### Handle Defaults As A Design Decision

The default value of every setting is itself a design choice that most users will live with. Choose defaults deliberately, document the reasoning, and revisit them as the product and user base change. A default that protects privacy, accessibility, or safety may be worth keeping even if it reduces engagement or convenience.

See the related skill on defaults and smart settings for deeper guidance on choosing and evolving defaults.

### Account For State, Sync, And Conflict

Settings often live across devices, sessions, accounts, and team members. Decide and communicate:

- whether a setting is per-device, per-account, or per-workspace;
- whether changes sync, and how quickly;
- what happens when two devices disagree;
- what happens when an admin overrides a personal preference;
- what happens when a setting is no longer available, such as after a downgrade or feature removal.

Unexplained conflicts erode trust. If a user changes a preference on their phone and it does not appear on their laptop, the design should make the scope clear before the confusion happens.

## Common Traps

### The Flat List Of Toggles

Dumping every option into one long scrollable list signals that no one prioritized the user's task. It forces scanning, invites mistakes, and hides the few consequential controls among dozens of trivial ones. Group, rank, and progressively disclose instead.

### Exposing Engineering Knobs As User Settings

Internal configuration such as cache size, polling interval, or API endpoint should not appear in a user-facing settings screen unless a real user has a reason to change it. Exposing implementation details turns users into accidental operators and generates support load.

### Labels That Describe The Control, Not The Outcome

"Enable flag X" describes the mechanism. "Let teammates invite new members" describes the outcome. Users act on outcomes. When a label only names the internal switch, users guess and guess wrong.

### Conflating Personal And Administrative Power

A single settings page that mixes personal theme choices with the ability to remove team members makes both feel casual. Administrative actions deserve visual and structural separation because their consequences extend beyond the current user.

### Hidden Scope And Silent Override

A setting that looks personal but actually governs a whole workspace, or a personal preference that an admin policy silently overrides, breaks the user's model of what they control. Make scope and override visible.

### No Path Back To Default

If a user cannot tell what the original default was, they cannot recover from experimentation. Provide reset-to-default or a visible indication of non-default state for settings where the original value matters.

### Search As An Excuse For Poor Organization

Search helps users who know what they want. It does not excuse a structure that no one can browse. A settings surface must be navigable without search, because many users do not know the exact vocabulary.

## Self-Check

- [ ] Every setting has a clear reason to exist, with a defined user who would change it and why.
- [ ] Personal preferences, feature configuration, and administrative controls are separated rather than mixed in one flat list.
- [ ] Settings are grouped by user goals and mental models, not by backend modules or service names.
- [ ] Each consequential setting shows what changes, who is affected, when it takes effect, and whether it is reversible.
- [ ] The surface supports both browsing by category and searching by intent, and is usable without search.
- [ ] Destructive, hard-to-reverse, or shared-impact settings require explicit confirmation and explain the blast radius.
- [ ] Defaults are chosen deliberately, documented, and revisit-ready rather than left to whatever the engineer picked.
- [ ] The scope of each setting (per-device, per-account, per-workspace) is clear, and sync or override behavior is communicated.
- [ ] Advanced or rarely needed controls are progressively disclosed without becoming the only path to important options.
- [ ] Users can identify which settings differ from default and can reset them when needed.
