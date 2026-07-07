---
name: capacity_and_load_management.md
description: Use when the agent is diagnosing or preventing capacity incidents and load spikes, distinguishing overload from functional bugs, configuring autoscaling limits and warm-up, designing graceful degradation, load shedding, backpressure, and rate limiting, implementing queue-based load leveling, sizing capacity headroom, or planning capacity as an incident-prevention measure. Also covers the failure modes of cascading failure under overload, autoscaling that scales too late or hits a hard ceiling, retry storms that amplify load, backends that fail open and flood a saturated dependency, queues that absorb load then take hours to drain, and the recurring mistake of treating capacity as a static provisioning number rather than a resilience property that must be designed, monitored, and tested under realistic load.
---

# Capacity And Load Management

A capacity incident looks like a bug but is not one. The system was healthy at 9 a.m., and by 9:15 every request is timing out — yet no code changed, no deploy shipped, and no dependency failed. The cause is load that exceeded the system's capacity, and the correct response is rarely "find the bug"; it is to shed load, add capacity, or degrade gracefully until the system catches up. The judgment problem is that capacity is not a static provisioning number. It is a dynamic property shaped by traffic patterns, autoscaling lag, dependency saturation, connection limits, and the failure modes that emerge only when a component crosses its limit. Agents tend to treat overload as a performance bug to optimize rather than a load condition to manage, and they tend to reach for "add more servers" without examining why the existing capacity was exhausted, whether autoscaling was supposed to handle it, and whether adding capacity will actually help or merely move the bottleneck downstream.

The harm appears under load, when mistakes are most expensive. A service autoscales but scales too slowly to catch a traffic spike, so the first minutes are an outage. A backend fails open under pressure and floods an already-saturated dependency, turning a slowdown into a cascade. A retry policy turns a transient blip into a self-inflicted denial of service. A queue absorbs the spike but then takes hours to drain, extending the incident long after traffic returns to normal. A database hits its connection limit and takes down every service that shares it. The discipline is to distinguish capacity incidents from functional bugs before chasing the wrong fix, to design the system to degrade gracefully rather than collapse when overloaded, to treat load shedding and backpressure as first-class mechanisms rather than afterthoughts, and to treat capacity headroom as a deliberate, monitored incident-prevention tool rather than a one-time provisioning decision. This skill complements the chaos-engineering skill (testing overload behavior under failure), the recovery-and-failover skill (sizing capacity for failover), and the incident-response skill (diagnosing under pressure). Here the focus is managing load and capacity before and during overload.

## Core Rules

### Distinguish A Capacity Incident From A Functional Bug Before Chasing The Fix

The first decision during a degradation is whether this is a bug (something broke) or a capacity event (something is overwhelmed), because the responses are entirely different. Chasing a bug that is actually overload wastes the most valuable minutes; adding capacity to a bug does nothing.

- **Look for the load signal.** A capacity incident correlates with load: traffic volume, queue depth, connection count, or resource utilization (CPU, memory, threads, database connections) climbing to a ceiling as the degradation begins. A functional bug usually degrades regardless of load or appears at a specific code path. If utilization is pinned at 100% and latency rose with it, suspect capacity first.
- **Check whether anything changed in code or config.** A deploy, a flag flip, or a schema change points toward a functional regression; a traffic spike, a marketing event, a dependent retry storm, or time-of-day growth points toward capacity. Absence of a change is strong evidence for overload.
- **Let the distinction drive the response.** A capacity event is addressed by shedding load, scaling out, or degrading; a bug is addressed by rollback or a fix. Applying the wrong response extends the outage, because scaling out a service whose backend is the real bottleneck does not help, and rolling back code that was never the cause wastes time.

### Understand Autoscaling Limits, Lag, And Warm-Up

Autoscaling is not a guarantee that capacity will meet demand; it is a mechanism with limits, delays, and warm-up costs that determine whether it actually catches a spike.

- **Autoscaling reacts after the fact.** Most autoscalers scale on metrics that lag the load (average CPU over a window), so a fast spike lands before new capacity is provisioned. Know the reaction time, and provision baseline headroom to absorb the spike that arrives before scaling catches up.
- **Scaling out has a warm-up cost.** New instances take time to boot, attach, warm caches, and become healthy before they accept load. A cold replica that joins under load may itself be slow until warm, so the capacity does not help instantly. Account for warm-up when sizing for spikes.
- **Autoscaling hits hard ceilings.** Account limits, quota, instance availability, maximum pool size, or a saturated dependency can cap scaling, so the system stops scaling exactly when it needs to most. Know the ceilings, and ensure they are high enough to absorb the largest plausible spike, not just steady state.
- **Scale-in can cause flapping.** Aggressive scale-in after a spike can remove capacity that is still needed if traffic is bursty, causing the system to oscillate. Tune scale-in to be slower than scale-out.

### Design Graceful Degradation And Load Shedding, Not Just Collapse

When load exceeds capacity, the question is not whether the system degrades but how. A system that collapses (returns errors for everything, including cheap requests) is far worse than one that degrades (keeps the critical path working and sheds the rest).

- **Identify the critical path and protect it.** Know which functionality must keep working under load (login, checkout, the core read path) and which can be shed or degraded (recommendations, analytics, background jobs, expensive reports). Under overload, protect the critical path and sacrifice the rest deliberately.
- **Shed load early and explicitly, rather than failing late and randomly.** Return a fast, clear "service busy" or throttle response for non-critical or excess requests, rather than letting every request queue until timeouts cascade. Deliberate load shedding keeps the system responsive for the traffic it does serve.
- **Degrade features rather than failing the whole request.** Serve cached or approximate results, disable expensive optional features, or return a reduced payload, so the user gets a useful (if reduced) response instead of an error.
- **Avoid failing open.** A component that, under load, silently stops enforcing a limit or validating input and lets everything through can flood a downstream dependency. Prefer failing closed (rejecting excess) over failing open (passing through) when a dependency is saturated.

### Use Backpressure And Rate Limiting To Protect Critical Paths Under Load

Backpressure and rate limiting are the mechanisms that keep load from overwhelming a component faster than it can process it. Without them, a fast producer overwhelms a slow consumer and the queue grows unbounded until memory or timeouts fail.

- **Propagate backpressure, do not buffer indefinitely.** When a downstream is slow, signal the slowness upstream (reject, slow the producer, apply flow control) rather than buffering every request in an unbounded queue. Unbounded buffering converts a slow downstream into an out-of-memory crash.
- **Apply rate limiting at the edges and between services.** Limit the rate of requests per client, per tenant, and per downstream dependency, so a single noisy source cannot consume a shared resource. Rate limits should protect the system, not just enforce plans.
- **Bound queues and define overflow behavior.** If you use a queue, bound it and decide what happens when it is full (drop oldest, drop newest, reject the producer, shed load). An unbounded queue hides overload until it becomes a crash; a bounded queue with explicit overflow degrades predictably.
- **Coordinate retries to avoid amplification.** Retries without backoff, jitter, and a retry budget multiply load on an already-slow downstream. Cap retries, use exponential backoff with jitter, and consider circuit breaking so retries do not turn a slowdown into a storm.

### Use Queue-Based Load Leveling To Absorb Spikes Without Overwhelming Backends

A queue between a frontend and a backend decouples the rate of incoming requests from the rate the backend can process, absorbing spikes that would otherwise overwhelm the backend directly.

- **Level load so the backend processes at a sustainable rate.** The queue lets the frontend accept bursts while the backend drains at its capacity, smoothing the load and protecting the backend from spikes it cannot handle synchronously.
- **Make the tradeoff explicit: latency for survivability.** Queue leveling trades increased latency (requests wait in the queue) for the backend staying up. This is the right trade under overload, but only if the added latency is acceptable to the caller; a queue that grows without bound adds unbounded latency.
- **Monitor queue depth and drain time.** Queue depth and estimated drain time are the key health signals; a growing queue means the backend cannot keep up and the incident is not over when traffic drops, because the backlog must still drain.

### Treat Capacity Headroom As A Monitored Incident-Prevention Tool

Capacity headroom is not leftover budget to be consumed; it is the buffer that prevents a traffic spike from becoming an incident. It must be sized deliberately, monitored continuously, and defended against silent erosion.

- **Size headroom for the largest plausible spike, not for average load.** Headroom should absorb the traffic spike you actually expect (a launch, a marketing event, a dependency retry storm, the loss of a zone), not just keep average utilization comfortable.
- **Monitor utilization against limits, not just against average.** Track how close each resource (connections, threads, memory, CPU, queue depth, database capacity) is to its hard limit, and alert before the limit is hit, so headroom erosion is caught before it causes an incident.
- **Defend headroom against silent consumption.** Headroom is eroded by traffic growth, new features, larger payloads, and shared-tenant noise. Re-measure capacity regularly, because yesterday's headroom may be gone today without any single change that announces it.

## Common Traps

### Treating Overload As A Performance Bug

Diagnosing a capacity event as a functional bug and hunting for a code cause, or rolling back a deploy that was never the problem, while the real issue is load exceeding capacity. Check the load and utilization signals first; the distinction drives a completely different response.

### Autoscaling That Scales Too Late Or Hits A Hard Ceiling

Trusting autoscaling to handle any spike, when it reacts after the fact, has warm-up lag, and stops at account quotas or pool limits that are hit exactly when demand peaks. Provision baseline headroom for the pre-scale window and verify the hard ceilings are high enough for the largest plausible spike.

### Retry Storms That Amplify Load Into Self-Inflicted Denial Of Service

A retry policy without backoff, jitter, or a retry budget that multiplies load on an already-slow downstream, turning a transient blip into a storm that overwhelms the dependency the retries were trying to protect. Cap retries, use exponential backoff with jitter, and apply circuit breaking.

### Failing Open Under Pressure And Flooding A Saturated Dependency

A component that, under load, stops enforcing limits or validating and lets everything through, flooding a downstream that was already saturated and turning a local slowdown into a cascade. Prefer failing closed (rejecting excess) over failing open (passing through) when a dependency is degraded.

### A Queue That Absorbs Load Then Takes Hours To Drain

A queue that successfully absorbs a spike but leaves the incident running long after traffic returns to normal, because the backlog drains slowly — and the incident is declared over while users still see latency from the draining queue. Monitor queue depth and drain time, and recognize the incident is not over until the backlog clears.

### Unbounded Buffering That Converts A Slow Downstream Into A Crash

Buffering every request in an unbounded queue or in-memory buffer when a downstream is slow, until memory is exhausted and the service crashes. Propagate backpressure and bound queues with explicit overflow behavior instead of buffering indefinitely.

### Headroom Consumed Silently Until A Spike Leaves None

Capacity headroom that was sized once at launch and never re-measured, eroded by traffic growth and new features until a routine spike finds none left and becomes an incident. Re-measure capacity regularly and alert on utilization against hard limits, not just against average.

## Self-Check

- [ ] Capacity incidents are distinguished from functional bugs before the response: load and utilization signals (traffic volume, queue depth, connection count, CPU/memory/thread saturation) are checked, code/config changes are ruled in or out, and the distinction drives whether the team sheds load and scales or rolls back and fixes.
- [ ] Autoscaling is understood as reactive with limits: baseline headroom covers the pre-scale window, warm-up cost is accounted for, and hard ceilings (account quota, pool max, instance availability, saturated dependencies) are verified high enough for the largest plausible spike, not just steady state.
- [ ] Graceful degradation is designed in: the critical path is identified and protected, non-critical features are shed or degraded under load, load is shed early and explicitly rather than failing late and randomly, and components fail closed (reject excess) rather than failing open (flood downstream) when a dependency is saturated.
- [ ] Backpressure and rate limiting protect critical paths: backpressure is propagated upstream rather than buffered indefinitely, rate limits are applied at edges and between services per client/tenant/dependency, queues are bounded with defined overflow behavior, and retries are capped with exponential backoff, jitter, and circuit breaking to avoid amplification.
- [ ] Queue-based load leveling is used where appropriate, with the latency-for-survivability tradeoff made explicit, queue depth and drain time monitored as health signals, and the incident recognized as ongoing until the backlog drains — not just until traffic drops.
- [ ] Capacity headroom is treated as a monitored incident-prevention tool: sized for the largest plausible spike (launch, marketing event, retry storm, zone loss), monitored as utilization against hard limits with alerts before limits are hit, and re-measured regularly to catch silent erosion from growth and new features.
- [ ] The highest-risk cases were verified — a spike that arrived before autoscaling caught up, a retry policy that amplified a slowdown into a storm, a component that failed open and cascaded, a queue that drained long after traffic normalized, and headroom that had eroded silently before the spike — not only the clean scale-up path.
