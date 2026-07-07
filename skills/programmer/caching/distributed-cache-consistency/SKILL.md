---
name: distributed_cache_consistency.md
description: Use when the agent is designing or debugging a cache that spans multiple nodes, regions, or processes — Redis Cluster, Memcached, a cache service, replicated caches, or a cache fronting a shared database — and must reason about consistency between cache and database, stale reads across replicas, read-after-write expectations, cache invalidation ordering, write-through versus write-around versus delayed double-delete, cross-region cache coherence, session affinity and cache ownership, or the tradeoffs between strong and eventual consistency for cached data. Also use when a bug manifests as stale data, lost updates, or inconsistency that disappears on a cache flush, or when choosing a consistency model for a caching layer.
---

# Distributed Cache Consistency

A single-process cache is simple: the cache and the data it fronts live in the same memory, and invalidation is a function call. A distributed cache is a different problem entirely. The cache runs on separate nodes from the database, often replicates across regions, is read and written by many processes concurrently, and sits on the read path of requests that expect to see the effect of recent writes. Every one of those properties is a place where the cache can serve data that is stale, inconsistent, or contradictory with what the database holds and what other readers see. A user updates a profile, the write commits to the database, but the next read returns the old cached value because invalidation raced with the read. Two regions cache the same key independently and serve different values for minutes. A cache replica lags behind the primary and a read-after-write returns the pre-update state. These are not exotic failures; they are the default behavior of a distributed cache used naively, and they are among the most common sources of confusing, intermittent user-facing bugs.

Agents tend to treat a distributed cache as if it were a single-process cache — write to the database, delete the cache key, assume the next read is fresh. That mental model hides the races, the replication lag, and the ordering problems that define distributed caching. The judgment problem is recognizing that a distributed cache is a separate system with its own consistency semantics, that "cache and database agree" is a property that must be designed rather than assumed, and that the choice of consistency model (strong, read-your-writes, eventual) is a real tradeoff against latency, availability, and complexity. This skill covers the discipline of keeping a distributed cache consistent with its source of truth, across concurrency, replication, and geography.

## Core Rules

### Choose A Consistency Model Deliberately, Not By Default

A distributed cache does not come with a single consistency level; it comes with a spectrum, and the level you get is whatever the default configuration and your access pattern happen to produce. Name the requirement and design for it.

- **Eventual consistency** is the cheapest and most common: the cache converges to the database's value eventually, but reads in between may be stale. Acceptable for data that changes rarely or where stale reads are tolerable (product catalog, config). Unacceptable for data the user just changed and expects to see (their own profile, their own balance).
- **Read-your-writes (session) consistency** ensures a user sees their own writes: after a write, that user's subsequent reads reflect it, even if other users see stale data briefly. Achieved via session affinity to a cache region, version-stamped cache keys, or a read-through-the-database bypass for a short window after a write. This is the right target for most user-facing mutable data.
- **Strong consistency** (cache always reflects the committed database state) is expensive in a distributed system and often unnecessary; it typically requires synchronous invalidation or write-through with a distributed lock, sacrificing the latency and availability the cache exists to provide. Reserve it for data where any stale read is incorrect (a balance used for an authorization decision).

### Handle The Cache-Database Race In The Read-Through Pattern

The classic cache inconsistency is the race in the read-through (cache-aside) pattern: a read misses, the database is read, but before the cache is populated, a concurrent write updates the database and deletes the cache key — leaving the cache populated with the now-stale value. This race is real and common under concurrency.

- **Be aware that "read DB then fill cache" has a TOCTOU window.** Between the database read and the cache write, a concurrent writer can change the database and invalidate the cache, and your fill then overwrites the invalidation with stale data.
- **Mitigate with versioning or TTL.** Stamp cached values with a version (updated_at, version counter) and only fill the cache if the version is newer than what is there; or accept a short TTL so stale values self-correct. A pure fill-without-check is the vulnerable pattern.
- **Prefer write-through or write-behind for data where the race matters.** In write-through, the write goes to the cache and database together (often via the cache layer), eliminating the read-fill race at the cost of write latency. Choose based on whether reads or writes dominate and how stale-tolerant the data is.

### Order Invalidation And Write Correctly Across The Database And Cache

The sequence in which you update the database and the cache determines which failure leaves the cache stale. The two common orders have opposite failure modes, and neither is universally correct.

- **Update database, then delete cache (cache-aside invalidation).** The common pattern. If the delete fails, the cache is stale until TTL; if a read interleaves between update and delete, it may see stale data briefly. Robust to crashes between write and invalidate if the delete is retried.
- **Delete cache, then update database.** Avoids serving stale data to a read that interleaves after the delete, but introduces a window where a concurrent read misses, reads the old database value, and repopulates the cache with stale data after the write completes. Vulnerable to the read-fill race.
- **Delayed double-delete.** Delete the cache, update the database, then delete the cache again after a short delay (longer than a read takes). The second delete cleans up any stale repopulation from a concurrent read. A pragmatic mitigation when you cannot fully eliminate the race.
- **Make invalidation idempotent and retriable.** A failed invalidation must be retried (via a queue, a retry, or a database transaction outbox) so a transient cache failure does not leave permanent staleness. Treat cache invalidation as a critical operation, not a fire-and-forget.

### Account For Cache Replication Lag

When the cache itself is replicated (Redis replication, a cache service with read replicas), a write or invalidation to the primary is not immediately visible to replica reads. A user who writes and then reads from a replica may see the pre-write state.

- **Know whether your cache reads go to the primary or a replica.** Read replicas reduce load on the primary but introduce lag. For read-your-writes consistency, route a user's post-write reads to the primary, or use a region/session-affinity that keeps the user on the node that saw their write.
- **Bound and monitor replication lag.** If lag can be seconds or minutes, stale reads persist that long. Alert on lag; a cache replica that falls behind is silently serving stale data.
- **Do not assume replication makes the cache highly available and consistent.** Replication improves read throughput and survivability; it does not provide strong consistency without additional mechanism.

### Design Cross-Region Cache Coherence Explicitly

When the cache is deployed per-region for latency, the same key may be cached independently in multiple regions, each with its own view of the database. Cross-region coherence is not automatic and is a frequent source of "the data is different depending on where I read it" bugs.

- **Decide whether each region caches independently or shares a cache.** Independent regional caches minimize latency but diverge; a shared cache (or a cache service with global consistency) avoids divergence but adds latency.
- **For independent regional caches, propagate invalidations globally.** A write in one region must invalidate the key in all regions, via a pub/sub, an invalidation queue, or a global event. Without this, regions serve stale data until TTL.
- **Accept eventual consistency across regions and bound it with TTL.** For most globally-cached data, eventual consistency with a short TTL is the pragmatic choice; reserve global strong consistency for the rare data that requires it, and pay its latency cost knowingly.

### Version Cache Keys To Handle Schema And Ownership Changes

The cache holds data in a shape and under an ownership that can change over time. A cache key that does not encode the relevant version can serve data that is structurally wrong or belongs to a different context.

- **Include a version or schema tag in the cache key when the cached shape changes.** A deployment that changes the cached object's fields must not serve old-shaped cached values under the new code; versioning the key (`user:42:v2`) forces a clean miss after the change.
- **Include ownership/tenant boundaries in the key for protected data.** A cache key that omits tenant or actor scope can serve one tenant's data to another (see the authentication-authorization skill). The key must encode sufficient boundary.

## Common Traps

### Treating A Distributed Cache Like A Single-Process Cache

Assuming "delete the key after write" is sufficient, ignoring concurrency races, replication lag, and cross-region divergence. Design distributed cache consistency explicitly.

### The Read-Through Race (Fill Overwriting Invalidation)

A read miss reading the database and filling the cache concurrently with a writer's update-and-invalidate, leaving stale data in the cache. Mitigate with versioning, TTL, or write-through.

### Choosing Delete-Then-Update Without Handling Concurrent Reads

Deleting the cache before updating the database, allowing a concurrent read to repopulate the cache with the old database value after the write. Use delayed double-delete or version-checked fills.

### Fire-And-Forget Invalidation

Deleting the cache key without retry, so a transient cache failure leaves the cache stale until TTL. Treat invalidation as critical and retriable.

### Ignoring Cache Replication Lag

Reading from a cache replica immediately after writing to the primary and seeing the pre-write state. Route post-write reads to the primary or use session affinity; monitor lag.

### Independent Regional Caches Without Global Invalidation

Caching the same key per region without propagating invalidations, so regions diverge and serve different values until TTL. Propagate invalidations globally or accept and bound eventual consistency.

### Unversioned Cache Keys Across Schema Changes

Serving old-shaped cached values under new code after a deployment that changed the cached object, causing deserialization or logic errors. Version the cache key when the shape changes.

### Assuming Strong Consistency From A Cache Marketed As "Consistent"

Trusting that a cache provides strong consistency without verifying the mechanism, when the default is eventual. Name the required level and confirm the cache provides it.

## Self-Check

- [ ] A consistency model is chosen deliberately for each cached data type (eventual, read-your-writes/session, or strong) based on whether stale reads are tolerable, and the access pattern and configuration are designed to provide that level rather than relying on defaults.
- [ ] The read-through (cache-aside) race is addressed — version-stamped or TTL-bounded fills, write-through/write-behind for data where the race matters — so a concurrent write and read-fill cannot leave stale data in the cache.
- [ ] The invalidation order (update-then-delete, delete-then-update, or delayed double-delete) is chosen with awareness of its failure mode, and invalidation is idempotent and retried (queue, outbox, retry) so transient failures do not cause permanent staleness.
- [ ] Cache replication lag is accounted for: post-write reads route to the primary or use session/region affinity for read-your-writes, lag is bounded and monitored with alerting, and the cache is not assumed strongly consistent because it is replicated.
- [ ] Cross-region cache coherence is designed explicitly — either a shared/global cache, or independent regional caches with global invalidation propagation (pub/sub, event) — and the eventual-consistency window is bounded by TTL where strong consistency is not required.
- [ ] Cache keys are versioned (schema/version tag) so a deployment that changes the cached shape forces clean misses, and keys include tenant/actor/ownership boundaries so protected data is not served across contexts.
- [ ] The consistency design has been reviewed against the specific failure modes: read-fill race, invalidation ordering, replication lag, cross-region divergence, schema change, and ownership boundary — each addressed at the layer where it occurs.
- [ ] Where strong consistency is required for correctness (e.g., a cached value used for an authorization decision), it is enforced via synchronous invalidation, write-through, or a database read bypass, and the latency/availability cost is accepted knowingly.
