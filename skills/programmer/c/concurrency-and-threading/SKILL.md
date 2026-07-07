---
name: c_concurrency_and_threading.md
description: Use when the agent is writing or reviewing C code that uses threads, mutexes, condition variables, atomics, thread-local storage, or synchronization primitives via C11 threads.h, pthreads (POSIX), or C11 stdatomic.h, or when diagnosing data races, deadlocks, missed wakeups, memory-ordering bugs, or portability problems across threading implementations on Windows, Linux, macOS, and embedded RTOS targets.
---

# C Concurrency And Threading

C has no built-in concurrency model. Threading and synchronization are provided by libraries layered on top of the language: C11 `<threads.h>` and `<stdatomic.h>`, POSIX `pthreads`, Windows threads, or an RTOS-specific API. The language itself provides only the `volatile` qualifier (which is widely misunderstood as a synchronization primitive and is not one) and the memory model added in C11 that defines what a data race is and what atomics guarantee. The result is that correct concurrent C is a layered judgment problem: you must choose a threading implementation that exists on every target, use synchronization primitives correctly, and reason about memory ordering and visibility — none of which the compiler or the language will check for you.

Agents tend to write threaded C as if it were sequential code with a few locks sprinkled in, then encounter data races that pass on one platform and corrupt state on another, deadlocks from inconsistent lock ordering, missed condition-variable wakeups, or code that relies on `volatile` to publish data across threads and fails under a relaxed-memory architecture. The judgment problem is to pick the right primitive for the synchronization need, to understand that atomics have memory-ordering semantics that must be chosen deliberately (not defaulted to), and to recognize that C11 threads.h is not universally available and pthreads is not universally available either. This skill is about making concurrency choices deliberately rather than reaching for the first primitive that compiles.

## Core Rules

### Choose A Threading Implementation Based On Portability, Not Familiarity

C11 `<threads.h>` is standard but optional in practice: several major toolchains (notably older MSVC and some embedded compilers) never implemented it, and the macro `__STDC_NO_THREADS__` signals its absence. POSIX `pthreads` is available on Linux, macOS, and other Unix-like systems but not on bare Windows. Windows threads are Windows-only. For portable code, the realistic choices are: a portability wrapper (a thin internal abstraction over pthreads/threads.h/Win32), or a library like C11 threads-on-pthreads shim, or restricting to one platform family.

Do not assume `<threads.h>` exists because the compiler advertises C11 support — verify with `__STDC_NO_THREADS__` and provide a fallback. Do not assume pthreads exists on Windows unless you depend on a pthreads-win32 port. Document the threading dependency as a build requirement.

### Use Atomics For Flags And Counters, Locks For Multi-Field State

`<stdatomic.h>` provides atomic types (`_Atomic int`, `atomic_int`) and operations (`atomic_load`, `atomic_store`, `atomic_fetch_add`, compare-and-swap). Use atomics for single-word state that multiple threads read and write independently: progress flags, counters, sequence numbers, a "stop requested" boolean. Atomics avoid the overhead of a lock for simple shared state.

Use a mutex when the shared state spans multiple fields that must be updated together (a buffer and its length, a linked list and its head pointer), or when an operation must be atomic with respect to other operations. A mutex protects a critical section: acquire before touching the shared state, release after. The granularity tradeoff is real — a coarse lock serializes more but is simpler to reason about; fine-grained locks allow more parallelism but invite deadlock. Start coarse and split only when profiling shows contention.

### Understand Memory Ordering Before Defaulting To seq_cst

C11 atomics take a `memory_order` argument. The default (`memory_order_seq_cst`) provides sequentially consistent ordering: the strongest guarantee, the most expensive on weak architectures (ARM, POWER). The weaker orderings (`acquire`, `release`, `acq_rel`, `relaxed`) allow more optimization but require the programmer to reason about happens-before relationships. Misusing a weaker ordering produces races that are invisible until the code runs on a relaxed-memory CPU.

- `relaxed`: no ordering, only atomicity. Safe for counters where no other memory depends on the count.
- `acquire` (load) / `release` (store): the standard publish/consume pair. A release store publishes data; an acquire load on the same atomic by another thread sees the data published before the release. This is the pattern for lock-free message passing.
- `acq_rel` (read-modify-write like compare-and-swap): both acquire and release.
- `seq_cst`: total global ordering of all seq_cst operations; the safe default when you are unsure.

Default to `seq_cst` unless you can articulate why a weaker ordering is correct and have reasoned through the happens-before graph. Premature weakening is a common source of bugs that only manifest under specific hardware and timing.

### Always Pair A Condition Variable With A Mutex And A Predicate Loop

Condition variables (`cnd_t` in C11, `pthread_cond_t`) let a thread wait for a state change signaled by another thread. The correct usage is non-obvious and almost always gotten wrong the first time: the wait must be inside a loop that re-checks the predicate, the mutex must be held while checking the predicate and while waiting, and `signal`/`broadcast` should be called after changing the shared state under the mutex.

The mandatory loop guards against spurious wakeups (permitted by the standard) and against the lost-wakeup race where the state changes between checking the predicate and calling wait. Never write `if (pred) cnd_wait(...)`; always write `while (!pred) cnd_wait(...)`. Broadcast when multiple waiters could proceed; signal when exactly one should (signal is cheaper but can wake the "wrong" waiter).

### Use Thread-Local Storage For Per-Thread State, Not Globals

Thread-local storage (`_Thread_local` / `thread_local` in C11, `__thread` GCC extension) gives each thread its own instance of a variable. Use it for per-thread state that would otherwise require a global: an error context, a scratch buffer, an allocator arena. Thread-local variables avoid synchronization because no two threads share the instance.

Be aware of lifetime: a non-static thread-local variable lives for the thread's lifetime; a static thread-local with a non-trivial initializer has per-thread initialization. Do not store pointers to thread-local data and share them across threads — the pointer refers to one thread's instance, which defeats the purpose.

### Treat volatile As Not A Synchronization Primitive

`volatile` tells the compiler not to optimize away reads and writes of a variable (every access hits memory). It is required for memory-mapped hardware registers and for variables modified by signal handlers or outside the program's flow. It is not, and has never been, a synchronization primitive: a `volatile int` does not prevent a data race, does not provide memory ordering, and does not make a read-modify-write atomic. Two threads incrementing a `volatile int` race.

Use `_Atomic` for shared atomic state, `volatile` only for hardware/signal-handler access. Mixing them (`volatile _Atomic int`) is almost never what you want.

## Common Traps

### Using volatile For Inter-Thread Communication

A `volatile int flag` set by one thread and polled by another is a data race under the C11 memory model; the polling thread may never see the update on a relaxed-memory CPU because volatile does not emit a memory barrier. Use `_Atomic` with release/acquire ordering, or a memory barrier, instead.

### Forgetting The while Loop Around Condition Variable Wait

`if (ready) cnd_wait(&cv, &mtx);` misses wakeups and races. The predicate must be re-checked in a loop because of spurious wakeups and the predicate-changed-before-wait race.

### Signaling Before Waiting, Or Without The Mutex

Signaling a condition variable when no thread is waiting loses the wakeup. Signal after changing the shared state that the predicate depends on, while holding (or just after releasing) the mutex, so the waking thread sees the updated state.

### Assuming <threads.h> Exists Everywhere

C11 threads.h is not implemented on several mainstream toolchains. Check `__STDC_NO_THREADS__` and fall back to pthreads or a portability layer.

### Deadlock From Inconsistent Lock Ordering

Two mutexes acquired in different orders by two threads deadlock. Establish a global lock-acquisition order and document it; if a function holds lock A and needs lock B, it must acquire A before B everywhere.

### Weakening Memory Order Without Understanding Happens-Before

Using `memory_order_relaxed` for a flag that publishes other data is a bug: the reader may see the flag set but the data not yet visible. Use release/acquire for publish/consume patterns.

### Holding A Lock Across A Blocking Call

Acquiring a mutex and then calling a blocking operation (read, sleep, another lock) serializes all other users of that mutex and can deadlock. Release the lock before blocking, or use a non-blocking variant.

## Self-Check

- [ ] The threading implementation is chosen for portability (`<threads.h>` gated with `__STDC_NO_THREADS__`, pthreads on POSIX, or a documented portability wrapper), not assumed to exist on every target.
- [ ] Single-word shared state uses `_Atomic` types; multi-field shared state is protected by a mutex whose critical section covers all related fields.
- [ ] Memory ordering is chosen deliberately: `seq_cst` by default, weakened to `acquire`/`release` only with a reasoned happens-before argument, and `relaxed` only for independent counters.
- [ ] Every condition variable wait is inside a `while (!pred)` loop, the predicate is checked under the held mutex, and signal/broadcast is called after changing the shared state.
- [ ] `volatile` is used only for hardware registers and signal-handler-modified variables, never as a synchronization primitive; shared mutable state uses `_Atomic` or a mutex.
- [ ] Thread-local storage is used for per-thread state instead of globals, and no pointers to thread-local data are shared across threads.
- [ ] Lock acquisition order is consistent and documented across the codebase to prevent deadlock, and no lock is held across a blocking call.
- [ ] The code has been run under ThreadSanitizer (TSan) and is race-free, and the memory-ordering choices have been justified against the happens-before graph rather than defaulted blindly.
