---
name: jvm_internals_and_memory_model.md
description: Use when the agent is writing or reviewing Java code where JVM internals affect correctness or performance — choosing primitive vs boxed types, designing for garbage collection, tuning heap or metaspace, using finalizers or Cleaner, reasoning about the Java Memory Model and happens-before, volatile and synchronized visibility, classloading, reflection, or diagnosing OutOfMemoryError, GC thrash, metaspace leaks, classloader leaks, stale reads, safe-publication bugs, or finalization never running. Covers heap generations, GC algorithms, object layout, the JMM, classloading, and the hazards of finalization and native memory.
---

# JVM Internals And Memory Model

Java's promise is "write once, run anywhere" with automatic memory management, but the JVM underneath is a concrete machine with a concrete memory model, a concrete garbage collector, and concrete rules about when one thread's writes become visible to another. Code that ignores these details usually works in development and fails in production: an `OutOfMemoryError` under sustained load because boxed integers inflated the heap, a `ConcurrentModificationException` or silent stale read because visibility was assumed rather than established, a metaspace leak because a classloader was pinned, a "finalizer never ran" leak because the JVM made no guarantee it would. The JVM does not protect you from these — it makes them possible and then makes them invisible until they bite.

The judgment problem is twofold. First, memory: where objects live (heap vs stack vs metaspace vs native), how big they are, how long they live, and what collects them. Second, visibility: the Java Memory Model (JMM) defines when a write by one thread is guaranteed visible to another, and most concurrency bugs are visibility bugs dressed as logic bugs. Agents tend to treat Java as a high-level language where "the GC handles memory" and "synchronized makes it thread-safe," both of which are true in trivial cases and dangerously incomplete in real systems. The cost of getting this wrong ranges from a 3x memory overhead to a heisenbug that corrupts state once a week and vanishes under a debugger.

## Core Rules

### Know Where Each Value Lives, And Prefer Primitives Over Boxes

Every value in the JVM lives somewhere, and the choice has direct cost. Primitives (`int`, `long`, `double`) are stack-allocated in method frames (or fields inside their enclosing object) and cost exactly their bit width. Boxed types (`Integer`, `Long`, `Double`) are heap objects with header overhead — typically 16 bytes for an `Integer` on a 64-bit JVM versus 4 bytes for an `int`. A `Map<Integer, String>` keyed on boxed integers allocates an object per key; a `HashMap<Integer, Integer>` can use 4-5x the memory of an `int[]`-based structure. Under load, this is the difference between a service that fits in 2GB and one that needs 8GB.

Default to primitives for fields, locals, and collection keys where possible. Use boxed types only when nullability is genuinely needed (a field that may be "unset") or when an API forces them (generics before specialized collections). When you need primitive-keyed maps, use trove/Eclipse Collections/fastutil or primitive arrays. Treat every `Integer`/`Long`/`Double` in a hot path as a memory and allocation concern to justify.

### Understand Heap Generations And Choose Collection Lifetimes Deliberately

The generational hypothesis — most objects die young — drives the JVM's heap layout: a young generation (Eden + survivor spaces) for new objects, collected frequently and cheaply; an old generation (tenured) for long-lived objects, collected less often with a more expensive algorithm. Objects that survive minor collections are promoted to the old generation. This means the lifetime of your objects determines your GC cost: short-lived objects are nearly free (Eden allocation is bump-a-pointer and the dead ones cost nothing to collect), while objects that live long enough to tenure add to old-gen pressure and trigger expensive major collections.

The design implication is to make object lifetimes match their semantics. A request-scoped object should be created, used, and discarded within the request so it dies in Eden. An object that escapes into a long-lived cache or a static field tenures and adds to old-gen pressure. The worst pattern is "medium-lived" objects — created per request but held in a queue or buffer that outlives the request — which survive minor GCs, get promoted, and then die in a major collection, maximizing GC cost. When you cache, bound the cache; when you buffer, bound the buffer; when an object escapes, ask whether it should.

### Match GC Choice To Latency And Throughput Requirements

Modern JVMs offer several collectors (G1, ZGC, Shenandoah, Parallel, Serial), each with different pause-time and throughput tradeoffs. G1 (the default since Java 9) balances throughput and pause times by dividing the heap into regions; ZGC and Shenandoah aim for sub-millisecond pauses at some throughput cost; Parallel maximizes throughput with longer pauses; Serial is for small heaps. The choice is a deployment decision driven by your SLA: a batch job tolerates Parallel's pauses; a latency-sensitive service needs G1 or a low-pause collector.

Do not tune GC blindly. The common failure is cargo-cult flags (`-XX:+Use...`, aggressive ergonomics) copied from a blog post that mismatch the workload. Start with the default, measure pause times and throughput under realistic load, and change one thing at a time. Heap sizing (`-Xms` matching `-Xmx` to avoid resizing) often matters more than collector choice. A GC problem is usually a "too many medium-lived objects" problem, and the fix is in the application code, not the flags.

### Establish Happens-Before For Every Cross-Thread Visibility

The Java Memory Model defines a happens-before relationship: if action A happens-before action B, then the results of A are visible to B. Without happens-before, the JVM is free to reorder instructions, cache values in registers, and show a thread a stale or partially-constructed view of another thread's writes. This is not theoretical: the classic bug is a `boolean ready` flag set by one thread and polled by another that never observes the set, because nothing established happens-before.

Happens-before is established by: releasing/acquiring a lock (`synchronized` block exit happens-before the next entry on the same monitor), writing/reading a `volatile` field, starting a thread (`Thread.start` happens-before the new thread's first action), thread termination (a thread's actions happen-before another thread observes it terminated via `join` or `isAlive`), and actions in the `java.util.concurrent` machinery (each concurrent primitive documents its happens-before guarantees). The rule: if two threads share a variable and at least one writes, there must be a happens-before edge between the write and every read, or the behavior is undefined. `volatile` for a single flag, `synchronized` or `Atomic*` for compound state, and concurrent collections for shared data structures.

### Safely Publish Every Shared Mutable Object

A subtle consequence of the JMM is safe publication. Publishing an object (making it visible to other threads) without happens-before lets another thread observe it half-constructed: fields at default values, the object visible before its constructor finished, or fields in an inconsistent order. The object's invariants, established in its constructor, are not guaranteed visible to a reader that obtained the reference through an unsynchronized path.

Safe publication requires one of: storing the reference in a `volatile` field or `AtomicReference`, storing it in a `final` field (final fields have special constructor-publication guarantees), synchronizing all access through a lock, or storing it in a properly synchronized concurrent collection (`ConcurrentHashMap`, `BlockingQueue`). The most robust default is to make shared immutable objects use `final` fields throughout — then safe publication is free. Mutable shared objects must be published through a synchronization mechanism every time, on both the write and read sides.

### Treat Classloading And Reflection As Stateful, Leaky Machinery

Classloading in the JVM is stateful: each classloader holds the classes it defined, and a class is identified by (classloader, name) — not just name. In long-running processes (application servers, plugin systems), this is the source of the classic "metaspace leak": a classloader that cannot be garbage-collected because something holds a reference to one of its classes, and every redeploy loads a new classloader full of new classes that never unload, until metaspace fills and the JVM refuses to load more classes.

Reflection (`Class.forName`, `Method.invoke`, dynamic proxies) amplifies this: it caches classes and methods, it can pin classloaders, and it bypasses compile-time type checking. Use reflection deliberately — for frameworks, serialization, plugins — and avoid it in application code where interfaces and generics suffice. In container/plugin/redeploy scenarios, audit what holds references across classloader boundaries (thread locals, static caches, logging frameworks, JDBC drivers) and clean them up on undeploy. A "permgen"/metaspace leak that appears after N redeploys is a classloader pin.

### Never Rely On Finalizers; Use Cleaner Or try-with-resources

`finalize()` (and finalization generally) is deprecated and broken in several ways: the JVM makes no guarantee a finalizer will ever run, finalizers can delay reclamation, they can resurrect objects, and they run on a low-priority thread that may starve. A class whose cleanup depends on `finalize` (closing a native handle, releasing a lock) leaks when finalization does not fire — which is often, under load or at shutdown. Java 9+ deprecates `finalize`; Java 18+ marks it for removal.

The replacements are explicit. For resources (files, sockets, locks, connections), implement `AutoCloseable` and require callers to use try-with-resources — this makes cleanup deterministic and tied to scope. For native resources that must have a safety net, use `Cleaner` (or `PhantomReference`), but treat it strictly as a last resort, never the primary cleanup path, and document that it may not run. Never put correctness-critical cleanup in a finalizer or cleaner; always provide an explicit `close()` and require it.

### Size And Bound Native And Off-Heap Memory Deliberately

The JVM heap is not the only memory a Java process uses. Direct byte buffers (`ByteBuffer.allocateDirect`), native memory (JNI, JNI-loaded libraries), thread stacks (each thread ~512KB-1MB by default), code cache (JIT-compiled code), and the JVM's own internal structures all consume native (off-heap) memory. A process can OOM at the OS level while the JVM heap reports plenty of free space, because the off-heap memory was never bounded.

Bound these deliberately. Limit direct buffer pools (`-XX:MaxDirectMemorySize`), limit thread counts (each thread is real memory), and audit JNI/native libraries for leaks. Off-heap memory is attractive for caches and big data structures (no GC pressure), but it moves the memory management burden to you: you must allocate and free it explicitly, and a leak is invisible to the heap tools. Use off-heap only when you have measured GC pressure and have a plan to manage the lifecycle.

## Common Traps

### Boxing Primitives In Hot Paths And Collections

`Map<Integer, X>` or `List<Long>` in a hot path inflates memory 3-5x and adds allocation pressure. Use primitive arrays or primitive-specialized collections.

### Medium-Lived Objects That Tenure And Trigger Major GC

Per-request objects held in a queue or buffer that outlives the request; they survive minor GC, get promoted, then die expensively in a major collection. Match object lifetimes to request scope.

### Cargo-Cult GC Flags

Copy-pasted `-XX` flags that mismatch the workload. Start with defaults, measure under realistic load, change one variable at a time; fix allocation patterns before tuning flags.

### Missing Happens-Before On A Shared Flag Or Field

A `boolean ready` set by one thread and polled by another without `volatile` or synchronization; the poller may never see the set. Every cross-thread write needs a happens-before edge to every read.

### Unsafely Publishing A Mutable Object

Storing a reference to a not-fully-constructed or mutable object in a non-volatile static field; readers observe half-built state. Publish through `volatile`, `final`, a lock, or a concurrent collection.

### Classloader/Metaspace Leak On Redeploy

A static cache, thread local, or logging framework holding a reference across a classloader boundary, so the old classloader (and all its classes) never unload. Audit cross-classloader references on undeploy.

### Relying On `finalize` For Cleanup

A native handle or lock released only in `finalize`, which may never run. Use try-with-resources and `AutoCloseable`; reserve `Cleaner` strictly as a non-critical safety net.

### Off-Heap Or Direct Memory Unbounded

Direct buffers or JNI memory that grows until the OS kills the process while the heap looks fine. Bound direct memory and audit native libraries.

### Assuming `synchronized` Makes Everything Correct

`synchronized` provides mutual exclusion and visibility for the guarded state, but a compound operation across two synchronized blocks on different locks is still a race. Name the invariant; the lock follows.

## Self-Check

- [ ] Primitives are used for fields, locals, and hot-path collection keys; boxed types appear only where nullability or generics force them, and primitive-specialized collections are used where boxing would dominate.
- [ ] Object lifetimes match their semantics (request-scoped objects die in Eden; long-lived caches are bounded), and there are no medium-lived objects escaping into queues or buffers that would tenure and trigger major GC.
- [ ] GC choice and heap sizing (`-Xms`/`-Xmx`) are driven by measured pause times and throughput under realistic load, not cargo-cult flags; allocation patterns were addressed before tuning.
- [ ] Every cross-thread shared variable has a documented happens-before edge (lock, `volatile`, `Atomic*`, thread start/join, or a `j.u.c.` primitive) between every write and every read.
- [ ] Every shared mutable object is safely published through `volatile`, `final` fields, a lock, or a concurrent collection; no half-constructed object can escape through an unsynchronized path.
- [ ] Reflection and dynamic classloading are used deliberately (frameworks/plugins), not in application code; cross-classloader references (static caches, thread locals, logging, JDBC) are cleaned up on undeploy to prevent metaspace leaks.
- [ ] No correctness-critical cleanup relies on `finalize`; resources implement `AutoCloseable` with try-with-resources, and `Cleaner`/`PhantomReference` are used only as a non-critical safety net.
- [ ] Direct buffers, thread stacks, code cache, and native/JNI memory are bounded and audited; off-heap memory has an explicit lifecycle plan and is not used to hide leaks from heap tooling.
- [ ] `synchronized`, `volatile`, and `Atomic*` are matched to the actual invariant (single field vs compound state), and no compound operation is assumed atomic just because individual accesses are synchronized.
