---
name: failure_and_recovery_design.md
description: Use when the agent is designing what happens when a feature fails, including loading, empty, error, partial, offline, retry, degraded, and broken states, planning recovery paths, fallbacks, and graceful degradation across forms, dashboards, data views, and async flows.
---

# Failure And Recovery Design

Most design effort goes into the state where everything works. Most user trust is built or destroyed in the states where it does not. A product that loads perfectly but collapses into confusion when the network drops, a request times out, or a third party returns garbage will feel fragile no matter how polished the happy path is. Failure-and-recovery design is the discipline of deciding, in advance, what every surface does when its assumptions break.

Use this skill before shipping any feature that depends on a network request, a data source, an async job, a third-party integration, or user-generated content. The goal is to prevent the agent from leaving the failure states undefined, defaulting to a generic spinner or "error" screen, or building recovery paths that lose the user's context and work.

## Core Rules

### Enumerate The Failure Modes Before You Design The Success State

You cannot design recovery for failures you have not named. For each data-driven surface, list the realistic ways it can fail before choosing components.

Typical failure modes include:

- loading (no data yet, first load versus refetch);
- empty (valid state: no items exist yet);
- error (request failed, server error, malformed response);
- partial (some sections loaded, some failed);
- offline (device or network unavailable);
- stale (cached data shown while a refresh fails);
- permission (user lacks access to this resource);
- rate-limited or quota-exhausted;
- timeout (request in flight too long);
- degraded (a dependency is down but core still works).

Each mode deserves its own treatment. A single "loading or error" component guarantees that several of these states will be wrong.

### Separate Empty From Error

Empty state and error state are different problems with different recovery paths, and conflating them is one of the most common failures in data views.

- Empty means the request succeeded and there is genuinely nothing to show. Recovery is action-oriented: create, invite, search differently, add the first item.
- Error means the request failed and the absence of data is not trustworthy. Recovery is system-oriented: retry, check connection, contact support.

Showing "No results found" during a network failure tells the user their data is gone when it may simply be unreachable. Showing an error when the list is genuinely empty blocks the user from the create action they need.

### Make Loading States Informative And Bounded

A spinner with no context works for short waits and becomes anxiety-inducing for long ones. Loading states should communicate progress, set expectations, and avoid layout shift when data arrives.

Prefer:

- skeleton screens that mirror the final layout, reducing shift and signaling what is coming;
- progressive loading of sections that are ready, rather than blocking the whole page;
- elapsed or estimated time for long operations (uploads, exports, processing);
- a clear distinction between initial load (nothing yet) and refetch (stale data still visible);
- a way to cancel or navigate away from a long-running operation.

If a load can hang indefinitely, the loading state is incomplete. Define the timeout behavior and what the user sees when it triggers.

### Design Retry That Preserves Context

Retry is the most common recovery action, and it is frequently implemented in a way that throws away the user's position, selections, or draft. Good retry resumes rather than restarts.

Requirements:

- retry the failed operation, not the whole page or flow;
- keep the user on the same screen, scroll position, and selection state where possible;
- for forms, retry the submission without clearing fields;
- for lists, retry the failed request without resetting filters or pagination;
- show the outcome of the retry clearly (success confirmation, or the error again with the option to retry once more).

Retry that reloads the page or returns the user to the start punishes them for a failure they did not cause.

### Plan Partial And Degraded States

Many failures are not all-or-nothing. A dashboard may load four of five widgets; a page may show cached data while a refresh fails; a feature may work without its optional enrichment. Designing for partial success keeps the product usable instead of fully broken.

Approaches:

- render the sections that succeeded and mark the failed ones individually with their own retry;
- show stale cached data with a clear "could not refresh" indicator rather than hiding everything;
- disable or hide non-essential features that depend on a down service, while keeping core flows working;
- explain the degradation honestly ("Comments are temporarily unavailable") rather than silently failing.

A product that degrades gracefully feels robust; one that fails completely because one minor dependency is down feels fragile.

### Handle Offline And Connectivity Loss Explicitly

Mobile and unstable-network contexts need explicit offline handling. Silent failures during connectivity loss are a leading cause of "I saved it but it is gone" support tickets.

Design for:

- detecting loss of connectivity and signaling it without blocking unrelated actions;
- queuing actions (saves, sends, submissions) to retry when connectivity returns;
- distinguishing "saved locally, pending sync" from "saved to server" so the user knows what is durable;
- resolving conflicts when queued actions collide with server state;
- never implying success for an action that has not actually completed server-side.

### Preserve Intent Across Auth And Session Expiry

Session expiry mid-task is common and routinely destructive: the user submits a form, gets bounced to login, and returns to an empty page. The intent, the draft, and the destination are all lost.

Design session recovery so that:

- the action that triggered re-auth is queued and completed on return;
- the user returns to the page or step they were on, not the home screen;
- in-progress drafts are preserved across the auth interruption;
- the re-auth flow itself does not lose query parameters or modal state.

### Give Every Failure A Forward Path

No failure state should be a dead end. Even when the system cannot recover automatically, the user should have a clear next action: retry, go back, contact support, switch to an alternative, or understand that the problem is being addressed.

A dead-end error page with no action tells the user the product has given up. Always provide at least one concrete, working path forward.

## Common Traps

### One Generic Error State For Everything

A single full-screen "Something went wrong" applied to network errors, permission errors, empty states, and partial failures hides the real problem and removes the correct recovery path for each.

### Treating Empty As Error

Showing an error screen when a list is legitimately empty blocks the user from the create or invite action they need, and makes the product feel broken when it is working correctly.

### Infinite Or Silent Loading

A spinner that never resolves, with no timeout and no error, traps the user. If a load can fail, the loading state must be able to become an error state.

### Retry That Restarts The Flow

Reloading the page or returning to step one on retry discards the user's context and compounds the frustration of the original failure.

### Silent Success On Failure

Showing a success checkmark for an action that actually failed, or that was only saved locally, destroys trust when the user later discovers the work is missing.

### Failing Completely When One Part Breaks

Rendering an entire dashboard or page as broken because a single non-critical widget failed to load makes the product feel more fragile than it is.

### Hiding Staleness

Serving cached or outdated data with no indication that a refresh failed misleads users into acting on stale information, which is especially dangerous in financial, operational, or collaborative contexts.

### Dead-End Error Pages

An error screen with no retry, no back link, no contact option, and no explanation abandons the user at the moment they most need a path forward.

## Self-Check

- [ ] The realistic failure modes for each data-driven surface (loading, empty, error, partial, offline, stale, permission, timeout, degraded) are enumerated and each has a defined treatment.
- [ ] Empty states and error states are visually and behaviorally distinct, with action-oriented recovery for empty and system-oriented recovery for error.
- [ ] Loading states are informative, bounded by timeouts, and avoid layout shift when data arrives.
- [ ] Retry preserves the user's screen, scroll position, selections, filters, and draft rather than restarting the flow.
- [ ] Partial and degraded states render successful sections while clearly marking failed ones, rather than failing the whole surface.
- [ ] Offline and connectivity-loss states queue actions, distinguish local from server-saved, and never claim success for incomplete actions.
- [ ] Session and auth expiry preserves the user's intent, draft, and destination across the re-auth interruption.
- [ ] Every failure state offers at least one concrete forward path: retry, back, alternative, contact, or status.
- [ ] Stale or cached data is clearly indicated when a refresh fails, so users do not act on outdated information unknowingly.
- [ ] The failure states were reviewed under realistic conditions: slow network, lost connection, expired session, and partial dependency outage.
