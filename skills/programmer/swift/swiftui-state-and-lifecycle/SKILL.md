---
name: swiftui_state_and_lifecycle.md
description: Use when the agent is building SwiftUI views, choosing between @State @Binding @ObservedObject @StateObject @EnvironmentObject @Environment, managing view identity and lifecycle, diagnosing unnecessary re-renders, state propagation issues, navigation state, or performance problems in SwiftUI iOS and macOS applications.
---

# SwiftUI State and Lifecycle

SwiftUI makes declarative UI look effortless, and that effortlessness is the source of its hardest bugs. The property wrappers (`@State`, `@ObservedObject`, `@StateObject`, `@EnvironmentObject`) look interchangeable but have fundamentally different ownership and lifecycle semantics, and choosing the wrong one produces a distinct class of bug: a view that resets its state every time its parent re-renders, an observed object that is recreated and loses its state, an environment object that disappears and crashes the app, a list whose rows lose identity and animate incorrectly. The judgment problem is not "how do I add @State" but which wrapper owns the state, who observes it, and what happens to identity across re-renders.

The recurring failure mode is a developer who uses `@ObservedObject` where `@StateObject` was required (the object is recreated and loses state on parent re-render), or who shares a single `@State` across sibling views expecting them to sync (they do not), or who relies on `body` being called as a lifecycle hook (it is not, it is called for diffing and can be called many times or zero times). None of these surface as compile errors; they surface as flickering UI, lost scroll position, or crashes in production when an environment object is missing.

## Core Rules

### Match the property wrapper to ownership, not to convenience

The wrappers differ by who owns and who observes:

- **`@State`**: the *current view* owns a value-type state. Use for local, view-private UI state (toggle, selection, text field). The storage survives re-renders but is tied to the view's identity.
- **`@Binding`**: the view receives a reference to state owned by a parent. Use to pass writable state down without exposing the whole owner.
- **`@ObservedObject`**: the view *observes* a reference-type (`ObservableObject`) owned elsewhere. The view does not own it; if the parent recreates it, the view loses its observed state.
- **`@StateObject`**: the view *owns* a reference-type `ObservableObject` for its lifetime. Use when the view creates and owns the object (the correct choice for view models created in place).
- **`@EnvironmentObject`**: the view reads a shared `ObservableObject` injected from an ancestor. Missing it crashes at runtime.
- **`@Environment`**: reads environment values (system or injected), not for arbitrary shared state.

The decision is about ownership. If the view creates the object, use `@StateObject`. If it receives it, use `@ObservedObject`. If it shares it app-wide, use `@EnvironmentObject`.

### Identity drives state lifetime and animation

SwiftUI ties `@State` and `@StateObject` storage to the view's *identity*, not to its position in the tree. Two views with the same identity share state; views whose identity changes lose state. Rules:

- In `ForEach` and `List`, use stable identifiers (`id:`). Using array index as the id means reordering or deleting rows reassigns state to the wrong row.
- Conditional views (`if show { A() } else { B() }`) are different identities; state does not carry across the branch swap.
- Use `.id()` to force a view to reset, deliberately. Avoid `.id(someChangingValue)` accidentally, which resets state on every change.

If a view unexpectedly loses state, the first suspect is identity changing across re-renders.

### `body` is for diffing, not for lifecycle

`body` is called when SwiftUI decides the view needs diffing, which can be many times (parent re-render, animation frame) or zero times (view is offscreen). Do not:

- Perform side effects (network, persistence) in `body`. Use `.task`, `.onAppear`, or `.onChange`.
- Start long-running work in `body`.
- Read mutable global state in `body` expecting it to be called once.

Use `.task { }` for async work tied to the view's lifetime, `.onAppear`/`.onDisappear` for appear/disappear side effects, and `.onChange` for reacting to value changes.

### Hoist state to the common ancestor that needs to share it

Two sibling views that need to share state cannot each own it; the state must live in their common ancestor and be passed down via bindings or environment. Rules:

- Identify the lowest common ancestor that needs the state.
- Pass writable state down via `@Binding`; pass shared models via `@EnvironmentObject`.
- Avoid global singletons for view state; they break preview, testing, and multi-window scenarios.

### Minimize observable granularity to control re-renders

An `ObservableObject` notifies on any `@Published` change, and every observing view re-evaluates `body`. Rules:

- Split large objects so that a change in one area does not re-render unrelated views.
- In iOS 17+/macOS 14+, the `@Observable` macro enables per-property observation, reducing unnecessary re-renders; prefer it for new code.
- Avoid publishing in `body` or in a way that causes feedback loops (a change triggers a re-render that triggers a change).

### Navigation state is state, not configuration

Navigation (`NavigationStack`, `NavigationLink`, sheets, alerts) is driven by state (path bindings, item bindings). Rules:

- Drive navigation with bindings (`navigationDestination(for:)`, `.sheet(item:)`) so the destination is a function of state, not of side effects.
- Avoid deprecated programmatic navigation patterns that rely on `isActive` and side effects.
- Keep navigation state in the owning view or a router so deep links and state restoration work.

### Previews and tests need injected dependencies

`@EnvironmentObject` and injected services must be supplied in previews and tests or they crash. Design views so dependencies are injectable, and provide preview-friendly default values. A view that can only be instantiated with a real app environment is hard to test and preview.

## Common Traps

### `@ObservedObject` where `@StateObject` was needed

A view model created in place as `@ObservedObject` is recreated when the parent re-renders, losing state. Use `@StateObject` for objects the view owns.

### Index-based `ForEach` ids

`ForEach(items, id: \.self)` where items are value types can misbehave on reorder; using the array index reassigns state to the wrong row on deletion. Use a stable unique id per item.

### Side effects in `body`

Network calls, logging, or state mutation in `body` run unpredictably (many times or never). Move to `.task`, `.onAppear`, or `.onChange`.

### Missing `@EnvironmentObject` crashes

A view that reads `@EnvironmentObject` crashes if no ancestor injects it. Always inject at the root or guard with a default, and supply it in previews and tests.

### Accidental `.id()` resets

`.id(someValue)` on a view resets its state whenever the value changes. If `someValue` changes frequently, the view flickers and loses state. Use `.id()` deliberately to force reset, not incidentally.

### Sharing one `@State` across siblings

`@State` is owned by one view. Siblings reading the same value each have their own copy. Hoist to the common ancestor and pass bindings.

### Over-publishing causing render storms

A large `ObservableObject` that publishes on every minor change re-renders every observing view. Split the model or use the `@Observable` macro for per-property observation.

## Self-Check

- For each piece of state, is the property wrapper chosen by ownership (`@StateObject` for view-owned reference types, `@ObservedObject` for received, `@State` for local value, `@Binding` for passed-down writable, `@EnvironmentObject` for shared)?
- In every `ForEach`/`List`, is the identifier a stable unique id per item, not the array index or a value that can collide?
- Are side effects (network, persistence, logging) in `.task`/`.onAppear`/`.onChange`, never in `body`?
- Is shared state hoisted to the lowest common ancestor and passed via bindings or environment, rather than duplicated across siblings?
- Are observable models split or using `@Observable` to avoid re-rendering unrelated views on minor changes?
- Is navigation driven by state bindings, with deprecated side-effect-based patterns removed?
- Are `@EnvironmentObject` and injected dependencies supplied in previews and tests, with no missing-ancestor crashes?
- Are there any accidental `.id(changingValue)` usages that reset view state on every change?
