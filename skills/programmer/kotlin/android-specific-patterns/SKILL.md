---
name: android_specific_patterns.md
description: Use when the agent is building Android apps in Kotlin, handling activity and fragment lifecycle, using ViewModel and LiveData or StateFlow, managing Jetpack Compose state, scheduling background work with WorkManager or services, requesting runtime permissions, avoiding memory leaks from contexts and observers, or scoping coroutines to lifecycle in Android applications.
---

# Android-Specific Patterns

Android development is dominated by lifecycle, and lifecycle is the root of most non-trivial bugs. Activities and fragments are destroyed and recreated on configuration changes; background work outlives the UI that started it; observers and listeners hold references to destroyed contexts; the main thread blocks if you do work on it and crashes if you touch views off it. The judgment problem is not "how do I use a ViewModel" but designing around the fact that any component you hold a reference to may be destroyed and recreated at any time, and that the OS will kill your process to reclaim memory whenever it likes.

The recurring failure mode is a developer who treats Android like a normal Kotlin app: stores state in an Activity field, holds a reference to a View in a long-lived object, starts a coroutine on `GlobalScope` that outlives the screen, or assumes `onDestroy` means the user is done. Configuration change recreates the Activity and wipes the field; the long-lived View reference leaks the old Activity context; the global coroutine updates a dead UI; the "done" assumption breaks because the process is recreated from a saved instance state. Every Android pattern (ViewModel, lifecycle scopes, WorkManager, saved state) exists to manage one of these lifecycle realities, and using them correctly is the difference between a stable app and one that crashes on rotation.

## Core Rules

### Design around configuration change, not against it

By default, the system destroys and recreates Activities and Fragments on configuration changes (rotation, locale, dark mode). Any state stored in the Activity/Fragment is lost. Rules:

- Store UI state in a `ViewModel` (survives configuration change) or in `SavedStateHandle`/saved instance state (survives process death).
- Do not disable configuration change (`configChanges`) to avoid recreation; that papers over the lifecycle and breaks multi-window and other features.
- Treat the Activity/Fragment as a transient controller; the ViewModel is the stable owner of state.

If a field on the Activity is non-null after rotation and you expected it to be set, the lifecycle model has been violated.

### Scope coroutines to lifecycle, never to global scope

A coroutine started on `GlobalScope` outlives the screen, leaks, and updates a destroyed UI. Rules:

- Use `lifecycleScope` for work tied to the Activity/Fragment lifecycle; it cancels on `DESTROYED`.
- Use `viewModelScope` for work tied to the ViewModel; it cancels when the ViewModel is cleared.
- In Compose, use `rememberCoroutineScope()` tied to the composition.
- Never use `GlobalScope` or `Dispatchers.Main` ad hoc without a scope that will cancel.

Cancellation must propagate: a coroutine that ignores cancellation keeps running after the user navigated away.

### Choose ViewModel state exposure by observer model

A ViewModel exposes state to the UI. The choice of `LiveData`, `StateFlow`, or `SharedFlow` depends on the observer model:

- `StateFlow`: the modern default for state; requires an initial value, conflates to the latest, and is lifecycle-aware via `repeatOnLifecycle`/`collectAsStateWithLifecycle`.
- `SharedFlow`: for one-shot events (navigation, snackbar) where you do not want replay to new collectors; configure replay and buffering deliberately.
- `LiveData`: older, lifecycle-aware by default; acceptable in existing code but prefer Flow in new code.

When collecting a Flow from the UI, use `repeatOnLifecycle(STARTED) { collect }` so collection pauses when the UI is not visible, avoiding wasted work and stale updates.

### Manage Compose state and recomposition deliberately

Jetpack Compose recomposes when observed state changes. Incorrect state hoisting causes unnecessary recompositions or lost state. Rules:

- Hoist state to the lowest common ancestor that needs it; pass state down and events up.
- Use `remember`/`rememberSaveable` for UI state; `rememberSaveable` survives process death.
- Avoid creating new objects on every recomposition (e.g., lambdas, modifiers) without `remember`; they cause unnecessary recomposition and break equality-based skipping.
- Mark composables `stable` or use stable types so the compiler can skip recomposition when inputs are equal.

A composable that recomposes on every frame is usually creating unstable inputs (new lambda/modifier instances) on each pass.

### Avoid context leaks

Holding an Activity context in a long-lived object (singleton, static field, observer not tied to lifecycle) leaks the entire Activity and its view hierarchy. Rules:

- Use the Application context where a long-lived component needs a context (e.g., for `LayoutInflater`-independent work, database access).
- Use the Activity context only where an Activity-scoped API is required (e.g., showing a dialog, inflating views).
- Detach observers and listeners in `onDestroy`/`onCleared`; an observer on a long-lived subject that holds an Activity reference leaks the Activity.

The rule of thumb: if the object outlives the Activity, it must not hold the Activity context.

### Schedule background work with the right primitive

- **Foreground work the user is waiting on**: run on a lifecycle scope, with progress in the UI.
- **Deferrable, guaranteed background work**: `WorkManager` (persists across reboot, respects constraints like network/charging).
- **Immediate background not needing persistence**: a foreground service (with notification) for short, user-visible work.
- **Exact timing**: `AlarmManager`, reserved for user-visible time-sensitive events.

Do not use a raw service or thread for work that must survive process death; it will not. Do not use `WorkManager` for work the user is actively waiting on; it is deferred.

### Request permissions at the point of need, not at startup

Runtime permissions requested at startup annoy users and are often denied. Request when the feature requiring the permission is first used, with a rationale if previously denied. Handle denial gracefully (degrade the feature, explain, link to settings). Rationale and denial paths are part of the feature, not afterthoughts.

### Persist state for process death explicitly

The system can kill your process at any time to reclaim memory, then recreate it from saved instance state. Rules:

- Use `SavedStateHandle` in the ViewModel, or `onSaveInstanceState`, for minimal UI state needed to restore.
- Do not put large objects in saved instance state (it is serialized and has size limits); persist to database/storage and restore by ID.
- Test process death restoration using developer options ("Don't keep activities") to catch state-loss bugs.

## Common Traps

### Storing state in Activity fields

Rotation wipes Activity fields. Use ViewModel or saved state for anything that must survive configuration change.

### `GlobalScope` coroutines

Coroutines on `GlobalScope` outlive the screen, leak, and update destroyed UIs. Use `lifecycleScope`/`viewModelScope`.

### Holding Activity context in a singleton

A singleton holding an Activity context leaks the Activity. Use Application context for long-lived components.

### Observers not removed

An observer on a long-lived subject (e.g., a repository singleton) that holds an Activity/Fragment reference leaks it. Tie observers to lifecycle or detach in `onDestroy`.

### Disabling configuration change to avoid recreation

`android:configChanges` papers over lifecycle bugs and breaks multi-window and locale handling. Fix the lifecycle instead.

### Collecting Flows without lifecycle awareness

`flow.collect` in `onCreate` keeps collecting after the UI is destroyed and can crash or update a dead UI. Use `repeatOnLifecycle` (or `collectAsStateWithLifecycle` in Compose).

### Compose unstable inputs causing recomposition storms

New lambda/modifier instances on every recomposition prevent skipping and recompose the whole subtree. Remember stable inputs or hoist them.

## Self-Check

- Is UI state stored in `ViewModel`/`SavedStateHandle`/saved instance state rather than Activity/Fragment fields, surviving configuration change and process death?
- Are coroutines scoped to `lifecycleScope`/`viewModelScope`/composition scope, with no `GlobalScope` usage, and do they propagate cancellation?
- Is state exposed via `StateFlow`/`SharedFlow` (or `LiveData`), collected with lifecycle awareness (`repeatOnLifecycle`/`collectAsStateWithLifecycle`)?
- In Compose, is state hoisted to the lowest common ancestor, saved with `rememberSaveable` where needed, and are inputs stable to allow skipping?
- Do long-lived components hold the Application context, not the Activity context, and are observers/listeners detached on `onDestroy`/`onCleared`?
- Is background work scheduled with the right primitive (lifecycle scope for foreground, `WorkManager` for persistent deferrable, foreground service for immediate user-visible)?
- Are runtime permissions requested at point of need with rationale and graceful denial paths?
- Is minimal UI state persisted for process death via `SavedStateHandle`/`onSaveInstanceState`, with large data stored externally and restored by ID, and has process-death restoration been tested?
