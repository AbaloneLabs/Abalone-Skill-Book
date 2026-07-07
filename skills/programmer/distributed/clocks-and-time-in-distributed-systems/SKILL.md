---
name: clocks_and_time_in_distributed_systems.md
description: Use when the agent is choosing a time or ordering mechanism in a distributed system — wall-clock timestamps, Lamport (logical) clocks, vector clocks, version vectors, hybrid logical clocks (HLC), or a synchronized clock assumption like Google TrueTime; reasoning about happens-before and causality versus wall-clock ordering; ordering events, logs, transactions, or last-write-wins resolution across nodes; dealing with clock skew, drift, leap seconds, and NTP/PTP synchronization error; deciding whether a system can rely on bounded clock error or needs logical ordering; or debugging anomalies caused by out-of-order events, duplicate timestamps, or non-monotonic clocks. Covers the time-and-ordering judgment problem, distinct from consistency-model theory (distributed-consistency-and-cap), consensus internals (consensus-and-raft), and failure detection.
---

# Clocks And Time In Distributed Systems

Time in a distributed system is not a fact you read; it is an approximation you reason about. Every node has its own clock, no two clocks agree exactly, and clocks drift apart continuously — by milliseconds under good NTP discipline, by seconds when synchronization breaks, and arbitrarily when a VM is live-migrated, paused, or resumed after sleep. The central judgment problem is that agents reach for wall-clock time (`now()`) as if it were a global, monotonic truth, then build correctness-critical features — ordering, deduplication, conflict resolution, cache expiry, transaction serialization, audit logs — on an assumption the system cannot honor. The result is silent data corruption: events reordered, duplicates admitted, conflicts resolved the wrong way, leases expiring early or late, and audit trails that cannot be replayed because their timestamps do not reflect causality.

The harm is invisible in the happy path and appears only under the conditions real systems hit: a leap second or NTP step that makes `now()` go backwards and breaks a monotonic assumption; a clock skew of a few hundred milliseconds that resolves a last-write-wins conflict in the wrong direction and loses an update; a paused process whose wall-clock lease appears valid to itself but has expired to everyone else; a distributed trace whose spans sort out of order because each host stamped them locally. Agents tend to under-invest here because "just use a timestamp" is the path of least resistance and works in tests. This skill is the discipline of choosing the right notion of time for the guarantee you need — physical time for coarse scheduling, logical time for causality, hybrid time when you need both, and bounded-error time only when the infrastructure truly provides it. For consistency-model theory see distributed-consistency-and-cap; for the consensus that underpins agreed ordering see consensus-and-raft; for leases and fencing see distributed-caching-and-coordination. This skill is clocks and time themselves.

## Core Rules

### Never Trust Wall-Clock Time For Correctness — Know What It Can And Cannot Guarantee

Wall-clock time (the OS `gettimeofday` / `Instant.now`) is the most convenient timestamp and the most dangerous, because it carries guarantees it does not actually provide. It is **not monotonic** (it can jump backwards on an NTP step, a leap second, a manual correction, or a VM migration); it is **not synchronized** across nodes (skew between two machines is routinely tens of milliseconds and can be seconds); and it is **not fine-grained enough** to order events that arrive within the same resolution. Any correctness decision that depends on "A happened before B because A's timestamp is smaller" is built on sand when the timestamps come from different clocks.

The strong design distinguishes what wall-clock time *is* good for — coarse human-facing timestamps, log correlation within a host, scheduling a job to run at 2 a.m., measuring elapsed duration *on a single monotonic source — from what it is not good for: ordering events across nodes, resolving concurrent updates, detecting duplicates, enforcing uniqueness, or deciding causality. When you need any of the latter, you need a logical clock or a bounded-error clock with proven guarantees. The single most valuable habit is to audit every `now()` in a correctness path and ask: "does this assume two clocks agree, or that one clock never goes backwards?" If yes, the assumption is false and the design must change.

### Use Logical Clocks (Lamport, Vector, Version Vectors) To Capture Causality

When the guarantee you need is "did event A causally happen-before event B," no physical clock can answer it — causality is not derivable from timestamps. The tool is a **logical clock**. A **Lamport clock** assigns each event a monotonically increasing counter, and the rule (on receive, set your counter to max(local, incoming) + 1) preserves happens-before: if A happened-before B, then Lamport(A) < Lamport(B). Lamport clocks are cheap (one integer, no network) and give a total order consistent with causality — but the converse is not true (A < B does not prove A happened-before B), so they cannot detect concurrent events.

When you need to *detect concurrency* (two updates that are not ordered by happens-before and therefore conflict), you need a **vector clock** or **version vector** — a map of {node → counter} per event, where comparison reveals whether one event dominates another or the two are concurrent. Vector clocks are how Dynamo-style stores (Riak, Voldemort) detect conflicting writes that must be reconciled. The cost is size (a vector entry per node) and the need to garbage-collect entries as nodes leave, which is why practical systems prune or use dotted version vectors. The judgment is: use Lamport when you need a total order and do not care about detecting concurrency; use vector/version vectors when you must distinguish "A before B" from "A and B are concurrent conflicts." Choose the mechanism against the guarantee, and never substitute a wall-clock timestamp where a logical comparison is required.

### Handle Clock Skew And Drift Explicitly When Physical Ordering Is Required

Some systems genuinely need physical ordering (a TTL that must expire, a lease that must time out, a log sorted by wall-clock) and cannot use logical clocks alone. Here the judgment is how much clock error you can tolerate and how you bound it. **NTP** keeps well-behaved clocks within tens of milliseconds but can step the clock discontinuously; **PTP (IEEE 1588)** achieves sub-millisecond accuracy on disciplined hardware but is uncommon outside specialized deployments; and most cloud VMs offer no hard clock guarantee at all.

The robust patterns: design for the worst-case skew your platform actually exhibits (measure it, do not assume it); use **hybrid physical/logical bounds** like Google Spanner's TrueTime, which returns an interval `[earliest, latest]` and waits out the uncertainty (`commit-wait`) before committing, so timestamps reflect a guaranteed ordering — but TrueTime only exists on Google's atomic-clock/GPS infrastructure and cannot be replicated by calling `now()` on a normal VM. For systems without TrueTime, treat any cross-node timestamp comparison as having an uncertainty window of at least the measured skew, and design the correctness property to tolerate it (accept that two events within the skew window cannot be reliably ordered). A weak design assumes NTP makes clocks agree; a strong design measures the agreement and designs around its absence.

### Manage Leap Seconds, Time Zones, And Non-Monotonic Sources

Even single-host time has hazards that corrupt timestamps. **Leap seconds** can make a wall clock repeat a second (or stall), breaking monotonicity and any duration computed as end-minus-start across the leap; the defenses are smear (Google's smear distributes the leap over 24 hours), or using a monotonic clock for durations and POSIX time (which ignores leap seconds) for timestamps, while documenting the discrepancy. **Time zones and DST** corrupt timestamps stored in local time or computed without a fixed offset — always store and transmit UTC, convert to local only at the presentation layer, and never store "2:30 a.m." ambiguously on a DST transition day.

**Monotonic clocks** (`clock_gettime(CLOCK_MONOTONIC)`, `Instant`) are the correct source for measuring elapsed duration on a single host because they never go backwards — but they are not comparable across hosts, not correlated with wall-clock time, and may reset across a reboot. The discipline: use monotonic sources for durations and timeouts on one host; use wall-clock UTC for human-facing and cross-host-correlated timestamps (accepting skew); use logical clocks for causality; and never compute a cross-host duration by subtracting two wall-clock timestamps. Each source has one job; conflating them is the root of most time bugs.

### Choose The Time Mechanism Against The Guarantee, And Make The Choice Explicit

The overarching rule is that "time" is not one thing — it is several distinct guarantees (causality, ordering, duration, scheduling, expiry), and each is provided by a different mechanism with different costs. The failure mode is choosing by convenience ("use a timestamp everywhere") and inheriting the wrong guarantee for each use. The strong design names, for each time-dependent decision, which guarantee it needs and which mechanism provides it, then documents the assumption so it can be audited.

For a cache TTL or a session expiry, wall-clock time plus tolerance for skew is usually fine. For ordering events in a replicated log, use the consensus log's own ordering (see consensus-and-raft), not timestamps. For detecting concurrent conflicting writes, use vector/version vectors. For a global total order consistent with real time at a single geo-region, use TrueTime if you have it, else accept bounded uncertainty. For a lease that gates correctness, use a bounded lease with fencing (see distributed-caching-and-coordination), never a bare wall-clock comparison. Making these choices explicit — in comments, in design docs, in the type of the timestamp field — is what separates a system whose time behavior is understood from one whose time behavior is an accident waiting for a clock skew to expose.

## Common Traps

### Using Wall-Clock Timestamps To Order Or Deduplicate Across Nodes

Stamping each event with `now()` and assuming the smaller timestamp happened first, when cross-node skew routinely reorders events within hundreds of milliseconds — admitting duplicates, losing updates, and producing audit logs that cannot be replayed. Use logical clocks or vector clocks for cross-node ordering and causality.

### Last-Write-Wins Resolved By Wall-Clock On Concurrent Writes

Resolving a conflict by keeping the update with the latest wall-clock timestamp, when the two writes were concurrent and the "winner" was determined by which node's clock happened to be ahead — silently discarding the other update. Detect concurrency with version vectors and resolve deliberately (merge, CRDT, or prompt the user); do not let clock skew decide.

### Assuming NTP Makes Clocks Agree (Or That `now()` Never Goes Backwards)

Treating synchronized clocks as if they were identical, or trusting monotonicity of wall-clock time, then being broken by an NTP step, a leap second, a manual correction, or a VM live-migration. Measure actual skew, design for the worst case, and use monotonic clocks for single-host durations.

### Trusting A Lease Based On Wall-Clock Comparison Without Fencing

Granting a lease that expires at wall-clock time T and letting the holder act as long as its own `now()` is before T, so a paused or slow-clock holder continues acting after others believe the lease expired. Use bounded leases with fencing tokens enforced at the protected resource, not bare timestamp comparisons.

### Conflating Clock Sources (Monotonic For Durations Mixed With Wall-Clock, Or Local Time For Storage)

Computing a cross-host duration by subtracting two wall-clock timestamps (absorbing skew into the duration), or storing timestamps in local time / without a fixed offset so a DST transition or a region change corrupts them. Use monotonic clocks for single-host durations, UTC for stored and transmitted timestamps, and convert to local only at presentation.

### Ignoring Leap Seconds And Time-Zone Edge Cases

Storing local time and being broken by a DST gap/overlap, or computing a duration across a leap second and getting a negative or doubled interval. Store and transmit UTC, use monotonic clocks for durations, and decide a leap-second policy (smear or POSIX-ignore) explicitly.

## Self-Check

- [ ] No correctness-critical decision (ordering, deduplication, conflict resolution, causality, uniqueness) relies on comparing wall-clock timestamps from different nodes; cross-node ordering uses logical or vector/version clocks, and every `now()` in a correctness path was audited for the "two clocks agree" or "clock never goes backwards" assumption.
- [ ] The logical-clock choice matches the guarantee: Lamport clocks where a total order suffices and concurrency detection is not needed; vector or version vectors (with pruning/dotted variants at scale) where concurrent conflicts must be detected and resolved.
- [ ] Where physical ordering is required (TTL, lease, log sort), the worst-case clock skew of the actual platform was measured (not assumed), and the design tolerates the uncertainty window — or a bounded-error mechanism like TrueTime with commit-wait is used where the infrastructure genuinely provides it.
- [ ] Leap seconds are handled by an explicit policy (smear or POSIX-ignore), durations on a single host use a monotonic clock, and timestamps are stored and transmitted as UTC with conversion to local time only at the presentation layer.
- [ ] Last-write-wins conflict resolution does not use wall-clock timestamps on concurrent writes — concurrency is detected with version vectors and resolved by a strategy chosen for the data's semantics (merge, CRDT, or manual), not by which clock was ahead.
- [ ] Leases and expiries that gate correctness use bounded leases with fencing tokens enforced at the protected resource, not bare wall-clock comparisons that a paused or slow-clock holder can defeat.
- [ ] Clock sources are not conflated: monotonic clocks for single-host durations and timeouts, wall-clock UTC for cross-host correlation and human-facing timestamps, logical clocks for causality, and no cross-host duration is computed by subtracting two wall-clock values.
- [ ] The time mechanism for each time-dependent feature is named explicitly (which guarantee, which mechanism, what cost) and documented, so the system's time behavior is auditable rather than an accumulation of convenient defaults.
