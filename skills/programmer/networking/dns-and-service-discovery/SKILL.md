---
name: dns_and_service_discovery.md
description: Use when the agent is designing or debugging DNS resolution, choosing DNS TTLs, implementing service discovery for a distributed system (client-side, server-side, or service mesh), configuring health checks and failover, or diagnosing split-brain and stale-endpoint problems. Also covers the failure mode of DNS caching that delays failover, service discovery that routes to dead instances, health checks that report healthy while the service is broken, and split-brain where two nodes both believe they are primary.
---

# DNS And Service Discovery

DNS and service discovery are how services find each other in a distributed system, and they are load-bearing in a way that is easy to underestimate because they are invisible when they work. The judgment problem is that both involve caching, propagation, and eventual consistency, and the failure modes appear at the worst times — during a failover, when a node dies, when a network partitions — exactly when correctness matters most. DNS resolution caches results for the TTL, so a failover that changes the target address is not seen by clients until the cache expires, which can be minutes; service discovery that tracks instances can route to a dead instance if its view is stale; health checks that report "healthy" while the service is actually broken cause traffic to keep flowing to a failing node. The discipline is to treat DNS TTL as a failover-speed constraint, to make service discovery's view converge quickly and never route to a dead instance, to design health checks that reflect the service's ability to serve (not merely its process being alive), and to handle the partition and split-brain cases that break the assumption that there is one agreed-upon truth.

Agents tend to treat DNS and service discovery as plumbing that "just works." The harm appears as failovers that take minutes longer than expected (because of DNS caching), as traffic routed to terminating instances (because the discovery view is stale), as health checks that pass while users fail (because they check the wrong thing), and as split-brain data corruption when a partition lets two nodes both act as primary. The judgment is to set DNS TTL consistent with the failover speed you need, to make service discovery refresh fast and to drain instances before they disappear from routing, to design health checks that verify real serving capability, and to use quorum or fencing to prevent split-brain. Finding a service is a distributed-systems problem with distributed-systems failure modes, not a lookup.

## Core Rules

### Treat DNS TTL As A Failover-Speed Constraint

DNS clients cache resolved addresses for the TTL, so the TTL is the maximum delay before a DNS change (a failover, a migration) is seen by all clients. A long TTL makes failover slow; a short TTL increases DNS query volume and dependency on the resolver.

- **Set TTL consistent with the failover speed you need.** If failover must be fast, the TTL must be short (seconds to tens of seconds); if failover can be slow, a longer TTL reduces query load. Do not set a long TTL and expect fast failover.
- **Do not rely on DNS alone for fast failover.** DNS propagation is bounded by TTL but also by intermediate resolvers and client caches that may ignore it; for fast failover, combine DNS with a faster mechanism (health-checked load balancing, service mesh).
- **Account for clients that cache beyond the TTL.** Some clients and libraries cache DNS aggressively and ignore TTL; for critical services, design as if some clients will not see the change promptly.

### Make Service Discovery Converge Quickly And Never Route To A Dead Instance

Service discovery maintains the set of healthy instances and routes traffic to them. Its correctness depends on the view converging quickly when instances come and go, and on never routing to an instance that cannot serve.

- **Refresh the instance view frequently.** A discovery view that updates every 60 seconds routes to dead instances for up to 60 seconds; a faster refresh (or push-based updates) reduces the window.
- **Drain instances before they disappear.** When an instance is terminating, deregister it from discovery and let in-flight requests finish before killing it; routing to an instance that is mid-shutdown causes failures.
- **Prefer health-integrated discovery.** Discovery that routes only to instances passing health checks avoids routing to dead or unhealthy instances; discovery without health integration routes blindly.
- **Handle the registration race on startup.** An instance that registers before it is ready receives traffic it cannot serve; register only after startup and warm-up complete.

### Design Health Checks That Reflect Real Serving Capability

A health check determines whether an instance receives traffic, and it must reflect whether the instance can actually serve users — not merely whether its process is alive. A check that reports healthy while the service is broken keeps traffic flowing to a failing node.

- **Check the service's ability to serve, not just liveness.** A deep health check exercises the critical path (can it connect to its dependencies, can it serve a representative request); a shallow check (TCP port open, process alive) passes while the service is broken.
- **Distinguish liveness from readiness.** Liveness (the process is healthy and should not be restarted) differs from readiness (the instance is ready to serve traffic); an instance can be live but not ready (still warming up, dependency unavailable), and should not receive traffic.
- **Avoid cascading failures from deep health checks.** A health check that calls downstream dependencies can amplify a failure (every instance's health check hammers the dependency); design the check to reflect the instance's state without creating a dependency storm.
- **Set health-check thresholds to avoid flapping.** A check that flips healthy/unhealthy on transient blips causes traffic to flap; require sustained failure before marking unhealthy.

### Prevent Split-Brain With Quorum Or Fencing

In systems with a primary (a leader, a writable node), a network partition can leave two nodes both believing they are primary, each accepting writes — split-brain — which corrupts data when the partition heals. Prevent this with a quorum (a majority must agree on the primary) or fencing (a mechanism to ensure only one primary can act at a time).

- **Use quorum-based leader election.** A node becomes primary only with majority agreement, so a partition cannot produce two primaries each with a majority; one side lacks quorum and stands down.
- **Use fencing to revoke a stale primary's authority.** Even with quorum, a former primary may not know it has been superseded; fencing (a lease, a generation counter, an I/O barrier) ensures its writes are rejected after it is superseded.
- **Prefer systems with built-in split-brain prevention** (consensus systems like Raft/Paxos) over ad-hoc primary election that lacks quorum.
- **Test the partition case.** Split-brain prevention that has never been tested under a partition will fail when one occurs; inject partitions and verify only one primary acts.

### Handle DNS And Discovery As Eventual Consistency, Not Instant Truth

Both DNS and service discovery are eventually consistent: changes propagate with delay, and different clients may see different views at the same instant. Design as if the view may be stale, not as if it is instant truth.

- **Tolerate stale views gracefully.** Clients should retry on a failed instance and let discovery route elsewhere, not fail hard because one resolved address is dead.
- **Do not assume all clients see the same view simultaneously.** During a change (deploy, failover), some clients see the old view and some the new; design for the overlap.

### Document the Basis and the Reasoning

Every conclusion should be traceable to its evidence, assumptions, and alternatives considered. Record not only the outcome but the reasoning path: what was checked, what was ruled out, what uncertainty remains, and what would change the conclusion. Documentation that captures the basis allows another practitioner to review, reproduce, or challenge the work, and it prevents confident conclusions from becoming unrepeatable assertions. A decision made without a recorded basis cannot be audited, improved, or safely handed off.

## Common Traps

### Long DNS TTL Blocking Fast Failover

A DNS TTL of minutes or hours that delays failover beyond what the system requires, because clients cache the old address until the TTL expires. Set TTL consistent with the needed failover speed; do not rely on DNS alone for fast failover.

### Routing To Terminating Or Dead Instances

A discovery view that is stale or that does not drain instances before removal, so traffic routes to instances that are shutting down or dead. Refresh the view frequently, deregister before termination, and drain in-flight requests.

### Shallow Health Checks Passing While The Service Is Broken

A health check that verifies only the process or port while the service cannot serve (dependency down, internal error), keeping traffic flowing to a failing node. Check the service's ability to serve, distinguish liveness from readiness, and avoid amplifying failures with overly deep checks.

### Registration Race On Startup

An instance that registers with discovery before it is ready, receiving traffic it cannot serve. Register only after startup and warm-up complete.

### Split-Brain From Partition Without Quorum

A primary-election scheme without quorum or fencing, where a partition leaves two nodes both acting as primary and accepting writes that corrupt data on heal. Use quorum-based election and fencing; prefer consensus systems; test the partition case.

### Assuming Instant Propagation

Designing as if DNS and discovery changes propagate instantly to all clients, when both are eventually consistent and clients may see stale views during a change. Tolerate stale views; do not assume all clients see the same truth simultaneously.

### Health-Check Flapping

A health check that flips healthy/unhealthy on transient blips, causing traffic to flap and amplifying instability. Require sustained failure before marking unhealthy; set thresholds to avoid flapping.

## Self-Check

- [ ] DNS TTL is set consistent with the failover speed required (short TTL for fast failover), DNS is not relied on alone for fast failover (combined with health-checked load balancing or a service mesh), and clients that cache beyond TTL are accounted for.
- [ ] Service discovery converges quickly (frequent refresh or push-based updates), instances are drained (deregistered with in-flight requests allowed to finish) before they disappear from routing, discovery is health-integrated so it never routes to dead/unhealthy instances, and the startup registration race is handled (register only after warm-up).
- [ ] Health checks reflect real serving capability (deep checks exercise the critical path and dependencies), liveness is distinguished from readiness, checks do not amplify failures by storming downstream dependencies, and thresholds prevent flapping.
- [ ] Split-brain is prevented with quorum-based leader election (a node becomes primary only with majority agreement) and fencing (leases, generation counters, I/O barriers) so a superseded primary's writes are rejected, consensus-based systems are preferred over ad-hoc election, and the partition case is tested.
- [ ] DNS and discovery are treated as eventual consistency (changes propagate with delay, clients may see stale views during a change), and clients tolerate stale views gracefully (retry and reroute rather than fail hard).
- [ ] The highest-risk cases were verified — a failover that was not delayed by DNS caching, an instance removed from routing before termination, a health check that caught a broken-but-alive service, and a partition that did not produce split-brain — not only the clean steady-state path.
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
- Is the reasoning documented clearly enough that another practitioner could review the basis and reproduce the conclusion?
