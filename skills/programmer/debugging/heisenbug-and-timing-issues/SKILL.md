---
name: heisenbug_and_timing_issues.md
description: Use when the agent is debugging heisenbugs that disappear under observation, investigating race conditions or timing-dependent bugs, cases where logging hides the bug, debugger side effects, or devising reproduction strategies for nondeterministic failures that never appear when instrumented.
---

# Heisenbug and Timing Issues

A heisenbug is a defect that changes behavior when you observe it. You add a print statement and the bug vanishes; you attach a debugger and the crash stops; you run the slow debug build and the race never triggers. These bugs are almost always rooted in timing, ordering, or memory behavior that the observation itself perturbs. They are among the hardest bugs to diagnose precisely because the act of looking changes what you see, and the instinct to "add more logging" often makes the bug harder to catch, not easier. The discipline is to understand why observation changes behavior, to reproduce the failure without perturbing it, and to use tools that reveal the race without masking it.

The judgment problem is recognizing when a bug is timing-dependent, choosing reproduction and observation strategies that do not mask the defect, and using the right tools (sanitizers, stress tests, deterministic schedulers) rather than flailing with logging. The agent should not treat a disappearing bug as fixed; the disappearance is information, not resolution.

This skill applies whenever you are chasing a bug that is intermittent, disappears under debugging, or depends on timing, ordering, or load.

## Core Rules

### Recognize the signature of a heisenbug

A bug is likely timing- or observation-dependent when it shows any of these signs:

- It disappears when you add logging, run under a debugger, or compile with debug flags.
- It is intermittent, with no deterministic reproducer, and the failure rate varies with load or hardware.
- It involves concurrency (threads, async tasks, signals, interrupts) and the failure is sensitive to scheduling.
- The crash site (the symptom) is far from the cause, often in memory corruption that manifests later.
- Running the same input twice gives different results.

When you see these signs, stop trying to reproduce with the exact failing input and start thinking about timing and ordering. The input is not the variable; the schedule is.

### Understand why observation changes behavior

Observation perturbs timing-sensitive code through several mechanisms:

- **Timing changes**: logging, debug builds, and debugger breakpoints slow execution, changing the relative timing of concurrent operations. A race that triggers when thread A wins by a microsecond may not trigger when A is slowed.
- **Memory layout changes**: debug builds, sanitizers, and some logging change memory layout and alignment, hiding buffer overflows and use-after-free that corrupt adjacent data in release.
- **Scheduling changes**: a debugger or tracing tool can alter the scheduler's decisions, changing which thread wins a race.
- **Optimization changes**: debug builds disable optimizations that the release build relies on, masking undefined behavior and optimization-dependent bugs.

Knowing the mechanism tells you which observation is masking the bug and which tool will reveal it instead.

### Reproduce without perturbing: stress and scale the timing

Since observation perturbs timing, reproduce by stressing the timing rather than instrumenting it:

- **Run the operation in a tight loop** (thousands or millions of iterations) to surface low-probability races.
- **Increase concurrency**: more threads, more connections, more load, to widen the race window.
- **Vary scheduling**: use thread schedulers that randomize or interleave execution (some languages offer randomized schedulers or model checkers for concurrency).
- **Reduce resources**: fewer cores, less memory, slower I/O, to widen timing windows and expose ordering dependencies.
- **Run on the target environment**: a bug that does not reproduce on your fast dev machine may reproduce on the slower, more contended production hardware.

The goal is to widen the race window so the bug triggers reliably, without adding observation that narrows it again.

### Use sanitizers and race detectors instead of logging

The right tools for timing and memory bugs observe without perturbing the relevant dimension:

- **Thread sanitizers (TSan)**: detect data races (unsynchronized concurrent access) at compile/run time, pinpointing the exact conflicting accesses. They add overhead but reveal races that logging cannot.
- **Address/Memory sanitizers (ASan, MSan)**: detect memory errors (use-after-free, buffer overflow, uninitialized reads) that often manifest as heisenbugs in release. They change layout to catch corruption, which is the opposite of masking it.
- **Lock-order analysis**: detect potential deadlocks by analyzing lock acquisition order, without needing the deadlock to trigger.
- **Deterministic execution / record-replay**: record a production run and replay it deterministically, so an intermittent failure can be reproduced exactly and debugged repeatedly.

Prefer these over ad-hoc logging, which perturbs timing and rarely pinpoints the cause.

### When logging is necessary, log the schedule, not the data

If you must add observation, observe the dimension that matters:

- Log thread IDs, timestamps, and operation ordering, not just data values, so you can reconstruct the interleaving that triggered the bug.
- Log at trace level so it can be enabled only during reproduction, not always-on.
- Use lock-free or async logging so the logging itself does not serialize threads and change scheduling.
- Capture the failing run's schedule (ordering of operations) so you can compare a failing run to a passing run.

Logging data values is usually useless for a race; logging the schedule is what reveals it.

### Separate the cause from the symptom

Heisenbugs often crash far from the cause. A use-after-free corrupts memory that manifests as a crash in unrelated code much later. A race writes a partial value that a later reader misinterprets. Do not assume the crash site is the bug:

- Use memory sanitizers to catch the corruption at the point it occurs, not where it crashes.
- Use race detectors to catch the unsynchronized access, not the corrupted read.
- Trace backward from the symptom to the earliest observable deviation.

Fixing the symptom (the crash site) without finding the cause leaves the bug latent.

### Make the fix and verify it eliminates the race, not just the symptom

A correct fix removes the race or memory error at its source (proper synchronization, fixing the lifetime, removing the undefined behavior). Verify the fix by re-running the stress/sanitizer reproduction:

- The race detector should report no races.
- The sanitizer should report no memory errors.
- The stress loop should run cleanly for far more iterations than the bug previously needed to trigger.

A fix that only stops the crash at the observed site, without addressing the underlying race or corruption, is not a fix; it is a mask that will fail under different timing.

## Common Traps

### Treating disappearance under observation as a fix

The bug vanishes when you add logging or run under a debugger, so you assume it is resolved. The disappearance is the heisenbug's defining behavior, not resolution. The race or corruption is still present.

### Adding logging that masks the race

Print statements slow execution and change scheduling, narrowing the race window so it no longer triggers. More logging can make the bug harder to catch, not easier. Use sanitizers and race detectors instead.

### Reproducing with the failing input only

For a timing bug, the input is not the variable; the schedule is. Running the same input once and seeing no failure proves nothing. Stress the timing by looping, increasing concurrency, and varying scheduling.

### Assuming the crash site is the cause

Memory corruption and races manifest far from their origin. Fixing the crash site without tracing to the corruption or race leaves the latent defect.

### Debug-build reproduction that masks release bugs

Debug builds disable optimizations and change memory layout, hiding undefined behavior and corruption that the release build exhibits. Reproduce in the release configuration, or use sanitizers designed to surface such bugs.

### Fixing the symptom, not the race

Adding a retry, a sleep, or a defensive check at the crash site without removing the underlying race or corruption. The fix works for the observed timing and fails under different timing.

### Ignoring concurrency as the root cause

Intermittent bugs in multithreaded code are races until proven otherwise. Treating them as flukes or environmental noise, rather than investigating the synchronization, lets the race persist.

## Self-Check

- Have you recognized the heisenbug signature (disappears under observation, intermittent, concurrency-related, symptom far from cause) and shifted to timing-based reasoning?
- Do you understand which observation mechanism (timing, layout, scheduling, optimization) is masking the bug, and are you using tools that reveal rather than mask?
- Are you reproducing by stressing timing (tight loops, increased concurrency, varied scheduling, reduced resources, target hardware) rather than re-running the failing input?
- Are you using sanitizers (ASan, MSan, TSan) and race detectors instead of ad-hoc logging to pinpoint the cause?
- Where logging is necessary, are you logging the schedule (thread IDs, timestamps, ordering) rather than just data values?
- Have you traced from the symptom (crash site) back to the earliest observable cause (the race or corruption), rather than fixing the symptom?
- Does the fix remove the race or memory error at its source (proper synchronization, correct lifetime, no undefined behavior), not just mask the crash?
- Have you verified the fix by re-running the stress/sanitizer reproduction for far more iterations than the bug previously needed?
- Are you reproducing in the release configuration (or with sanitizers), not only in the debug build that may mask the bug?
- Have you confirmed the fix does not rely on timing (sleeps, retries) that will fail under a different schedule?
