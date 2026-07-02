---
name: interrupt_safety_and_concurrency.md
description: Use when the agent is writing interrupt service routines (ISRs), sharing data between ISRs and main code or between tasks on a microcontroller or RTOS, using volatile, atomics, critical sections, or memory barriers; designing around DMA races; diagnosing race conditions, torn reads, lost updates, priority inversion, or data corruption between interrupt and task contexts; or reviewing concurrency in bare-metal or RTOS firmware. Covers ISR design principles, minimizing disabled-interrupt windows, volatile versus atomic versus lock-free patterns, priority inversion and inheritance, and DMA/CPU sharing hazards.
---

# Interrupt Safety And Concurrency

Concurrency in embedded systems is not threads-and-locks the way it is on a server. It is a hardware reality: an interrupt can fire at almost any point in your main loop, preempt it, and run an ISR that touches the same memory; DMA runs in parallel with the CPU and can be halfway through a buffer the CPU is also reading; on an RTOS, multiple tasks preempt each other on a single core with no language-level protection against shared-state races. The C and C++ compilers will not save you: a `volatile` does not make an operation atomic, an `int` read is not guaranteed atomic, and the compiler and CPU are free to reorder memory accesses in ways that break naive sharing. Agents trained on higher-level languages routinely ship firmware with data races that pass every test and corrupt state in the field, because the race requires a precise interleaving that only occurs under real interrupt load.

The judgment problem is deciding, for every piece of shared state, what concurrency actually threatens it and what the correct defense is — a critical section, an atomic, a memory barrier, a lock-free pattern, or a redesign that avoids sharing. The harm of casual decisions is the worst kind of bug: intermittent, hardware-dependent, and impossible to reproduce in a debugger that stops the very interrupts causing it.

## Core Rules

### Design ISRs To Be Minimal And Deferred

An ISR's job is to acknowledge the hardware, capture the minimum time-critical data, and defer everything else. The longer an ISR runs, the longer every other interrupt of equal or lower priority is blocked, increasing worst-case latency across the system (see the real-time skill). Keep ISRs short: no loops waiting on hardware, no heavy computation, no locks that could block, no heap allocation, no logging that blocks. The pattern is to do the urgent work in the ISR (read the data register before it is overwritten, clear the flag) and signal a task or set a flag to process the rest. An ISR that does "a little extra work while it is running" is the classic source of missed deadlines and unbounded latency.

### Make Every Shared Variable's Concurrency Defense Explicit

For each variable shared between an ISR and main code, or between tasks, name the defense. The options, in order of preference:

- **Avoid sharing.** Redesign so each context owns its state and they communicate by message passing (an RTOS queue, a ring buffer with a single producer and single consumer). This eliminates the race by construction and is the most robust pattern.
- **Atomic access.** If the operation is a single atomic read, single atomic write, or an atomic read-modify-write the hardware supports (test-and-set, compare-and-swap), use a language atomic or a guaranteed-single-instruction operation. Know what is actually atomic on your architecture: an aligned 32-bit write is atomic on most 32-bit MCUs, but a 64-bit write is not.
- **Critical section.** Disable interrupts (or lock a scheduler) around the multi-instruction sequence, do the work, re-enable. Correct but adds the section's duration to every ISR's worst-case latency, so sections must be minimal.

The recurring defect is *implicit* sharing with no named defense — a global flag the ISR sets and main reads, with no atomic, no barrier, and no acknowledgment that the read could be torn or reordered. Every shared variable gets a defense or it is a latent race.

### Understand What `volatile` Does And Does Not Do

`volatile` is one of the most misused qualifiers in embedded C. It tells the compiler "do not optimize away accesses to this memory; every read and write must actually happen in program order." It is required for memory-mapped hardware registers and for variables an ISR can change (so the compiler does not cache the value in a register across the access). What it does *not* do: it does not make a read-modify-write atomic, it does not prevent tearing of multi-word accesses, it does not act as a memory barrier for other variables, and in C++ it is not sufficient for inter-thread communication. Treating `volatile` as "thread-safe" is a category error that produces races. For atomic inter-context communication, use atomics or critical sections; reserve `volatile` for "the compiler must not elide or reorder this single access," which is a narrower and different guarantee.

### Use Memory Barriers Where Reordering Can Break Correctness

Both the compiler and the CPU can reorder memory accesses for optimization, and on a single core this is invisible — but the moment an ISR, DMA, or another core observes the memory, reordering becomes visible and can break correctness. A memory barrier forces ordering: a release barrier ensures prior writes are visible before a subsequent release, an acquire barrier ensures subsequent reads see the effects of a prior acquire. The canonical pattern for lock-free producer-consumer sharing is: the producer writes the data, then writes a "ready" flag with a release barrier; the consumer reads the "ready" flag with an acquire barrier, then reads the data. Without the barriers, the consumer may see the flag set before the data is actually written. On single-core Cortex-M, hardware interrupt entry/exit provides some ordering implicitly, but portable and DMA-involving code must use explicit barriers. When in doubt, use the platform's documented barrier primitives rather than reasoning about the architecture from scratch.

### Prevent Priority Inversion On Shared Resources

Priority inversion happens when a low-priority task holds a resource (a lock, a peripheral) that a high-priority task needs, and a medium-priority task preempts the low-priority task, so the high-priority task waits for the medium one. In a real-time system this causes the high-priority task to miss its deadline for reasons that have nothing to do with its own execution. The defenses are priority inheritance (the low-priority task temporarily inherits the high priority while holding the resource) and the priority ceiling protocol (each resource has a ceiling priority, and a task holding it runs at that priority). Use an RTOS mutex that implements inheritance rather than a binary semaphore, which does not. The deeper lesson: shared resources between tasks of different priorities are a real-time hazard, and minimizing them (message passing instead of shared state) is often better than managing the inversion.

### Treat DMA As Another Concurrent Context

DMA is a hardware engine that reads and writes memory in parallel with the CPU, and it does not respect the CPU's cache or the compiler's view of memory. Sharing a buffer between CPU and DMA without explicit management produces the classic "intermittently corrupt received data" bug. The rules:

- **Do not let the CPU and DMA access the same buffer region simultaneously.** Use double-buffering (ping-pong): DMA fills buffer A while the CPU processes buffer B, then swap. This is the standard robust pattern.
- **Maintain cache coherency.** If a data cache is present, the CPU may have a stale cached copy of memory DMA just wrote, or DMA may write around a dirty cache line the CPU has not flushed. Invalidate (for DMA-to-CPU) or clean/flush (for CPU-to-DMA) the relevant cache regions around the transfer, or place DMA buffers in non-cacheable memory.
- **Use barriers.** The "transfer complete" flag the DMA sets must be observed by the CPU after the data writes are visible; a barrier enforces this ordering.

DMA races are among the hardest embedded bugs to diagnose because they depend on cache state and exact transfer timing. Design them out with double-buffering and explicit coherency rather than debugging them in.

### Avoid Blocking And Reentrancy Hazards In ISRs And Shared Code

An ISR must never call code that blocks (waits on a semaphore held by a lower-priority task, takes a mutex, allocates memory). It must also be careful with reentrancy: if the ISR calls a function also called by main code, and that function uses a static buffer or non-reentrant state, a nested interrupt or a re-entry can corrupt it. RTOS APIs often distinguish "from ISR" variants that are safe to call in interrupt context — use them, never the blocking variants. Functions shared between ISR and task contexts must be reentrant (no static state, or state protected by a critical section). The safe default: ISRs call only a small set of documented-safe functions and signal tasks for everything else.

## Common Traps

### Treating `volatile` As "Thread-Safe"

Marking a shared flag `volatile` and assuming reads and writes are now safe, when `volatile` only prevents the compiler from optimizing the access — it does not make read-modify-write atomic, does not prevent tearing, and is not a memory barrier. Use atomics or critical sections for inter-context sharing.

### Doing Too Much In The ISR

Handling the full event in the ISR because "it is running anyway," then missing deadlines because the ISR blocks lower-priority interrupts for too long. ISRs do the minimum and defer.

### Sharing A Buffer Between CPU And DMA Without Coherency

Reading a buffer the CPU's cache holds a stale copy of, or that DMA is mid-write into, producing intermittent corruption. Double-buffer and maintain cache coherency.

### No Barrier Between Data Write And Ready Flag

Writing data then a ready flag without a release barrier, so the consumer sees the flag before the data; or reading the flag then the data without an acquire barrier. Use the platform's acquire/release primitives for lock-free handoff.

### Using A Binary Semaphore Where Priority Inheritance Is Needed

Protecting a shared resource with a binary semaphore (which does not implement inheritance), then hitting priority inversion where a high-priority task waits on a low-priority task preempted by a medium one. Use a mutex with inheritance for cross-priority resource sharing.

### Calling Blocking RTOS Functions From An ISR

Calling the standard `xQueueSend` or `xSemaphoreTake` from an ISR, which can block or corrupt scheduler state. Use the documented `...FromISR` variants and the `higher-priority-task-woken` yield pattern.

### Assuming A Read Or Write Is Atomic Because It "Looks Like One Instruction"

Believing an `int` write compiles to a single store and is therefore atomic, when on an 8- or 16-bit MCU a 32-bit write is multiple instructions and can be torn by an interrupt. Verify atomicity against the architecture; do not assume it from the source.

## Self-Check

- [ ] ISRs are minimal — they acknowledge hardware, capture urgent data, and defer all other work to a task via a flag, queue, or signal; no blocking, no allocation, no heavy computation.
- [ ] Every variable shared between ISR and main code, between tasks, or with DMA has an explicit named defense (avoidance via message passing, atomic access, or a minimal critical section).
- [ ] `volatile` is used only for what it guarantees (compiler must not elide or reorder the single access, for hardware registers and ISR-visible flags), not as a substitute for atomics or barriers.
- [ ] Lock-free producer-consumer handoffs use release/acquire memory barriers (or the platform equivalent) so a ready flag is never observed before the data it guards.
- [ ] Shared resources between tasks of different priorities are protected by a mutex implementing priority inheritance (or avoided via message passing); priority inversion was considered and addressed.
- [ ] DMA buffers are double-buffered (CPU and DMA never touch the same region simultaneously), in DMA-reachable memory, with cache invalidation/cleaning around transfers where a data cache is present.
- [ ] Functions called from ISR context are reentrant and use only ISR-safe RTOS API variants; no blocking calls or shared static state without protection.
- [ ] Atomicity assumptions (single-instruction reads/writes) were verified against the target architecture, especially for multi-word types on 8/16-bit MCUs.
- [ ] A test under realistic interrupt load (concurrent peripheral activity, nested priorities, DMA in progress) was run and no torn reads, lost updates, or corruption appeared.
