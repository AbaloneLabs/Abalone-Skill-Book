---
name: memory_ordering_and_visibility.md
description: Use when the agent is reasoning about memory ordering, happens-before relationships, volatile versus atomic semantics, memory barriers, instruction reordering, cache coherence, or cross-thread visibility of writes in any language with a memory model. Covers the memory model and why ordering is not free, happens-before and synchronizes-with relationships, the difference between volatile and atomic, acquire/release versus sequentially consistent ordering, relaxed atomics and their dangers, hardware memory models (x86 TSO versus ARM/POWER weak ordering), the cost of getting ordering wrong (silent corruption correct on one architecture, broken on another), and the discipline of using the strongest ordering by default and the language's standard primitives rather than hand-rolled barriers. Also use when a data race produces intermittent corruption, when code correct on x86 fails on ARM, when volatile is misused for thread communication, or when choosing memory ordering for performance.
---

# Memory Ordering And Visibility

Memory is not what intuition says it is. The intuitive model — that threads see each other's writes in the order they were written, and that a write becomes visible to other threads immediately — is false on every modern platform. Compilers reorder instructions for optimization, and CPUs reorder memory operations for performance, both within rules defined by the language's memory model. The result is that, without correct synchronization, one thread's write to a variable may be seen by another thread late, out of order, or never, and the program behaves correctly on a strongly-ordered architecture (x86) and corrupts data on a weakly-ordered one (ARM, POWER) — silently, intermittently, and only under the specific timing that appears in production. Memory ordering is the discipline of making cross-thread communication correct under these realities, and it is the area of programming where intuition is most reliably wrong and bugs are hardest to reproduce.

Agents tend to apply intuition to memory ordering because the code looks sequential and the failure is invisible. The defects live in the assumptions: using `volatile` (which prevents compiler reordering but not CPU reordering or cache visibility) where an atomic with ordering is required; using a relaxed atomic where an acquire/release pair is needed to publish non-atomic data; assuming sequentially consistent ordering when the language provides only acquire/release by default for certain operations; relying on x86's strong ordering in code that will also run on ARM. The judgment problem is treating every cross-thread read/write of shared state as a memory-model question — what ordering is required, what primitive provides it, and whether the chosen ordering is correct on every architecture shipped — and defaulting to the strongest, safest ordering, weakened only with measurement and full understanding of the happens-before relationship it provides.

This skill is about reasoning correctly about memory ordering and visibility. It complements the thread-safety skill (general shared-state correctness) and the lock-free skill (atomic-based algorithms); here the question is the memory model itself, the ordering primitives, and the specific failure modes of getting ordering wrong.

## Core Rules

### Treat Cross-Thread Shared Access As A Memory-Model Problem

The first principle is that any data accessed by more than one thread, where at least one accesses it for write, is governed by the memory model — not by sequential intuition. Correctness is not automatic; it requires synchronization that establishes the needed ordering and visibility.

- **Identify all shared mutable state.** Any variable, field, or object accessed by multiple threads where at least one writes is in scope. The question for each is: what synchronization makes the accesses correct?
- **Sequential intuition does not apply.** "A wrote X then Y, so B sees X before Y" is false without synchronization. The compiler may have reordered, the CPU may execute out of order, and B's cache may be stale. Assume nothing about cross-thread ordering without an explicit guarantee.
- **Unsynchronized shared access is a data race, which is undefined behavior** in most memory models — not just "maybe wrong," but formally undefined, meaning the compiler can do anything. A data race is not a bug to tune around; it is a bug to eliminate.

### Understand Happens-Before, And Establish It With Synchronization

The correctness condition for cross-thread communication is the happens-before relationship: if action A happens-before action B, then B observes A's effects, in order. Happens-before is established by synchronization primitives, and the programmer's job is to ensure the needed happens-before relationships exist.

- **A write is visible to a read only if the write happens-before the read.** Without a happens-before edge, the read may see the write late, partially, or not at all. The edge is created by synchronization: a lock acquire/release, an atomic operation with the right ordering, a thread start/join, a properly published reference.
- **Publishing shared data through an atomic establishes ordering for the data.** The canonical pattern: write the non-atomic data, then write (release) an atomic flag; another thread reads (acquire) the flag, and if it sees the write, all the non-atomic writes before the release are visible after the acquire. The atomic is the synchronization; the data piggybacks on its ordering.
- **Publishing a reference is not enough without the data's visibility.** Handing another thread a reference to an object does not make the object's initialized fields visible unless the handoff itself has a happens-before edge (through a lock, an atomic, a volatile/atomic field, or a thread start). A published-but-unsynchronized reference can expose a partially constructed object.
- **Verify the edge exists for every cross-thread communication.** For each piece of data one thread shares with another, identify the synchronization that creates the happens-before edge. If you cannot point to it, the communication is racy.

### Distinguish Volatile From Atomic, And Do Not Misuse Volatile

`volatile` (in C/C++/Java) and `volatile`/`volatile read` (in other languages) have specific, limited semantics, and they are routinely misused for thread communication where an atomic is required. The semantics differ by language, but the common trap is the same.

- **In C/C++, `volatile` does not make access atomic or provide memory ordering.** It prevents the compiler from optimizing away or reordering the specific access, but it does not prevent CPU reordering, does not guarantee atomicity of read/write, and does not establish happens-before. Using `volatile` for cross-thread communication in C/C++ is a data race. Use `std::atomic` instead.
- **In Java, `volatile` does provide memory ordering and visibility** (a volatile write happens-before a subsequent volatile read of the same field), making it suitable for publishing a flag — but it does not make compound operations atomic (`++` on a volatile is still read-modify-write and racy). Use `AtomicInteger` etc. for compound operations.
- **The safe rule: use the language's atomic primitives for cross-thread communication, not volatile, unless you are certain of volatile's exact semantics in your language and they suffice.** Atomics are explicit about ordering; volatile's meaning varies and is often misremembered.

### Default To Sequentially Consistent Ordering; Weaken Only With Understanding

Atomic operations take a memory ordering argument, and the choice governs the happens-before edges established. The default should be the strongest ordering (sequentially consistent), weakened only with measurement and full understanding.

- **Sequentially consistent ordering** provides a single total order of all SC operations across all threads, as if they all occurred in some interleaving. It is the easiest to reason about and is correct for virtually all cases. Default to it.
- **Acquire/release ordering** provides a happens-before edge (a release-store synchronizes-with an acquire-load that observes it) but does not impose a global order. Correct and faster than SC for publish/consume patterns, but only if every access to the published data is correctly ordered — a single missing ordering breaks the guarantee.
- **Relaxed ordering** provides atomicity but no ordering or visibility guarantees. Suitable only for operations where ordering does not matter (e.g., a counter where only the final count matters, not the order of increments). Using relaxed where ordering matters is a silent correctness bug.
- **Weaken only with measured benefit and full understanding.** The performance difference between SC and acquire/release is often small; the correctness difference, if you get it wrong, is catastrophic. Weaken ordering only when profiling shows the stronger ordering is a bottleneck and you understand the weaker ordering's implications.

### Account For Hardware Memory Models; Test On The Weakest Architecture

The memory model is defined by the language, but its performance and its failure modes depend on the hardware. Code correct under a strong hardware model may break under a weak one.

- **x86 is strongly ordered (TSO — total store order).** Many incorrect orderings happen to work on x86 because the hardware is conservative. This hides bugs.
- **ARM and POWER are weakly ordered.** The hardware reorders memory operations aggressively, and code that relied on x86's implicit ordering breaks. A program correct on x86 and broken on ARM is the classic memory-model failure.
- **Test on the weakest architecture you ship on.** If you ship on ARM, test on ARM; x86 correctness does not transfer. Where physical ARM hardware is unavailable, use tools that model weak ordering (TSan with a weak-model simulation, or formal verification for critical sections).
- **Do not rely on hardware strength.** Code that depends on x86's strong ordering is non-portable and will break when compiled for ARM. Rely on the language's memory model (which is defined independent of hardware), not on the hardware's behavior.

### Use The Language's Standard Primitives; Avoid Hand-Rolled Barriers

Memory barriers (`fence`, `mfence`, `dmb`) are the low-level mechanism, but they are error-prone and architecture-specific. Use the language's standard atomic and synchronization primitives, which encode the correct barriers portably.

- **Prefer locks, standard atomics, and synchronization primitives over explicit barriers.** A mutex, a condition variable, an atomic with the right ordering — these are correct, portable, and reviewed. Hand-rolled barriers are a source of subtle, architecture-specific bugs.
- **Reserve explicit barriers for the rare case where standard primitives do not fit** (a lock-free algorithm with unusual ordering needs), and even then, prefer the language's atomic fence primitives over inline assembly.
- **Do not assume a barrier does what you think.** Barriers have specific semantics (acquire, release, full) that differ across architectures; using the wrong one is as bad as no barrier. Use the abstraction, understand its guarantee, and verify.

## Common Traps

### Using Volatile For Cross-Thread Communication In C/C++

Treating `volatile` as if it provides atomicity or ordering, when in C/C++ it does neither, creating a data race. Use `std::atomic`; reserve `volatile` for memory-mapped I/O and signal handlers.

### Assuming Sequential Intuition Applies Across Threads

Reasoning that "A wrote X then Y, so B sees X before Y," when without synchronization the compiler and CPU may reorder and B may see them late or out of order. Establish happens-before with synchronization for every cross-thread communication.

### Publishing A Reference Without Establishing Visibility Of Its Data

Handing another thread a reference to an object whose fields were written without a happens-before edge, exposing a partially constructed object. Publish through a lock, an atomic, or a volatile/atomic field that establishes the edge.

### Relaxed Atomics Used Where Ordering Matters

Using relaxed ordering for an operation whose correctness depends on ordering (publishing data, signaling), causing silent corruption. Default to sequential consistency; use relaxed only where ordering truly does not matter.

### Code Correct On x86, Broken On ARM

Relying on x86's strong (TSO) ordering in code that also runs on ARM's weak ordering, so the bug is invisible in x86 testing and appears only on ARM in production. Test on the weakest architecture shipped.

### Weakening Ordering For Unmeasured Performance

Using acquire/release or relaxed ordering to "go faster" without measuring that sequential consistency was actually slow, trading correctness for an unmeasured gain. Default to SC; weaken only with measured benefit and full understanding.

### Hand-Rolled Barriers Instead Of Standard Primitives

Using inline assembly or explicit fence instructions instead of the language's atomic and synchronization primitives, introducing architecture-specific bugs. Use standard atomics and locks; reserve explicit barriers for rare, well-justified cases.

### A Single Missing Ordering Breaking The Whole Guarantee

Getting most accesses right but missing the ordering on one access to published data, breaking the happens-before chain so the data is visible but unordered or partial. Verify every access in a publish/consume pattern is correctly ordered, not just the obvious ones.

## Self-Check

- [ ] All shared mutable state accessed by multiple threads (with at least one writer) is identified, and each access is governed by synchronization that makes it correct — no unsynchronized cross-thread writes exist (no data races, which are undefined behavior).
- [ ] Sequential intuition is not applied across threads: no assumption that "A wrote X then Y so B sees X before Y" without an explicit happens-before edge, and every cross-thread communication has an identified synchronization establishing the edge.
- [ ] `volatile` is not misused: in C/C++ it is not used for thread communication (use `std::atomic`), in Java it is used only for visibility of a single field (not compound operations, which need atomics), and the language's exact volatile semantics are confirmed before use.
- [ ] Memory ordering defaults to sequentially consistent, weakened to acquire/release or relaxed only where profiling shows the stronger ordering is a bottleneck and the weaker ordering's happens-before implications are fully understood.
- [ ] Publish/consume patterns are correct: data is written, then published through a release-store on an atomic; the consumer acquire-loads and, on observing the release, sees all writes before it — and every access to the published data (not just the obvious ones) is correctly ordered.
- [ ] The code is tested on the weakest architecture shipped (e.g., ARM, not only x86), since x86's strong (TSO) ordering hides bugs that appear on weakly-ordered architectures; correctness relies on the language's memory model, not on hardware strength.
- [ ] Standard synchronization primitives (locks, atomics, condition variables) are used instead of hand-rolled barriers, and explicit fences are reserved for rare, well-justified cases with their exact semantics understood.
- [ ] Critical lock-free or ordering-sensitive sections are reviewed carefully (and stress-tested or formally verified where the cost of a bug is high), since memory-ordering bugs are silent, intermittent, and architecture-dependent, and rarely reproduce in ordinary testing.
