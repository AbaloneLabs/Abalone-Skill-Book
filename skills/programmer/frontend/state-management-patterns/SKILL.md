---
name: state_management_patterns.md
description: Use when the agent is designing application-level state architecture, classifying state into local, shared, server, URL, or derived categories, normalizing a store to avoid nested duplication, deciding whether a value should be computed or stored, persisting and rehydrating state across reloads or tabs, implementing optimistic updates with rollback, or planning undo and time-travel patterns. Also covers unidirectional data flow, cross-window synchronization, hydration-mismatch risks, and the cost of prematurely globalizing state. This skill is the app-wide state architecture layer and complements the component-level state placement covered in component-architecture-and-state.
---

# State Management Patterns

State management at the application level is a different problem from deciding where a single piece of state lives inside one component. The component-level decision (covered in component-architecture-and-state) asks "which component owns this value?" The architecture-level decision asks "what kind of value is this, what is its lifecycle, how does it relate to every other value in the app, and how do we keep the whole system consistent as it grows?" A feature can be built with perfect per-component boundaries and still collapse at the architecture level: the same entity stored in five shapes across four stores, derived values recomputed in eleven places, server data and client data merged into one mutating blob, a draft that survives reload in one tab but not another, an optimistic update that never rolls back when the request fails.

Agents tend to under-invest here because the architecture is invisible in the first feature. Dropping every value into one global object "for simplicity" feels pragmatic and ships the demo; the cost appears weeks later as a store nobody can reason about, where a change to one slice silently breaks an unrelated screen, where stale server copies drift from the real data, and where a "simple" undo feature turns into a rewrite because nothing was modeled as transitions. The judgment problem is not "which state library should I install" but a set of classification and discipline questions held together: *what kind of state is each value, where does its source of truth live, and what would have to stay consistent if this value changed, was reverted, or was loaded fresh in another tab?*

This skill assumes the component-level placement decisions are already handled well. It focuses on the cross-cutting architecture: the state taxonomy, normalization, derived-state discipline, persistence and rehydration, synchronization, and the patterns for optimistic updates and undo.

## Core Rules

### Classify Every Piece Of State Before Choosing Where It Lives

Most state-architecture bugs come from misclassification — treating one kind of state as if it were another. Before writing any value into a store, classify it against the taxonomy, because each category has different rules for ownership, staleness, and persistence.

- **Local / UI state.** Ephemeral, owned by one component, dies on unmount — an open/closed toggle, hover, in-progress text selection. Belongs in component state. Do not promote it.
- **Shared client state.** State needed across a subtree or the app that the client owns and the server does not — a draft, a multi-step wizard's accumulated input, a UI theme, a feature flag override. Belongs in the smallest scope that satisfies the sharing (context or a scoped store).
- **Server / remote state.** Data the server owns and the client caches — a user record, a list of orders, search results. Asynchronous, staleable, owned externally. Belongs in a server-state library with caching and invalidation, never hand-rolled into a client store.
- **URL / route state.** State that should survive a reload, a share, or a back-button press and that defines "what screen am I on" — filters, pagination, the active tab, a selected ID. Belongs in the URL/query string, not in memory.
- **Derived / computed state.** Anything that is a pure function of other state — a total from a list of items, a filtered list, a "can submit" flag. Belongs nowhere as stored state; compute it during render or in a selector.

The decisive test for each value: *who owns the truth, and what is its staleness lifecycle?* If the server owns it and it can be stale, it is server state. If it must survive a reload and be shareable, it is URL state. If it can be recomputed from other values, it is derived state. Write the classification down for non-trivial features; the act of classifying prevents most downstream mistakes.

### Normalize State To Avoid Nested Duplication

When the same entity appears in multiple places in a store, updates become a scavenger hunt: changing a user's name means finding every nested copy and updating each, and missing one produces UI that disagrees with itself. Normalization fixes this by storing each entity once, keyed by ID, with relationships expressed as references.

- **Keep entities in flat, ID-keyed maps.** A list of orders does not embed full user objects; it stores `userId`, and users live in a separate `users` map. Updating a user updates one record and every consumer sees it.
- **Express collections as arrays of IDs, not arrays of embedded entities.** A "recently viewed" list is `[id, id, id]`; the actual entities are resolved from the normalized map at read time.
- **Use selectors to denormalize at the read boundary.** Consumers ask for "the orders with their users" and a selector joins the normalized maps into the shape the view wants. The store stays normalized; the view gets the nested shape it needs.

Normalization is most valuable when entities are referenced from multiple places or updated independently. It is overkill for a one-off list that is never shared or mutated — do not normalize data that has no duplication to prevent. The cost of a normalized store is real (more indirection, selectors to write), so apply it where the duplication would otherwise hurt.

### Keep Derived State Computed, Never Stored

A value that can be computed from other state should be computed, not stored. Storing a derived value creates a second source of truth that must be synchronized back to the source on every change, and the synchronization is where bugs live — a missed update leaves the derived value stale, and the UI shows a total that does not match its line items.

- **Compute during render or in a memoized selector.** If the inputs are in the store, derive the output at read time. The source of truth is the inputs; the derived value is always consistent by construction.
- **Reserve stored state for values with an independent lifecycle.** A value is legitimately stored only when it is not a pure function of other state — user input, server data, a timestamped event. If you can write it as `f(otherState)`, it should not be stored.
- **Beware "I will keep them in sync" thinking.** Any time you find yourself writing code to update a stored value whenever another value changes, you have rediscovered derived state the hard way. Replace the sync code with a computation.

The exception that proves the rule: expensive derivations that recompute too often can be memoized (selectors, `useMemo`), but the memoization is a performance optimization of the computation, not a license to store the result as independent state.

### Maintain Unidirectional Data Flow So State Is Reasonable

State that changes through a single, traceable path is far easier to debug than state mutated from many call sites. Unidirectional flow means state updates flow one direction — events describe an intent, a reducer or store applies the change, and the view reads the new state. There is no back-channel, no direct mutation from a deeply nested component, no "I will just set this field directly."

The payoff is debuggability and predictability. When a bug appears, you can ask "what transition produced this state?" and find it in a serializable log of intents, rather than grepping for every place a field might be mutated. This is the real argument for reducer/store patterns over scattered `setState` calls in a complex feature: not that they are more powerful, but that they make the system legible. For simple local state the ceremony is not worth it; for state with many interacting transitions and many writers, unidirectional flow is what keeps it tractable.

### Persist And Rehydrate Deliberately, And Expect Hydration Mismatches

Persistence (saving state to `localStorage`, `sessionStorage`, or IndexedDB) and rehydration (restoring it on load) are where the most subtle bugs live, because the restored state must be consistent with the rest of the app and with the server.

- **Persist only what genuinely must survive, and version the persisted shape.** Persisting the entire store invites breakage when the shape changes; persist a minimal, explicitly-chosen subset, and include a version key so an old shape can be migrated or discarded rather than crash.
- **Never trust rehydrated state as authoritative.** Rehydrated client state is a hint, not truth — the server may have newer data. Treat persisted server-cache fragments as stale-on-arrival and revalidate.
- **Watch for SSR hydration mismatches.** If the server renders one value (e.g., a default theme, an empty cart) and the client rehydrates from `localStorage` to a different value, the framework throws a hydration mismatch and the DOM is patched or re-rendered. Gate persistence-dependent rendering behind a "mounted on client" check, or render a stable shell until hydration completes.
- **Avoid restoring state that references resources that no longer exist.** A persisted draft that points to a deleted entity produces broken UI. Validate rehydrated references against current data before trusting them.

### Synchronize Across Tabs And Windows Only When Required

Cross-tab synchronization (the `storage` event, BroadcastChannel, or a shared backend) is a real feature need for some apps — log out in one tab should log out everywhere; a cart update should reflect in an open second tab. But it adds complexity and failure modes, so apply it only where the behavior is actually required.

- **Use the `storage` event or BroadcastChannel for client-only sync.** When one tab writes, others listen and refresh their in-memory copy. Be explicit about what is synced and what is not — syncing everything re-introduces the coupling a scoped store was meant to avoid.
- **Treat the backend as the authoritative sync channel for server state.** For data the server owns, cross-tab consistency comes from revalidation against the server (or a websocket/push), not from clients broadcasting raw state to each other.
- **Handle the "two tabs edited the same thing" case.** Sync is not conflict resolution. Decide up front whether last-write-wins is acceptable or whether you need optimistic locking, and make the behavior obvious to the user.

### Model Optimistic Updates And Undo As Explicit Transitions

Optimistic updates (showing the result of a mutation before the server confirms it) and undo/time-travel are not ad-hoc tweaks; they are state-transition patterns that must be modeled deliberately, because the failure mode is a UI that lies.

- **For optimistic updates, keep the previous state so you can roll back.** Apply the optimistic change, fire the request, and on failure restore the snapshot you kept. An optimistic update with no rollback path is a bug waiting for the first network error.
- **Treat each change as an appendable transition for undo.** Undo works when state is a sequence of transitions (or snapshots) rather than an in-place-mutated blob. Reducer-style stores make this natural; direct mutation makes it impossible.
- **Bound the history.** Unlimited undo history is a memory leak. Keep a bounded ring of recent transitions or snapshots.
- **Reconcile undo with server state.** Undoing a client change that has already been persisted to the server means issuing a compensating mutation, not just reverting local memory. Decide whether your undo is local-only or full-round-trip and implement accordingly.

## Common Traps

### Stuffing Everything Into One Global Store And Calling It Architecture

Putting every value — UI toggles, server data, drafts, derived flags — into a single global object. The store becomes a coupling point where every screen depends on every slice, a change to one field re-renders broadly, and the file grows until nobody can hold it in their head. Classify first; keep server state in a server-state library and local state local.

### Denormalizing For Convenience And Then Drifting

Embedding full user objects inside every order "so the view doesn't have to join," then updating a user's name in three of the five places and watching the UI disagree with itself. Normalize by ID and denormalize at the read boundary with a selector.

### Storing A Derived Value And Writing Sync Code For It

Keeping a `total` in state and updating it in every place an item changes, then missing one update and showing a wrong total. Compute derived values from their inputs; delete the sync code.

### Persisting The Whole Store Unversioned

Serializing the entire store to `localStorage` with no version key. The next release changes one field's shape and every returning user crashes on load, or silently gets a corrupt hybrid. Persist a minimal, versioned subset and migrate or discard on version mismatch.

### Treating Rehydrated State As Authoritative

Restoring a persisted cart or user and trusting it without revalidating against the server, so a user sees items they already purchased or a role they no longer have. Treat rehydrated server-derived state as stale-on-arrival.

### SSR Hydration Mismatch From Client-Only Persistence

Rendering the persisted theme or locale on the client immediately, producing markup that differs from the server render and triggering a hydration error. Gate persistence-dependent UI behind a mounted check or render a stable shell first.

### Optimistic Update With No Rollback Path

Applying the optimistic change, firing the request, and not handling the failure case — so the first network error leaves the UI showing a state the server never accepted. Always keep the snapshot to restore on failure.

### Cross-Tab Sync Of Raw State Instead Of Revalidating and undo Implemented As Direct Mutation

Broadcasting the raw client store to other tabs for server-owned data, so two tabs converge on a stale copy instead of re-fetching. Use the server as the sync channel for server state; reserve client broadcast for client-owned state.

Allowing in-place mutation of state and then trying to add undo after the fact. Without transitions or snapshots, undo is impossible to reconstruct. Model changes as transitions from the start if undo is on the roadmap.

## Self-Check

- [ ] Every piece of state was classified before placement — local/UI, shared client, server/remote, URL/route, or derived — and the classification is consistent with who owns the truth and what the staleness lifecycle is (server state is in a server-state library, URL-shaped state is in the URL, derived state is not stored).
- [ ] Entities that appear in multiple places are normalized into flat ID-keyed maps with relationships expressed as references, and denormalization happens only in selectors at the read boundary — no nested duplicated entities that must be updated in more than one place.
- [ ] No value is stored that could be written as a pure function of other state; every "keep this in sync" code path was replaced by a computation or memoized selector, and stored state is reserved for values with an independent lifecycle.
- [ ] State updates flow through a single traceable path (events or intents into a reducer/store) for any feature with many interacting transitions, rather than being mutated from scattered call sites that make the system illegible.
- [ ] Persistence is deliberate and minimal — only state that must survive is persisted, the persisted shape is versioned, and old versions are migrated or discarded rather than crashing on load.
- [ ] Rehydrated state is treated as a hint, not truth — server-derived fragments are revalidated as stale-on-arrival, and references in restored state are validated against current data before being trusted.
- [ ] Persistence-dependent rendering does not cause SSR hydration mismatches — it is gated behind a client-mounted check or a stable shell until hydration completes.
- [ ] Cross-tab/window synchronization is applied only where the behavior is required, uses the appropriate channel (client broadcast for client state, server revalidation for server state), and has a defined behavior for concurrent edits.
- [ ] State that must be shared with or derived from the server is not duplicated into a client-owned global store where it can drift; the single source of truth for server-owned data is the server (or its cached representation with invalidation), and client stores hold only client-owned state.
- [ ] Every optimistic update keeps a rollback snapshot and restores it on failure, undo is modeled as appendable transitions or bounded snapshots rather than in-place mutation, and undo that must reach the server issues a compensating mutation rather than only reverting local memory.
