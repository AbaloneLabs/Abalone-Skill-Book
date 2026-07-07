---
name: latency_budget_and_slo_driven_design.md
description: Use when the agent is designing a service or request path against a latency target or SLO, allocating a latency budget across components (database, downstream calls, cache, serialization), deciding how much time each step may consume, reasoning about tail latency (p95, p99, p99.9) versus average, designing for a latency SLO under load, choosing where to parallelize or cache to meet a budget, or diagnosing why a request path exceeds its latency target. Covers latency budget allocation, tail-latency causes and mitigation, critical-path analysis, the difference between optimizing average and optimizing the tail, and designing systems whose latency is predictable and SLO-meetable rather than accidentally fast or slow.
---

# Latency Budget And SLO Driven Design

A latency target — "this endpoint must respond within 200ms at p99" — is a constraint on the entire request path, and meeting it is an allocation problem before it is an optimization problem. The request path is a sequence and fan-out of operations: a database query, two downstream service calls, a cache lookup, serialization, network transit. Each consumes time, their sum (with concurrency structure) is the response latency, and whether the sum fits within the target depends on how the time budget is allocated across them. A team that sets a latency SLO but never allocates the budget discovers, under load, that the database query alone consumes 150ms and the downstream calls add 80ms, and the SLO is unmeetable without a redesign. The budget was spent implicitly and incorrectly, because no one decided how much each step was allowed to cost.

Agents tend to treat latency as an outcome to be optimized after the fact ("the endpoint is slow, let's find the bottleneck") rather than as a budget to be allocated during design. The judgment problem is recognizing that meeting a latency SLO requires deciding, in advance, how much time each component on the critical path may consume, designing each component to fit its allocation, and measuring the tail (p95, p99) — where latency problems live — rather than the average, which hides them. This skill covers the discipline of designing for a latency target: allocating the budget, analyzing the critical path, controlling tail latency, and building systems whose latency is predictable enough to meet an SLO rather than accidentally fast on a good day and far over budget on a bad one.

## Core Rules

### Allocate The Latency Budget Across The Critical Path

A latency SLO is a budget, and like any budget it must be allocated to the components that spend it. Design begins with the allocation, not with implementation.

- **Identify the critical path: the sequence of operations whose combined time determines response latency.** A request that queries the database, then calls two downstream services in parallel, then serializes, has a critical path of (database + max(downstream1, downstream2) + serialize + network). The critical path, not the sum of all operations, is what determines latency.
- **Allocate the budget to each critical-path component.** If the SLO is 200ms and the critical path is database + downstream + serialize, decide how much each may consume (e.g., database 50ms, downstream 100ms, serialize + network 50ms). Each component is then designed and measured against its allocation.
- **Reserve margin for variance and load.** A budget allocated to 100% of the SLO at steady state leaves no room for the variance that produces tail latency. Allocate components to sum to less than the SLO at typical load, reserving margin for the p99 conditions that define whether the SLO is met.
- **Re-allocate when a component exceeds its budget.** If the database query consistently takes 80ms against a 50ms allocation, either optimize it, change the design (cache it, precompute), or re-allocate budget from elsewhere — but do not let the overrun silently consume the margin of the whole.

### Optimize The Tail, Not The Average

Latency SLOs are almost always specified at a percentile (p95, p99, p99.9) because the average hides the experiences that matter. A system with a 50ms average and a 500ms p99 is failing its users on the slow requests, and the average will not tell you.

- **Measure and target the percentile the SLO specifies.** If the SLO is p99 under 200ms, every measurement and optimization decision is against p99, not the mean. An optimization that improves the mean but not the p99 does not help meet the SLO.
- **Understand that tail latency has different causes than average latency.** The average is driven by the typical operation; the tail is driven by outliers — GC pauses, lock contention, slow downstream calls, cold caches, queueing under load, retries. Controlling the tail requires controlling these outlier causes, not making the typical case faster.
- **Beware the averaging fallacy in reporting.** Reporting average latency hides tail problems; a service with flat average and degrading p99 is getting worse in the way that matters while looking stable. Always report the relevant percentile alongside the average.
- **Recognize that tails compound across dependencies.** A service that calls three downstream services each at p99 100ms does not have p99 100ms; the probability that at least one is slow is higher, and the combined tail is worse than any individual tail. Fan-out multiplies tail risk.

### Control The Common Causes Of Tail Latency

Tail latency is caused by specific, addressable mechanisms. Knowing them lets you design them out rather than discover them in production.

- **Queueing under load.** As utilization approaches saturation, queueing delay grows non-linearly and dominates the tail. Keeping utilization below the knee of the queueing curve (often 70-80%) controls queueing-induced tail latency. See resource-and-capacity-planning.
- **Garbage collection pauses.** Stop-the-world GC pauses produce periodic latency spikes visible in the tail. Reduce allocation on the hot path, tune the GC, or choose a runtime with low-pause collection. See gc-tuning.
- **Lock contention.** Contended locks serialize requests, and the unlucky ones wait, producing tail latency. Reduce lock scope, use finer-grained or lock-free structures, or shed load under contention.
- **Slow downstream calls and retries.** A downstream service that is occasionally slow (or that times out and triggers a retry) adds latency to the calling request's tail. Apply timeouts shorter than the budget allocation, and bound retries so a slow downstream does not amplify into a slow caller. See retries-timeouts-and-circuit-breakers.
- **Cold caches and cache misses.** A cache miss falls through to a slower path, producing tail latency for the unlucky requests that miss. Warming, stampede protection, and bounding the miss-path cost control this.
- **Background work on the request path.** Logging, metrics emission, serialization of large objects, or other work done synchronously in the request adds variance. Move non-critical work off the request path or bound its cost.

### Use The Right Technique To Fit The Budget

Once the budget is allocated and the tail causes are understood, specific techniques fit specific overruns. Choose the technique by which component exceeds its allocation and why.

- **Cache to remove the component from the critical path.** A database query that exceeds its budget may be cacheable, removing it from the critical path for cached requests. Reserve caching for data that is expensive to compute and cheap to keep consistent.
- **Parallelize independent operations.** Operations on the critical path that are independent should run concurrently; the latency is the max, not the sum. Moving a sequential waterfall to parallel fan-out can halve the path.
- **Precompute or materialize expensive results.** An aggregation or derivation that is too slow to compute on the request path can be precomputed and stored, trading request-time cost for background cost. See cache-warming-and-precomputation.
- **Apply timeouts that enforce the budget.** Each downstream call should have a timeout shorter than its budget allocation, so a slow downstream fails fast rather than consuming the whole budget. The overall request should have a deadline propagated to all components, so they can give up cooperatively when the budget is exhausted.
- **Reduce payload and serialization cost.** Serialization of large responses consumes time on the critical path; smaller payloads, efficient encodings, and streaming reduce it. See network-and-protocol-performance.

### Propagate Deadlines, Not Just Requests

A latency budget is only enforceable if the components along the path know how much time remains. A downstream call made with no deadline awareness may take longer than the caller has left, wasting work that the caller will discard.

- **Propagate a deadline or remaining-time budget through the call chain.** Each component knows how much of the original budget remains and can decide whether to proceed, give up, or shed non-essential work. This prevents "tail amplification" where a slow upstream causes downstreams to do work that is discarded.
- **Make components deadline-aware.** A database query, downstream call, or computation that can check the remaining budget can abort early when the budget is exhausted, returning an error or partial result rather than completing work no one will use.
- **Reject work that cannot meet its deadline.** If a request arrives with insufficient remaining budget to complete, failing fast (and retrying or shedding) is better than starting work that will overrun. This is a form of load shedding (see backpressure-and-load-shedding).

## Common Traps

### Setting A Latency SLO Without Allocating The Budget

Defining a p99 target but never deciding how much each component may consume, so the SLO is missed because one component silently took more than its share. Allocate the budget during design.

### Optimizing The Average While The Tail Degrades

Improving the typical-case latency while the p99 worsens, and reporting the average that hides the degradation. Measure and optimize the percentile the SLO specifies.

### Ignoring Tail Compounding Across Fan-Out

A service calling multiple downstreams, each with acceptable individual tails, but a combined tail far worse because the probability of at least one being slow is high. Account for fan-out tail multiplication.

### No Timeouts Enforcing The Budget

Downstream calls with no timeout or timeouts longer than the budget, so a slow downstream consumes the entire request budget. Apply timeouts shorter than each component's allocation.

### No Deadline Propagation

Calling downstreams without communicating the remaining budget, so they do work the caller will discard when its own budget expires. Propagate deadlines; make components deadline-aware.

### Queueing-Induced Tail At High Utilization

Running resources near saturation, where queueing delay grows non-linearly and dominates the tail. Keep utilization below the knee; provision headroom.

### Synchronous Background Work On The Request Path

Logging, metrics, or serialization done inline, adding variance to the request path. Move non-critical work off the path or bound its cost.

### Discovering Budget Overruns In Production

Finding at launch (or under load) that a component exceeds its allocation, because the budget was never measured during design. Measure each component against its allocation before the SLO is relied upon.

## Self-Check

- [ ] The latency SLO is specified at a percentile (p95/p99/p99.9), the critical path is identified, and the budget is allocated across critical-path components during design — each with a measured or target cost and margin reserved for variance and load.
- [ ] Measurement and optimization target the SLO percentile, not the average; the average is reported alongside the relevant percentile and never used to hide tail degradation.
- [ ] Tail-latency causes are identified and controlled: queueing (utilization below the knee), GC pauses (reduced allocation, GC tuning), lock contention, slow downstream calls (timeouts, bounded retries), cold caches (warming, stampede protection), and synchronous background work (moved off the path).
- [ ] Fan-out tail compounding is accounted for: a service calling multiple downstreams recognizes that the combined tail is worse than individual tails, and the design (parallelism, caching, timeouts) reflects this.
- [ ] Each downstream call has a timeout shorter than its budget allocation, the overall request has a deadline, and deadlines/remaining-budget are propagated through the call chain so components can abort work that cannot complete in time.
- [ ] Techniques to fit the budget are chosen by which component overruns and why: caching (remove from path), parallelization (max not sum), precomputation (background cost), payload/encoding reduction, and load shedding for work that cannot meet its deadline.
- [ ] Each component is measured against its budget allocation before the SLO is relied upon (in design and load test), not discovered to overrun in production.
- [ ] The system degrades gracefully under load — shedding, caching, or rejecting work — rather than allowing latency to grow unbounded as the budget is exhausted, so the SLO is met or the failure is controlled, not a surprise collapse.
