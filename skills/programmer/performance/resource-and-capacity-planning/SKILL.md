---
name: resource_and_capacity_planning.md
description: Use when the agent is sizing infrastructure for a service or workload — choosing instance types and counts, sizing clusters, autoscaling policies, memory/CPU/disk/network budgets, connection pool sizes, queue capacity, thread or worker counts, or deciding how much headroom to provision; forecasting capacity from traffic growth; choosing between vertical and horizontal scaling; reasoning about utilization versus saturation; setting autoscaling thresholds and cooldowns; or diagnosing capacity-related incidents (OOM, throttling, queue buildup, latency under load). Covers capacity headroom, saturation points, autoscaling lag and overshoot, cost versus reliability tradeoffs, and sizing for peak versus steady state.
---

# Resource And Capacity Planning

Capacity is the question of whether the system has enough of each resource — CPU, memory, disk, network, connections, file descriptors, queue depth, worker threads — to handle the load it will actually face, with enough margin to survive the load it did not expect. It is a question engineers answer badly in both directions. The first failure is under-provisioning: a service sized for average load that saturates and fails at peak, a connection pool sized for steady state that exhausts under a burst, an autoscaler that scales up too slowly to absorb a traffic spike because instances take minutes to boot. The second failure is over-provisioning: a cluster sized for a peak that never comes, burning cost perpetually; an autoscaler whose thresholds are so conservative it runs at 20% utilization forever; headroom treated as a comfort rather than a quantified, justified margin. Both failures share a root cause: capacity decisions made by guesswork about load and resource consumption, rather than by measurement of the utilization curves, saturation points, and growth trends that actually determine whether the system will hold.

Agents tend to treat capacity as a deployment-time number ("give it 4 instances") rather than as an ongoing relationship between load, utilization, and saturation that changes as traffic grows and code changes. The judgment problem is recognizing that capacity is a function of the workload and the resource consumption per unit of work, that every resource has a saturation point beyond which latency collapses or failures cascade, and that autoscaling is not a substitute for understanding the system's limits — it is a mechanism whose lag, thresholds, and overshoot must be designed. This skill covers the discipline of sizing resources to the load they will face, provisioning headroom deliberately, and designing autoscaling that actually protects the system rather than creating a false sense of security.

## Core Rules

### Size To The Workload, Measured, Not Guessed

Capacity is not an abstract number; it is the relationship between the load the system handles and the resources each unit of load consumes. Size from measurement of that relationship.

- **Measure resource consumption per unit of work.** CPU per request, memory per active session, connections per concurrent user, disk per customer per month. Without per-unit consumption, capacity is guesswork; with it, you can forecast capacity from forecast load.
- **Know the saturation point of each resource, not just its utilization.** Utilization (how busy) is comfortable up to a point; saturation (how much queued, how much blocked) is where latency collapses and failures begin. A CPU at 90% may be fine; a connection pool at 100% means requests wait and time out. Know, per resource, where the cliff is.
- **Distinguish average load from peak load, and size for the relevant one.** Average load determines cost; peak load determines whether the system survives. A system sized for average will fail at peak; a system sized for peak runs with headroom the rest of the time. Decide which you are sizing for and why.
- **Account for growth and seasonality.** Traffic grows over time and varies by time of day, day of week, and season. Capacity planning that ignores growth will be chronically short; planning that ignores seasonality will over-provision for the trough and under-provision for the peak.

### Provision Deliberate Headroom, Not Arbitrary Slack

Headroom is the margin between current utilization and the saturation point, and it is the system's ability to absorb spikes, failures, and growth without degrading. It must be quantified and justified, not a vague "give it some extra."

- **Define headroom as a fraction of capacity at peak load.** A common target is peak utilization at 50-70% of saturation, leaving 30-50% headroom for spikes, failover (when surviving instances absorb a failed instance's load), and growth before the next capacity adjustment.
- **Size headroom to the system's tolerance for degradation.** A batch system can run near saturation with little harm; a latency-sensitive user-facing system needs more headroom because latency degrades non-linearly near saturation. Match headroom to the consequence of saturation.
- **Account for failover capacity.** In a redundant system, the failure of one node shifts its load to the others. If the system runs at 80% utilization with all nodes healthy, a single failure pushes the survivors to 100%+ and they saturate. Headroom must cover the loss of the largest expected failure (a node, a zone).
- **Treat headroom as a budget that gets spent.** As traffic grows, headroom shrinks. Track the remaining headroom and plan capacity increases before it is exhausted, not after a saturation incident forces it.

### Design Autoscaling Around Its Lag, Not As Instant Capacity

Autoscaling adjusts capacity to load, and it is valuable — but it is not instantaneous, and treating it as such produces systems that saturate before the scale-up takes effect. Autoscaling has lag at every stage, and the design must account for it.

- **Account for the full scale-up lag.** The time from threshold-crossing to metric observation, plus the scaling decision, plus instance provisioning and boot, plus application warmup (cache fill, JIT compilation, connection establishment). This can be minutes, during which existing capacity carries the load. If existing capacity plus headroom cannot survive the lag, autoscaling will not save you from the spike.
- **Set thresholds with hysteresis to avoid flapping.** A threshold that scales up at 70% and down at 69% will oscillate under load near the threshold, provisioning and deprovisioning repeatedly. Use separate scale-up and scale-down thresholds (hysteresis) and cooldown periods to stabilize.
- **Scale on the right metric.** Scaling on CPU utilization is easy but may not reflect the binding constraint (memory, connections, queue depth). Scale on the metric that actually predicts saturation for the workload — often a queue depth or latency, not raw CPU.
- **Bound scale-down conservatively.** Scaling down quickly after a spike reclaims cost but leaves the system vulnerable if the spike resumes or a follow-on event arrives. Scale down slowly (minutes to tens of minutes) so transient dips do not leave the system under-provisioned.
- **Have a maximum and a minimum.** Autoscaling without bounds can scale unboundedly (cost runaway) or to zero (cold-start on next request). Set a maximum the system will not exceed and a minimum that preserves baseline capacity and avoids cold-starts.

### Distinguish Vertical From Horizontal Scaling, And Know Each Limit

Scaling up (bigger instances) and scaling out (more instances) have different properties, and the choice affects capacity planning, failure modes, and cost.

- **Horizontal scaling improves fault tolerance and scales theoretically without limit.** More instances mean a single failure is a smaller fraction of capacity. It is the right model for stateless services and for workloads that grow without bound.
- **Vertical scaling is simpler but has a ceiling and a larger blast radius.** A bigger instance handles more load, but the largest instance size is a hard limit, and a single instance's failure is a larger fraction of capacity. Use vertical scaling within a node and horizontal scaling across the growth dimension.
- **Match the scaling model to the workload's bottleneck.** A memory-bound workload may need vertical scaling (more memory per node) because horizontal scaling does not help if each unit of work needs a large memory footprint. A CPU-bound workload scales horizontally well.
- **Beware resources that do not scale horizontally.** A database's primary, a singleton leader, or a shared resource (a lock, a central queue) does not gain capacity by adding instances behind it. Identify the horizontally-unscaleable component; it is often the eventual capacity ceiling.

### Treat Connection Pools, Queues, And Worker Counts As Capacity Too

Capacity is not only instances and CPU. The limits that cause saturation under load are often internal resource pools whose sizing is easy to overlook.

- **Size connection pools to the concurrent demand, not a default.** A pool of 10 connections serving 100 concurrent requests means 90 wait, and latency collapses. Size pools to the peak concurrent demand, and ensure the downstream (database) can handle the pool's total connection count.
- **Size queues and buffers to the burst, with a policy for overflow.** An unbounded queue absorbs bursts but can grow unboundedly under sustained overload, exhausting memory. A bounded queue with a defined overflow policy (reject, shed load, dead-letter) fails predictably rather than OOM-ing.
- **Size worker thread/goroutine counts to the concurrency the workload needs.** Too few workers underutilize CPU and queue work; too many cause context-switching overhead and resource contention. The right count depends on whether the workload is CPU-bound (≈ core count) or I/O-bound (many more than cores).

## Common Traps

### Sizing For Average Load Instead Of Peak

Provisioning to average utilization, so the system saturates and fails at peak. Size for the peak (or the relevant percentile), with headroom; average determines cost, peak determines survival.

### Ignoring The Saturation Point

Watching utilization rise to "acceptable" levels while the resource is actually saturating (connection pool exhausted, queue depth growing), collapsing latency. Know the saturation cliff per resource, not just utilization.

### Treating Autoscaling As Instant Capacity

Relying on autoscaling to absorb spikes, ignoring the minutes of provisioning and warmup lag during which existing capacity carries the load. Provision headroom that survives the scale-up lag.

### Scaling On CPU When The Bottleneck Is Elsewhere

Autoscaling on CPU utilization while the binding constraint is memory, connections, or queue depth, so the autoscaler never triggers until the system is already failing. Scale on the metric that predicts saturation for the workload.

### Flapping Autoscaler Without Hysteresis

Scale-up and scale-down at the same threshold, causing oscillation and repeated provisioning/deprovisioning. Use hysteresis and cooldowns.

### No Failover Headroom

Running instances near capacity with all healthy, so a single failure saturates the survivors. Provision headroom that covers the loss of the largest expected failure.

### Unbounded Queues Absorbing Overload

A queue with no bound that grows unboundedly under sustained overload, exhausting memory. Bound queues with a defined overflow policy.

### Default Pool And Worker Sizes

Connection pools and worker counts left at framework defaults that are wrong for the workload's concurrency, causing saturation under load. Size internal resource pools to measured concurrent demand.

## Self-Check

- [ ] Resource consumption per unit of work (CPU/request, memory/session, connections/concurrent user) is measured, the saturation point of each resource is known (not just utilization), and capacity is forecast from forecast load rather than guessed.
- [ ] Capacity is sized for peak load (or the relevant percentile), not average, with deliberate, quantified headroom (e.g., peak at 50-70% of saturation) sized to the system's tolerance for degradation and to failover (surviving the loss of the largest expected failure).
- [ ] Growth and seasonality are accounted for in capacity planning, and remaining headroom is tracked so capacity increases are planned before headroom is exhausted rather than after a saturation incident.
- [ ] Autoscaling is designed around its full lag (metric observation, decision, provisioning, boot, warmup), thresholds use hysteresis and cooldowns to avoid flapping, scaling targets the metric that predicts saturation (not just CPU), scale-down is conservative, and minimum/maximum bounds are set.
- [ ] The scaling model (vertical vs horizontal) matches the workload's bottleneck, horizontally-unscaleable components (database primary, singletons, shared resources) are identified as eventual capacity ceilings, and the design accounts for them.
- [ ] Internal resource pools — connection pools, queues, worker thread/goroutine counts — are sized to measured concurrent demand (not defaults), queues are bounded with a defined overflow policy, and downstream systems can handle the pool's total load.
- [ ] Capacity decisions balance cost against reliability explicitly: over-provisioning cost is quantified, under-provisioning risk is quantified, and the chosen headroom reflects a deliberate tradeoff rather than guesswork.
- [ ] The system has been load-tested or stress-tested to confirm its actual capacity and saturation behavior matches the plan, not assumed to handle the forecast load.
