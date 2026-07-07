---
name: caching_and_redis_patterns.md
description: Use when the agent is designing or operating a cache or using Redis (or Memcached) in a backend service — choosing what to cache and for how long, cache-aside vs read-through vs write-through, TTL and eviction strategy, Redis data structures (strings, hashes, lists, sets, sorted sets, streams), Redis persistence (RDB vs AOF) and durability tradeoffs, Redis clustering/sentinel and single-threaded performance, distributed locks (Redlock and its limits), rate limiting, session store, pub/sub vs streams, or diagnosing cache stampede/thundering herd, stale data, memory exhaustion, OOM kills, hot keys, or using Redis as a queue incorrectly. Covers Redis as a distributed system, its operational pitfalls, and the boundary between cache (lossy, optional) and source of truth.
---

# Caching And Redis Patterns

A cache is a store that is faster than the source of truth and is allowed to be wrong (stale, missing, or evicted), and Redis is the most common cache and the most commonly misused one because it can also be a database, a queue, a lock service, and a pub/sub broker — each of which has different durability and consistency guarantees that get blurred. The central judgment error is treating Redis as a source of truth when it is actually a cache (so data is lost on eviction or restart), or treating it as a queue when it is actually a cache (so messages are lost when a consumer crashes mid-processing), or treating it as a lock when it has no consensus guarantees (so two nodes can hold the "same" lock). A cache makes a service faster and a Redis misuse makes it silently incorrect.

The judgment problem is not "how do I call SET" but "what am I allowed to lose, what is the staleness tolerance, what happens when the cache misses or evicts, and is Redis being used as the right kind of system for this job." Agents tend to cache without a TTL (memory grows forever), to cache without handling the miss (cache stampede when many requests rebuild the same value), to treat Redis as durable (no persistence, lost on restart), to use Redis lists as a reliable queue (lost messages on consumer crash), and to reach for Redlock as a bulletproof distributed lock. Each produces a service that is fast and silently wrong under exactly the conditions (restart, crash, failover, load) that matter.

## Core Rules

### Decide Whether Redis Is A Cache Or A Source Of Truth, And Behave Accordingly

The first decision for any Redis usage is whether the data in Redis is a cache (a derived copy of data that lives authoritatively elsewhere, which may be lost at any time without correctness impact) or a source of truth (the authoritative store, whose loss means data loss). This decision determines everything else: persistence configuration, eviction policy, replication, backup, and the application's recovery behavior. A cache can run with no persistence and an LRU eviction policy, because losing it just means a slow rebuild from the database. A source of truth must have persistence (AOF with fsync), replication (replica + Sentinel or Cluster), backups, and must not use an eviction policy that can drop data under memory pressure.

The dominant error is ambiguity: data treated as a cache in the code (no persistence, evictable) but as a source of truth in operations (no recovery plan when it is lost), or vice versa. Be explicit. If Redis holds the session store and a restart logs everyone out, that is a cache-grade decision that must be deliberate (or persistence must be on). If Redis holds a counter or a job queue whose loss is a correctness bug, it is a source of truth and must be configured for durability — or, better, moved to a system designed for that (a database for the counter, a real message broker for the queue). Write down, for each Redis usage, "this is a cache / this is source of truth" and configure accordingly. Never let Redis drift into being an undeclared source of truth.

### Cache With A TTL And An Eviction Policy; Never Let Memory Grow Unbounded

A cache without a TTL is a memory leak: keys accumulate forever, Redis hits its `maxmemory` limit, and either it starts refusing writes (returning OOM errors) or, if an eviction policy is set, it evicts under pressure in a way you did not choose. Every cached key should have a TTL that reflects its staleness tolerance — seconds for volatile data, hours for slowly-changing data — so that stale data expires automatically and memory is reclaimed. The TTL is also the staleness bound: a cached value is at most as old as its TTL, which lets you reason about consistency ("the cache may be up to 5 minutes stale").

Pair TTLs with an explicit `maxmemory` and an eviction policy chosen for the workload: `allkeys-lru` or `volatile-lru` for caches (evict least-recently-used; `volatile-*` only evicts keys with TTLs, protecting any non-TTL'd keys), `noeviction` for source-of-truth data (refuse writes rather than lose data, so you fail loud rather than silently dropping). The default (`noeviction`) is wrong for a cache (it OOMs instead of evicting) and wrong for source-of-truth (it should be paired with replication and persistence). Set `maxmemory` to leave headroom for the OS and Redis's own overhead (do not give Redis all the machine's memory — fork-based persistence needs memory to copy-on-write). Monitor `used_memory`, `evicted_keys`, and the fragmentation ratio; a cache whose eviction rate is high is undersized or its TTLs are too long.

### Prevent Cache Stampede On Miss; Use Locking Or Probabilistic Early Refresh

When a popular cached key expires, every concurrent request for it misses the cache and rebuilds the value simultaneously — a cache stampede (or thundering herd) that hammers the database with N identical queries for one key. For an expensive rebuild (a slow query, a downstream call), this spike can overload the database or the downstream and cause an outage exactly when the cache was supposed to protect it. The defenses are: request coalescing (only one request rebuilds the value while others wait — via a distributed lock, Redis's `SET NX`, or a library like `singleflight`), or probabilistic early expiration (each request randomly refreshes the value slightly before the TTL expires, spreading the rebuild over time).

The discipline: for any cached value whose rebuild is expensive and whose key is hot, do not let every miss rebuild independently. Use a lock-around-the-rebuild (the first request takes a short-lived lock, rebuilds, sets the cache, releases; concurrent requests either wait or serve the stale value until refreshed), or serve stale-while-revalidate (return the expired value immediately while one request rebuilds in the background — acceptable when some staleness is tolerable). For cold starts (an empty cache after a deploy), pre-warm the hottest keys or accept a controlled stampede. The stampede is the single most common way a cache that "worked in testing" causes an outage in production, because tests rarely simulate the concurrent-miss condition.

### Choose The Right Redis Data Structure; Do Not Abuse Strings For Everything

Redis is not just a key-value store of strings; it has purpose-built data structures (hashes, lists, sets, sorted sets, streams, hyperloglog, bitmaps) each with specific commands and performance characteristics, and using the right one is the difference between efficient and pathological code. A hash (`HSET`/`HGET`) stores an object's fields efficiently (one key, small memory overhead) where a string-per-field would create many keys. A sorted set (`ZADD`/`ZRANGEBYSCORE`) is a built-in leaderboard or time-indexed structure where sorting in application code would be slow. A list or stream is a sequence; a set is a membership test. Each structure has O(1) or O(log N) operations and some O(N) ones to avoid (`KEYS`, `SMEMBERS` on a huge set, `HGETALL` on a huge hash).

The dominant misuse is using strings for everything: storing a JSON blob per key and reading/writing the whole blob for every change (no field-level access, no atomic updates), or using a string key per field of an object (key explosion, memory overhead). The second misuse is using O(N) commands on large structures in production: `KEYS *` blocks the (single-threaded) server and stalls every other command — use `SCAN` instead; `SMEMBERS` on a million-member set returns a million elements — use `SSCAN` or cursor. Match the structure to the access pattern (an object with field access is a hash; a ranked list is a sorted set; a queue is a list or stream), and avoid the blocking O(N) commands on unbounded structures.

### Understand Redis Persistence, Single-Threading, And Failover Before Relying On Them

Redis's operational properties shape what it can be used for. It is (largely) single-threaded: one command runs at a time, so a slow command (`KEYS`, a big `LRANGE`, a Lua script that loops) blocks every other client. It offers two persistence mechanisms — RDB (point-in-time snapshots via fork, fast restart, possible data loss between snapshots) and AOF (append-only log of commands, more durable with `fsync` policies, slower restart, larger files) — and the choice is a durability/performance tradeoff. With neither, Redis loses all data on restart. With AOF `appendfsync always`, it loses at most the last command (at a performance cost). Replication is asynchronous by default (a replica can lag or be stale), and failover (Sentinel for HA, Cluster for sharding) involves tradeoffs in consistency and split-brain risk.

Do not rely on Redis properties you have not configured. A Redis with no persistence that you treat as durable will lose data on restart. A Redis replica you read from for "fresh" data may be seconds stale. A Sentinel failover may promote a stale replica if the master was partitioned, losing acknowledged writes. A Cluster shard split has the usual split-brain risks. For a cache, none of this matters much (rebuild on loss). For a source of truth or a lock, it matters enormously, and you must configure persistence, understand the replication lag, and reason about failover correctness. The Redis documentation is explicit about these tradeoffs; read it before relying on Redis for anything stronger than a cache.

### Use Distributed Locks Cautiously; Redlock Is Not A Consensus Protocol

Redis is often used for distributed locks (`SET key value NX PX timeout` to acquire, a Lua script or `WATCH`/`DEL` to release), and a single-instance Redis lock is a reasonable, pragmatic mutual-exclusion for low-stakes coordination (a cron leader, a rate limiter). The trap is treating Redis locks as bulletproof: a lock with a TTL can expire while the holder is still working (a pause, a GC, a slow operation), and another holder acquires it, so two nodes think they hold the lock — a correctness bug if the lock guards a non-idempotent operation. Redlock (the multi-node Redis lock algorithm) is marketed as stronger but is not a consensus protocol and has been shown (in the well-known Redlock critique) to be unsafe under clock skew and process pauses; it does not provide the guarantees of a true consensus system (ZooKeeper, etcd, Consul).

The discipline: use Redis locks for coordination where a failure (two holders) is tolerable or where the operation is idempotent, and use a fencing token (a monotonically-increasing token returned with the lock that downstream services check) to defend against the expired-lock case. For locks that must be correct (a lock guarding a financial operation, a lock that must never be held by two nodes), use a system built on consensus (etcd, ZooKeeper) rather than Redis. Never treat a Redis lock as a substitute for a database transaction or a unique constraint — if the operation must not happen twice, enforce that at the source of truth (a unique constraint, an idempotency key), not with a lock that can fail open.

### Use Pub/Sub And Streams For Their Actual Semantics; Do Not Use Lists As A Reliable Queue

Redis offers several messaging primitives, and conflating them loses messages. **Pub/Sub** (`PUBLISH`/`SUBSCRIBE`) is fire-and-forget: if no subscriber is connected, or a subscriber is slow and hits its output buffer limit, the message is dropped — there is no persistence, no replay, no consumer acknowledgment. Pub/Sub is for real-time fan-out where dropping a message is acceptable (notifications, invalidation signals), not for reliable delivery. **Lists** (`LPUSH`/`BRPOP`) can be used as a queue, but they offer at-most-once or at-least-once-with-recovery-via-work-queues semantics and no native consumer groups or replay — using a list as a reliable queue requires application-level tracking of in-flight messages, and a consumer crash mid-processing loses the message unless you use a separate "processing" list and requeue on timeout. **Streams** (`XADD`/`XREAD`/consumer groups) are Redis's real durable messaging primitive: they persist messages, support consumer groups with acknowledgment, and allow replay.

The discipline: match the primitive to the delivery guarantee. For fire-and-forget fan-out, pub/sub is fine (and fast). For a reliable queue with at-least-once delivery, consumer groups, and replay, use streams — or, more often, use a real message broker (Kafka, RabbitMQ, SQS) which is purpose-built for reliable delivery and handles the edge cases (consumer crash, redelivery, dead-lettering) that Redis streams handle only partially. Do not use a Redis list as a reliable queue for anything you cannot afford to lose; the message-lost-on-consumer-crash failure is silent and common. The full message-queue discipline is covered in the message-queue-and-asynchronous-processing skill; the Redis rule here is that Redis's messaging primitives have specific, limited semantics, and using the wrong one loses messages.

## Common Traps

### Treating Redis As A Source Of Truth Without Durability

Data held only in Redis with no persistence or replication, lost on restart or failover. Decide cache vs source-of-truth; configure persistence, replication, and backups for source-of-truth data.

### Cache Keys With No TTL

Keys accumulate forever until `maxmemory` is hit and Redis OOMs or evicts unpredictably. Set a TTL on every cached key reflecting its staleness tolerance.

### Wrong Eviction Policy For The Workload

`noeviction` on a cache (OOMs instead of evicting) or `allkeys-lru` on source-of-truth data (drops data under pressure). Match the policy to cache vs source-of-truth; set `maxmemory` with headroom.

### Cache Stampede On A Hot Key's Expiry

Every concurrent request rebuilding the same value when the key expires, hammering the database. Use request coalescing (lock/singleflight), probabilistic early refresh, or stale-while-revalidate.

### Using Strings And JSON For Everything

Storing a JSON blob per key and rewriting the whole blob per field change, or a key per field. Use hashes for objects with field access; match the data structure to the access pattern.

### O(N) Commands On Large Structures In Production

`KEYS *`, `SMEMBERS` on a huge set, `HGETALL` on a huge hash — blocking the single-threaded server. Use `SCAN`/`SSCAN`/`HSCAN`; avoid unbounded commands.

### Relying On Redis Lock Correctness

A Redis lock expiring while the holder is still working, or trusting Redlock as consensus. Use fencing tokens; use a real consensus system (etcd/ZooKeeper) for locks that must be correct; enforce uniqueness at the source of truth.

### Using Lists As A Reliable Queue

A Redis list queue losing messages when a consumer crashes mid-processing. Use streams with consumer groups for reliable delivery, or a real broker.

### Pub/Sub Used Where Reliability Is Needed

Pub/Sub dropping messages when no subscriber is connected or a subscriber is slow. Use pub/sub only where dropping is acceptable; use streams or a broker for reliable delivery.

### Ignoring Replication Lag On Replica Reads

Reading "fresh" data from an asynchronously-replicating replica that is seconds stale. Understand the replication model; do not read from replicas for strongly-consistent checks.

## Self-Check

- [ ] For each Redis usage, it is explicitly decided whether Redis is a cache (lossy, rebuilt from a source of truth) or a source of truth (durable, recoverable), and persistence, eviction, replication, and backup are configured to match — Redis is not an undeclared source of truth.
- [ ] Every cached key has a TTL reflecting its staleness tolerance, `maxmemory` is set with OS/fork headroom, and the eviction policy matches the workload (`allkeys-lru`/`volatile-lru` for caches, `noeviction` + durability for source-of-truth); eviction rate and memory are monitored.
- [ ] Hot keys whose rebuild is expensive are protected against cache stampede (request coalescing via a lock/singleflight, probabilistic early refresh, or stale-while-revalidate), and cold starts are pre-warmed or accept a controlled stampede.
- [ ] The right Redis data structure is used for each access pattern (hashes for objects with field access, sorted sets for ranked/time-indexed data, sets for membership), and O(N) commands (`KEYS`, `SMEMBERS`, `HGETALL` on large structures) are avoided in favor of cursor-based `SCAN` variants.
- [ ] Redis persistence (RDB vs AOF, fsync policy), single-threaded performance implications, asynchronous replication lag, and Sentinel/Cluster failover tradeoffs are understood and configured for the durability and consistency the usage requires.
- [ ] Distributed locks use Redis only for coordination where a failure is tolerable or the operation is idempotent, employ fencing tokens to defend against expired-lock races, and use a real consensus system (etcd/ZooKeeper) for locks that must be correct; uniqueness is enforced at the source of truth, not by a lock.
- [ ] Messaging primitives are matched to delivery guarantees: pub/sub only for fire-and-forget fan-out, streams (or a real broker) for reliable at-least-once delivery with consumer groups and replay, and Redis lists are not used as a reliable queue for data that cannot be lost.
- [ ] The cache layer was tested under the conditions that break caches — concurrent miss on a hot key, restart with an empty cache, memory pressure triggering eviction, replica failover — and no silent data loss, stampede, or correctness failure appears.
