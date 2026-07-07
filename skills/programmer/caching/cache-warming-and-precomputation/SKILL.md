---
name: cache_warming_and_precomputation.md
description: Use when the agent is deciding whether and how to warm, precompute, or prefetch cache contents — preloading caches after a deploy or restart, precomputing expensive derived data (aggregates, recommendations, rankings, materialized views), refreshing caches before expiry to avoid stampedes, prefetching likely-next data, or building a cache-fill job. Covers cold-start performance, the tradeoff between precomputation cost and on-demand cost, staleness versus freshness in precomputed data, refresh-ahead timing and failure handling, and when warming is worth the complexity versus on-demand population.
---

# Cache Warming And Precomputation

A cold cache is slow. After a deploy, a restart, a failover, or a cache flush, every request that was served from cache now misses and rebuilds, and the rebuild cost — often a database query or an expensive computation — is paid by real traffic all at once. The instinctive response is to warm the cache: pre-populate it before traffic arrives, so the first request is already a hit. This is often the right move, and often the wrong one, and the difference is rarely considered. Warming that precomputes the wrong keys, at the wrong time, with stale data, or at a cost exceeding the cold-start it prevents, adds complexity and failure modes without buying the smoothness it promised. Precomputation — building derived data ahead of demand — has the same character: it trades on-demand compute for ahead-of-time compute, and the trade is worth making only when the demand pattern, the cost ratio, and the freshness requirement are understood.

Agents tend to reach for warming reflexively when a cold-start is slow, without asking whether the cold-start is the real problem, whether warming addresses it, or whether the warming itself introduces stampedes, staleness, or cost. The judgment problem is treating cache warming and precomputation as deliberate tradeoffs: precomputing is justified when the on-demand cost is high and the access pattern is predictable; it is unjustified when demand is sparse, the data is volatile, or the warming cost approaches the served cost. This skill covers the discipline of deciding when and how to warm and precompute, designing refresh-ahead that handles failure, and avoiding the stampedes, staleness, and wasted cost that naive warming introduces.

## Core Rules

### Decide Whether Warming Is Worth It Before Building It

Warming is engineering effort and operational complexity spent to avoid a cold-start cost. Whether it pays off depends on the shape of the cold-start and the access pattern. Decide deliberately, not reflexively.

- **Quantify the cold-start cost and who pays it.** Is the cold-start a few seconds of elevated latency on the first requests after deploy, or a multi-minute outage while the cache refills under load? A brief, tolerable cold-start may not warrant warming; a cold-start that causes timeout cascades or user-visible degradation does.
- **Assess the access pattern.** Warming pays off when a small, predictable set of keys receives most of the traffic (a hot working set you can enumerate). It pays off poorly when access is long-tail and unpredictable — you cannot warm what you cannot predict, and warming everything may exceed the cost of on-demand population.
- **Compare warming cost to on-demand cost.** If warming the entire keyspace costs as much as serving the cold-start traffic on-demand, warming buys nothing but complexity. Warming wins when the hot set is small relative to the whole, or when warming can be done off the request path (background, batched, cheaper).
- **Consider whether the cold-start is better solved elsewhere.** A slow cold-start may indicate a problem better fixed at the source — a slow query, a missing index, an expensive computation — rather than papered over with warming. Warming masks the symptom; fixing the source removes it.

### Warm The Right Keys, At The Right Time, From The Right Source

When warming is justified, its value depends on warming the keys that will actually be hit, at the moment before traffic arrives, from a source that is fresh and correct.

- **Warm the hot working set, not the entire keyspace.** Identify the keys most likely to be requested (top resources, active users, recent content) and warm those. Warming cold or never-accessed keys wastes resources populating data that will be evicted before use.
- **Time the warm to complete before traffic shifts.** After a deploy, warm before routing traffic to the new version; after a restart, warm before the instance accepts requests. A warm that completes after traffic arrives does not prevent the cold-start.
- **Warm from the authoritative source, with current data.** A warmed value is only as good as its source; warming from a stale snapshot or a replica with lag pre-populates the cache with data that is already wrong. Warm from the primary or a verified-fresh source.
- **Make warming observable and bounded.** A warm job that runs unbounded, fails silently, or warms the wrong keys is worse than no warming. Log what was warmed, how long it took, and whether it succeeded; bound its runtime and resource use.

### Use Refresh-Ahead To Avoid Expiry-Driven Stampedes

A related technique is refresh-ahead: proactively refresh a cache entry before its TTL expires, so that expiry does not trigger a synchronous rebuild (or a stampede of concurrent rebuilds). This smooths the load that TTL boundaries create.

- **Refresh entries before expiry, off the request path.** A background process refreshes entries whose TTL is near expiry, so the request path sees a hit, not a rebuild. This eliminates the latency spike and the stampede risk at expiry.
- **Handle refresh failure gracefully.** If the refresh fails (source unavailable, error), the entry should still expire on schedule rather than serve stale data indefinitely, unless stale-is-better-than-failure is the explicit policy (see graceful-degradation). Decide the failure policy deliberately.
- **Combine with stampede protection.** Even with refresh-ahead, an entry can expire before its refresh completes under load; pair refresh-ahead with request coalescing or locking (see the cache-stampede skill) so concurrent misses do not all rebuild.
- **Weigh refresh-ahead cost against the stampede it prevents.** Refreshing every entry ahead of expiry doubles the write load on the source for entries that may not be accessed again. Refresh-ahead is worth it for hot entries; for cold entries, let them expire and rebuild on demand.

### Precompute Derived Data When On-Demand Cost Is High And Demand Is Predictable

Precomputation builds derived data (aggregates, rankings, recommendations, materialized views) ahead of demand, trading compute-now for serve-later. It is the right choice when the derivation is expensive and the result is needed often.

- **Precompute when the derivation cost × frequency exceeds the precompute cost.** If a dashboard recomputes an aggregate on every view and the aggregate changes hourly, precompute it hourly and serve the stored result. If the aggregate is viewed once a month, compute on demand.
- **Bound and communicate the staleness of precomputed data.** Precomputed data is stale by definition — it reflects the source as of the last precompute. Make the freshness window explicit to consumers (a "as of" timestamp) so they do not treat it as live.
- **Schedule precomputation around the source's update cadence.** Precompute after the source data changes (event-driven), or on a schedule aligned with how stale the result may be (hourly, daily). Precomputing more often than the source changes wastes work; less often than consumers expect causes confusion.
- **Handle precompute failure with a fallback.** If the precompute job fails, consumers should get a defined fallback (stale data with a warning, a degraded view, an error) rather than silently stale or missing data. A precompute pipeline with no failure handling serves wrong data when it breaks.

### Avoid Warming-Induced Stampedes And Source Overload

Warming and precomputation themselves generate load on the source (database, downstream service). A warm job that rebuilds thousands of entries at once can overload the source it is meant to protect, turning a cold-start fix into a self-inflicted outage.

- **Rate-limit and batch warming.** Spread warming over time and batch source reads, so the warm load is bounded and does not saturate the source. A warm that runs as fast as possible competes with real traffic.
- **Coordinate warming across instances.** If multiple instances warm independently after a deploy, they multiply the source load and may each rebuild the same keys. Centralize warming (a single job, a leader) or deduplicate via stampede protection.
- **Prefer source-efficient warming.** Batch reads, use covering queries, read from replicas where staleness is acceptable — minimize the load warming places on the primary source.

## Common Traps

### Warming Reflexively Without Quantifying The Cold-Start

Building a warming pipeline for a cold-start that is brief or tolerable, adding complexity that buys little. Quantify the cold-start cost and access pattern before deciding to warm.

### Warming The Entire Keyspace Instead Of The Hot Set

Pre-populating every key when only a small hot set receives traffic, wasting resources and source load on entries that will be evicted unused. Warm the predictable hot working set.

### Warming From A Stale Or Lagging Source

Pre-populating the cache from a snapshot or replica with lag, so the cache starts with data that is already wrong. Warm from the authoritative, fresh source.

### Warm Completing After Traffic Arrives

Running the warm job after routing traffic to the new version, so the cold-start happens anyway and the warm competes with real traffic for source capacity. Complete warming before traffic shifts.

### Refresh-Ahead Serving Stale Data Indefinitely On Failure

Refreshing ahead of expiry but, on refresh failure, serving the stale entry past its TTL without a defined policy, silently providing wrong data. Decide the failure policy; let entries expire unless stale-is-better is explicit.

### Unbounded Or Uncoordinated Warming Overloading The Source

A warm job rebuilding thousands of entries as fast as possible, or multiple instances warming independently, saturating the source and causing the outage warming was meant to prevent. Rate-limit, batch, and coordinate warming.

### Precomputed Data Treated As Live

Consumers reading precomputed aggregates or rankings as if they were real-time, making decisions on data that is hours stale. Communicate the freshness window explicitly.

### Precompute Pipeline With No Failure Handling

A precompute job that fails silently, leaving consumers with missing or stale data and no defined fallback. Handle precompute failure with a documented fallback.

## Self-Check

- [ ] The decision to warm (or not) is based on a quantified cold-start cost, the access pattern (a predictable hot working set), and a comparison of warming cost to on-demand cost — not a reflexive response to a slow cold-start that might be better fixed at the source.
- [ ] Warming targets the hot working set (the keys most likely to be hit), not the entire keyspace; warming cold or never-accessed keys is avoided as wasted work.
- [ ] Warming completes before traffic shifts to the new version or instance, runs from the authoritative fresh source (not a stale snapshot or lagging replica), and is observable (what was warmed, duration, success) and bounded in runtime and resource use.
- [ ] Refresh-ahead, where used, refreshes entries off the request path before TTL expiry, has a defined failure policy (expire on schedule unless stale-is-better is explicit), and is paired with stampede protection (coalescing/locking) for entries that expire before refresh completes.
- [ ] Precomputation is justified by derivation-cost × frequency exceeding precompute cost, the staleness window is explicit and communicated to consumers (an "as of" timestamp), the schedule aligns with the source's update cadence, and precompute failure produces a defined fallback rather than silent stale or missing data.
- [ ] Warming and precomputation load on the source is bounded and coordinated — rate-limited, batched, reading from replicas where staleness is acceptable, and centralized or deduplicated across instances — so warming does not overload the source it protects.
- [ ] The warming/precomputation design has been reviewed against the specific failure modes: warming the wrong keys, warming at the wrong time, warming from a stale source, refresh-ahead staleness on failure, source overload, and silent precompute failure — each addressed.
- [ ] Where the cold-start or expensive derivation indicates a deeper problem (slow query, missing index, expensive computation), that problem is evaluated for a source-level fix rather than being masked indefinitely by warming.
