---
name: memory_leaks_and_lifetime.md
description: Use when the agent is writing or reviewing code that manages objects, buffers, file handles, connections, caches, listeners, callbacks, closures, static or global state, long-lived collections, or background tasks in any language — including garbage-collected languages where leaks are assumed impossible. Covers memory and resource leaks, object lifetime and ownership, who is responsible for release, RAII and try-finally and defer, reference holding via closures/listeners/registries, unbounded caches and maps, circular references, retained large objects, shortening the lifetime of big payloads, GC tuning and generational behavior, weak references, memory profiling, and diagnosing OOM and slow-growth / creeping-memory incidents. Also use when designing lifecycle for resources whose cleanup must be guaranteed (sockets, file descriptors, native handles, subscriptions).
---

# Memory Leaks And Lifetime

A memory leak is not "I forgot to free a pointer." A leak is any case where an object's lifetime is longer than the work it serves — where something keeps a reference to data long after that data is useful, so the memory can never be reclaimed. In a manual language the leak is an explicit missed free. In a garbage-collected language the leak is far sneakier: the collector is working perfectly, but a forgotten reference in a long-lived collection, a closure, a listener, or a static field keeps the object "reachable" forever, so it is never collected at all. The GC does not leak; the program leaks by holding on.

Agents tend to miss this because garbage collection is widely misunderstood as "the language handles memory, so I do not need to think about it." That belief produces the most common production leak: a map that grows forever because entries are added and never removed, a callback registered and never unregistered, a cache with no bound, a closure that captures a large object the author forgot it closed over. The judgment problem is not "did I free this" but "what is the intended lifetime of this object, who is responsible for ending it, and have I made sure nothing holds a reference past that point." Every object has a lifecycle question attached to it; most leaks come from never asking it.

This skill focuses on object and resource lifetime and the leaks that come from getting it wrong. It is distinct from the caching skill (which covers bounded caches, eviction, and invalidation as a strategy) and the concurrency skill (which covers safe access to shared state). Here the question is: when should this object die, and what is keeping it alive.

## Core Rules

### Make Every Object's Lifetime And Owner Explicit

For any non-trivial object, especially one that holds a resource (a connection, a file handle, a buffer, a subscription, a large payload), answer three questions before writing the code:

- **What is the intended lifetime?** Request-scoped, session-scoped, worker-scoped, process-scoped, immortal? The answer should be the shortest lifetime that satisfies the work. Most leaks come from defaulting to "process-scoped" when "request-scoped" would do.
- **Who owns the end of the lifetime?** A specific component must be responsible for releasing, closing, unregistering, or evicting. "The GC will handle it" is not ownership; it is an abdication that works only for pure memory with no retained references and no underlying resource.
- **What could extend the lifetime past the intent?** A reference held by a longer-lived object — a collection, a closure, a listener, a static field, a cache, a thread-local — silently promotes the lifetime. The leak is the promotion.

The strong design makes lifetime visible in the type or structure: RAII in C++/Rust, `using`/`try-with-resources`/`defer`, scope-bound locals, owned vs borrowed distinctions. The weak design leaves lifetime implicit and hopes.

### Guarantee Resource Release, Do Not Rely On Remembering

Memory in a GC language is reclaimed automatically; resources (file descriptors, sockets, database connections, locks, native handles, subscriptions, temp files) are not. Releasing them by remembering to call `close` is a leak waiting for the one path that returns early or throws. Use a structure that makes release automatic:

- **RAII / ownership types.** The resource is released when the owner goes out of scope. This is the strongest model because release is tied to the language's lifetime rules, not to programmer discipline.
- **try-finally / try-with-resources / `using` / `defer`.** Release runs whether the block exits normally or by exception. Use this whenever RAII is unavailable.
- **Explicit lifecycle managers with shutdown hooks.** For long-lived resources (connection pools, schedulers, background workers), wire shutdown into the component lifecycle and into process-shutdown hooks so a clean stop actually closes everything.

The test of a release strategy is: "if every line between acquire and release throws, is the resource still freed?" If not, the strategy is incomplete.

### Treat Long-Lived Collections As Leak Sources By Default

The single most common leak in GC languages is an unbounded long-lived collection: a cache with no eviction, a map of "active sessions" that never removes closed ones, a registry that never unregisters, a list that only appends. These grow without limit and are never collected because the collection itself is reachable. Rules:

- **Every long-lived collection needs a bound and an eviction policy.** Size limit, TTL, LRU/LFU, or explicit removal on lifecycle events. An unbounded long-lived map is a leak even if every entry was correctly added.
- **Removal must be wired to the event that ends the entry's usefulness.** Session closed → remove from session map. Listener done → unregister. Job complete → remove from in-flight set. If no event triggers removal, the entry lives forever.
- **Prefer bounded data structures from the standard library** (LRU cache, bounded queue, weak-valued map) over hand-rolled maps that "should be fine."

If you cannot state the condition under which an entry leaves a long-lived collection, assume it never leaves.

### Audit Closures And Callbacks For Captured References

A closure or callback captures the variables it references — and through them, transitively, everything those variables point to. This is a frequent and invisible leak: a short-lived callback registered with a long-lived dispatcher closes over a large request object, a session, or a UI component, and the dispatcher keeps the callback alive long after the captured object is useful. Patterns to watch:

- **Listener / subscriber registration without matching unregister.** Registering `onEvent` on a global bus and never removing it keeps the listener (and everything it captures) alive for the bus's lifetime.
- **Closures that capture `this` or a large outer object.** A tiny callback can pin a huge object graph if it closes over a field of a large owner.
- **Capturing only what you need.** If a callback needs one id, capture the id, not the whole request. Narrowing the capture shrinks the retained graph.
- **Weak references for back-references.** When a long-lived object must reference a shorter-lived one (parent → child, cache → value), a weak reference lets the shorter-lived object be collected when nothing else holds it.

For every callback you register or closure you store, ask: what does it capture, how long is the holder alive, and is there a matching unregister?

### Break Or Avoid Circular References

Two objects that reference each other (A → B → A) form a cycle. In reference-counted systems (including some GC strategies, ARC in Swift/ObjC, and manual shared pointers) a cycle is a permanent leak: each keeps the other's count above zero forever. Defenses:

- **Make one direction weak.** Parent holds child strongly; child holds parent weakly. The cycle no longer prevents collection.
- **Prefer acyclic ownership.** If the relationship can be expressed as one owner and borrowers, do that instead of mutual strong references.
- **In tracing GC (Java, Go, .NET, JS), cycles are usually collected** — but only if nothing outside the cycle holds a reference. Do not assume "tracing GC handles cycles" means "cycles are safe"; a cycle reachable from a live root is still a leak.

State the ownership direction explicitly. Symmetric shared ownership of two objects is almost always a design smell.

### Shorten The Lifetime Of Large Objects

A 50MB buffer that lives for the whole request, when it is only needed for one early step, costs 50MB of peak memory it did not need to cost. Under load, the peak is what kills you, not the average. Rules for large or numerous objects:

- **Scope large allocations tightly.** Acquire the big buffer where it is used; let it go (drop the reference, null the field, exit the scope) the moment the work is done, before the rest of the request runs.
- **Stream instead of buffer.** Process records as they arrive rather than loading the whole dataset into memory. Streaming transforms a memory problem into a throughput problem.
- **Avoid retaining large objects in long-lived state.** A cache that stores full response bodies, images, or documents can hold gigabytes; cache keys, ids, or compact summaries instead.
- **Free native/off-heap buffers explicitly.** Byte buffers backed by native memory, memory-mapped files, or GPU memory are often not tracked by the GC and must be released explicitly; relying on finalization is unreliable and deprecated in most runtimes.

The peak-memory question — "what is the largest set of live objects at the worst moment, and could it be smaller?" — is more important than the average.

### Distinguish Transient Growth From A Real Leak

Not every rising memory graph is a leak. A process can grow because of a cache warming up, a backlog under load, generational GC not having collected young objects yet, or a legitimate increase in live work. Before declaring a leak, distinguish:

- **Transient / warm-up growth** stabilizes after the working set is reached. The curve flattens.
- **Load-driven growth** tracks throughput and falls when load falls.
- **GC lag** collects eventually; forcing a collection shows the real live set.
- **A real leak** grows monotonically and never falls, even at idle, even after forced collection, and survives restarts only in the sense that a fresh process starts the same climb.

Measure with a memory profiler (heap dump, allocation tracking, live-set over time) before changing code. "It looks like a leak" without a profile leads to fixing the wrong thing.

### Profile Before Tuning The GC

GC tuning (heap sizes, generation ratios, collector choice, pause targets) is the last resort, not the first. Most "GC problems" are actually leak or working-set problems: the collector is working hard because the program is holding too much live data, and no tuning fixes that. Sequence:

1. Profile the live set. What is actually retained, by what?
2. Fix leaks and over-long lifetimes first. This usually resolves the symptom.
3. Shrink the working set (streaming, tighter scopes, smaller caches).
4. Only then tune collector parameters to match the genuine working set and latency target.

Tuning the GC to tolerate a leak is paying forever for a fixable design problem.

## Common Traps

### "Garbage Collection Means I Don't Think About Memory"

The GC reclaims unreachable objects. Leaks in GC languages are reachable objects that are no longer useful — held by a collection, closure, listener, or static field. The collector is doing its job; the program is defeating it by holding references. Assuming "GC = no leaks" is the root cause of most managed-language leaks.

### The Unbounded Cache Or Map

A `Map` or `Dict` on a long-lived object that only has `put` and never `remove`, or a cache with no size limit and no eviction. It grows for the lifetime of the process and is the classic slow leak. Every long-lived collection needs a bound, an eviction policy, and a removal trigger.

### Listener Registered, Never Unregistered

Adding an event handler, observer, or subscriber to a long-lived dispatcher and never removing it. The dispatcher retains the handler, the handler retains its captured state, and both live for the dispatcher's lifetime. Repeated registration (e.g., per request or per session) multiplies the leak and can also cause duplicate-event bugs.

### Closure Capturing More Than Intended

A stored callback that closes over `this`, a request object, or a large local, when it only needs one id. The captured graph stays alive as long as the callback is stored. Capture the minimum; if the holder outlives the captured object, use a weak reference or copy the needed primitive out.

### Releasing Only On The Happy Path

Calling `close` at the end of a function but not in the exception path, or returning early before the cleanup. The resource leaks on every failure. Use try-finally / `defer` / `using` / RAII so release is structural, not a line you have to remember on every path.

### Static Or Singleton Holding Mutable State

A static field or singleton that accumulates state — a map, a list, a cache — lives for the process lifetime and is never collected. Code that "just adds to this list" is leaking into an immortal container. Treat mutable static state as a bounded, synchronized, lifecycle-managed resource, not a convenient global.

### Relying On Finalizers / Destructors For Cleanup

Finalizers (Java `finalize`, C# finalizers, `__del__`) run non-deterministically, may never run, and are deprecated or restricted in modern runtimes. They cannot be relied on to release file descriptors, sockets, or locks. Use explicit close / dispose / RAII; treat finalizers as a last-resort safety net, never as the primary cleanup path.

### Circular Strong References Under Reference Counting

In ARC or shared-pointer systems, two objects holding strong references to each other never reach zero count and leak forever. The fix is a weak reference on one side. Symmetric strong ownership across two objects is almost always wrong; decide which side owns the lifecycle.

### Holding A Large Object's Reference In A Long-Lived Scope

Keeping a full request body, document, or image alive for the whole handler when it was only needed for parsing. The peak memory under concurrency is the sum of all such over-long lifetimes. Drop references as soon as the big data is no longer needed; stream where possible.

### Diagnosing A Leak Without A Profile

Changing code based on "memory feels high" or a single rising graph. Without a heap dump or allocation profile, you cannot know what is retained or why. Always profile the live set before editing; the leak is usually not where intuition points.

## Self-Check

- [ ] For every non-trivial object and resource, the intended lifetime (request / session / worker / process / immortal) is the shortest that satisfies the work, and a specific owner is responsible for ending it — not "the GC will handle it."
- [ ] Resource release (file descriptors, sockets, connections, locks, native handles, subscriptions) is guaranteed structurally via RAII, try-finally, try-with-resources, `using`, or `defer`, so that release survives early returns and exceptions on every path.
- [ ] Every long-lived collection (cache, map, registry, session store, in-flight set) has a size bound, an eviction policy, and a removal trigger wired to the event that ends each entry's usefulness — none grows without limit.
- [ ] Every registered listener, subscriber, and stored closure was audited for captured references; each has a matching unregister, captures only what it needs, or uses a weak reference where the holder outlives the captured object.
- [ ] Circular references are broken with a weak link on one side (or avoided by acyclic ownership), especially under reference-counted memory; symmetric strong ownership across two objects was not used.
- [ ] Large or numerous objects have the tightest viable scope: references are dropped as soon as the data is no longer needed, streaming is used instead of buffering where feasible, and large objects are not retained in long-lived state.
- [ ] Native, off-heap, or memory-mapped buffers are released explicitly and not left to finalizers; finalizers/destructors are not the primary cleanup path for any resource.
- [ ] Mutable static or singleton state is treated as a bounded, lifecycle-managed resource, not a convenience global that accumulates forever.
- [ ] Before any GC tuning or "memory is high" fix, the live set was profiled (heap dump, allocation tracking, live-set-over-time) and the growth was classified as transient, load-driven, GC lag, or a real leak — and leaks were fixed at the retention source rather than tuned around.
