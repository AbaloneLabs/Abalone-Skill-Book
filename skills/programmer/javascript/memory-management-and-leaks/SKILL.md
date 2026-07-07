---
name: javascript_memory_management_and_leaks.md
description: Use when the agent is writing long-running JavaScript (SPAs, Node servers, Electron, workers) and must reason about garbage collection, retention, closures capturing large scopes, detached DOM nodes, event listeners and observers that outlive their owners, timers and pending callbacks, caches and WeakMap/WeakSet/WeakRef/FinalizationRegistry usage, large buffers and typed arrays, or is diagnosing growing heap usage, "the tab gets slower over time", out-of-memory crashes, or retention via frameworks and global registries. Covers the reachability model, common retention patterns, weak-reference primitives, and heap profiling.
---

# Memory Management And Leaks In JavaScript

JavaScript is garbage-collected, and that fact produces a widespread but false belief that memory is managed for you. The collector reclaims only what is unreachable, and in a long-running program — a single-page application, a Node server, an Electron app, a worker — it is remarkably easy to keep objects reachable forever without noticing. A closure that captures a large object just to read one field; an event listener left attached to a node that was removed from the DOM but is still held by a registry; a `Map` keyed by objects that grows without bound; a `setInterval` callback holding a reference to a whole view model; a framework subscription never unsubscribed; a cache with no eviction; a detached DOM subtree retained by a single reference. None of these crash immediately; they accumulate, the heap grows, GC pauses lengthen, and the application degrades or dies over hours or days. The judgment problem is to design every long-lived binding — closures, listeners, caches, subscriptions, registries — with a deliberate end of life, and to verify reachability under profiling rather than assume the collector handles it.

Agents tend to add bindings (a listener here, a subscription there, a cache for speed) and never remove them, because the cost is invisible in a short session and there is no syntactic reminder to clean up. The remedy is to treat any cross-lifetime reference as a potential leak, to prefer weak primitives (`WeakMap`, `WeakSet`, `WeakRef`, `FinalizationRegistry`) where the lifetime is genuinely "as long as the key exists", to scope listeners and subscriptions to a component lifetime with explicit teardown, and to profile the heap under realistic long sessions before claiming the code is leak-free.

## Core Rules

### Understand Reachability: An Object Is Retained While Anything Reachable References It

The GC traces from a set of roots (globals, the current stack, active listeners) and reclaims only unreachable objects. A single reachable reference — even an indirect one through a closure, a collection, or a registry — keeps an object and everything it transitively references alive. Leaks are therefore not "I forgot to free it" but "I left a path from a root to it."

- Before adding any long-lived binding, ask: what removes this reference, and when? If the answer is "nothing", it is a leak.
- Closures capture by reference the entire scope they are defined in; a closure that uses one field of a large object keeps the whole object (and everything it references) alive.
- Transitively, retaining one node of a large linked structure retains the whole structure.

### Scope Listeners, Observers, And Subscriptions To A Lifetime

The dominant leak class in browser and Node applications is a listener/observer/subscription whose owner is gone but the binding remains. Each binding holds the listener function, which closes over its definition scope, which holds the component's state and DOM nodes.

- Remove listeners with `removeEventListener` (requires the identical function reference and matching options; an inline anonymous function cannot be removed).
- Disconnect `IntersectionObserver`, `ResizeObserver`, `MutationObserver` when the component is destroyed.
- Unsubscribe framework subscriptions (RxJS, store subscriptions, event emitters) on teardown.
- Use `AbortController` to remove a batch of listeners and abort fetches at once: pass `signal` to each `addEventListener` and call `controller.abort()` on teardown.
- In Node, remove `EventEmitter` listeners (`emitter.removeListener` / `off`) and clear intervals/timeouts.

### Do Not Retain Detached DOM Nodes

A node removed from the document is garbage only if nothing reachable references it. A common leak is a reference in a `Map`, a closure, or a `WeakMap`-that-should-have-been-weak holding a detached subtree (a node plus all its descendants and their listeners). Detached subtrees are especially costly because each node carries listeners and child references.

- After removing a subtree, clear any cached references to it.
- Prefer event delegation (a single listener on a stable ancestor) over per-node listeners, so removing a node requires no listener cleanup.
- Use `WeakMap` when you need to attach metadata to a node without extending its lifetime.

### Bound Caches Explicitly Or Use Weak Primitives

A `Map` or `Set` with no eviction grows forever; in a long-running server it is a slow leak that eventually exhausts memory. Decide the cache policy explicitly: an LRU bound, a TTL, or a weak structure whose entries die with their keys.

- `WeakMap` and `WeakSet` hold keys weakly: when the key is otherwise unreachable, the entry is collected. Use them to attach metadata to objects without leaking.
- `WeakRef` lets you hold a reference that does not prevent collection; pair with `FinalizationRegistry` to clean up associated resources when the target is collected — but treat finalization as best-effort and non-timely, never as a deterministic destructor.
- For a bounded cache, use an LRU library; for a TTL cache, sweep on access and on a timer. Never use an unbounded `Map` for data whose key set grows without limit.

### Be Deliberate About Large Buffers And Typed Arrays

`ArrayBuffer`, `Uint8Array`, `Buffer` (Node), and large arrays of objects are retained as long as any view or reference exists. Transferring a buffer to a worker (`postMessage(buf, [buf])`) moves ownership and detaches the original, which is the intended zero-copy path; failing to transfer copies the buffer and doubles memory. Retaining a small view (`new Uint8Array(buf, 1000, 10)`) keeps the entire underlying buffer alive.

- Transfer buffers to workers when you no longer need them in the sender.
- Be aware a subarray view retains the whole buffer; copy (`slice`) if you need to release the parent.
- Stream large data rather than buffering it whole when possible.

### Verify With Heap Snapshots And Allocation Timelines

Reachability is hard to reason about fully by inspection; verify with tooling. In browsers, the DevTools Memory panel (heap snapshot, allocation timeline) lets you compare snapshots before and after a session iteration and see what is retained and by whom (the "retainers" view). In Node, `--inspect` plus the same panel, or `heapdump`/`v8.writeHeapSnapshot`, supports the same workflow.

- Take a snapshot, perform the suspected leaking action several times, take another snapshot, and compare; objects that grow across iterations are suspects.
- Inspect the retainer chain to find the reachable path keeping them alive; that path is the leak to break.

## Common Traps

### Closure Capturing More Than Intended

A `setTimeout(() => { log(this.bigObject.id) }, 1000)` keeps `this.bigObject` (and everything it references) alive until the timeout fires. Capture only what you need (`const id = this.bigObject.id; setTimeout(() => log(id), ...)`), and keep timers short or clear them.

### Listener Left On A Removed Node

`node.addEventListener('click', handler)` followed by `node.remove()` without `removeEventListener` leaks if `handler` is reachable (it usually is, via the component). Remove the listener, or use delegation.

### Unbounded Map Cache

`const cache = new Map(); cache.set(key, compute(key))` with unbounded keys leaks in a long-running server. Add an LRU/TTL bound or use `WeakMap` if keys are objects with their own lifetime.

### setInterval Holding A View Model

`setInterval(() => this.tick(), 1000)` on a component retains the component forever unless cleared. Clear the interval on teardown; prefer `requestAnimationFrame` or a framework lifecycle-aware timer.

### Detached Subtree Retained By A WeakMap-That-Is-Not-Weak

Storing DOM nodes as values in a strong `Map` (rather than as keys in a `WeakMap`) keeps them alive after removal. Use the node as the key, not the value, when attaching metadata.

### Framework Subscription Never Unsubscribed

A store/observable subscription created in a component but never unsubscribed retains the component via the subscription's callback. Unsubscribe in the destroy/teardown hook.

### Holding A Buffer View Keeps The Whole Buffer

`const header = new Uint8Array(hugeBuffer, 0, 20)` stored long-term retains `hugeBuffer`. Copy the slice if the parent can be released.

## Self-Check

- [ ] Every long-lived binding (listener, observer, subscription, timer) has a defined teardown path (`removeEventListener`, `disconnect`, `unsubscribe`, `clearInterval`, or `AbortController.abort`), and inline anonymous listeners that cannot be removed have been eliminated.
- [ ] Closures capture only the fields they need, not entire large objects or scopes, and short-lived closures over large state have been narrowed or replaced.
- [ ] Removed DOM subtrees have no lingering references in maps, caches, or closures; per-node listeners are replaced by delegation where the node set is dynamic.
- [ ] Caches have an explicit bound (LRU, TTL) or use `WeakMap`/`WeakSet`/`WeakRef` keyed by objects whose lifetime governs eviction; no unbounded `Map`/`Set` accumulates data with an unbounded key set.
- [ ] Large buffers are transferred (not copied) to workers when ownership moves, subarray views that would retain a parent are copied when the parent can be released, and large data is streamed rather than buffered whole.
- [ ] `FinalizationRegistry` (if used) is treated as best-effort and non-timely, not as a deterministic destructor, and has a fallback cleanup path.
- [ ] The application was profiled under a realistic long session: heap snapshots before/after repeated iterations were compared, retainers of growing objects were identified, and the retention paths were broken.
- [ ] The design has been considered under component teardown, route change, long-running session, and rapid creation/destruction, and remains bounded.
