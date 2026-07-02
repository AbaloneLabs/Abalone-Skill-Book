---
name: cache_stampede_and_thundering_herd.md
description: Use when the agent is protecting a cache against stampedes and thundering-herd events, diagnosing load spikes when a cache expires or after a deploy, choosing among request coalescing, locking, probabilistic early expiration, jitter, and precomputation, or handling hot keys and replication lag under burst load. Covers the causes of cache stampedes (simultaneous expiry, cold cache after deploy, hot-key concentration), the failure cascade when many requests rebuild the same missing entry at once, mitigation patterns (single-flight locks, stale-while-revalidate, jittered TTLs, probabilistic early expiration, prewarming), hot-key detection and mitigation, the interaction with replication lag, and the tradeoff between stampede protection and latency tail. Also use when a cache miss causes a backend outage, when a scheduled expiry triggers a spike, or when a single hot key saturates a cache node.
---

# Cache Stampede And Thundering Herd

A cache stampede is what happens when the cache stops protecting the backend and, for a moment, every request goes through to the source of truth at once. The trigger is usually mundane: a popular cache entry expires, the cache is flushed after a deploy, or a hot key's TTL elapses at a moment of high traffic. The consequence is disproportionate: where the backend was serving zero requests for that key (all absorbed by the cache), it suddenly serves hundreds or thousands of concurrent rebuilds — the same expensive query or computation, run in parallel, by every request that arrived during the window the entry was missing. The backend, sized to handle the cache-protected load, is overwhelmed; latencies spike; connections exhaust; and in the worst case the backend failure causes the cache to fail too, spreading the stampede to whatever survives. A thundering herd is the same phenomenon at the request level: many waiters wake on a single event and all rush the same resource.

Agents tend to treat caching as a read-optimization and miss that the moment of cache miss is a load-amplification event. The defects live in the expiry: TTLs set to identical values so a batch of entries expires simultaneously; no request coalescing so every concurrent miss rebuilds independently; hot keys concentrated on one cache node so a single popular entry's expiry spikes that node; cold caches after deploys synchronized across the fleet so every instance misses at once. The judgment problem is treating the cache-miss path as the critical path it becomes under load, and designing the expiry and rebuild so that a miss produces one rebuild (or a bounded few), not a herd.

This skill is about preventing cache stampedes and thundering herds. It complements the invalidation skill (keeping cached data correct) and the multi-layer skill (how tiers fit together); here the question is how to keep the moment of cache miss from becoming a load spike that breaks the backend.

## Core Rules

### Understand The Three Common Stampede Triggers

Stampedes do not happen at random; they happen at predictable moments. Knowing the triggers tells you where to defend.

- **Simultaneous expiry.** When many entries share the same or similar TTL (e.g., all computed at startup, or all refreshed on the same schedule), they expire together, and the traffic that was spread across their lifetimes concentrates into the expiry window as every request rebuilds its missing entry.
- **Cold cache after deploy, flush, or failure.** A deploy that restarts instances, a manual flush, or a cache-node failure empties the cache, and the first wave of traffic finds nothing cached. Because all instances often deploy together, the whole fleet goes cold simultaneously, multiplying the effect.
- **Hot-key concentration.** A single very popular key (a homepage, a top product, a config) accounts for a disproportionate share of traffic. Its expiry, or the failure of the node holding it, turns a large fraction of total traffic into misses at once.

For each trigger, ask whether your system is exposed, and apply the matching defense.

### Coalesce Concurrent Misses Into A Single Rebuild

The core defense against a stampede is ensuring that when an entry is missing, only one request rebuilds it while the others wait for the result. Without coalescing, N concurrent misses produce N rebuilds; with it, they produce one.

- **Request coalescing / single-flight locking.** The first request to find a miss acquires a lock (or marks the key as in-flight) and rebuilds; concurrent requests for the same key wait on the lock and receive the rebuilt result. This collapses the herd to a single rebuild. Implement with a per-key lock, a future/promise shared across waiters, or a feature often called "request collapsing" or "dogpile protection" in cache libraries.
- **Handle lock holder failure.** If the lock holder crashes or times out, the waiters must not wait forever. Use a lock TTL and have waiters fall back (serve stale, rebuild themselves, or error) if the holder does not produce in time.
- **Beware cross-process and cross-node coalescing.** A per-process lock collapses the herd within one process, but concurrent requests across processes or nodes still each rebuild. For hot keys, coalescing must be shared (e.g., a distributed lock or a cache-level collapsing feature) to be effective across the fleet.

### Serve Stale While Rebuilding To Avoid Blocking On The Miss

The strongest defense is to never let a request wait for a rebuild at all: serve the stale (expired) entry immediately while one request rebuilds in the background.

- **Stale-while-revalidate (SWR).** When an entry expires, serve the stale value to all requesters and trigger a single background refresh. Users see slightly stale data instead of waiting; the backend sees one rebuild instead of a herd. This decouples user latency from backend load.
- **Requires keeping the stale entry past its TTL.** SWR needs the expired entry to remain available during the refresh window, so the cache must retain entries briefly past expiry (a "soft" TTL separate from the "hard" TTL at which the entry is discarded).
- **Accept the staleness tradeoff explicitly.** SWR serves data that is past its nominal expiry; confirm the data class tolerates this. For most read-heavy, eventually-consistent data it is ideal; for data that must be fresh (e.g., a balance after a write), use explicit invalidation instead.

### Spread Expiry With Jitter And Probabilistic Early Expiration

If entries never expire all at once, the herd never forms. Two techniques spread expiry across time:

- **Jittered TTLs.** Add random jitter to each entry's TTL (e.g., base TTL ± 10-20%) so entries computed together expire at slightly different times, spreading the rebuild load. Simple and effective for entries that share a computed-at time.
- **Probabilistic early expiration (XFetch).** Each request, near the end of the TTL, has a small probability of triggering a rebuild, increasing as the TTL approaches. This spreads rebuilds across the last portion of the TTL so that by the time the hard expiry arrives, the entry has likely already been refreshed — preventing the synchronized miss. Tunable to balance extra rebuilds against stampede protection.

Either technique converts a synchronized expiry spike into a smooth, low rate of rebuilds. Use jitter as the simple default; consider probabilistic early expiration for the hottest keys where even a small synchronized miss is costly.

### Prewarm And Stagger For Cold-Cache Scenarios

Cold caches (after deploy, flush, or failure) cannot be defended by expiry jitter alone, because every entry is missing at once regardless of TTL. Defend by controlling when the cache becomes warm.

- **Prewarm critical entries before serving traffic.** After a deploy or restart, populate the hottest entries (homepage, top products, configs) before the instance accepts production traffic, so the first requests hit a warm cache. Prewarm in the startup path or via a warmup job.
- **Stagger deploys.** Deploying the whole fleet at once synchronizes cold caches across all instances. Rolling or canary deploys keep some instances warm while others come up, so total cache capacity degrades gracefully rather than vanishing.
- **Plan for cache-node failure.** If a cache node fails, its keys redistribute to other nodes or rebuild from the backend. Provision spare capacity and use consistent hashing to limit redistribution, so a single node failure does not trigger a fleet-wide rebuild.

### Detect And Mitigate Hot Keys

A hot key concentrates so much traffic on one entry (and often one cache node) that its behavior dominates. Hot keys make every other defense harder, because a single key's expiry or node failure is a large event.

- **Detect hot keys via metrics.** Track the top keys by request rate; a long-tailed distribution where the top few keys dominate is a hot-key problem. Cache-node CPU imbalance often reveals a hot key on one shard.
- **Mitigate by replication or scattering.** Replicate a hot key across multiple cache nodes (or append a random suffix to the key and read from a random replica) so the load spreads across nodes. This sacrifices some consistency (replicas may differ briefly) for availability under load.
- **Mitigate by local caching.** A short-TTL local (per-process) cache in front of the shared cache absorbs the hottest reads without hitting the shared layer, reducing the hot-key pressure on the shared cache.
- **Address the root cause where possible.** Some hot keys are inherent (the homepage), but others are artifacts of design (all users reading one shared config that could be inlined, or a popularity skew that could be flattened).

### Account For Replication Lag In Replicated Caches

When a cache is replicated (for hot-key mitigation or read scaling), writes propagate to replicas with a lag, and that lag interacts with stampede behavior.

- **Replication lag creates a window of staleness and inconsistency.** A read may hit a replica that has not yet received a write or invalidation, serving stale data. Bound this with the replication design and accept the window explicitly.
- **A failed primary can cause a thundering herd on failover.** When a primary cache node fails and replicas promote, the new primary may be undersized or cold, triggering a rebuild spike. Plan failover capacity.
- **Do not assume replication solves stampedes.** Replication spreads reads but does not coalesce writes or rebuilds; a stampede on a rebuild still hits the source of truth. Combine replication (for read load) with coalescing (for rebuild load).

## Common Traps

### Identical TTLs Causing Synchronized Expiry

Setting the same TTL on many entries computed together, so they all expire at once and the traffic spreads across their lifetime concentrates into the expiry window. Add jitter to TTLs; use probabilistic early expiration for hot keys.

### No Request Coalescing, So Every Miss Rebuilds

Allowing every concurrent miss to rebuild the entry independently, so N concurrent misses produce N backend rebuilds. Coalesce concurrent misses with a per-key lock or single-flight mechanism (shared across nodes for hot keys).

### Cold-Cache Spike After A Fleet-Wide Deploy

Deploying all instances at once, emptying every cache simultaneously, so the first traffic wave misses through the whole fleet. Prewarm critical entries and stagger deploys so some instances stay warm.

### Blocking Every Requester On The Rebuild

Treating a cache miss as a synchronous wait, so all requesters block until the rebuild completes, spiking latency. Serve stale while revalidating to decouple user latency from rebuild time.

### Hot Key On A Single Cache Node

A dominant key living on one cache shard, so its expiry or the node's failure turns a large fraction of traffic into misses at once. Detect hot keys via metrics; replicate or scatter them across nodes.

### Lock Holder Failure Hanging Waiters

A single-flight lock whose holder crashes, leaving waiters blocked indefinitely. Use a lock TTL and a fallback so waiters are not dependent on the holder's survival.

### Assuming Replication Prevents Stampedes

Adding cache replicas to handle read load and assuming this also prevents rebuild stampedes, when rebuilds still hit the source of truth uncoalesced. Combine replication with request coalescing.

### Prewarming The Wrong Things

Prewarming every entry (wasting resources on cold data) or missing the actually-hot entries, so the cold-cache spike happens anyway. Prewarm the small set of keys that dominate traffic, identified from metrics.

## Self-Check

- [ ] The system's exposure to each stampede trigger is assessed: simultaneous expiry (TTLs checked for synchronization), cold cache after deploy/flush/failure (fleet-wide cold scenarios identified), and hot-key concentration (top keys by request rate measured).
- [ ] Concurrent misses are coalesced into a single rebuild via per-key locking or single-flight, the coalescing is shared across processes/nodes for hot keys, and lock-holder failure is handled with a TTL and fallback so waiters are not stranded.
- [ ] Stale-while-revalidate is used where the data class tolerates it, so requesters are served stale data during a refresh instead of blocking — and the stale-entry retention past TTL is implemented to support it.
- [ ] TTLs are jittered to spread synchronized expiry, and probabilistic early expiration is considered for the hottest keys where even a small synchronized miss is costly.
- [ ] Cold-cache scenarios are defended: critical entries are prewarmed before serving traffic (the small set that dominates traffic, identified from metrics), deploys are staggered so some instances stay warm, and cache-node failure capacity and redistribution are planned.
- [ ] Hot keys are detected (top keys by request rate, cache-node CPU imbalance), mitigated by replication/scattering and/or local caching, and their root cause is addressed where feasible.
- [ ] Replicated caches' replication lag is bounded and accepted explicitly, failover capacity is planned (a promoted replica does not trigger a rebuild spike it cannot handle), and replication is combined with (not substituted for) request coalescing.
- [ ] The chosen stampede protections are matched to the actual triggers present, and the latency/staleness/resource tradeoffs of each (SWR staleness, jitter extra rebuilds, replication consistency) are accepted explicitly rather than applied blindly.
