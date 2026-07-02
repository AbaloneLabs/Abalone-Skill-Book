---
name: load_balancing_strategies.md
description: Use when the agent is choosing a load balancing algorithm (round-robin, least-connections, consistent hashing), deciding between L4 and L7 load balancing, configuring session affinity or stickiness, setting up health checks for the pool, designing failover and overflow capacity, or diagnosing uneven distribution and hotspots. Also covers the failure mode of an algorithm that looks balanced but creates hotspots, session affinity that defeats failover, health checks that remove too many or too few instances, and a pool sized for average load that collapses at peak.
---

# Load Balancing Strategies

Load balancing distributes traffic across a pool of instances, and the algorithm choice determines whether the load is actually balanced, whether stateful sessions work, and whether failures are contained. The judgment problem is that "balanced" means different things for different workloads: for stateless uniform requests, round-robin distributes evenly; for requests with wildly varying cost, round-robin overloads the instance that drew the expensive ones; for stateful sessions, distribution must respect affinity; for cache-friendly workloads, consistent hashing localizes access. The discipline is to match the algorithm to the workload's characteristics (uniform vs. varying cost, stateless vs. stateful, cache-sensitive), to choose L4 vs. L7 based on whether routing decisions need application-layer information, to size the pool for peak with headroom and to plan for the loss of instances, and to design health checks that keep the pool healthy without over-removing or under-removing instances.

Agents tend to pick the default algorithm and assume balance. The harm appears as hotspots (one instance overloaded because it drew the expensive requests or the popular keys), as session breakage (when affinity is missing or misconfigured), as cascading removals (a health check that removes too many instances, overloading the rest), and as collapse at peak (a pool sized for average load). The judgment is to understand what the algorithm actually optimizes (count vs. cost vs. locality), to handle the stateful and varying-cost cases deliberately, to size for peak and failure scenarios, and to treat the health check as a pool-management decision with failure-cascade consequences. Load balancing is not "spread requests around"; it is "keep the pool healthy and the latency low under real traffic and real failures."

## Core Rules

### Match The Algorithm To The Workload's Characteristics

Different algorithms optimize different things, and the wrong choice creates hotspots or breaks semantics. Understand what each algorithm distributes (request count, connection count, cost, locality) and choose for the workload.

- **Round-robin / random** distribute request count evenly and work for stateless, uniform-cost requests. They overload instances when request cost varies widely (one instance draws the expensive requests).
- **Least-connections** distributes by current load and handles varying-cost requests better, because an instance processing expensive requests has more connections and draws fewer new ones. Prefer this when request cost varies.
- **Consistent hashing** routes by key to localize access (cache affinity, session affinity, sharding), so the same key always goes to the same instance (until the pool changes). Use it for cache-friendly or partitioned workloads; understand that it creates uneven distribution if keys are skewed.
- **Weighted variants** handle heterogeneous pools (larger instances get more traffic); use when the pool is not uniform.

### Choose L4 Vs. L7 Based On Whether Routing Needs Application-Layer Information

L4 (transport-layer) load balancing routes on connection-level information (source/destination IP and port); L7 (application-layer) routes on request content (path, header, cookie, host). The choice depends on whether routing decisions need to see inside the request.

- **L4 is faster and simpler** and works when any instance can handle any request and routing does not depend on content. It terminates the TCP connection at the balancer or passes it through.
- **L7 is necessary when routing depends on content** (route /api to one pool, /static to another; route by host or cookie; perform SSL termination with per-certificate logic). It is more flexible but more expensive and a potential bottleneck.
- **L7 enables smarter routing and traffic control** (path-based routing, header-based canary, retry and circuit-breaking at the proxy), at the cost of the proxy doing more work and being a more involved failure point.

### Handle Session Affinity Deliberately

Some workloads require that requests from a session go to the same instance (a stateful session, a cached-on-instance resource). Affinity must be designed deliberately, because it conflicts with even balancing and with failover.

- **Use affinity only when needed, and understand its cost.** Affinity concentrates a session's load on one instance, defeating even distribution; it also means the session fails when that instance fails unless state is replicated or externalized.
- **Prefer externalized state over instance-affinity.** Keeping session state in a shared store (Redis, database) lets any instance handle any request, removing the need for affinity and making failover seamless. Affinity is a fallback when externalization is not feasible.
- **If using affinity, choose a robust mechanism** (a cookie, consistent hashing on a session ID) and handle instance loss (re-route to another instance, re-establish state).
- **Avoid IP-based affinity when clients are behind NAT.** Many users share one IP (a corporate NAT, a mobile carrier), so IP affinity concentrates all their traffic on one instance; use a session-level identifier instead.

### Design Health Checks That Keep The Pool Healthy Without Cascading

The health check determines which instances receive traffic, and its design has cascade consequences: too sensitive, and a blip removes instances, overloading the rest and causing more failures; too insensitive, and traffic flows to broken instances.

- **Check what matters for serving.** A health check should reflect whether the instance can serve, not just whether it is alive (see the DNS and service-discovery skill for depth on this); but for pool management, also consider the check's cost and failure-cascade behavior.
- **Avoid removing too many instances at once.** If a shared dependency fails, every instance's health check may fail simultaneously, emptying the pool; design the check (or the pool) so a shared-dependency failure does not remove all instances.
- **Set removal and re-addition thresholds to avoid flapping.** An instance should be removed after sustained failure, not a single blip, and re-added after sustained recovery, so the pool does not churn.
- **Plan for the pool operating with fewer instances.** The pool must tolerate losing instances (to failure, to deploys) without collapsing; size for the degraded case, not only the full case.

### Size The Pool For Peak And Failure Scenarios

A pool sized for average load collapses at peak or when instances fail. Size for the realistic peak, with headroom, and for the loss of some instances.

- **Size for peak traffic with headroom.** Traffic spikes (events, growth, retries); a pool at 90% utilization at average load has no room for the spike.
- **Plan for instance loss.** If the pool loses instances (failure, rolling deploy), the remaining instances must handle the load; size so the loss of N instances does not collapse the pool.
- **Account for retries amplifying load.** During a partial failure, clients retry, increasing load above the original; the pool must absorb the retry multiplier.
- **Consider multi-zone or multi-region distribution.** A pool in one zone fails when the zone fails; distribute across zones/regions for availability, and ensure the balancer routes around a failed zone.

## Common Traps

### Round-Robin On Varying-Cost Requests

Round-robin (distributing request count) on requests whose cost varies widely, overloading the instance that drew the expensive ones. Use least-connections or a cost-aware algorithm when request cost varies.

### Consistent Hashing With Skewed Keys

Consistent hashing on skewed keys (a few popular keys), concentrating load on the instances that own those keys. Be aware of key distribution; consider bounded-load consistent hashing to cap imbalance.

### Session Affinity That Defeats Failover

Affinity that concentrates a session on one instance with no failover, so the session breaks when the instance fails. Prefer externalized state; if using affinity, handle instance loss.

### IP-Based Affinity Behind NAT

IP-based affinity where many users share one IP (NAT, mobile carrier), concentrating their traffic on one instance. Use a session-level identifier, not the IP.

### Health Check That Cascades

A health check that removes too many instances at once (shared-dependency failure empties the pool) or that flaps on blips, churning the pool. Design the check and pool so a shared failure does not remove all instances, and use sustained-failure thresholds.

### Pool Sized For Average Load

A pool sized for average load that collapses at peak or when instances fail, with no headroom and no plan for instance loss. Size for peak with headroom and for the loss of N instances, accounting for retry amplification.

### L4 Where Content-Based Routing Is Needed

L4 load balancing where routing depends on request content (path, host, cookie), because L4 cannot see inside the request. Use L7 when routing decisions need application-layer information.

## Self-Check

- [ ] The load balancing algorithm is matched to the workload: round-robin/random for stateless uniform-cost requests, least-connections for varying-cost requests, consistent hashing for cache/session/sharding affinity (with awareness of skewed-key hotspots), and weighted variants for heterogeneous pools.
- [ ] L4 vs. L7 is chosen based on whether routing needs application-layer information (L7 for path/host/cookie-based routing, SSL termination with per-certificate logic, retries/circuit-breaking; L4 for speed and simplicity when any instance can handle any request).
- [ ] Session affinity is used only when needed, with its cost understood (concentrates load, conflicts with failover); state is externalized where possible to remove the need for affinity; affinity uses a session-level identifier (not IP, which breaks behind NAT); and instance loss is handled.
- [ ] Health checks keep the pool healthy without cascading: they reflect serving capability, a shared-dependency failure does not empty the pool, removal/re-addition uses sustained thresholds to avoid flapping, and the pool is designed to operate with fewer instances.
- [ ] The pool is sized for peak traffic with headroom, for the loss of N instances (failure, rolling deploy), and for retry amplification during partial failures; multi-zone/multi-region distribution is used for availability with routing around failed zones.
- [ ] The highest-risk cases were verified — a varying-cost workload balanced by least-connections not round-robin, a session that survived instance loss via externalized state, a health check that did not cascade on a shared-dependency failure, and a pool that absorbed a peak without collapsing — not only the clean uniform-load path.
