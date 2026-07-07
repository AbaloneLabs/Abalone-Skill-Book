---
name: loading_and_waiting_states.md
description: Use when the agent is designing the full range of loading and waiting states across a product, including initial loading, background refresh, empty states that follow loading, partial data, offline and reconnect states, stale data indicators, and the messaging that keeps users oriented while the system works.
---

# Loading And Waiting States

Loading is not a single spinner. It is a family of states that span the entire range from a sub-second fetch to a long offline wait, and each member of that family needs different treatment. Products fail at loading in two opposite ways: they either show nothing, leaving users to wonder if the system is broken, or they show a generic spinner for every situation, communicating nothing about what is happening, how long it will take, or what the user should do. The judgment problem is that loading states are usually designed last, for the happy path, and then collapse the first time the network is slow, the data is partial, or the user goes offline.

Use this skill before finalizing any screen that fetches, refreshes, or depends on remote data, and before deciding how the product behaves during initial load, background refresh, empty results, partial failure, and connectivity loss. The goal is to prevent the agent from shipping a single loading treatment and ignoring the full spectrum of waiting the user will actually encounter.

## Core Rules

### Design The Full Spectrum Of Waiting States

Loading is not one state; it is a spectrum, and each point needs its own design. At minimum, plan for:

- initial load, when content is fetched for the first time and nothing exists yet;
- background refresh, when existing content is being updated without blocking the user;
- empty state, when loading completes but there is no data to show;
- partial data, when some content arrived but some failed;
- stale data, when the displayed content may be out of date;
- slow load, when the wait extends beyond expectations;
- offline and reconnect, when the network is unavailable or returns.

A single spinner cannot serve all of these. Initial load may need a skeleton; background refresh may need only a subtle indicator; empty state needs explanation and next steps; partial data needs to distinguish what loaded from what did not.

### Distinguish Initial Load From Background Refresh

Initial load and background refresh are different experiences. On initial load, the screen has no content, so the user needs structure and reassurance that something is coming; skeleton screens or branded loading states work well. On background refresh, content already exists, so the user should not be blocked or have their place disrupted; a subtle indicator such as a small spinner in the header or a pull-to-refresh confirmation is enough.

Treating background refresh like initial load, by blanking the screen and showing a full spinner, destroys the user's context and makes the product feel slow. Treating initial load like background refresh, by showing nothing, makes the user think the screen is broken.

### Always Design The Empty State

The empty state is the moment after loading completes and there is nothing to show: no messages, no results, no recent activity, no data yet. It is one of the most commonly skipped states and one of the most important, because it is often the first thing a new user sees. A blank screen or a bare "no results" leaves the user uncertain whether the system is broken, the data failed to load, or there is genuinely nothing.

A good empty state explains why it is empty, what the user can do to change it, and offers a clear next action: create the first item, adjust filters, invite collaborators, or check back later. It turns a dead end into an on-ramp.

### Handle Partial And Stale Data Honestly

Real systems often return partial data: some items loaded, some failed, some are cached and possibly out of date. Hiding this from the user creates false confidence; the user acts on stale or incomplete information and is surprised by the consequences. surfacing it honestly builds trust.

Use indicators to distinguish fresh data from cached or stale data, show which sections failed to load with a retry option, and never present partial data as complete. A small "updated 2 hours ago" or "some items could not be loaded" badge is far better than silently showing an incomplete picture.

### Provide Escape Paths For Long Waits

When a load extends beyond a few seconds, the user needs options beyond staring at a spinner. They should be able to continue using other parts of the product, retry the load, cancel it, or be notified when it completes. Trapping the user on a loading screen for an extended duration is a usability failure.

For genuinely long operations, consider asynchronous patterns: let the user move on, complete the work in the background, and notify them when it is done. The loading state should respect the user's time, not hold it hostage.

### Communicate Connectivity Status Clearly

Offline and reconnect states are part of the loading family and are frequently neglected. When the user is offline, the product should say so clearly, show what is still available from cache, queue actions to sync later, and avoid letting the user attempt actions that will silently fail. When connectivity returns, the product should reconnect, sync queued actions, and confirm that the system is back in a working state.

Silent failure during connectivity loss, where actions appear to succeed but actually fail, is one of the most damaging loading-related bugs. Always make the connectivity state visible and its consequences clear.

### Keep Loading Feedback Proportional And Calm

Loading feedback should match the situation without alarming the user. A brief background refresh does not need a prominent banner; a total failure does not need to be hidden. Over-alerting during routine loads makes the product feel fragile; under-alerting during real problems leaves users stranded.

Aim for calm, proportional feedback: subtle indicators for normal operations, clear messaging for problems, and never panic-inducing language for routine waits.

### Preserve User State Across Loads

When a load completes or refreshes, the user's context should survive: scroll position, selections, expanded sections, and form input should not be reset. A refresh that returns the user to the top of a long list, or that discards a half-entered form, punishes the user for the system's need to reload.

Design loads and refreshes to be non-destructive, updating content in place and preserving the user's working state wherever possible.

## Common Traps

### One Spinner For Every Situation

Using a generic loading indicator for initial load, refresh, empty results, partial failure, and offline collapses meaningfully different states into one uninformative symbol.

### Blank Screen During Initial Load

Showing nothing while content fetches makes the user assume the screen is broken, especially on slow connections.

### Skipping The Empty State

Failing to design the post-load empty case leaves new users staring at a dead screen with no path forward.

### Presenting Partial Or Stale Data As Complete

Hiding which sections failed to load or which data is outdated leads users to act on false information.

### Trapping Users In Long Loads

Loading screens that block navigation for extended durations offer no escape and disrespect the user's time.

### Silent Failure When Offline

Actions that appear to succeed during connectivity loss, then fail without notice, destroy trust and cause data loss.

### Resetting User State On Refresh

Refreshes that discard scroll position, selections, or input punish users for routine system behavior.

### Over-Alerting During Routine Loads

Prominent banners and warnings for normal background operations make the product feel fragile and anxious.

## Self-Check

- [ ] The full spectrum of waiting states is designed: initial load, background refresh, empty, partial data, stale data, slow load, offline, and reconnect.
- [ ] Initial load and background refresh use different treatments, with context-preserving indicators for refresh.
- [ ] Every empty state explains why it is empty and offers a clear next action rather than leaving a dead end.
- [ ] Partial and stale data are surfaced honestly, with indicators distinguishing fresh, cached, and failed sections.
- [ ] Long waits provide escape paths: continue elsewhere, retry, cancel, or be notified on completion.
- [ ] Offline and reconnect states are visible, with cached content shown, actions queued, and sync confirmed on return.
- [ ] Loading feedback is proportional and calm, subtle for routine operations and clear for real problems.
- [ ] User state, including scroll position, selections, and input, survives loads and refreshes.
- [ ] No loading treatment silently fails or presents incomplete data as complete.
