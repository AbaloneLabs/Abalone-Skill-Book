---
name: distributed_caching_and_coordination.md
description: Use when the agent is designing or reviewing a distributed cache (Redis, Memcached, CDN), choosing a cache update strategy (cache-aside, read-through, write-through, write-behind, refresh-ahead), handling cache invalidation and TTL expiry races, mitigating thundering herd or cache stampede under cold-cache or eviction storms, building or using a distributed lock (Redis SETNX, Redlock, etcd lease, ZooKeeper lock), deciding whether a lock is for correctness or efficiency, adding fencing tokens to protected resources, or adopting a coordination service (etcd, Consul, ZooKeeper) for leader election, service discovery, or configuration. Covers the caching and coordination pattern mechanics and their failure modes, distinct from consistency-model theory and from the transaction patterns (2PC/saga/outbox).
---

# Distributed Caching And Coordination

Distributed caching and coordination are the patterns you reach for to make a system fast (cache) or to make independent nodes cooperate safely (locks, coordination services). Both are deceptively simple in their happy path and dangerous in their failure modes, and both share a common root hazard: they are auxiliary systems sitting beside the source of truth, and agents routinely treat them as more authoritative than they are. A cache becomes an accidental source of truth; a lock used for correctness is trusted to guarantee mutual exclusion it cannot provide; a coordination service is used to gate an operation whose real safety depends on something the service does not control. The judgment problem is deciding, for each cache, lock, and coordination use, what guarantee it actually provides, what happens when it is stale, evicted, expired, or split-brained, and whether the downstream system is robust to that — because it will happen.

Agents tend to under-invest here because the tutorial path is clean: read from the cache, fall back to the database on miss, write through on update; acquire a lock with a TTL, do the work, release it. The harm appears only under failure — a cache invalidation that races with a read and serves stale data to a correctness-critical decision, a stampede that overwhelms the database when a hot key expires, a GC pause that outlasts a lock lease and lets two clients corrupt a shared resource, a cache that silently becomes the only copy of data nobody backed up, or a "distributed lock" that guards money movement with no fencing and corrupts it under pause. For consistency-model theory see distributed-consistency-and-cap; for the transaction and outbox patterns see distributed-transactions; for consensus internals see consensus-and-raft. This skill is the caching and coordination patterns themselves and their failure modes.

## Core Rules

### Decide Whether The Cache Is A Performance Optimization Or A Source Of Truth — And Protect The Source

The most consequential caching decision is whether the cache is a pure performance optimization (the database remains the source of truth and the cache is disposable) or whether the cache has quietly become a source of truth (data lives only in the cache, or the cache is the only thing backing user-facing reads). The failure mode of the second case is catastrophic and silent: a cache flush, an eviction policy, a memory limit, or a rolling restart destroys data that exists nowhere else, or makes the system unavailable because there is no fallback.

Be explicit about which case you are in. If the cache is an optimization, design so that losing it degrades performance but not correctness — the database must always be able to reconstruct any cached value, and the system must function (slowly) with the cache empty. If the cache has become a source of truth, you must treat it as a database: persistence (AOF/RDB for Redis), replication, backups, durability tuning, and an explicit recovery story for data loss. The common and dangerous middle ground is a cache that *de facto* holds state nobody else holds (session data, rate-limit counters, dedup sets, job queues) while being operated as if it were disposable. Name every piece of state in the cache as either "reconstructable from the source of truth" or "must survive cache loss," and operate the second category as durable storage.

### Choose The Cache Update Strategy For The Read And Write Pattern

Cache update strategy is not a style preference; it determines the staleness window, the write cost, and the consistency anomalies the system exhibits. Know the options and their tradeoffs:

- **Cache-aside (lazy loading).** The application reads the source, and on a miss populates the cache; writes update the source and invalidate (or update) the cache. Simple, resilient to cache failure, but every miss pays the source cost and there is a staleness window between source write and cache invalidation. Good default for read-heavy workloads where staleness is tolerable.
- **Read-through.** The cache library fetches from the source transparently on a miss. Same staleness semantics as cache-aside, but the application is unaware of the source — cleaner code, but couples the cache layer to the source-fetch logic.
- **Write-through.** Writes go to the cache and the source synchronously. The cache is always consistent with the source (no invalidation race on the write path), at the cost of write latency (every write pays the source round trip). Good when reads must be fresh and writes are not latency-critical.
- **Write-behind (write-back).** Writes go to the cache and are persisted to the source asynchronously. Very fast writes, but the cache is now ahead of the source — a cache crash before flush loses committed writes, and the cache is effectively a source of truth for a window. Use only where write loss is tolerable or the cache is made durable.
- **Refresh-ahead.** The cache proactively refreshes popular entries before they expire. Eliminates the staleness cliff at expiry for hot keys but adds complexity and background load.

The weak choice is to use cache-aside everywhere because it is the default, then be surprised by invalidation races and stampedes. The strong choice names the staleness window each read can tolerate and picks the strategy that bounds it — and explicitly handles the write-behind data-loss window if that strategy is chosen.

### Solve Cache Invalidation Races And Stampede At The Same Time

Two failure modes dominate cache-aside and read-through designs, and they are linked: the **invalidation race** (a read fetches a stale value in the window between a source write and the cache update, or between a cache invalidation and the repopulation) and the **thundering herd / cache stampede** (a hot key expires or is invalidated, and dozens of concurrent requests all miss and all hit the source simultaneously to rebuild it). Both must be designed for, not discovered in production.

For invalidation races: decide whether a stale read during the window is acceptable, and if not, use write-through (no race on the write path), versioned cache keys (write the new version and switch reads atomically), or read-through with a short lock so only one rebuild happens. For stampedes: use **request coalescing / cache lock / single-flight** so that when a key misses, one request rebuilds it and concurrent requests wait on the same lock rather than all hitting the source. Add jitter to TTLs so that many keys do not expire at the same instant, and consider refresh-ahead for hot keys. A weak design has every expired key rebuilt N times under load and a source that collapses; a strong design bounds concurrent rebuilds and degrades gracefully under a cold cache.

### Treat Distributed Locks As Correctness-Hazardous: Correctness Versus Efficiency

A distributed lock coordinates exclusive access across nodes, and the single most important judgment is whether the lock is being used for **efficiency** (avoiding redundant work, reducing load, preventing duplicate computation) or for **correctness** (preventing two clients from performing an operation that would corrupt state if both did it — money movement, double-issuance, concurrent writes to a shared mutable resource). The distinction changes everything, because the lock implementations that are acceptable for efficiency are unsafe for correctness.

For efficiency locks, a rare violation (two clients briefly holding the lock) is tolerable — it wastes some work but does not corrupt anything. Redis-based locks (`SET NX PX`, or Redlock across nodes) are acceptable here. For correctness locks, a single violation corrupts the invariant, and you must use a mechanism that makes violation impossible or detectable: a consensus-backed lease (etcd, ZooKeeper) **plus fencing tokens** checked at the protected resource. The fencing token is the critical piece — without it, even a consensus lease cannot prevent the stale-holder problem (a client paused longer than its lease, which then expires and is acquired by another, so two clients believe they hold it). Every lock acquisition returns a monotonically increasing token, and the protected resource rejects operations carrying a stale token. This requires the protected resource to check the token, which works when you control the resource (a database row with a version column) and fails when you do not (an external API with no token concept). Match the lock to the stakes: efficiency locks can use Redis; correctness locks need consensus plus fencing, or should be restructured away entirely (make the operation idempotent, use a database constraint, use optimistic compare-and-set). For more on the fencing and lock mechanics, see distributed-transactions.

### Know What A Coordination Service Can And Cannot Guarantee

Coordination services (etcd, Consul, ZooKeeper) are consensus-based systems used for leader election, distributed locks, service discovery, configuration distribution, and membership tracking. They are robust and battle-tested, but agents routinely misuse them by trusting them to guarantee something they do not. A coordination service guarantees agreement among the nodes that participate in it — it does **not** guarantee anything about the external systems you gate with it. Electing a leader in etcd does not prevent the old leader from acting; it only means etcd agrees on who the new leader is. The old leader can still be paused, partitioned, or mid-operation when it is deposed, and will act on its stale belief unless you add fencing.

The strong patterns: use the coordination service as the source of agreement, but enforce the agreement at the point of action with fencing tokens, leases with bounded expiry, and resources that reject stale tokens. Use it for service discovery with the understanding that discovery data is eventually consistent with reality — a registered service may be down, and a health-checked service may pass a check and fail a millisecond later; consumers must retry and handle transient unavailability rather than trusting the registry as live truth. Use it for configuration with awareness that config changes propagate with a delay and that a consumer may act on old config during the window. A weak design treats the coordination service as an oracle that makes the distributed system correct; a strong design treats it as an agreement layer whose guarantees must be enforced at every external boundary. For the consensus internals and split-brain prevention, see consensus-and-raft.

## Common Traps

### Letting The Cache Quietly Become The Source Of Truth

Storing session data, rate-limit counters, dedup sets, or job queues in a cache operated as disposable (no persistence, no backup, eviction enabled), then losing data on a flush or restart. If the cache holds state that exists nowhere else, it is a database — operate it as one with persistence, replication, and backups, or move the state to a real store.

### Ignoring The Invalidation Race On A Correctness-Critical Read

Reading from a cache that is updated cache-aside, in the window between a source write and the cache invalidation, and acting on stale data for a decision that required freshness (e.g., reading a cached balance before a withdrawal). Use write-through, versioned keys, or accept and bound the staleness — do not assume the cache reflects the latest write.

### Suffering Cache Stampede Because Every Miss Rebuilds Independently

Letting dozens of concurrent requests all miss the same expired key and all hit the source to rebuild it, collapsing the database under a cold-cache or eviction storm. Use request coalescing / single-flight, jitter TTLs, and refresh-ahead for hot keys so concurrent misses share one rebuild.

### Using A Redis Lock With TTL For A Correctness Invariant And No Fencing

Relying on `SETNX` with an expiry to prevent double-spend or double-issuance, with no fencing token checked at the protected resource. A GC pause or network delay longer than the TTL lets two clients hold the lock and corrupt the invariant. Use consensus-backed leases with fencing for correctness, or restructure to avoid the lock.

### Trusting The Coordination Service As An Oracle For Exclusivity, Discovery, Or Config

Believing that because etcd elected a leader or granted a lock the old holder cannot act, that a service registered in discovery is live, or that a config value read from the coordination service is current. A coordination service guarantees agreement among its participants, not about the external world: without fencing tokens enforced at the protected resource a paused or partitioned node acts on stale belief, and discovery and config are eventually consistent with reality. Enforce the agreement at the point of action, and make consumers retry, handle transient unavailability, and tolerate acting on slightly-stale state rather than treating the registry as an oracle.

## Self-Check

- [ ] Every piece of state held in a cache is classified as either "reconstructable from the source of truth" (cache is disposable) or "must survive cache loss" (cache is a source of truth and is operated with persistence, replication, and backups accordingly).
- [ ] The cache update strategy (cache-aside, read-through, write-through, write-behind, refresh-ahead) was chosen per workload based on the staleness window each read tolerates and the write latency cost, not left at a default.
- [ ] Cache invalidation races are handled for correctness-critical reads (write-through, versioned keys, or accepted-and-bounded staleness), and stampedes are mitigated (request coalescing/single-flight, TTL jitter, refresh-ahead for hot keys).
- [ ] Every distributed lock is classified as efficiency or correctness: efficiency locks may use Redis; correctness locks use consensus-backed leases plus fencing tokens checked at the protected resource, or are restructured away (idempotency, database constraint, optimistic compare-and-set).
- [ ] No correctness invariant relies on lock expiry timing alone — fencing tokens are issued on every acquisition and the protected resource rejects stale tokens, including for resources you do not fully control (where the lock is recognized as insufficient and the design is changed).
- [ ] The coordination service (etcd, Consul, ZooKeeper) is used as an agreement layer whose guarantees are enforced at every external boundary via fencing/leases, not trusted as an oracle that makes the system correct.
- [ ] Service discovery and configuration consumers tolerate eventual consistency with the real world — they retry, handle transient unavailability, and do not assume a registered service is live or a config value is current.
- [ ] The cache failure mode was traced end to end: what happens on a cold cache, a flush, an eviction storm, a cache node loss, and a split from the source — and the system degrades (slowly) rather than corrupting data or going unavailable where the cache is an optimization.
