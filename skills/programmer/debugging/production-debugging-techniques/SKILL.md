---
name: production_debugging_techniques.md
description: Use when the agent is debugging issues in production, using observability tooling and distributed tracing, performing remote diagnosis without a reproducer, reading core dumps, deploying safe probes and dynamic instrumentation, identifying hotspots, or preserving safety nets while investigating live systems.
---

# Production Debugging Techniques

Production bugs are categorically harder than development bugs. You cannot attach a debugger freely, you cannot always reproduce the conditions, the system is serving real users, and a wrong move turns an investigation into an outage. The discipline of production debugging is extracting enough signal to diagnose without perturbing the live system, using observability that was hopefully put in place before the bug occurred, and adding observation safely when it was not. The failure modes are two-sided: too little observation leaves you blind, and too much (or the wrong kind) can itself cause the outage you are trying to prevent.

The judgment problem is choosing observation that reveals the cause without risking the system, diagnosing from indirect evidence when no reproducer exists, and knowing when to stop poking a live system and instead reproduce offline. The agent should not treat production like a development environment; every action must be weighed against blast radius.

This skill applies whenever you are investigating a bug or performance issue in a live, user-serving system.

## Core Rules

### Lean on observability before live instrumentation

The first move in production is to read what the system already tells you, before adding anything new:

- **Logs**: filter by trace ID, request ID, user ID, or time window to find the failing request's path. Structured logs with correlation IDs are essential; unstructured logs force grepping.
- **Metrics**: compare the failing period to a baseline. Did latency, error rate, CPU, memory, or queue depth change? Correlate the change time with deploys, config changes, or traffic shifts.
- **Distributed traces**: follow a failing request across service boundaries to find where time was spent or where the error originated. Traces reveal the causal path that logs alone cannot.
- **Dashboards and alerts**: check what fired and when, and what the steady-state looked like before the incident.

Exhaust observability first. If it pinpoints the cause, you avoid the risk of live instrumentation entirely.

### Diagnose from indirect evidence when no reproducer exists

Often you cannot reproduce the production failure, but you can infer the cause from evidence:

- **Compare failing and passing requests**: what differs between requests that fail and requests that succeed in the same period? A differing header, payload shape, tenant, or shard points to the trigger.
- **Correlate the failure with environmental changes**: deploys, config rollouts, dependency versions, traffic patterns, time of day. A failure that began at a deploy time is likely deploy-related.
- **Statistical analysis**: if 0.1% of requests fail, find what the failing 0.1% have in common (a specific user agent, a payload size threshold, a particular data value).
- **Read the code path** that the failing requests traverse, looking for assumptions that the evidence contradicts.

Build a hypothesis from the evidence, then seek the cheapest way to confirm it—ideally offline, not in production.

### Add observation safely when existing telemetry is insufficient

When observability does not reveal the cause, you may need to add observation. Do it safely:

- **Prefer additive, read-only observation**: new log lines, new metrics, new trace spans. These do not change behavior, only reveal it.
- **Sample and bound**: do not log every request; sample (e.g., 1 in 1000) or gate behind a flag for specific tenants. Unbounded logging in a hot path can itself cause overload.
- **Dynamic instrumentation**: use tools that inject probes at runtime (eBPF, dynamic tracing, bytecode instrumentation) without redeploy, so you can observe without a deploy and remove the probe instantly.
- **Canary the observation**: roll out new logging to one instance or one canary first, to confirm it does not destabilize.
- **Avoid invasive changes**: do not add synchronous I/O, locks, or heavy computation to a hot path "just to debug." These can cause the outage you are investigating.

The rule: observation must be cheaper than the bug it hunts, and reversible in seconds.

### Use core dumps and snapshots for post-mortem diagnosis

When a process crashes or hangs in production, capture its state for offline analysis:

- **Core dumps**: enable core dumps (or capture via a crash handler) so a crash preserves the memory image. Analyze offline with a debugger to see the stack, registers, and heap at the crash.
- **Heap/profile snapshots**: capture a heap profile or CPU profile (most runtimes support this via signals or endpoints) to diagnose memory leaks or hotspots without stopping the process.
- **Goroutine/thread dumps**: capture all thread/goroutine stacks to diagnose deadlocks, hangs, or contention. A single dump shows state; periodic dumps show progression.

These capture state without requiring a reproducer, and analysis happens offline where mistakes are safe.

### Identify hotspots and bottlenecks via profiling

For performance issues (not crashes), profiling reveals where time or memory goes:

- **CPU profiling**: identifies the functions consuming the most CPU. A hotspot in an unexpected function often reveals the bug (an O(n^2) loop, an accidental sort, a redundant computation).
- **Allocation/heap profiling**: identifies allocation sources, useful for GC pressure and memory leaks.
- **Lock/contention profiling**: identifies where threads block, useful for latency caused by contention rather than computation.
- **Flame graphs**: visualize where time is spent across the call stack, making hotspots obvious.

Profile in production (sampled, low-overhead) rather than relying on synthetic benchmarks, because production workloads expose bottlenecks that benchmarks miss.

### Know when to stop poking production and reproduce offline

Production diagnosis has diminishing returns and rising risk. Recognize when to shift:

- Once you have a hypothesis and a plausible reproducer, move to a staging or local environment to confirm, rather than continuing to perturb production.
- If the investigation itself is consuming production capacity or risking stability, capture the evidence (dumps, profiles, traces) and analyze offline.
- If the bug cannot be safely reproduced in production (it requires corrupting state or stressing a dependency), reproduce in an isolated environment.

The discipline is to extract maximum signal from production with minimum perturbation, then move the destructive confirmation elsewhere.

### Preserve safety nets while investigating

Do not disable safety mechanisms to debug:

- Keep circuit breakers, retries, rate limits, and fallbacks enabled. Disabling them to "see the raw error" can turn a degraded state into an outage.
- Keep monitoring and alerting on. The worst outcome is causing a second incident while debugging the first, and not noticing.
- Have a rollback path ready for any change you make (a flag flip, a deploy revert). Never make an irreversible change during an investigation.

## Common Traps

### Treating production like a development environment

Attaching a debugger, disabling safety mechanisms, or making invasive changes to a live system. Every action must be weighed against blast radius; production is not your personal lab.

### Insufficient observability when you need it most

Discovering, mid-incident, that logs are unstructured, traces are missing, or correlation IDs are absent. The time to invest in observability is before the bug, not during it.

### Unbounded logging causing overload

Adding verbose logging to every request in a hot path, which consumes CPU, disk, and I/O and can itself cause the outage. Sample and bound observation.

### Fixing the symptom at the crash site without finding the cause

Restarting the process, adding a retry, or reverting a deploy to stop the bleeding—without diagnosing the cause. Stabilization is necessary but is not diagnosis; the bug will recur.

### Poking production past the point of safe returns

Continuing to add instrumentation and changes to a live system after enough evidence is captured, increasing the risk of causing an outage. Move confirmation offline.

### Disabling safety nets to see raw errors

Turning off circuit breakers or rate limits to observe the underlying failure, which removes the protection that is keeping the system alive. Investigate with safety nets on.

### Relying on synthetic benchmarks instead of production profiles

A bottleneck that does not appear in the benchmark but dominates production, because the workload differs. Profile in production to see real behavior.

### No rollback path during investigation

Making a change to debug and having no fast way to undo it, so a bad probe becomes a prolonged outage. Always have a rollback ready.

## Self-Check

- Have you exhausted existing observability (structured logs with correlation IDs, metrics, distributed traces, dashboards) before adding live instrumentation?
- For a failure with no reproducer, have you compared failing vs passing requests and correlated the failure with deploys, config, and environmental changes to form a hypothesis?
- When adding observation, is it additive, sampled/bounded, dynamically injectable, and reversible in seconds, rather than invasive to a hot path?
- Are core dumps, heap profiles, and thread/goroutine dumps captured for post-mortem offline analysis of crashes, hangs, and leaks?
- For performance issues, are you profiling in production (sampled, low-overhead) and using flame graphs to find hotspots, rather than relying on benchmarks?
- Have you recognized when to stop perturbing production and move confirmation to an isolated environment?
- Are safety nets (circuit breakers, retries, rate limits, monitoring) kept enabled during the investigation?
- Is there a rollback path ready for every change you make during the investigation?
- Have you separated stabilization (stopping the bleeding) from diagnosis (finding the cause), and committed to diagnosing the cause so the bug does not recur?
- Is the observation you added temporary, with a plan to remove debug probes and logging once the cause is found?
