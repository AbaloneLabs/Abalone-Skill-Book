---
name: cache_eviction_and_memory_management.md
description: Use when the agent is choosing a cache eviction policy (LRU, LFU, TTL, FIFO, ARC, W-TinyLFU), sizing a cache's memory budget, estimating entry sizes, preventing cache OOM or unbounded growth, deciding between TTL expiry and policy-based eviction, compressing cache contents, or diagnosing cache thrashing and low hit rate from poor eviction. Covers the tradeoffs among eviction policies and when each fits an access pattern, memory budgeting and admission control, size estimation and skew, the difference between time-based expiry and capacity-based eviction, compression and serialization cost, OOM and memory-pressure handling, and the interaction between eviction and hit rate. Also use when a cache grows without bound, when hit rate collapses as the cache fills, when a working set exceeds memory, or when choosing between a size-bounded and TTL-bounded cache.
---

# Cache Eviction And Memory Management

A cache is a bounded amount of memory holding a subset of a larger dataset, and the central question of its design is what stays and what leaves when the memory is full. This is the eviction policy, and it is the dominant determinant of hit rate after the working set size. The recurring failure is choosing the eviction policy by familiarity (LRU by default) without considering the access pattern, or sizing the cache by guess without estimating the working set, so the cache thrashes — evicting entries that will be needed next to make room for entries that will not — and provides far less benefit than its memory budget could deliver. A cache that is too small for its working set, or that evicts on the wrong signal, is not a cache; it is an expensive, miss-prone lookup table in front of the source of truth.

Agents tend to treat eviction as a configuration knob ("set LRU, give it some memory") rather than a design decision tied to the access pattern and the working set. The defects live in the mismatch: LRU on a scan-heavy workload that evicts the hot set to absorb a one-time scan; LFU on a workload where popularity shifts, so yesterday's hot entries never leave; a TTL-only cache with no capacity bound that grows without limit on a long-tailed key distribution; a memory budget set without estimating entry sizes, so 100k entries turn out to need 10x the allocated memory and the cache OOMs or evicts constantly. The judgment problem is treating eviction and memory management as a coupled decision — policy, budget, sizing, and expiry working together — driven by the actual access pattern and working set, not by defaults.

This skill is about choosing eviction policies and managing cache memory so the cache achieves a high hit rate within its budget. It complements the invalidation skill (correctness of cached data) and the multi-layer skill (how tiers combine); here the question is what the cache keeps and what it discards when memory is constrained.

## Core Rules

### Match The Eviction Policy To The Access Pattern

Eviction policies differ in what they assume about future access, and the right choice depends on the workload. There is no universally best policy; there is the policy that fits the access pattern.

- **LRU (Least Recently Used)** evicts the entry not accessed for the longest time. It assumes recency predicts future access (temporal locality), which holds for many workloads. Weak on scans (a full-table scan evicts the hot set as it passes through) and on entries accessed rarely but repeatedly over a long horizon. The common default; often good, but not always.
- **LFU (Least Frequently Used)** evicts the entry with the lowest access count. It assumes frequency predicts future access, which holds for stable popularity distributions. Weak when popularity shifts (yesterday's hot entries never leave) and on new entries (which start with low count and get evicted before they can prove popular).
- **TTL-based** evicts entries at a fixed time after insertion or last update, regardless of access. It bounds staleness rather than responding to access pattern; useful when freshness is the constraint, but it does not adapt to which entries are hot. Pair with a capacity policy, or accept that a TTL-only cache either wastes memory (long TTL, low access) or thrashes (short TTL, high access).
- **FIFO** evicts in insertion order, ignoring access entirely. Cheap but rarely competitive; useful only when access pattern is uniform or the cache is a short buffer.
- **Adaptive / frequency-recency hybrids (ARC, W-TinyLFU, LFRU).** These combine recency and frequency to adapt to workloads with both scan resistance and popularity. W-TinyLFU in particular is designed for skewed popularity with scan resistance and is a strong choice when the access pattern is mixed or unknown.

Choose based on the access pattern: recency-dominated (LRU), frequency-dominated (LFU), freshness-dominated (TTL plus a policy), or mixed (a hybrid). When in doubt, measure hit rate under candidate policies on a trace of real access.

### Size The Cache To The Working Set, With Headroom

Hit rate is governed by the ratio of cache size to working set (the distinct entries that are repeatedly accessed). A cache smaller than the working set thrashes; a cache larger than the working set provides diminishing returns. Sizing is therefore a question about the working set, not about "how much memory feels right."

- **Estimate the working set.** How many distinct hot keys are accessed repeatedly, and how large is each? The working set size is distinct-keys times average entry size. A cache that holds the working set with headroom achieves a high hit rate; one that holds a fraction of it thrashes.
- **Size to the working set with headroom, not to the total key space.** The total number of possible keys is usually irrelevant; what matters is the hot subset that drives traffic. A cache sized to total keys wastes memory; a cache sized below the working set thrashes.
- **Handle working sets larger than affordable memory explicitly.** If the working set exceeds what you can allocate, no eviction policy will give a high hit rate — the cache will miss often regardless. In that case, accept the lower hit rate, increase memory, shard, or reconsider whether caching is the right tool.
- **Re-evaluate as the working set grows.** A cache sized correctly at launch may thrash a year later as the key space or entry sizes grow. Track hit rate over time and resize before thrashing degrades performance.

### Estimate Entry Sizes Accurately, Especially Under Skew

Cache memory is entries times entry size, and entry size is often underestimated or assumed uniform when it is skewed. A few large entries can dominate memory and distort eviction.

- **Measure actual entry sizes, do not assume.** Serialization overhead, embedded structures, and value skew make real sizes differ from estimates. A cache budgeted for "100k entries at 1KB" may hold far fewer if entries average 5KB or if a few are 100KB.
- **Account for per-entry overhead.** Each entry carries metadata (key, TTL, access metadata, pointers); in caches with many small entries, the overhead can rival the value size. A cache of 1M tiny entries may spend half its memory on metadata.
- **Handle size skew explicitly.** If a few entries are much larger than the rest, they can evict many small entries and distort the policy. Consider size-aware admission (do not admit entries that would evict disproportionate value) or separate caches for large and small entries.
- **Size by bytes, not by entry count, when sizes vary.** A count-bounded cache with variable entry sizes is effectively unbounded in memory; bound by bytes and measure sizes accurately.

### Decide Deliberately Between TTL Expiry And Capacity Eviction

These are two different mechanisms answering two different questions, and conflating them produces a cache that neither stays fresh nor fits memory.

- **TTL answers "how stale can data be."** It is a correctness/freshness mechanism: entries expire at a bounded age regardless of access. Use TTL when freshness is the constraint (data changes and must not be served too old).
- **Capacity eviction answers "what fits in memory."** It is a memory mechanism: entries leave when memory is full, based on access pattern. Use capacity eviction when memory is the constraint (the key space exceeds memory).
- **Most production caches need both.** TTL bounds staleness; capacity eviction bounds memory. A TTL-only cache with no capacity bound grows without limit on a long-tailed key distribution; a capacity-only cache with no TTL serves arbitrarily stale data. Set both, with TTL as the freshness floor and capacity eviction as the memory ceiling.
- **Order of application matters.** Typically expired entries are removed first (they are stale regardless of capacity), then capacity eviction applies to what remains. Confirm the cache applies them in this order.

### Use Admission Control To Keep The Cache Hot

Eviction decides what leaves; admission control decides what enters. A cache that admits every requested entry fills with one-time-access entries that evict the hot set. Admission control keeps the cache hot by admitting only entries likely to be re-accessed.

- **Do not admit one-time scans.** A full-table scan that reads every key would, without admission control, evict the entire hot set. Scan-resistant policies (W-TinyLFU, ARC) or explicit admission gates (admit only on second access) keep scans from polluting the cache.
- **Admission based on a tiny frequency sketch.** Track access frequency in a small probabilistic structure (a Count-Min sketch, as in W-TinyLFU) and admit an entry only if its frequency suggests it will be re-accessed. This keeps the cache populated with genuinely hot entries.
- **Size-aware admission.** Decline to admit an entry so large it would evict many smaller hot entries, when the smaller entries' aggregate value exceeds the large entry's. This prevents large entries from distorting the cache.

Admission control is what separates a cache that achieves a high hit rate under a mixed workload from one that thrashes whenever a scan passes through.

### Handle Memory Pressure And OOM Gracefully

A cache must not bring down the process when memory is constrained. Caches that grow without bound, or that do not yield memory under pressure, cause OOM kills that take the whole system with them.

- **Bound the cache by bytes, and enforce the bound.** An unbounded cache (or one bounded only by entry count with variable sizes) will eventually consume all memory. Bound by bytes and evict to stay within the bound.
- **Respond to memory pressure, not just to the bound.** Some caches can shrink proactively when the process is under memory pressure (the JVM's SoftReference/WeakReference, or explicit pressure hooks), yielding memory before OOM. Use these where available; otherwise ensure the bound is low enough to leave headroom.
- **Prefer to miss than to OOM.** A cache that evicts aggressively under pressure (lower hit rate) is better than one that holds entries until the process is killed. Configure the cache to degrade (more misses) rather than fail (OOM).
- **Measure cache memory in production.** Track the cache's actual memory use, eviction rate, and hit rate; a cache silently growing toward OOM, or thrashing at a low hit rate, must be visible in metrics.

## Common Traps

### LRU On A Scan-Heavy Workload

Using LRU on a workload with periodic full scans, so each scan evicts the hot set to absorb one-time-access entries. Use a scan-resistant policy (W-TinyLFU, ARC) or admission control that does not admit one-time scans.

### LFU With Shifting Popularity

Using LFU on a workload where popularity changes over time, so yesterday's hot entries accumulate high counts and never leave, blocking today's hot entries. Use a recency-aware or decaying-frequency policy.

### TTL-Only Cache With No Capacity Bound

Relying on TTL alone to bound memory, so a long-tailed key distribution grows the cache without limit until OOM. Set a capacity bound in addition to TTL; TTL bounds freshness, capacity bounds memory.

### Sizing By Guess Instead Of Working Set

Allocating "some memory" without estimating the working set, so the cache is far below the working set and thrashes, or far above and wastes memory. Estimate distinct hot keys times entry size; size to the working set with headroom.

### Assuming Uniform Entry Sizes

Budgeting by entry count when entries vary widely in size, so a few large entries consume the budget or per-entry overhead dominates. Measure actual sizes; bound by bytes; account for per-entry overhead and size skew.

### Count-Bounded Cache With Variable Sizes

Bounding the cache by entry count while entries vary in size, so memory use is effectively unbounded. Bound by bytes when sizes vary.

### Admitting Every Requested Entry

Admitting every read into the cache, so one-time scans and long-tail entries fill the cache and evict the hot set. Use admission control (frequency sketch, second-access admission) to keep the cache hot.

### Cache Growth Causing OOM

An unbounded or poorly-bounded cache consuming all process memory under a long-tailed workload, triggering OOM kills. Bound by bytes, respond to memory pressure, and prefer aggressive eviction (more misses) over OOM.

## Self-Check

- [ ] The eviction policy (LRU, LFU, TTL, FIFO, or a hybrid like W-TinyLFU/ARC) is chosen to match the access pattern (recency-dominated, frequency-dominated, freshness-dominated, or mixed), not applied as a default — and where the pattern is uncertain, hit rate was measured under candidate policies on a real access trace.
- [ ] The cache is sized to the working set (distinct hot keys times average entry size) with headroom, not to the total key space or by guess; working sets larger than affordable memory are recognized and handled (accept lower hit rate, increase memory, shard, or reconsider caching).
- [ ] Actual entry sizes are measured (not assumed uniform), per-entry overhead is accounted for, size skew is handled (size-aware admission or separate caches for large/small entries), and the cache is bounded by bytes rather than by entry count when sizes vary.
- [ ] TTL (freshness) and capacity eviction (memory) are both set where both constraints apply, applied in the correct order (expiry before capacity eviction), and not conflated — TTL-only caches have a capacity bound, and capacity-only caches have a freshness mechanism where needed.
- [ ] Admission control prevents one-time scans and long-tail entries from polluting the cache (scan-resistant policy, frequency-sketch admission, or second-access gating), keeping the cache populated with genuinely hot entries.
- [ ] The cache is bounded by bytes and enforces the bound, responds to memory pressure (proactive shrinking where available), degrades to more misses rather than OOM under pressure, and its memory use, eviction rate, and hit rate are visible in production metrics.
- [ ] Hit rate is tracked over time and the cache is resized or re-tuned before working-set growth causes thrashing; a cache that thrashes at a low hit rate is recognized as failing its purpose and corrected.
