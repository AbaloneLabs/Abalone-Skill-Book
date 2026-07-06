---
name: exactly_once_and_state.md
description: Use when the agent is designing stream processing with exactly-once semantics, managing operator state and checkpoints, choosing a state backend, ensuring idempotent sinks, reasoning about transactions across source and sink, or diagnosing duplicate or lost results after a failure or recovery. Also covers the failure mode of assuming exactly-once when only at-least-once is achieved (because the sink is not idempotent), state that grows unbounded, checkpoints that fail or restore incorrectly, and the end-to-end gap between framework-level exactly-once and a pipeline that is actually exactly-once from source to sink.
---

# Exactly-Once And State

Exactly-once semantics (EOS) is the guarantee that each event affects the result exactly once, despite failures, retries, and recovery — no duplicates, no loss. The judgment problem is that exactly-once is an end-to-end property, not a framework feature, and it is easy to believe it is achieved when it is not. A stream processor may guarantee exactly-once internally (via checkpoints and deterministic processing), but if the sink applies effects non-idempotently, a recovery that replays events produces duplicates at the output; if the source cannot replay, a failure between checkpoint and commit loses events. The discipline is to make exactly-once end-to-end (source that replays, deterministic processing, idempotent or transactional sink), to checkpoint state consistently with the output position so recovery resumes without duplication or loss, to choose a state backend matched to the state's size and access pattern, and to verify the guarantee under failure injection rather than assuming the framework's claim covers the whole pipeline. Exactly-once that is not verified end-to-end is at-least-once with confidence.

Agents tend to enable the framework's exactly-once flag and assume the pipeline is correct. The harm appears as duplicate outputs after a recovery (the sink applied effects twice because it was not idempotent), as lost outputs after a failure (the source could not replay, or the checkpoint did not cover the output), as state that grows unbounded (no eviction, accumulating forever), and as checkpoints that restore to an inconsistent state. The judgment is to treat exactly-once as a contract spanning source, processor, and sink; to make the sink idempotent or transactional; to align checkpoints with output commits; to bound and manage state; and to test recovery under failure. A pipeline's correctness claim is only as strong as its weakest link, and the sink is usually the weakest link.

## Core Rules

### Make Exactly-Once End-To-End, Not Just Framework-Level

Exactly-once is a property of the path from source to sink, and every link must preserve it. The framework's internal exactly-once is necessary but not sufficient: the source must replay events from the last committed position, the processing must be deterministic, and the sink must apply effects idempotently or transactionally.

- **The source must be replayable and position-tracked.** After a failure, the source must re-read from the last committed offset; a source that cannot replay (or whose position is not checkpointed with the state) loses events on recovery.
- **Processing must be deterministic.** Non-determinism (depending on wall-clock time, encounter order, external state) means replaying the same events produces different results, defeating exactly-once. Make processing a deterministic function of the inputs.
- **The sink must be idempotent or transactional.** This is the most common failure point: an idempotent sink (writing by key so a replay overwrites identically) or a transactional sink (committing output and checkpoint atomically) ensures replays do not duplicate. A non-idempotent, non-transactional sink turns framework exactly-once into at-least-once at the output.

### Align Checkpoints With Output Commits

A checkpoint captures the state and the input position; the output must be committed consistently with the checkpoint, or recovery produces duplication or loss. If the output is committed before the checkpoint, a failure between them replays and re-commits (duplicate); if the checkpoint is taken before the output commit, a failure between them loses the output.

- **Commit output and checkpoint atomically (two-phase commit / transactional sinks)** where the sink supports it, so recovery resumes from the committed position with the committed output.
- **Use idempotent sinks where transactional commit is not available**, so that replaying from the last checkpoint re-applies effects identically rather than duplicating.
- **Never commit output ahead of the checkpoint without idempotency.** This is the classic duplication bug: output committed, then failure before checkpoint, then replay re-commits.

### Choose A State Backend Matched To State Size And Access Pattern

The state backend (in-memory, embedded RocksDB, external) determines how much state the operator can hold and how fast it accesses. The choice should match the state's characteristics, not be left at the default.

- **In-memory backends are fast but bounded by RAM.** Suitable for small state; large state spills or fails.
- **Embedded key-value backends (e.g., RocksDB) hold large state on disk with in-memory caching.** Suitable for large keyed state with random access; slower than memory but unbounded by RAM.
- **External state stores separate state from the operator.** Adds network overhead and a dependency, but enables larger or shared state; weigh the operational cost.
- **Match access patterns.** A backend optimized for key-value access is wrong for iterative scans; choose for how the state is actually accessed.

### Bound And Manage State To Prevent Unbounded Growth

State in a long-running stream grows without intervention: keyed state accumulates per key, windowed state holds in-flight windows, appends accumulate. Unbounded state eventually exhausts memory or disk and slows checkpoints. Bound it deliberately.

- **Evict state that is no longer needed.** Keyed state for stale keys, windowed state for closed windows, expired sessions — define and enforce retention so state does not grow forever.
- **Use state TTL where the framework supports it.** Time-based eviction of stale state keeps the footprint bounded for time-windowed semantics.
- **Compact or aggregate state where possible.** Storing an aggregate rather than raw events, or a bounded sample rather than the full history, keeps state manageable.
- **Monitor state size and checkpoint duration.** Growing state or slowing checkpoints signal that eviction is insufficient; track these as operational metrics.

### Verify Exactly-Once Under Failure Injection

Exactly-once is a claim about behavior under failure, and it must be tested under failure. A pipeline that produces correct results in steady state may duplicate or lose on recovery, and only failure injection reveals this.

- **Inject failures during processing and verify recovery.** Kill an operator, trigger a checkpoint failure, simulate a sink outage, and verify the output is exactly-once (no duplicates, no loss) after recovery.
- **Verify the sink's idempotency or transactionality under replay.** Send the same events twice (simulating a replay) and confirm the sink's output is unchanged.
- **Test at the boundary conditions.** Failures during checkpoint, failures during commit, failures during state restore — these are where exactly-once breaks.

### Document the Basis and the Reasoning

Every conclusion should be traceable to its evidence, assumptions, and alternatives considered. Record not only the outcome but the reasoning path: what was checked, what was ruled out, what uncertainty remains, and what would change the conclusion. Documentation that captures the basis allows another practitioner to review, reproduce, or challenge the work, and it prevents confident conclusions from becoming unrepeatable assertions. A decision made without a recorded basis cannot be audited, improved, or safely handed off.

## Common Traps

### Assuming Framework Exactly-Once Covers The Pipeline

Enabling the framework's exactly-once flag and assuming the pipeline is exactly-once, when the sink is non-idempotent and duplicates on replay. Make exactly-once end-to-end: replayable source, deterministic processing, idempotent or transactional sink.

### Non-Idempotent Sink Duplicating On Recovery

A sink that applies effects non-idempotently (append without a key, increment without deduplication), duplicating output when recovery replays events. Make the sink idempotent (write by key) or transactional (commit with checkpoint).

### Output Committed Before Checkpoint

Output committed before the checkpoint is taken, so a failure between them replays and re-commits, duplicating. Align output commit with the checkpoint (two-phase commit) or use an idempotent sink.

### Source That Cannot Replay

A source whose position is not checkpointed with the state, or that cannot re-read, so recovery loses events. Use a replayable source and checkpoint its position with the state.

### Non-Deterministic Processing

Processing that depends on wall-clock time, encounter order, or external state, so replaying the same events produces different results. Make processing a deterministic function of the inputs.

### Unbounded State Growth

Keyed or windowed state that accumulates without eviction, eventually exhausting memory or disk and slowing checkpoints. Evict stale state, use TTL, compact or aggregate, and monitor state size.

### Untested Recovery

Exactly-once assumed without failure injection, so duplication or loss on recovery is discovered in production. Inject failures during processing, at checkpoint/commit boundaries, and during state restore.

## Self-Check

- [ ] Exactly-once is treated as end-to-end: the source is replayable with its position checkpointed alongside state, processing is deterministic (no dependence on wall-clock time, encounter order, or un-snapshotted external state), and the sink is idempotent (writes by key so replay overwrites identically) or transactional (commits output and checkpoint atomically).
- [ ] Checkpoints are aligned with output commits (two-phase commit / transactional sink where supported, idempotent sink where not), so recovery resumes from the committed position without duplication or loss — output is never committed ahead of the checkpoint without idempotency.
- [ ] The state backend is matched to the state's size and access pattern (in-memory for small state, embedded KV like RocksDB for large keyed state, external stores for shared/large state with their operational cost weighed, access-pattern matched).
- [ ] State is bounded and managed: stale keyed/windowed/session state is evicted, TTL is used where supported, state is compacted or aggregated where possible, and state size and checkpoint duration are monitored as operational metrics.
- [ ] Exactly-once is verified under failure injection: failures during processing, checkpoint failures, sink outages, and failures at checkpoint/commit boundaries and during state restore are injected, and the output is confirmed exactly-once (no duplicates, no loss) after recovery.
- [ ] The sink's idempotency or transactionality is verified under replay (sending events twice produces unchanged output).
- [ ] The highest-risk cases were verified — a recovery that did not duplicate because the sink was idempotent, a checkpoint that restored consistently with the output position, a failure at the commit boundary that did not lose output, and state that stayed bounded over a long run — not only the clean steady-state path.
- Is the reasoning documented clearly enough that another practitioner could review the basis and reproduce the conclusion?
