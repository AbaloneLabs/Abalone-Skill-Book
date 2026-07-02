---
name: dotnet_runtime_and_gc.md
description: Use when the agent is writing .NET/C# code that allocates on the managed heap, implements IDisposable or finalizers, deals with large object allocation, boxing, GC pressure, pinning, weak references, or diagnosing memory leaks and latency spikes in managed applications.
---

# .NET Runtime and GC

The .NET garbage collector makes memory management feel automatic, and that feeling is the single biggest source of subtle performance and correctness bugs in managed code. Most developers treat "the GC handles it" as a license to allocate freely, then discover that generational promotion, large object heap fragmentation, finalizer queues, and unmanaged resource lifetimes quietly turn a responsive service into one with unpredictable latency spikes measured in hundreds of milliseconds.

The judgment problem here is not "how do I call Dispose." It is understanding that the managed heap is a *generational, compacting, concurrent* system whose behavior depends on object size, age, roots, and pinning, and that your allocation patterns directly control when and how long collections run. A class with a finalizer can keep an entire graph of objects alive across collections it would otherwise survive. A `List<T>` grown to 85,000 bytes lands on the LOH and never compacts. A captured variable in a closure roots a delegate that roots the object you thought you released. None of these show up as bugs in a unit test; they show up in production as memory growth, Gen 2 collections, and unexplained GC pauses.

## Core Rules

### Understand the generational model before optimizing

The GC uses three generations (Gen 0, Gen 1, Gen 2) plus the Large Object Heap (LOH) and the Pinned Object Heap (POH). The central assumption is that young objects die young and old objects are rarely referenced. Your job is to make that assumption hold:

- Short-lived allocations that die in Gen 0 are nearly free. Do not micro-optimize every `new`.
- Objects that survive to Gen 2 are expensive to collect. The goal is to minimize *promotion*, not to minimize allocation.
- Anything that roots an object longer than necessary (static fields, event handlers, caches, captured closures) pushes it toward Gen 2.

Ask, for any long-lived reference: what is the intended lifetime, and what is the *actual* lifetime including indirect roots?

### Size thresholds are load-bearing decisions

Objects >= 85,000 bytes go to the LOH. The LOH is not compacted by default, so it fragments. A pattern of allocating and releasing large arrays (e.g., per-request buffers) fragments the LOH and causes OutOfMemoryException even when total committed memory looks fine. Prefer:

- Array pooling (`ArrayPool<T>.Shared`) for transient large buffers.
- Reusing buffers across calls instead of allocating per request.
- `GCSettings.LargeObjectHeapCompactionMode = CompactOnce` for services that need periodic defragmentation, understanding the cost.

The 85,000-byte boundary also applies to objects *containing* large fields, so a struct with a large inline array crosses it too.

### IDisposable is about unmanaged and scarce resources, not memory

Implement `IDisposable` when your type owns unmanaged resources (handles, native memory) or owns other `IDisposable` objects (streams, connections, timers). The pattern is well-defined but commonly implemented wrong:

- Provide a `protected virtual void Dispose(bool disposing)` so subclasses can hook in.
- In `Dispose(true)`, dispose owned managed resources; in `Dispose(false)`, only release unmanaged resources (called from finalization where managed state may already be collected).
- Suppress finalization in `Dispose(true)` if you have one.
- Make `Dispose()` idempotent and safe to call multiple times.

Do not add `IDisposable` to pure managed types that hold no disposable resources. It adds ceremony without value and signals a lifetime contract callers must honor that the type does not actually need.

### Finalizers are a safety net, not a cleanup mechanism

Finalizers run on a dedicated thread at non-deterministic time, add pressure to the GC (objects with finalizers survive an extra collection), and can resurrect objects. They exist to release unmanaged resources when `Dispose` was forgotten. Rules:

- If you implement a finalizer, you almost always also implement `IDisposable`.
- Keep finalizer logic minimal and do not touch other managed objects from it (they may be finalized already).
- In .NET 5+, prefer `SafeHandle` to wrap unmanaged resources and let the runtime handle finalization rather than writing your own.
- Avoid finalizers on types that could be numerous; each one doubles the GC cost of that object.

### Boxing is a silent allocation tax

Boxing a value type allocates a new object on the heap. In hot paths it dominates allocation and causes Gen 0 churn and promotion. Common sources:

- Value types in non-generic collections (`ArrayList`, `Hashtable`) or via `object`/`interface` parameters.
- `enum` used as a dictionary key with a non-comparer that boxes.
- Logging calls that box arguments before a cheap guard check.
- `foreach` over non-generic `IEnumerable` on a struct collection.

Prefer generic collections, generic interfaces, and in-place formatting. When boxing is unavoidable in an interface boundary, measure whether the cost is real before restructuring.

### Pinning and unsafe references block the compactor

`fixed`, `GCHandleType.Pinned`, and `ref struct` parameters that the JIT must keep in place all prevent the GC from moving objects during collection. Short-lived pinning is fine. Long-lived pinning fragments the heap because the compactor works around the pinned region. In interop scenarios, prefer marshalling small structures by value over pinning large buffers, and prefer the POH (`GC.AllocateArray` with `pinned: true`) for buffers that must stay pinned.

### Measure before tuning GC behavior

GC settings (`<ServerGarbageCollection>`, `<ConcurrentGarbageCollection>`, latency modes) change collection frequency and pause characteristics. Server GC uses multiple heaps and is throughput-oriented; workstation GC is single-heap and lower-latency for UI. `GCLatencyMode` (Batch, Interactive, LowLatency, SustainedLowLatency, NoGCRegion) trades collection aggressiveness for pause length. Never change these based on intuition; capture GC event counts, Gen 2 collection frequency, and pause durations under realistic load first.

## Common Traps

### Treating `using` as optional for "short-lived" resources

A `FileStream` or `SqlConnection` opened in a method and not disposed relies on finalization, which delays resource release unpredictably and, for connections, exhausts the pool. "It gets garbage collected eventually" is the wrong mental model for scarce resources. Always `using`/`using var` disposable resources, even in tests.

### Event subscriptions that root objects forever

`publisher.Event += handler` creates a strong reference from the publisher to the subscriber. If the publisher outlives the intended lifetime of the subscriber, the subscriber is never collected. This is the most common managed memory leak. Detach on dispose, or use a weak-event pattern for long-lived publishers and short-lived subscribers.

### Static caches and singletons holding large graphs

A `static Dictionary` used as a cache grows without bound and every value is Gen 2 forever. Use `MemoryCache` with eviction policies, `ConditionalWeakTable` for attaching data without extending lifetime, or bounded structures. Audit every static field as a permanent root.

### Capturing `this` in long-lived lambdas

`someService.Register(x => this.Handle(x))` captures `this`. If the service holds the delegate indefinitely, you are rooted. Either unregister, capture a weak reference, or design the registration to be scoped.

### Over-relying on `GC.Collect()` to "fix" leaks

Calling `GC.Collect()` in production to mask a leak does not fix the leak; it hides the symptom and adds pause cost. The real fix is removing the root. Use it only in diagnostics or controlled shutdown, never as a runtime policy.

### Assuming LOH compaction is free

Setting `LargeObjectHeapCompactionMode = CompactOnce` then calling `Collect` does a full compacting Gen 2 collection. It is a full stop-the-world event. Use it during known idle windows, not per request.

### Finalizers that throw

An exception thrown from a finalizer is swallowed (or, depending on runtime version and config, terminates the process) and the resource is not properly released. Finalizers must never throw; guard everything.

## Self-Check

- For each `IDisposable` you own, is it disposed in every code path including exceptions, and is `Dispose(bool)` idempotent?
- Are there any event subscriptions (`+=`) without a matching detach or a clear owning scope? Trace each publisher's lifetime.
- Do any static fields, singletons, or caches hold object graphs that grow without bound or eviction?
- Have you checked whether hot-path allocations box value types or land repeated large buffers on the LOH? If so, is pooling in place?
- For every finalizer, is the logic minimal, free of managed-object access, and paired with `IDisposable` + `GC.SuppressFinalize`?
- Have you measured Gen 2 collection count and pause duration under realistic load before changing GC configuration or latency mode?
- Are pinned buffers either short-lived or allocated on the POH, rather than pinned long-term on the regular heap?
- Does any captured lambda in a long-lived registration implicitly capture `this`? If so, is the registration scoped or unregistered?
