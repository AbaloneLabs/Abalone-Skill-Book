---
name: customization_and_user_control.md
description: Use when the agent is designing settings, preferences, themes, layouts, configurable dashboards, user-controlled defaults, permission toggles, or any surface where the user can change how the product behaves, looks, or organizes information.
---

# Customization And User Control

Customization is not a feature you add. It is a contract with the user about who owns the interface. Every setting, toggle, theme, layout option, and configurable default is a decision about how much control the user holds, how much complexity they must absorb, and how much the product is willing to be different for different people.

The judgment problem is finding the right altitude. Too little control and the product feels rigid, paternalistic, or inaccessible to people whose needs differ from the assumed default. Too much control and the product drowns the user in options, breaks predictability, and shifts the burden of good design onto the user. A settings panel with two hundred toggles is not empowerment. It is abandonment dressed as flexibility.

Use this skill before designing or reviewing settings, preferences, personalization controls, configurable dashboards, themes, notification rules, layout editors, or permission surfaces. The goal is to prevent the agent from shipping customization that creates clutter, confusion, irreversible mistakes, or maintenance burdens while still giving users meaningful control over the things that genuinely vary.

## Core Rules

### Distinguish Needs That Genuinely Vary From Preferences That Fragment

Not everything users might want to change should be exposed. Before adding a setting, ask whether the need genuinely varies across users in a way the product cannot satisfy with a single good default.

Customization is usually justified when:

- accessibility needs differ (text size, contrast, motion, input method);
- workflows differ by role, expertise, or frequency;
- privacy and notification tolerances differ;
- cultural, linguistic, or regional expectations differ;
- device or environment constraints differ;
- the user's relationship to the content differs (owner versus viewer).

Customization is usually not justified when:

- it compensates for a weak default that could simply be better;
- it offers options whose effects the user cannot evaluate;
- it fragments the experience into combinations too complex to test;
- it exists because two stakeholders disagreed and neither would decide.

When two reasonable defaults compete, pick one and make it excellent rather than offering both as settings. Reserve customization for genuine, durable variation.

### Make Defaults Excellent Before Offering Escapes

The existence of a setting is not a substitute for a good default. If the default is poor, most users suffer it; only the small fraction who find and understand the setting escape. Design the default as though settings did not exist, then add customization for the cases the default cannot cover.

A strong default:

- works for the majority without configuration;
- is safe, accessible, and predictable;
- degrades gracefully when the user never opens settings;
- does not require the user to understand the product's internals.

If you find yourself adding a setting to fix a default that feels wrong, first try to fix the default. Settings should expand the space of good experiences, not patch holes in a mediocre one.

### Match Control Granularity To User Understanding

Users can only control what they can understand. A toggle whose effect is invisible, delayed, or entangled with other settings does not give control. It gives anxiety.

Offer control at a level the user can reason about:

- outcome-oriented controls ("show fewer notifications") over mechanism-oriented ones ("disable web push for category B");
- grouped preferences ("compact view") over dozens of individual toggles;
- previews of the effect where possible;
- clear statements of what changes and what does not.

Avoid exposing internal architecture as settings. The user does not care which service, cache, sync engine, or rendering layer is involved. They care about the visible outcome. If a setting must reference internals, explain the consequence in plain terms.

### Make State Visible, Reversible, And Resettable

Customization changes state, and changed state is a common source of confusion, support load, and perceived bugs. The user should always be able to see what they changed, undo it, and return to a known baseline.

For every customization surface:

- show the current value, not just the control;
- indicate when a value differs from the default;
- provide undo for destructive or disorienting changes;
- provide a reset-to-default path that is easy to find;
- persist changes predictably and communicate when they apply.

A user who turned on a high-contrast theme six months ago and forgot should be able to recognize that state and revert it. A user who accidentally collapsed their dashboard should not need to hunt for the restore control.

### Separate Personal Expression From Functional Configuration

Some customization is functional (it changes what the user can do or how efficiently). Some is expressive (it changes how the product feels or looks to reflect identity or taste). These have different design needs.

Functional configuration should be:

- organized by task and outcome;
- consistent across the product;
- documented and predictable;
- shared or synced where the user expects continuity.

Expressive customization (themes, avatars, layouts, names) should be:

- visible and rewarding;
- safe to change often;
- clearly scoped (just me versus everyone);
- not entangled with functional behavior.

Do not bury functional settings inside cosmetic menus, and do not force users through cosmetic choices before they can work. Keep the two layers distinct.

### Consider The Cost Of Every Supported Combination

Every setting multiplies the state space the product must support, test, and document. Ten independent binary settings produce over a thousand combinations. Most teams test only a handful.

Before adding settings, consider:

- how many combinations become possible;
- which combinations are known to break;
- how settings interact with each other;
- how settings interact with features, roles, and devices;
- how to test and support the long tail.

Prefer fewer, well-chosen, composable controls over many narrow toggles. If two settings interact, either make the interaction explicit or merge them into one control with clear modes.

### Respect Shared, Organizational, And Multi-User Scope

Customization often exists in a social or organizational context. A theme that is personal is safe to let users change freely. A layout, default, or permission that affects teammates, reports, or customers needs clearer scope and often needs governance.

Clarify for each setting:

- whether it affects only the current user;
- whether it affects people they share with;
- whether it affects subordinates, customers, or the public;
- who is allowed to change it;
- what happens when scopes conflict.

Do not let a personal preference silently override a shared standard, and do not let an administrator's choice trap an individual user with no override.

## Common Traps

### Settings As A Substitute For Decisions

When stakeholders disagree about the right default, the temptation is to add a setting so both win. This shifts the decision onto every user forever. Make the product decision, or split the feature, rather than exporting the disagreement.

### Exposing Implementation Detail As User Control

Toggles named after internal services, sync intervals, cache layers, or rendering flags confuse users and couple the UI to architecture that may change. Expose outcomes, not mechanisms.

### Irreversible Or Hard-To-Find Customization

A layout editor that cannot be reset, a theme that cannot be located once changed, or a collapsed panel with no restore path turns customization into a trap. Always provide a visible, low-friction way back to the default.

### Too Many Toggles, Too Little Grouping

A flat list of dozens of switches overwhelms users and makes the right choice impossible to find. Group by intent, lead with the most common decisions, and hide rarely needed advanced options behind a clear path.

### Customization That Cannot Be Shared Or Synced

Users expect their preferences to follow them across devices and sessions. Customization that is device-local, silently lost on logout, or impossible to export feels fragile and punishes investment in setup.

### Assuming Users Will Configure Before They Use

Most users never open settings. Designing the first-run experience as though users will configure first guarantees a poor default experience. Make the out-of-box path excellent and treat configuration as an enhancement.

## Self-Check

- [ ] Each exposed setting corresponds to a genuine, durable variation in user need, not a patch for a weak default.
- [ ] The default experience is strong, safe, and accessible without requiring any configuration.
- [ ] Controls are offered at a level the user can understand, with visible effects, previews, and plain-language consequences.
- [ ] Current values are visible, deviations from default are indicated, and reset-to-default and undo paths are easy to find.
- [ ] Functional configuration and expressive customization are separated and do not entangle each other.
- [ ] The supported combination space is understood, interactions are documented or merged, and critical combinations are tested.
- [ ] Scope is explicit for every setting: personal, shared, organizational, or public, with appropriate permissions and conflict behavior.
- [ ] Settings are grouped by intent, lead with common decisions, and avoid overwhelming flat lists of toggles.
- [ ] Preferences persist, sync, and export in the way users expect, and are not silently lost across devices or sessions.
- [ ] The first-run experience does not depend on configuration, and a user who never opens settings still gets a coherent product.
