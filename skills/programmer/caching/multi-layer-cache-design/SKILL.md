---
name: multi_layer_cache_design.md
description: Use when the agent is designing or reasoning about a multi-tier cache architecture spanning browser, CDN, application, and database caches, choosing cache write strategies (write-through, write-back, write-around, cache-aside), deciding cache placement and consistency across layers, sizing and compressing caches, or diagnosing cross-layer inconsistency and stale-data problems. Covers the roles and tradeoffs of each cache tier, consistency models across layers, write strategies and their failure modes, cache compression and serialization, synchronization and invalidation across tiers, the cost of layered complexity, and when a single layer suffices. Also use when a system has multiple caches that disagree, when stale data appears despite invalidation, when evaluating whether to add another cache layer, or when a cache layer masks a deeper scaling problem.
---

# Multi-Layer Cache Design

A multi-layer cache is several caches stacked between the user and the source of truth, and its appeal is obvious: each layer absorbs a class of requests and reduces load on the layer behind it. Its failure mode is less obvious and more common: the layers disagree. The browser shows one value, the CDN serves another, the application cache holds a third, and the database has the truth — and because each layer was added to solve a real problem, none is clearly wrong, yet the user sees stale or inconsistent data and no single layer is responsible. Multi-layer caching trades a simple, slow, consistent system for a fast, complex, eventually-consistent one, and the complexity is the dominant cost. Teams add layers one at a time as performance band-aids and end up with a stack whose consistency properties no one fully understands, where a bug requires reasoning across four caches with different TTLs, invalidation mechanisms, and failure modes.

Agents tend to add cache layers because each layer locally improves a metric, without accounting for the cross-layer consistency and operational complexity that accumulates. The defects live in the seams: a write that updates the database and invalidates the application cache but not the CDN, so users see stale data for the CDN's TTL; a write-back strategy that acknowledges a write to the cache before persisting it, then loses the write on a crash; a compression scheme that trades CPU for network and turns out to be net-negative on the hot path; a "cache everything" strategy that stores per-user data in a shared layer and leaks it across users. The judgment problem is treating a multi-layer cache as a distributed consistency problem with a budget, where each layer is justified only if its benefit exceeds the consistency and complexity cost it adds, and where the cross-layer behavior is designed, not emergent.

This skill is about designing multi-layer caches whose layers cooperate rather than fight. It complements the invalidation skill (which covers keeping a single cache correct) and the stampede skill (which covers load spikes on cold caches); here the question is how multiple cache tiers fit together, what each is for, and how to keep them consistent without drowning in complexity.

## Core Rules

### Assign Each Layer A Clear Role, And Do Not Layer Without Justification

Each cache tier exists to solve a specific problem: the browser cache avoids re-fetching across navigations; the CDN absorbs geographic and traffic load and serves edge-close; the application cache avoids recomputation or database round-trips; the database cache buffers hot rows in memory. A layer without a clear role is complexity without benefit.

- **Name what each layer is for before adding it.** If you cannot say which requests this layer absorbs that the layer in front cannot, and which load it removes from the layer behind, the layer is not justified. "Another cache for speed" is not a role.
- **Prefer fewer layers.** Each layer adds a consistency surface, an invalidation path, a failure mode, and operational cognitive load. A single well-tuned layer often suffices; adding a second is justified only when the first cannot absorb its target load or when the layers serve genuinely different access patterns.
- **Do not use a cache layer to mask a deeper problem.** A cache that hides a slow query, an N+1, or an under-provisioned database delays the problem and makes it explode on a cache miss. Fix the source before caching; cache to add headroom, not to conceal a fault.

### Choose The Write Strategy Deliberately, Knowing Each Failure Mode

How writes interact with the cache determines consistency and durability, and the choice is rarely free. Each strategy has a characteristic failure:

- **Cache-aside (lazy loading).** The application reads the cache; on miss, reads the source and populates the cache. Simple and resilient (the cache can fail without data loss), but the cache is stale between a write and the next read-driven refresh, and the first read after a write is slow. Suitable for read-heavy data tolerant of brief staleness.
- **Write-through.** Writes update the cache and the source together (cache first, then source, or vice versa). The cache stays consistent with the source at the cost of write latency (both must complete). No data loss if the cache fails (the source is authoritative), but a write that reaches the cache and not the source (partial failure) creates inconsistency.
- **Write-back (write-behind).** Writes acknowledge from the cache and persist to the source later. Fast writes and write coalescing, but acknowledged writes can be lost if the cache fails before persistence — a durability sacrifice. Use only where the data can be reconstructed or loss is tolerable, and with explicit persistence and recovery machinery.
- **Write-around.** Writes go to the source only, bypassing the cache; reads populate the cache. Avoids cache pollution from write-once data, but the first read after a write misses.

Choose based on the read/write ratio, latency requirements, and durability needs, and know which failure each strategy exposes. Never apply write-back to data whose loss would be unacceptable.

### Design Cross-Layer Consistency Explicitly, Not As An Emergent Hope

When multiple caches sit in front of the same data, their consistency is a design property, not something that happens by default. Stale data across layers is the most common multi-layer failure.

- **Define the freshness contract.** How stale can each layer be, and what is the maximum cross-layer divergence a user can observe? If the browser can be stale for a day but the CDN for a minute, a user may see newer data after a hard refresh than after a soft one — define whether that is acceptable.
- **Invalidation must reach every layer that holds the data, or each layer must bound its own staleness.** Either propagate invalidations through the stack (write updates DB, invalidates app cache, purges CDN, sets short browser TTL), or ensure each layer's TTL keeps it fresh enough for the contract. Mixing long TTLs with explicit invalidation that does not reach every layer is how stale data persists.
- **Beware the layer that cannot be invalidated.** Browser caches are hard to invalidate from the server (you cannot force a user's browser to drop an entry); design around this with cache-busting (versioned URLs, cache-control on the response) rather than relying on purge. CDNs usually support purge but with propagation delay.
- **Order writes to avoid torn states.** When updating multiple layers, decide the order (source first, then cache, or cache first) and handle partial failure. Updating the cache then failing the source write leaves the cache ahead of the truth; updating the source then failing the cache invalidation leaves the cache stale until TTL.

### Bound Staleness With TTLs As A Safety Net, Not The Primary Mechanism

TTLs are the floor of freshness: even without explicit invalidation, data cannot be older than its TTL. But relying on TTL alone means every entry is stale up to its TTL, which is usually unacceptable for the hot, frequently-updated data.

- **Use TTL as the safety net beneath explicit invalidation.** Invalidate on write for freshness; set a TTL as a backstop in case invalidation is missed or fails. The TTL should be longer than the invalidation propagation delay but short enough to bound worst-case staleness.
- **Do not use a long TTL to avoid implementing invalidation.** A long TTL is cheap to implement but means data is stale for that long on every update, which users notice. Implement invalidation for data that changes; reserve long TTLs for genuinely immutable or slowly-changing data.
- **Vary TTL by data volatility.** Not all data needs the same freshness. Immutable assets (versioned JS/CSS) can have long TTLs; user data needs short TTLs or invalidation; reference data may sit between. Tune per data class, not globally.

### Size, Compress, And Serialize Each Layer For Its Constraints

Each tier has different constraints — the browser has limited storage and no CPU to spare, the CDN has memory and network but charges for both, the application cache has memory and serialization cost, the database cache has memory and row-format constraints. A scheme that is right for one layer is wrong for another.

- **Size each layer to its working set.** A cache too small for the hot working set thrashes and provides little benefit; a cache far larger than the working set wastes resources. Estimate the working set (distinct hot keys times entry size) and size to hold it with headroom.
- **Compress where the tradeoff favors it.** Compression trades CPU for memory/network. On the hot path, compression CPU may cost more than the bytes saved; on cold or large entries, or over the network, it often wins. Measure before assuming compression helps.
- **Choose serialization for the layer's access pattern.** A format that is fast to deserialize matters in the application cache; a compact format matters over the network; a format the browser can use directly (no parse) matters client-side. Do not assume one serialization fits all layers.

### Handle Failure Of Each Layer Without Breaking The System

A cache layer will fail — OOM, network partition, eviction storm, deployment. The system must degrade gracefully, not fail.

- **Each layer must be optional.** If a cache layer fails, the system must continue correctly (if slower) by reading through to the layer behind. Never make the cache the source of truth without persistence (except in deliberate write-back with recovery).
- **Beware cascading failure on loss of a layer.** If the CDN fails, all traffic hits the application; if the application cache fails, all traffic hits the database. Each layer's failure multiplies load on the layer behind, which can cause that layer to fail in turn. Provision the backend to survive the loss of the cache layer in front, or accept the risk explicitly.
- **Plan for cold cache.** After a deploy, a flush, or a failure, the cache is empty and every request misses (see the stampede skill). Multi-layer systems make this worse, because all layers may be cold simultaneously. Warm critical layers or protect against the cold-cache spike.

## Common Traps

### Adding Layers Without Justification

Stacking browser, CDN, app, and DB caches because more caches sound faster, when one well-tuned layer would suffice and the stack's consistency is now unmanageable. Name each layer's role; prefer fewer layers.

### Cross-Layer Inconsistency From Incomplete Invalidation

A write that invalidates the application cache but not the CDN, so users see stale data for the CDN's TTL while the application cache is fresh. Propagate invalidation through every layer that holds the data, or bound each layer's TTL to the freshness contract.

### Write-Back Losing Acknowledged Writes On Failure

Using write-back for data whose loss is unacceptable, then losing acknowledged writes when the cache fails before persisting. Reserve write-back for reconstructable or loss-tolerant data; use write-through or cache-aside for data that must not be lost.

### Caching To Mask A Deeper Problem

Adding a cache to hide a slow query, an N+1, or an under-provisioned database. The cache delays the problem and makes it catastrophic on a miss. Fix the source; cache for headroom, not concealment.

### Long TTLs Replacing Invalidation

Setting long TTLs to avoid implementing invalidation, so every update leaves data stale for the TTL. Implement invalidation for changing data; use TTL as a safety net, not the primary freshness mechanism.

### A Layer That Cannot Be Invalidated Breaking Freshness

Relying on server-driven purge of a browser cache, which is impossible, so stale data persists until the TTL expires. Use cache-busting (versioned URLs) and cache-control headers for the browser; reserve purge for CDN and app caches.

### Compression That Is Net-Negative On The Hot Path

Compressing every cache entry by default, adding CPU cost that exceeds the memory/network savings on hot entries. Measure the CPU/byte tradeoff; compress selectively where it wins.

### Cold-Cache Cascade After Deploy Or Failure

All layers empty simultaneously after a deploy or failure, so every request misses through every layer and the backend is overwhelmed. Warm critical layers, stagger, or protect against the cold-cache spike (see the stampede skill).

### Cascading Failure When A Layer Is Lost

A cache layer failing and multiplying load on the layer behind until it too fails. Provision the backend to survive the loss of the cache in front, or accept the risk explicitly; never assume the cache cannot fail.

## Self-Check

- [ ] Each cache layer (browser, CDN, application, database) has a named role describing which requests it absorbs that the layer in front cannot and which load it removes from the layer behind; layers are added only when justified, and fewer layers are preferred.
- [ ] The write strategy (cache-aside, write-through, write-back, write-around) is chosen deliberately per data class based on read/write ratio, latency, and durability needs, and write-back is used only for data whose loss is tolerable or reconstructable.
- [ ] A cross-layer freshness contract is defined (maximum acceptable staleness and cross-layer divergence), and invalidation reaches every layer that holds the data, or each layer's TTL bounds its staleness within the contract.
- [ ] TTLs are used as a safety net beneath explicit invalidation (not as a replacement for it), varied by data volatility, and long TTLs are reserved for genuinely immutable or slowly-changing data.
- [ ] Each layer is sized to its working set with headroom, compression is applied only where the CPU/byte tradeoff is measured to win (not by default on the hot path), and serialization is chosen for each layer's access pattern.
- [ ] Every cache layer is optional (the system degrades correctly, if slower, on its loss), the backend is provisioned to survive the loss of the cache layer in front (or the risk is accepted explicitly), and cold-cache scenarios after deploy/failure are planned for.
- [ ] The cache is not masking a deeper problem (slow query, N+1, under-provisioning); the source is fixed and the cache adds headroom rather than concealing a fault.
- [ ] The added complexity of the multi-layer stack is justified by measured benefit, and the team can reason about the cross-layer consistency behavior of the whole stack, not just each layer in isolation.
