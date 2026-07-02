---
name: real_time_constraints_and_deadlines.md
description: Use when the agent is developing firmware or software for embedded controllers, motor drives, sensors, robotics, automotive ECUs, avionics, medical devices, or industrial control systems where tasks must complete within a deadline; deciding between hard and soft real-time requirements; analyzing worst-case execution time (WCET); setting task priorities and scheduling policies; choosing between bare-metal, RTOS, or general-purpose OS; reasoning about interrupt latency, jitter, priority inversion, or deadline misses; or diagnosing glitches where an actuator misfires, a control loop destabilizes, or a safety-critical response arrives too late. Covers the consequences of missing deadlines, schedulability analysis, and the tradeoffs of real-time OS selection.
---

# Real-Time Constraints And Deadlines

Real-time does not mean "fast." It means *predictable*: a deadline is a correctness criterion, and a result that arrives late is as wrong as a result that is wrong. This distinction is the entire difficulty. A general-purpose programmer is trained to optimize average case and throughput; a real-time system is judged by its worst case and its jitter, and the worst case is what arrives the day a safety interlock must fire. Agents raised on web and server code routinely ship "real-time" firmware that works in the lab and misses deadlines in the field, because the rare path — a cache miss, a GC pause, an interrupt storm, a higher-priority task preempting the critical one — was never measured.

The judgment problem is deciding, for each task, whether its deadline is hard (a miss is a system failure, possibly dangerous) or soft (a miss degrades quality but is tolerated), how to prove the worst-case execution time fits within the deadline under all conditions, and what platform and scheduling choices make that provable. The harm of getting this wrong ranges from a glitchy consumer device to a motor that overshoots, a brake that engages late, or a medical infusion that delivers the wrong dose.

## Core Rules

### Classify Each Deadline As Hard Or Soft Before Designing Around It

The classification drives every other decision, and it must be made per task, not per system.

- **Hard real-time.** A missed deadline is a system failure. Examples: motor commutation, airbag deployment, pacemaker pacing, anti-lock brake modulation. Missing the deadline can cause physical harm, equipment damage, or certification failure. These tasks require provable schedulability — a demonstrated bound that the deadline is always met.
- **Firm real-time.** A missed deadline makes the result useless, but the system survives (a late sensor sample is discarded, not harmful). Common in streaming and signal processing.
- **Soft real-time.** Missed deadlines degrade quality of service but are tolerated statistically. Examples: UI rendering, telemetry upload, user-facing animations. Average performance matters; occasional misses are acceptable.

The trap is treating a hard deadline as soft because "it usually finishes in time." For a hard deadline, "usually" is a failure mode. Write down each task's deadline and its classification; if a task is hard, the rest of the design must prove it is always met.

### Analyze Worst-Case Execution Time, Not Average Case

Real-time correctness is governed by the worst-case path, which is almost always rarer and longer than the average. WCET analysis asks: what is the longest this task can possibly take, considering every branch, every loop bound, every cache state, and every interference from other tasks and interrupts? On a modern processor with caches, branch prediction, and dynamic frequency scaling, WCET can be many times the average — and it is extremely hard to bound without either measurement on the target hardware or static analysis with a processor model.

Two approaches, often combined:

- **Measurement-based.** Instrument the task on the real hardware, exercise the worst-case paths, and observe the maximum. Pragmatic, but only as good as the test coverage of worst-case paths — an untested branch can hide a longer path.
- **Static analysis.** A tool analyzes the code and a processor model to compute a safe upper bound. More conservative (slower than reality) but covers paths the tests missed. Required for safety-certification regimes (DO-178C, ISO 26262, IEC 61508).

The common error is reporting average execution time as if it were the deadline budget. It is not. The deadline must accommodate the WCET plus interference, or the task will miss under the conditions that matter most.

### Verify Schedulability, Not Just "It Runs"

A set of tasks is schedulable if every task meets its deadline under all conditions, not if it happens to in a demo. The standard tools:

- **Rate Monotonic Analysis (RMA).** For fixed-priority preemptive scheduling, assigns priority by period (shorter period = higher priority) and provides a utilizations bound test. If total utilization is below the bound, all deadlines are guaranteed; if above, further analysis is needed.
- **Response-time analysis.** Computes each task's worst-case response time (its own WCET plus preemption by all higher-priority tasks) and checks it against the deadline. More precise than the utilization bound and the workhorse of real schedulability proofs.
- **Deadline Monotonic.** For tasks where deadline differs from period, priority follows deadline, not period.

The point is not to memorize formulas but to recognize that "will it meet its deadline" is a question with a rigorous answer, and that answer is not "we ran it for an hour and it was fine." A schedulability analysis covers the worst-case release pattern, the worst-case execution, and the worst-case interference simultaneously.

### Account For Interrupt Latency And Jitter

The time between a hardware event and the start of the ISR that handles it is interrupt latency, and it is not zero or constant. It includes the time to finish the currently-executing instruction, any period during which interrupts are disabled, the time to save context, and — critically — the duration of any higher-priority ISR running first. Jitter is the variation in latency. For a task that must respond to an event within a tight window, both the latency and its jitter must be bounded.

Sources of latency and jitter to audit: long critical sections (interrupts disabled), nested interrupt priorities, DMA stealing bus cycles, cache effects on the ISR's first execution, and any code that disables interrupts for "just a moment." A design that disables interrupts for 50 µs to update a shared structure has just added 50 µs to every ISR's worst-case latency. Minimize and measure disabled-interrupt windows; they are the dominant source of latency jitter.

### Choose The Platform Against The Determinism Requirement

The platform determines what determinism is even achievable.

- **Bare metal / super-loop.** Maximum control, minimum interference, but the developer owns all scheduling and must implement it correctly. Best for the tightest, most predictable deadlines on small MCUs.
- **RTOS.** Provides preemptive priority scheduling, ISRs, and primitives (semaphores, queues) with known behavior. Determinism depends on the RTOS being certified for real-time use and on correct priority assignment. Avoid RTOS features that break determinism (unbounded priority inheritance, unbounded message queues).
- **General-purpose OS (Linux without PREEMPT_RT).** Not real-time. The kernel can preempt unpredictably, memory allocation can stall, and there is no WCET guarantee. Suitable only for soft real-time or the non-critical layers of a layered system.
- **Linux with PREEMPT_RT / Xenomai.** Can achieve bounded latency for many hard-ish use cases, but requires careful configuration and still has worse jitter than a dedicated MCU. Verify, do not assume.

The recurring mistake is running a hard real-time task on a general-purpose OS because "Linux is fast." Speed is not determinism; the worst-case latency of standard Linux is unbounded for practical purposes.

### Design For The Worst-Case Release Pattern

Tasks release periodically or in response to events, and the worst case is often when many release at once. A set of periodic tasks whose periods align will all become ready simultaneously on their least-common-multiple boundary; the schedulability analysis must assume that burst. Event-driven tasks can burst under fault conditions (a fault storm triggering every alarm handler). Size stacks, queues, and CPU reserves for the burst, not the average. A system sized for the average release pattern misses deadlines the day everything happens at once.

## Common Traps

### Reporting Average Execution Time As The Deadline Budget

Measuring a task's typical runtime and assigning it as the deadline, then missing the deadline the day a rare branch or cache-cold path executes. The deadline budget must be WCET plus interference, measured on worst-case paths or bounded by static analysis.

### Treating A Hard Deadline As Soft

Classifying a safety-critical task as soft because "it usually finishes," then relying on luck for safety. Hard deadlines require provable schedulability; if you cannot prove it, the design is incomplete.

### Disabling Interrupts For "Just A Moment"

Bracketing a shared-variable update with interrupt disable/enable and assuming the window is negligible, then discovering it is 50 µs and adds that to every ISR's worst-case latency. Measure and minimize critical sections; prefer lock-free atomics or short ISRs that defer work to a task.

### Running Hard Real-Time On A General-Purpose OS

Putting a motor-control loop on standard Linux because the hardware is convenient, then hitting unbounded latency from kernel preemption, paging, or device-driver IRQ handling. Hard real-time needs a bare-metal MCU, an RTOS, or a real-time-patched kernel with verified latency.

### Ignoring Priority Inversion

Allowing a low-priority task holding a resource to block a high-priority task (with a medium-priority task preempting the low one), so the high-priority task misses its deadline while waiting. Use priority inheritance or ceiling protocols on shared resources; this is what they exist to prevent.

### Assuming Cache Behavior Is Predictable

Treating instruction/data cache as transparent, then finding the ISR's first invocation after a context switch is much slower because its working set was evicted. On cache-sensitive hardware, WCET analysis must account for cache state; lock critical code/data into cache where the platform allows.

### Sizing For Average Load

Allocating a queue or stack for typical event rates, then overflowing it during a burst or fault condition. Size for the worst-case simultaneous release; an overflowed queue in a fault handler turns a recoverable fault into a crash.

### Confusing Throughput With Latency

Optimizing a real-time task for total throughput (more samples per second) while increasing its per-invocation jitter, so individual deadlines are missed even though aggregate work is higher. Real-time is about per-deadline predictability, not aggregate throughput.

## Self-Check

- [ ] Each task has a documented deadline and a hard/firm/soft classification; hard-deadline tasks are designed for provable schedulability, not "usually finishes in time."
- [ ] Worst-case execution time was measured on worst-case paths or bounded by static analysis on the target hardware; average execution time was not used as the deadline budget.
- [ ] A schedulability analysis (response-time analysis or equivalent) was performed covering the worst-case release pattern, WCET, and interference from higher-priority tasks and ISRs.
- [ ] Interrupt latency and jitter were measured or bounded; disabled-interrupt critical sections are minimized and their contribution to worst-case latency is accounted for.
- [ ] The platform (bare metal, certified RTOS, or real-time-patched OS) matches the determinism requirement; no hard real-time task runs on an unbounded-latency general-purpose OS.
- [ ] Priority inversion is prevented (priority inheritance or ceiling protocol on shared resources), and priority assignment follows the scheduling policy (rate/deadline monotonic).
- [ ] Stacks, queues, and CPU reserves are sized for the worst-case simultaneous release (periodic alignment or event burst), not the average load.
- [ ] Cache effects on ISR and critical-task execution were considered; critical code/data is locked into cache where the platform supports it.
- [ ] A stress test exercising worst-case paths, interrupt load, and burst release patterns confirmed no deadline was missed.
