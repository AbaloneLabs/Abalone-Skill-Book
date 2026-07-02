---
name: memory_and_resource_limits.md
description: Use when the agent is developing firmware or software for memory-constrained microcontrollers, IoT devices, sensor nodes, or battery-powered embedded systems; sizing RAM and flash budgets; deciding between static and dynamic allocation; designing stack sizes; deciding whether and where the heap may be used; reasoning about power consumption versus memory use; managing buffers, DMA, and constant data; or diagnosing out-of-memory, stack overflow, heap fragmentation, flash exhaustion, or excessive power draw. Covers the tradeoffs of static vs dynamic allocation, no-heap designs, linker-script budgeting, and the memory-power-cost triangle.
---

# Memory And Resource Limits

On an embedded target, memory is a physical budget, not an elastic cloud. A device has a fixed number of kilobytes of RAM and a fixed number of kilobytes or megabytes of flash, and exceeding either is not a slowdown — it is a build failure, a link error, or worse, a runtime crash on hardware that has no page file to fall back to. Worse still, the budget is shared: the same RAM holds the stack, the heap, global buffers, and DMA regions, and the same flash holds code, constant tables, and assets. An agent trained on systems with gigabytes of RAM routinely treats memory as fungible — allocating buffers per connection, caching freely, growing vectors on demand — and ships firmware that works in the lab and hard-faults in the field when the real workload pushes it over.

The judgment problem is deciding, before writing code, how the fixed memory budget is partitioned; whether allocation is static or dynamic and where the heap is permitted at all; how stacks are sized against the real call depth (including interrupts); and how memory choices interact with power, since every byte of RAM retained in a low-power mode is current the battery pays for. The harm of casual decisions here is latent: a stack overflow that only triggers on the deepest error-handling path, a heap that fragments until allocation fails after days of uptime, a flash image that no longer fits after a feature is added.

## Core Rules

### Build A Memory Budget Before Writing Allocation Code

Treat RAM and flash as a fixed currency with named accounts. Before allocating, know: total RAM, total flash, how much is reserved for the stack(s), how much for the heap (if any), how much for global/static buffers, how much for DMA and peripheral buffers, and how much for the runtime/RTOS. Then assign each feature a budget within those accounts. A firmware project without a memory budget discovers its limits at runtime, in the field, under load — which is the most expensive possible time. The linker script and a map-file review are the enforcement tools: a section that grows beyond its budget must fail the build, not the device.

### Prefer Static Allocation; Justify Every Use Of The Heap

Static allocation (globals, static buffers, stack-local fixed arrays) is deterministic, has no fragmentation, and its cost is visible at link time. Dynamic allocation (malloc/free, new/delete) is flexible but introduces fragmentation, non-deterministic timing, and failure modes (allocation returns null) that must be handled at every call site. On small embedded targets, the strong default is no heap at all — many safety-critical and certification regimes (MISRA, DO-178) restrict or forbid dynamic allocation after initialization for exactly this reason.

Where the heap is used, constrain it:

- **Allocate once at startup, never free.** A "heap" that only grows and never frees cannot fragment. This is the common pattern for long-lived buffers whose lifetime equals the process.
- **Use a fixed-block allocator (memory pool).** Same-sized blocks eliminate external fragmentation and bound allocation time. Acceptable in many constrained designs where the heap is needed but fragmentation is not.
- **Never allocate on a hot path or in an ISR.** Allocation timing is non-deterministic and allocation failure in an ISR is unrecoverable.

The recurring error is reaching for malloc because it is familiar, then debugging fragmentation weeks into a soak test. Default to static; justify each heap use against its fragmentation and timing cost.

### Size Stacks Against Real Worst-Case Depth, Including Interrupts

A stack overflow on a microcontroller is typically a hard fault with no diagnostic — the stack collides with another region and corrupts state silently until the device crashes. Size every stack against the actual worst-case call depth, which means: the deepest normal call chain, plus the deepest error/exception path (which is often deeper), plus the maximum nested interrupt depth (an ISR can preempt a task, and a higher-priority ISR can preempt that ISR, all sharing or having separate stacks depending on the architecture). Use stack-painting (fill unused stack with a sentinel pattern and measure the high-water mark at runtime) to validate that the assumed worst case matches reality. A stack sized by guesswork will overflow on the path no one tested — usually an error handler called from the deepest normal path.

### Account For Flash As A Finite, Partitioned Resource

Flash holds code, constant data (lookup tables, strings, fonts, assets), and often configuration/calibration data in reserved sectors. Each is a separate budget. The common failures:

- **Code size creep.** Adding a library or a debug formatter pushes the image over the flash budget. Track code size per module and set a build-time alarm well below the limit.
- **Constant data in RAM by mistake.** A `const` array without the right section attribute may land in RAM (initialized at startup) instead of flash, silently consuming scarce RAM. Verify constant tables are placed in flash (read-only) sections, and that initialized data is minimized.
- **Flash wear and reserved sectors.** If flash also stores configuration that is rewritten over the device life, those sectors are a wear budget as well as a size budget (see the OTA and flash-wear considerations).

Review the linker map file: every byte of RAM and flash should be attributable to an account, and no surprise should appear at link time.

### Treat Memory And Power As A Coupled Tradeoff

In a battery-powered device, retained RAM costs current. Most microcontrollers offer low-power modes that retain some or all of RAM; the more RAM retained in sleep, the lower the sleep-mode savings. This couples memory design to power budget: large buffers retained across sleep keep the device drawing more current than necessary. Options include reducing buffer sizes, moving retained state into a smaller backup-RAM domain, or recomputing state on wake instead of retaining it. An agent who sizes buffers for convenience ("8 KB is fine") may be paying for those bytes in battery life across millions of devices. For ultra-low-power designs, every retained kilobyte is a deliberate decision against sleep current.

### Manage DMA And Shared Buffers Explicitly

DMA transfers run concurrently with the CPU and share memory and buses. A buffer under DMA must not be modified by the CPU mid-transfer (data tearing), must be in a memory region the DMA can reach (some architectures restrict DMA to specific RAM), and must be cache-coherent if a data cache is present (CPU and DMA may see different views of the same address). These are not optimizations; they are correctness requirements. Use memory barriers, cache-maintenance operations, and double-buffering (ping-pong buffers) so the CPU works on one buffer while DMA fills another. A "sometimes corrupts the received packet" bug is almost always a missing barrier or a shared-buffer race with DMA.

### Make Resource Exhaustion A Build-Time Or Defined-Runtime Failure

The principle across all constrained resources: exhaustion should fail loudly and early, not silently and late. Prefer build-time limits (static allocation, linker-section budgets) so exceeding them fails the build. Where runtime limits are unavoidable (a fixed pool of connections, a bounded queue), make exhaustion a defined, handled condition — reject the operation, drop the oldest entry, or shed load by policy — never an unhandled null dereference or an out-of-bounds write. A device that handles "queue full" by a defined drop policy survives overload; one that assumes the queue never fills crashes.

## Common Traps

### Using malloc Because It Is Familiar

Reaching for dynamic allocation in firmware as if it were a server, then debugging fragmentation and non-deterministic allocation timing in a soak test. Default to static; the heap is a justified exception, not the default.

### Sizing Stacks By Guesswork

Picking a stack size that "looks big enough," then hard-faulting on the deepest error path that no one tested. Measure worst-case depth with stack painting, including interrupt nesting.

### Placing Constant Tables In RAM

Declaring a lookup table `const` and assuming it lives in flash, when without the right attribute it is copied to RAM at startup, consuming scarce RAM for data that is never written. Verify placement in the linker map.

### Growing A Buffer "To Be Safe"

Tripling a buffer size to avoid an overflow, then discovering the device no longer fits its RAM budget or its sleep current rose. Buffers are a budgeted account; growth must be justified against the total, not just the local need.

### Assuming DMA And CPU See The Same Memory

Letting the CPU read a buffer while DMA is filling it, or skipping cache maintenance, producing intermittent data corruption that looks like a flaky peripheral. DMA buffers need barriers, cache management, and double-buffering by design.

### Ignoring Flash Wear For Configuration Storage

Rewriting a configuration sector on every parameter change, exhausting the flash's erase-cycle budget months into the field. Configuration storage in flash is a wear-budgeted resource; use wear leveling, write-on-change, or EEPROM emulation.

### Treating The Linker Map As Someone Else's Concern

Never reading the map file, so a surprise — a library pulling in 20 KB, a constant table in RAM, the heap sized larger than intended — appears only at runtime or in field returns. The map file is the memory budget's ledger; review it on every change.

## Self-Check

- [ ] A memory budget exists partitioning total RAM and flash into named accounts (stack(s), heap, globals/statics, DMA/peripheral buffers, runtime, code, constants, reserved config); exceeding any account fails the build or is a deliberate, documented decision.
- [ ] Allocation defaults to static; every use of the heap is justified, confined to startup or fixed-block pools, and never occurs on a hot path or in an ISR.
- [ ] Every stack is sized against measured worst-case call depth (including error paths and interrupt nesting) validated by stack-painting at runtime.
- [ ] Constant tables are verified to reside in flash (read-only) sections via the linker map, not copied to RAM at startup.
- [ ] For battery-powered devices, retained RAM in sleep modes is a deliberate decision against sleep current; buffer sizes and retained-state choices were reviewed against the power budget.
- [ ] DMA buffers are in DMA-reachable memory, protected by barriers and cache maintenance, and double-buffered where the CPU and DMA share access.
- [ ] Flash used for configuration storage is wear-budgeted (wear leveling, write-on-change, or EEPROM emulation), not rewritten on every change.
- [ ] Runtime resource limits (queues, pools, connections) have a defined exhaustion policy (reject, drop, shed) rather than assuming the limit is never hit.
- [ ] The linker map file is reviewed on changes, and code/const/heap sizes are tracked against budgets with build-time alarms well below hard limits.
