---
name: phoenix_and_liveview.md
description: Use when the agent is building Phoenix web applications or LiveView features, managing LiveView state and socket assigns, handling connection lifecycle and params, using PubSub and Presence, designing live navigation and form handling, optimizing re-rendering and diff payload size, or diagnosing state loss, memory growth, and slow LiveView updates in Elixir Phoenix applications.
---

# Phoenix and LiveView

Phoenix LiveView promises rich, reactive UIs without writing JavaScript, and that promise shifts the complexity from the client to the server's state model. A LiveView is a stateful process holding the UI state in `socket.assigns`, and every interaction re-renders the relevant template against that state and sends a diff to the client. The judgment problem is that this model makes state management look trivial (just update assigns) while hiding the costs of unbounded state growth, full re-renders, and diff payload size, and hiding the lifecycle reality that a LiveView process holds state for every connected client and must be torn down cleanly on disconnect. Get this wrong and you ship a feature that works for one user and falls over under concurrent load because each client's process holds too much state or recomputes too much on every keystroke.

The recurring failure mode is a developer who treats LiveView like a server-rendered page with magic updates: stuffs the entire database result set into assigns, re-runs expensive queries on every `phx-change`, holds large nested structures that trigger broad re-renders, or relies on process state that survives navigation when it should reset. The result is sluggish UIs, large diff payloads, and memory growth proportional to connected clients. Real LiveView performance and correctness come from modeling state minimally, scoping re-renders narrowly, and treating each LiveView as a process whose assigns, subscriptions, and lifecycle must be deliberate.

## Core Rules

### Model `socket.assigns` minimally and deliberately

`assigns` is the state the LiveView renders against, and it lives in a process per connected client. Rules:

- Store only what the UI needs to render, not the full domain model. Project domain data into a render-shaped struct.
- Avoid storing large collections in assigns; paginate, or store only the current page and a total count.
- Avoid storing derived values that can be computed in the template from base assigns; recompute on render rather than storing stale derivations.
- Every byte in assigns is held per connected client; multiply by expected concurrency when sizing.

A LiveView whose assigns grow with usage (appending history, caching results) leaks memory per client.

### Scope re-renders narrowly with `live_patch`, `live_redirect`, and targeted updates

LiveView sends only the diff, but the diff size depends on how much of the template changed. Rules:

- Use `phx-target` and form recovery to limit which component handles an event.
- Split large templates into function components and slots so changes localize to the changed component.
- Use `phx-update` (`append`, `prepend`, `ignore`) for lists that grow incrementally, so the diff appends one row instead of re-rendering the list.
- Avoid `phx-change` on large forms that re-validates and re-renders the whole form on each keystroke; debounce and target the field.

Broad re-renders on every interaction produce large diffs and sluggish UIs.

### Manage the LiveView lifecycle: mount, params, and navigation

- `mount/3` runs once on initial HTTP render (dead render) and again when the WebSocket connects (connected render). Distinguish with `connected?(socket)` for work that should run only when live (subscriptions, timers).
- `handle_params/3` runs on navigation (including `live_patch`/`live_redirect`) and on initial mount; use it to load data scoped to the current URL params, and to reset state that should not survive navigation.
- `live_patch` updates the URL without remounting (state preserved); `live_redirect` remounts a new LiveView (state reset). Choose by whether the new page should keep current state.

State that should reset on navigation but does not is a common bug; `handle_params` is where to reset it.

### Handle PubSub and Presence with explicit attach/detach

LiveView subscribes to PubSub topics in `mount`/`handle_params` and receives messages in `handle_info`. Rules:

- Subscribe to topics scoped to the view's data; avoid subscribing to global topics that flood every client.
- Unsubscribe is automatic when the LiveView process terminates, but be careful with resources you allocate (timers, ETS entries, external connections) that need cleanup in `terminate` or via process linking.
- Presence tracks connected clients per topic; use it for "who is online" features, and scope the topic to avoid cross-talk.

### Make event handlers pure functions of assigns + event

`handle_event` receives the event and params, updates assigns, and returns the new socket. Rules:

- Keep handlers focused: validate input, update assigns, and optionally push events back to the client. Do not do heavy work inline; offload to a GenServer or task for expensive operations.
- Return `{:noreply, socket}` with updated assigns; the re-render is automatic.
- Avoid side effects in handlers that depend on timing (e.g., reading external state that may change between render and handler); pass it through assigns.

### Use LiveComponents for stateful, repeated UI islands

A `LiveComponent` is a separate process with its own assigns, useful for repeated, stateful UI units (a row in a table that tracks its own edit state). Rules:

- Use LiveComponents when a UI unit has local state and repeated instances; do not use them for stateless presentation (function components suffice).
- Pass a stable `id` to each LiveComponent instance so LiveView can track it across updates.
- Keep LiveComponent state local; coordinate with the parent via sends/`send_update`, not by reaching into the parent's assigns.

### Handle forms and changesets with changeset-driven validation

Phoenix forms bind to Ecto changesets, and LiveView can validate on change. Rules:

- Use `phx-change` to validate incrementally, but debounce and target to avoid re-validating the whole form per keystroke.
- Use `phx-submit` for the actual save; validate the changeset again server-side.
- For large forms, consider recovering only changed fields (`phx-update` targeted) rather than re-rendering the whole form.

### Size and monitor for concurrency

Each LiveView is a BEAM process; BEAM handles many processes well, but each holds assigns and may subscribe to topics or hold resources. Rules:

- Profile assigns size and re-render cost under realistic concurrency.
- Use presence and connection counts to understand the live population.
- For broadcast-heavy features (notifications to many clients), prefer a single broadcast that each LiveView receives, over per-client sends.

## Common Traps

### Storing the full result set in assigns

Putting thousands of rows in assigns holds them per client and re-renders broadly. Paginate and store only the current page.

### `phx-change` re-rendering the whole form per keystroke

Without debouncing and targeting, each keystroke validates and re-renders the entire form, producing laggy input. Debounce and target the field.

### State surviving navigation unintentionally

`live_patch` preserves assigns; state that should reset on navigation persists and produces stale UI. Reset in `handle_params`.

### Subscribing to global topics

A global PubSub topic floods every LiveView with every change. Scope topics to the view's data.

### Broad re-renders from large nested assigns

Deeply nested assigns that change trigger large diffs. Split into components and use `phx-update` for incremental lists.

### Heavy work inline in `handle_event`

Expensive queries or computations in an event handler block the LiveView process and delay re-render. Offload to a GenServer or task.

### Forgetting cleanup in `terminate` or via linking

Resources allocated in `mount` (timers, ETS entries, external connections) leak if not cleaned up on disconnect. Link processes or clean up in `terminate`.

## Self-Check

- Are `socket.assigns` minimal and render-shaped, holding only what the UI needs, with large collections paginated rather than stored in full?
- Are re-renders scoped narrowly via components, `phx-target`, `phx-update` for incremental lists, and debounced/targeted form changes?
- Is the lifecycle handled correctly (`connected?` for live-only work, `handle_params` for data loading and state reset on navigation, `live_patch` vs `live_redirect` chosen by whether state should persist)?
- Are PubSub subscriptions scoped to the view's data (not global), with resources cleaned up in `terminate` or via process linking?
- Are event handlers focused on validate-update-return, with heavy work offloaded to GenServers/tasks rather than blocking the LiveView?
- Are LiveComponents used only for repeated stateful UI islands with stable ids, and is their state kept local with parent coordination via `send_update`?
- Are forms changeset-driven with debounced/targeted `phx-change` and server-side re-validation on `phx-submit`?
- Has assigns size and re-render cost been profiled under realistic concurrency, with broadcast-heavy features using single broadcasts rather than per-client sends?
