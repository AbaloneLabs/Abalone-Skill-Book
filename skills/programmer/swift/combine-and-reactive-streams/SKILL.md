---
name: swift_combine_and_reactive_streams.md
description: Use when the agent is writing Swift using Combine, publishers and subscribers, @Published, ObservableObject, sink/receive/assign, operators like map/flatMap/merge/combineLatest/debounce/throttle, Future and Promises, Subjects (PassthroughSubject/CurrentValueSubject), AnyCancellable lifecycle, custom publishers, or is diagnosing "the subscription never fires", "the publisher completes immediately", cancellables collected in the wrong place, backpressure, memory leaks from strong references in sink, or "the value updates twice". Covers Combine's publisher/subscriber model, cancellation and Cancellable lifecycle, operator composition, threading and receive(on)/receive(on:), and the pitfalls of uncollected cancellables and strong-reference cycles.
---

# Combine And Reactive Streams In Swift

Combine is Swift's declarative, publisher-subscriber pipeline for asynchronous values over time, and its core difficulty is that a pipeline is only alive while something holds its `Cancellable`. That single fact produces most Combine bugs. A publisher that emits values, a chain of operators that transforms them, and a `sink` that consumes them — none of this does anything unless the `AnyCancellable` returned by `sink`/`assign` is retained for the pipeline's lifetime. Store it in a property that lives long enough, and values flow; let it go out of scope, and the pipeline is cancelled silently, with no error, no warning — the subscription simply never fires. Layered on this are the threading rules (`receive(on:)`, `receive(on:options:)`, scheduler), the completion semantics (a publisher finishes once and then emits nothing), the backpressure model (a subscriber demands values), and the retain-cycle risk of capturing `self` strongly in a `sink`. The judgment problem is to manage the `Cancellable` lifecycle deliberately, to compose operators with their completion/error semantics in mind, to control threading at the boundaries, and to break retain cycles.

Agents build a pipeline, call `.sink { ... }`, forget to store the cancellable, and then debug "the network call never runs" or "the UI never updates" — the pipeline was cancelled the moment the local cancellable went out of scope. Or they capture `self` strongly in a `sink` stored on `self`, creating a retain cycle and a leak. The remedy is to store cancellables in a `Set<AnyCancellable>` (or a property) with the right lifetime, to use `[weak self]` in sinks that close over `self`, and to reason about completion and threading explicitly.

## Core Rules

### Manage The Cancellable Lifecycle: A Pipeline Lives Only While Its Cancellable Is Retained

Every subscription (`sink`, `assign`, `subscribe`) returns an `AnyCancellable`; when that cancellable is deallocated, the pipeline is cancelled and emits nothing further. This is Combine's central rule. Store cancellables in a collection (`Set<AnyCancellable>`) or property whose lifetime matches the pipeline's intended lifetime: a `Set<AnyCancellable>` on a view model for the view model's pipelines, a collection on a service for its long-lived subscriptions. A cancellable stored in a local variable or a function's temporary scope cancels the pipeline when that scope ends — the most common "it never fires" bug.

- `private var cancellables = Set<AnyCancellable>()` on the owner; store with `.store(in: &cancellables)`.
- For a one-shot pipeline that should run to completion independently, use `sink` and store the cancellable somewhere that lives long enough (or accept it may be cancelled when the owner deallocs).
- A `Future`-based pipeline and a `sink` whose cancellable is discarded cancels immediately; the work may or may not run.

### Break Retain Cycles With weak self In Sinks

A `sink` (or `assign`) closure that captures `self` strongly, stored in a cancellable collection on `self`, creates a retain cycle: `self` holds the cancellable, the cancellable holds the closure, the closure holds `self`. The object never deallocates. Use `[weak self]` in the closure and guard (`guard let self else { return }`) or optional chaining, so the closure does not keep `self` alive. `assign(to:\.property, on: self)` is a special case that does not retain — prefer it for property assignment. The cycle is invisible in normal operation and shows up as a leak in long sessions.

- `[weak self]` in any `sink` stored on `self` (or stored where `self` owns the storage).
- `assign(to:on:)` for direct property assignment (non-retaining).
- Verify with the memory graph debugger; a leaked view model with a Combine subscription is the classic finding.

### Control Threading With receive(on:) And sink At The Boundaries

Publishers emit on whatever thread/queue the source uses (a URL session on its background queue, a `Timer` on the run loop, a `PassthroughSubject` on the caller's thread). UI updates must happen on the main thread. Control threading explicitly with `receive(on:)` (switches the downstream to a scheduler) at the boundaries: `receive(on: DispatchQueue.main)` before a `sink`/`assign` that touches the UI. Do not assume the publisher's thread; do not call `DispatchQueue.main.async` inside a sink (use `receive(on:)` instead, which is the declarative Combine way). `subscribe(on:)` controls where the subscription/upstream work runs (e.g., a background queue for expensive setup), distinct from `receive(on:)` which controls where downstream values arrive.

- `receive(on: DispatchQueue.main)` before UI-affecting sinks.
- `subscribe(on:)` for where the upstream does its work.
- Be explicit; never rely on the publisher's default thread for correctness.

### Respect Completion And Failure Semantics

A publisher can emit zero or more values and then either complete successfully (`.finished`) or fail (`.failure`). Once it completes, it emits nothing further — a second subscriber to a completed `CurrentValueSubject` gets the current value then completion, a finished publisher does not restart. Operators propagate completion: `flatMap` completes when the outer and all inner publishers complete; `merge`/`combineLatest` complete per their rules. Error types matter: a pipeline's failure type must be compatible across operators; use `catch`/`replaceError`/`mapError` to transform or recover. A sink that ignores `.failure` (using the single-argument form) requires the publisher's failure to be `Never`; otherwise use the two-argument `sink(receiveCompletion:receiveValue:)` and handle errors.

- Handle completion and failure in `sink(receiveCompletion:receiveValue:)` for fallible publishers.
- Use `catch`/`replaceError` to recover; `mapError` to unify error types.
- Remember a completed publisher does not emit again; design subjects and pipelines around that.

### Choose Subjects And @Published For Bridging Imperative And Reactive

`PassthroughSubject` (no current value, broadcasts) and `CurrentValueSubject` (holds a current value) bridge imperative code (a delegate callback, a button tap) into a Combine pipeline. `@Published` on a property publishes its changes. Use them at the boundary between imperative and reactive code; do not expose subjects as the public API of a type (expose a publisher via `map`/`eraseToAnyPublisher()` to hide the subject and prevent external `send`). Subjects are hot: they multicast to current subscribers and do not replay (except `CurrentValueSubject`'s one current value).

- Expose publishers, not subjects, from a public API: `public var values: AnyPublisher<T, Never> { subject.eraseToAnyPublisher() }`.
- `@Published` for simple property observation in `ObservableObject`.
- Subjects multicast; use `share()`/`multicast` for explicit sharing of an upstream.

### Compose Operators With Their Semantics In Mind

Combine operators have specific semantics that are easy to misuse: `flatMap` can create many inner publishers (fan-out, cancellation concerns); `combineLatest` emits only after all sources have emitted at least once; `debounce`/`throttle` differ (debounce waits for quiet, throttle emits on a schedule); `removeDuplicates` requires `Equatable`. Read each operator's contract; do not assume SQL-like or Rx-like semantics without checking. Compose small pipelines and test them; a long opaque chain is hard to debug when a value does not arrive.

## Common Traps

### Cancellable Discarded, Pipeline Never Fires

`sink { ... }` whose returned cancellable is not stored cancels when it deallocates (immediately, if local). Store in a `Set<AnyCancellable>` with the right lifetime.

### Strong self In A Sink Stored On self

A retain cycle leaks the object. Use `[weak self]`.

### UI Updated Off The Main Thread

A publisher emitting on a background queue, sink touching the UI, no `receive(on: DispatchQueue.main)`. Add the scheduler.

### Ignoring Failure In A Single-Argument sink

`sink { value in ... }` requires failure `Never`; a fallible publisher does not compile or (with `replaceError`) swallows the error. Use the two-argument form.

### Completed Publisher Expected To Restart

A finished publisher emits nothing further; a second subscription gets completion. Design subjects/pipelines around single completion.

### Subject Exposed As Public API

External code calling `subject.send(...)` breaks encapsulation. Expose an `eraseToAnyPublisher()` view.

### Confusing debounce And throttle

`debounce` waits for a quiet period; `throttle` emits on a cadence. Mixing them up gives the wrong rate-limiting behavior.

### flatMap Fan-Out Without Limits

`flatMap` over an unbounded source can create many concurrent inner publishers, exhausting resources. Use `flatMap(maxPublishers:)` to bound concurrency.

## Self-Check

- [ ] Every subscription's `AnyCancellable` is stored in a collection/property whose lifetime matches the pipeline's intended lifetime; no cancellable is discarded to local scope.
- [ ] Sinks (and closures) that capture `self` use `[weak self]` or `assign(to:on:)`, so no retain cycle leaks the owner; verified with the memory graph debugger.
- [ ] Threading is explicit: `receive(on: DispatchQueue.main)` before UI-affecting sinks, `subscribe(on:)` for upstream work, and no reliance on the publisher's default thread for correctness.
- [ ] Completion and failure are handled: fallible publishers use the two-argument `sink(receiveCompletion:receiveValue:)`, errors are transformed/recovered with `mapError`/`catch`/`replaceError`, and a completed publisher is not expected to emit again.
- [ ] Subjects (`Passthrough`/`CurrentValue`) and `@Published` bridge imperative to reactive at boundaries, and subjects are not exposed as public API (publishers are exposed via `eraseToAnyPublisher()`).
- [ ] Operators are used with their actual semantics (`debounce` vs `throttle`, `combineLatest` all-must-emit, `flatMap` fan-out bounded with `maxPublishers`), not assumed from other frameworks.
- [ ] No pipeline silently fails to fire due to a discarded cancellable, and no UI update happens off the main thread.
- [ ] The reactive design has been considered under cancellation, completion, error, threading, and ownership, and remains correct and leak-free.
