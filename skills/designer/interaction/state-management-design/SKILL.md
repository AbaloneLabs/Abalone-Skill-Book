---
name: state_management_design.md
description: Use when the agent is designing how an interface models, tracks, and reflects state, including UI states, loading and error states, transient and persistent state, optimistic updates, caching and staleness, undo and history, offline and sync conflict states, or deciding which states must be represented to users and which can stay invisible without misleading them.
---

# State Management Design

Every interface is a view over state, and the correctness of the user's experience depends far more on how that state is modeled than on how the pixels are arranged. Users do not see databases or caches; they see whatever the interface chooses to surface, and the gap between the true system state and the displayed state is where most confusion, data loss, and broken trust originate. A design that handles only the happy, current, online state works in a demo and fails in the real world, where networks drop, requests race, data goes stale, and users undo their own actions. The judgment problem is deciding which states must be made visible to the user, which can remain invisible, how transient and persistent state interact, and how the interface stays honest when the underlying state is uncertain. Agents tend to fail by designing only the present-and-correct state, by hiding loading and error conditions that users need, and by treating state as an engineering concern rather than a design one.

Use this skill when designing any interface that reads, writes, or depends on data: forms, lists, dashboards, editors, collaborative tools, offline-capable apps, or any flow with async operations. The goal is a state model whose visible representation users can trust.

## Core Rules

### Enumerate The Full State Space, Not Just The Happy Path

The most common failure is designing the screen for the case where data exists, is current, and loaded successfully, and treating every other case as an edge case to handle later. Those other cases are where users actually get stuck.

Enumerate at minimum:

- loading and waiting states, for initial load and for subsequent updates;
- empty states, where no data exists yet or ever;
- error states, for network failure, permission denial, validation failure, and server errors;
- partial states, where some data loaded and some did not;
- stale states, where the displayed data may be out of date;
- offline and degraded states, where the connection is absent or unreliable.

A state you have not designed is a state the user will encounter with no guidance. Design the full space explicitly.

### Decide Which States Are Visible And Which Stay Invisible

Not every internal state belongs in the interface. Surfacing too much creates noise; hiding too much creates deception. The test is whether the user needs the information to act, trust, or understand.

Make a state visible when:

- it changes what the user can or should do, such as disabled actions during loading;
- it affects trust, such as showing that data may be stale or cached;
- it explains an unexpected result, such as why a value differs from expectation;
- the user's next action depends on knowing it.

Keep a state invisible when:

- it is purely internal plumbing that does not change the user's decisions;
- showing it would create anxiety without enabling action;
- the system can resolve it transparently and correctly.

The line is honesty, not completeness. Hide what does not matter; never hide what does.

### Distinguish Transient, Session, And Persistent State

State lives for different durations, and confusing them causes bugs and confusion. A draft that disappears on refresh, a filter that resets unexpectedly, or a preference that fails to save all stem from mismatched state lifetimes.

Clarify lifetimes:

- transient state, such as hover or an in-progress drag, lives only for the current interaction;
- session state, such as an unsaved draft or a temporary filter, lives until the session ends;
- persistent state, such as saved settings or created content, survives across sessions and devices.

Decide explicitly where each piece of state lives, communicate it through the interface (a saved indicator, a draft label), and never leave the user guessing whether their work is safe.

### Handle Asynchrony And Race Conditions Explicitly

Real interfaces make requests that take time, arrive out of order, or fail mid-sequence. Designs that assume operations complete instantly and in order break under real conditions.

Design for async reality:

- show that an operation is in progress, so the user knows the system received the input;
- prevent conflicting actions during pending operations, such as disabling a button after a submit;
- handle out-of-order responses, so a stale reply does not overwrite a newer one;
- decide what happens if the user navigates away mid-operation.

Optimistic updates, where the UI reflects the expected result immediately, can feel fast but must roll back cleanly on failure. An optimistic update that stays after an error is a lie the interface tells the user.

### Make Loading States Informative, Not Just Present

A spinner communicates only that something is happening. Better loading states tell the user what is happening, how long it might take, and what to do while waiting.

Improve loading states by:

- using skeleton screens that approximate the final layout, reducing perceived wait and layout shift;
- naming the operation in progress where it adds clarity;
- providing a path forward, such as a retry or a way to continue other work, when waits are long;
- avoiding indefinite spinners with no timeout or recovery path.

A bare spinner on a blank screen feels broken within seconds. Design loading as part of the experience, not a placeholder.

### Design Error And Recovery States As First-Class

Errors are not exceptional; they are inevitable, and the quality of the error state determines whether the user recovers or abandons. Treat error states with the same care as success states.

Strong error states:

- name the problem plainly and without blaming the user;
- preserve the user's input and context so they do not start over;
- offer a concrete recovery action, such as retry, edit, or contact support;
- distinguish retryable errors from those requiring a different action;
- keep working areas functional when only part of the screen failed.

An error that wipes the user's work or offers no path forward converts a recoverable moment into a lost one.

### Plan For Offline, Conflict, And Sync States

For any interface that creates or edits data, assume the connection will drop. Offline and sync states are not exotic; they are the normal operating condition for mobile and unreliable networks.

Plan for:

- explicit offline indication, so the user knows changes are not yet saved server-side;
- local persistence of unsaved work, so it survives app closure or refresh;
- clear conflict resolution when the same data changed in two places;
- sync status, showing what is saved, what is pending, and what failed.

Silently failing to save because the network dropped is among the most damaging state failures, because the user believes their work is safe when it is not.

### Provide Undo And History Where Actions Are Reversible

State changes that the user cannot reverse become sources of anxiety and error. Where an action is destructive or easily mistaken, provide a way back.

Provide recovery by:

- offering undo for a window after destructive or significant actions;
- maintaining edit history where feasible, so users can return to a prior state;
- confirming only genuinely irreversible actions, and making confirmation meaningful;
- avoiding the pattern of confirm-everything, which trains users to click through warnings.

Undo is almost always better than a confirmation dialog, because it lets users act and recover rather than predict and hesitate.

## Common Traps

### Designing Only The Happy, Current, Online State

The present-and-correct screen works in demos and fails in reality. Enumerate the full state space.

### Hiding States Users Need To Trust The Interface

Concealing loading, error, or staleness to look clean deceives users at the moments they most need honesty.

### Mismatched State Lifetimes

Drafts that vanish, filters that persist unexpectedly, or settings that fail to save all stem from unclear state duration.

### Optimistic Updates That Do Not Roll Back

Showing success before confirmation and leaving it on failure tells the user a lie. Roll back on error.

### Indefinite Spinners With No Recovery

A spinner with no timeout, no context, and no path forward feels broken within seconds.

### Errors That Destroy User Work

An error that clears input or offers no recovery converts a small failure into a lost user.

### Silent Save Failure On Unreliable Networks

Failing to save without telling the user, or letting unsaved work vanish on refresh, is among the most damaging failures.

### Confirm-Everything Dialogs

Overusing confirmation trains users to dismiss warnings and slows every action. Prefer undo.

## Self-Check

- [ ] The full state space is enumerated, including loading, empty, error, partial, stale, and offline states.
- [ ] Each state is explicitly decided as visible or invisible based on whether the user needs it to act or trust.
- [ ] Transient, session, and persistent state lifetimes are defined and communicated through the interface.
- [ ] Asynchrony and race conditions are handled, including out-of-order responses and navigation mid-operation.
- [ ] Optimistic updates roll back cleanly on failure rather than leaving a false success state.
- [ ] Loading states are informative, with skeletons, context, and recovery paths, not bare spinners.
- [ ] Error states preserve user input, avoid blame, and offer concrete recovery actions.
- [ ] Offline, sync, and conflict states are designed, with explicit status and local persistence of unsaved work.
- [ ] Reversible actions offer undo or history instead of relying on confirmation dialogs.
- [ ] No state is silently misrepresented in a way that could cause data loss or broken trust.
