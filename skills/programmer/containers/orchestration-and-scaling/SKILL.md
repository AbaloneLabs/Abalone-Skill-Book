---
name: orchestration_and_scaling.md
description: Use when the agent is configuring how containers run and scale in a cluster, tuning an HPA or autoscaler on CPU, memory, custom, or request-rate metrics, setting target utilization and scale-down stabilization windows to avoid flapping, designing load balancing and service discovery for a service, wiring readiness and liveness probes so readiness gates traffic and liveness restarts without causing kill loops, implementing graceful shutdown with SIGTERM handling, preStop hooks, termination grace period, and connection draining, configuring rolling updates with surge and maxUnavailable, handling pod eviction, disruption budgets, and node-not-ready rescheduling, or deciding how stateful versus stateless workloads scale. Use when pods are thrashing on restart, a service is flapping under load, deploys drop in-flight requests, scale-up is too slow or scale-down too aggressive, or a node failure strands traffic. This skill is about cluster runtime behavior across container platforms (k8s as the dominant example), not k8s manifest design, image building, or release strategy.
---

# Orchestration And Scaling

Running a container is easy; running a *fleet* of them that scales with load, survives node failures, drains cleanly on deploy, and restarts only when it should is a different problem entirely. Orchestration is the layer that decides how many copies of a service run, which copy receives a request, when a copy is considered healthy enough to serve or sick enough to kill, and what happens to in-flight work when a copy is shut down, evicted, or rescheduled. Each of those decisions has a correct configuration and a large space of subtly wrong ones, and the wrong ones do not fail loudly — they fail as flapping replicas, dropped connections on every deploy, a service that will not scale up under load but scales down the moment it eases, or a node failure that strands traffic for minutes.

Agents tend to take the orchestrator's defaults and ship, because the defaults produce a service that *appears* to work under light load. The defaults are tuned for safety and simplicity, not for your service's actual behavior: a liveness probe that is too aggressive restarts a slow-but-healthy pod in a loop (the kill loop); a scale-down window that is too short destroys replicas the moment a traffic blip ends, then scales them back up when the next blip arrives (flapping); a missing preStop hook and a too-short grace period force-kill pods that are still draining connections, dropping requests on every rollout. The judgment problem is treating each orchestration knob as a decision about your service's real runtime behavior — its startup time, its request latency, its connection lifetimes, its statefulness — and configuring it deliberately rather than accepting the default.

This skill is the runtime behavior of orchestrated containers. It is platform-general but uses Kubernetes as the dominant example, because that is where most of these decisions are made explicitly. It is distinct from `container-image-design` (building the image) and `dockerfile-best-practices` (writing the Dockerfile); the release-strategy and blue-green/canary skills cover *how a new version is rolled out*, while this skill covers *how the running fleet scales, stays healthy, and shuts down* regardless of whether a deploy is happening.

## Core Rules

### Scale On The Metric That Actually Tracks Demand

An autoscaler scales when its target metric crosses a threshold, so the choice of metric decides what "load" means for your service. The wrong metric scales on something unrelated to real demand (or fails to scale on real demand). Choose deliberately:

- **CPU utilization** is the default and works for compute-bound services where CPU saturation tracks request load. It fails for I/O-bound, latency-bound, or queue-driven services, where CPU stays low while the service is overloaded.
- **Memory utilization** is rarely a good *scaling* signal (high memory usually indicates a leak or a per-replica workload, not a need for more replicas), but it is a useful saturation guard. Do not autoscale stateless services on memory.
- **Custom and external metrics (request rate, queue depth, concurrent connections, p99 latency)** scale on the actual driver of demand. A queue worker should scale on queue depth; an HTTP service on requests-per-second-per-pod or concurrent in-flight requests; a streaming service on active connections. Custom metrics (via the metrics pipeline — Prometheus adapter, KEDA) let you scale on what actually predicts overload.
- **Target utilization is a per-replica target, not a fleet average.** Set it low enough that the fleet has headroom before saturation (e.g., 70% CPU, not 95%), so scale-up happens *before* latency degrades, not after.

The strong pattern: scale on the metric whose growth predicts overload for *this* service, set the target with headroom, and confirm empirically that scaling fires before latency or errors rise. The weak pattern: CPU at 80% because it is the default, for a service whose real bottleneck is queue depth.

### Tune Scale-Up Aggressiveness And Scale-Down Stabilization Separately

Scale-up and scale-down have opposite requirements, and treating them symmetrically produces both kinds of flapping. Scale-up should be fast and willing to overshoot (it is cheaper to run extra replicas briefly than to be overloaded); scale-down should be slow and conservative (killing a replica that is immediately needed again is pure churn).

- **Scale-up: react fast and allow a jump.** Permit scaling more than one replica at a time when the deficit is large, and do not require a long observation window before adding capacity. The cost of being slow here is overload and latency; the cost of being fast is a little wasted capacity.
- **Scale-down: require a sustained period of low utilization before removing replicas.** The scale-down stabilization window (HPA's behavior, or the orchestrator's cooldown) should be long enough to span a normal traffic dip — minutes, not seconds — so a momentary lull does not trigger removal. Tune `scaleDown.stabilizationWindowSeconds` (default 300) to your traffic pattern; for spiky traffic, raise it.
- **Cap the min and max replica counts realistically.** The min protects against scaling to zero under noise (and keeps warm capacity for sudden spikes); the max bounds cost and prevents a runaway autoscaler from saturating the cluster.

A service that adds replicas under load and removes them the instant load eases, only to re-add them moments later, is flapping — the signature of a too-short scale-down window. A service that takes minutes to scale up under a spike is under-provisioned for the spike — the signature of too-conservative scale-up or too-low a max. Tune each direction against the symptom you see.

### Use Readiness Probes To Gate Traffic And Liveness Probes Only To Restart The Wedged

The two probe types do different things and conflating them is the most common orchestration mistake. **Readiness** determines whether a pod receives traffic — a failing readiness probe removes the pod from the service's endpoints but does not restart it. **Liveness** determines whether a pod is restarted — a failing liveness probe kills and recreates the pod. Configure them for their distinct purposes:

- **Readiness should reflect "can this pod serve a request right now."** It should fail while the app is starting (so traffic is not sent before it is ready), fail if the pod is temporarily overloaded (so load sheds to healthy replicas), and pass once the app can handle requests. A readiness probe that checks a real endpoint (`/readyz` that verifies downstream dependencies and warmup) is far better than a TCP check.
- **Liveness should reflect "is this pod wedged in a way only a restart can fix."** It must be much more conservative than readiness, because a false positive restarts a healthy pod and, in a loop, can take down a service. Liveness should not check downstream dependencies (a database blip would restart every pod), should have a generous failure threshold and timeout, and should target a signal that indicates a true deadlock (e.g., the app stops responding to its own health endpoint).
- **Do not point liveness at the same endpoint as readiness with the same thresholds.** That makes every transient overload into a restart, which is the kill loop.

The kill loop: an aggressive liveness probe fails, the pod is killed mid-request, restarted, fails again before warming up, killed again — the service never converges and drops traffic continuously. If pods are restarting frequently under load, suspect the liveness probe before the app.

### Implement Graceful Shutdown End-To-End

When a pod is terminated — for a deploy, a scale-down, or a node drain — it receives a `SIGTERM` and is given a grace period (default 30s) before `SIGKILL`. What happens in that window decides whether in-flight requests complete or are dropped. Graceful shutdown is a chain, and every link must hold:

- **The process must handle `SIGTERM`** (not be run under a shell that swallows it — see `dockerfile-best-practices` on exec form) and begin shutting down: stop accepting new work, finish in-flight requests, close connections.
- **The orchestrator must stop sending new traffic**, but there is a race: endpoint propagation takes time, so the pod may receive requests for a brief window after `SIGTERM`. A **preStop hook** (a small sleep, or a deregistration call) delays `SIGTERM` delivery until the load balancers have de-registered the pod, closing the race.
- **The termination grace period must exceed the real drain time.** If the app needs 20 seconds to drain long-lived connections (WebSockets, streaming, slow uploads) and the grace period is 30s, fine; if it needs 45s and the grace period is 30s, the pod is force-killed mid-drain. Measure the real drain time and set the grace period above it.
- **Drain or hand off stateful connections explicitly.** For long-lived connections, the app should signal clients to reconnect elsewhere (or the load balancer should drain) rather than dropping the socket.

Test shutdown under load: trigger a rollout and confirm that request error rate does not spike, that pods exit within the grace period (not at it), and that there are no `SIGKILL` lines in the logs. A deploy that drops requests is almost always a shutdown-chain bug, not a deploy-strategy bug.

### Configure Rolling Updates To Bound Surge And Unavailability Together

A rolling update replaces old pods with new ones incrementally, and two knobs bound the blast radius: **maxUnavailable** (how many old pods can be gone at once) and **maxSurge** (how many extra new pods can exist beyond the desired count). Their interaction decides whether an update trades capacity for speed:

- **maxUnavailable: 0** guarantees the desired capacity is always met — new pods must be ready before old ones are removed. Safest for capacity, but slower and requires headroom. Required when you cannot tolerate any capacity loss during rollout.
- **maxSurge > 0** allows temporary extra capacity to speed the rollout — new pods are created alongside old ones, then old ones removed. Faster, but requires the cluster to fit the surge.
- **The combination matters.** `maxUnavailable: 0, maxSurge: 25%` (a common default) means never below desired count, temporarily up to 125%. `maxUnavailable: 25%, maxSurge: 0` means never above desired count, temporarily down to 75% — cheaper but reduces capacity during rollout, which can cause overload if you are already near limits.

Choose by whether your service can tolerate capacity loss during rollout and whether the cluster has headroom for surge. A service running at 90% utilization cannot afford `maxUnavailable > 0`; a cluster with no spare quota cannot afford `maxSurge > 0`. Pair this with the graceful-shutdown chain above — a rolling update is only as clean as the shutdowns it triggers.

### Plan For Node Failure And Voluntary Disruption

Pods do not only die for deploys; nodes fail, get drained, and autoscale away, and the orchestrator must reschedule. Two controls govern this:

- **PodDisruptionBudgets (PDB)** protect against *voluntary* disruption (node drain, cluster autoscaler, evictions) by requiring a minimum number of replicas to stay available. A PDB with `minAvailable: 2` prevents a drain from evicting below 2 healthy pods, so voluntary maintenance cannot take down a service. Without a PDB, a node drain can evict all replicas of a service at once.
- **Node-not-ready behavior** is *involuntary* and a PDB does not protect against it — when a node dies, its pods are gone regardless. Protection here is redundancy: enough replicas spread across enough nodes (anti-affinity/topology spread) that losing one node does not drop below the service's minimum viable capacity, plus fast rescheduling.

Configure anti-affinity or topology spread constraints so replicas land on different nodes (and, for critical services, across zones), so a single node failure is not a capacity event. Set a PDB so voluntary drains cannot over-evict. And confirm the rescheduling time (how long until a new pod is scheduled, image-pulled, started, and ready on a healthy node) is within your service's tolerance for reduced capacity — for a service that cannot tolerate minutes at reduced capacity, run more replicas or across more zones.

### Treat Stateful And Stateless Scaling As Fundamentally Different

The scaling rules above assume stateless services, where any replica can serve any request and replicas are interchangeable. Stateful services (databases, queues, caches with affinity, anything with a durable identity or local data) scale by completely different rules, and applying stateless patterns to them is destructive:

- **Stateless:** scale freely, terminate any replica, balance round-robin, no per-replica identity. Horizontal scaling is the norm.
- **Stateful:** each replica has an identity and durable state (a shard, a partition, a persistent volume). Scaling changes the data topology, not just the copy count. Adding a replica may require rebalancing data; removing one requires draining its data first. Use StatefulSets (stable identity, ordered rollouts, persistent volume per pod), and scale deliberately, often one at a time, with application-level coordination.

Do not put a stateful workload behind a stateless autoscaler that kills replicas freely — you will lose data or trigger expensive rebalancing. Identify which services are stateful, and for those, scale by the rules of the data system (sharding, replication, planned adds/removes), not the rules of a stateless web service.

### Match Service Discovery And Load Balancing To The Client And Topology

How a client finds and reaches a service affects latency, failover, and session behavior:

- **DNS-based service discovery** (the cluster DNS, or external DNS) is the default and works for most internal services. Be aware of DNS caching in clients — a client that caches a resolved IP for minutes will not fail over quickly when endpoints change. Tune client DNS TTL or use a client that re-resolves.
- **Service mesh / sidecar LB** (Istio, Linkerd) gives client-side load balancing, retries, mTLS, and fine-grained traffic control, at the cost of operational complexity and a new failure domain. Worth it for large fleets with cross-cutting concerns; overkill for a small service.
- **Session affinity (sticky sessions)** is usually a smell — it indicates the service is not truly stateless (in-memory session state), and affinity breaks even load distribution and slows failover. Prefer externalizing session state so any replica can serve any request. If you must use affinity, understand it trades load balancing quality for session continuity and complicates rolling updates.

Strong default: stateless service behind cluster DNS with kube-proxy (or a mesh) doing per-connection load balancing, no session affinity, session state externalized. Reach for a mesh when you need its cross-cutting features, not as a default.

## Common Traps

### Liveness Probe That Restarts Healthy Pods Under Load

An aggressive liveness probe (short timeout, low threshold, checking a downstream dependency) fails when the pod is momentarily slow or a dependency blips, restarting a pod that would have recovered — and in a loop, restarting the whole fleet. Make liveness conservative, dependency-free, and tuned to detect true wedging, not transient slowness.

### Scale-Down Window So Short The Service Flaps

Removing replicas the instant utilization dips, then re-adding them when the next request burst arrives, churning the fleet and warming/cold-starting pods repeatedly. Set the scale-down stabilization window to span normal traffic dips (minutes), and keep scale-up aggressive.

### Forgetting The preStop Hook, So Requests Arrive After SIGTERM

The pod gets `SIGTERM` and starts shutting down, but the load balancer still sends it traffic for a second or two because endpoint propagation lags, so requests hit a dying pod and fail. A preStop hook (a short sleep or deregister) bridges the propagation delay; without it, every deploy drops a small fraction of requests.

### Grace Period Shorter Than The Real Drain Time

The app needs 40 seconds to finish long-lived connections, but the termination grace period is 30, so the pod is `SIGKILL`ed mid-drain. Measure real drain time (especially for WebSockets, streaming, and slow uploads) and set the grace period above it.

### Scaling On CPU For A Non-Compute-Bound Service

Autoscaling a queue worker or an I/O-bound API on CPU, so it never scales up while queues back up or latency climbs (CPU stays low). Scale on the metric that tracks real demand — queue depth, request rate, concurrent connections — via custom metrics.

### maxUnavailable That Drops Capacity Below The Service's Limit

A rolling update with `maxUnavailable: 25%` on a service already running near capacity, so during rollout the fleet is at 75% and overloads, causing errors that look like a bad release but are really a capacity event. Use `maxUnavailable: 0` for services without headroom.

### No PDB, So A Node Drain Takes Down The Service

A voluntary node drain evicts all replicas of a service at once because there is no PodDisruptionBudget, causing an outage during routine maintenance. Set a PDB requiring minimum available replicas for every service that must stay up.

### Session Affinity Masking A Statefulness Problem

Using sticky sessions because "the app needs them," when the real issue is in-memory session state that should be externalized. Affinity hides the problem, breaks even load balancing, and slows failover and rollouts. Externalize the state and drop the affinity.

### Treating A Stateful Workload Like A Stateless One

Autoscaling a database or sharded cache with a stateless HPA that freely kills replicas, causing data loss or rebalancing storms. Identify stateful services and scale them by their data system's rules (ordered, coordinated, planned), with StatefulSets and persistent volumes.

## Self-Check

- [ ] The autoscaler scales on the metric that tracks real demand for the service (CPU for compute-bound, custom/external metrics like queue depth, request rate, or concurrent connections for I/O- or latency-bound), with a target utilization that leaves headroom before saturation.
- [ ] Scale-up is aggressive (allows multi-replica jumps, short observation) and scale-down is conservative (stabilization window spanning normal traffic dips), min/max replica counts are set realistically, and the service does not flap under variable load.
- [ ] Readiness probes gate traffic (fail during startup and overload, check a real readiness endpoint including downstream deps) and liveness probes only restart truly wedged pods (conservative, dependency-free, generous thresholds) — they are not the same endpoint with the same thresholds.
- [ ] Graceful shutdown is implemented end-to-end: the process handles SIGTERM (exec form, not shell), a preStop hook bridges endpoint-propagation delay, the termination grace period exceeds the measured real drain time, and a rollout under load shows no request-error spike or SIGKILLs.
- [ ] Rolling update maxUnavailable and maxSurge are chosen by whether the service tolerates capacity loss and whether the cluster has surge headroom, with maxUnavailable: 0 for services running near capacity.
- [ ] A PodDisruptionBudget protects every must-stay-up service against voluntary eviction, replicas are spread across nodes and zones (anti-affinity/topology spread) so a single node failure is not a capacity event, and rescheduling time is within the service's tolerance.
- [ ] Stateful workloads are identified and scaled by their data system's rules (StatefulSets, ordered/coordinated/planned adds and removes, persistent volumes), not by a stateless autoscaler that freely kills replicas.
- [ ] Service discovery and load balancing match the client and topology (DNS or mesh, client DNS re-resolution for fast failover), session affinity is avoided in favor of externalized session state, and a mesh is adopted only when its cross-cutting features justify the complexity.
