---
name: windowing_and_watermarks.md
description: Use when the agent is designing stream processing windowing (tumbling, sliding, session windows), choosing between event time and processing time, configuring watermarks and allowed lateness, handling late and out-of-order events, defining triggers and window emission, or diagnosing incorrect aggregation results caused by time semantics. Also covers the failure mode of using processing time where event time is required, watermarks too aggressive that drop late data or too lax that delay results indefinitely, windows that never close, and the gap between how a window behaves in tests and under real out-of-order traffic.
---

# Windowing And Watermarks

Windowing is how stream processing groups unbounded streams into finite chunks for aggregation, and time semantics is the choice of which time drives that grouping. The judgment problem is that "time" in streaming is ambiguous — event time (when the event occurred at the source), processing time (when the system processes it), and ingestion time (when the system received it) are different, and they diverge under delay, reordering, and backpressure. A window defined on processing time produces results that shift when the system is slow or replayed; a window defined on event time produces deterministic, replayable results but requires a mechanism (the watermark) to decide when the window can close. The watermark is the system's estimate of how complete the event-time stream is — a claim that "we believe no events with event time earlier than X will arrive" — and setting it too aggressively drops late events (declaring the window closed before stragglers arrive), while setting it too lax delays results indefinitely (waiting forever for events that will never come). The discipline is to choose event time when correctness and replayability matter, to set the watermark and allowed lateness based on the actual distribution of delay in the source, to handle late events deliberately (drop, side-output, or update), and to test under realistic out-of-order traffic rather than the ordered data of a dev environment.

Agents tend to default to processing time (because it is simpler and needs no watermark) and to set watermarks by guess. The harm appears as results that change when the system is replayed or slowed (processing time is not deterministic), as late events silently dropped because the watermark declared the window closed, as windows that never close because the watermark is too lax, and as aggregations that pass in testing and miss real out-of-order events in production. The judgment is to use event time for correctness-sensitive aggregations, to measure the source's delay distribution and set the watermark and lateness from data, to make late-event handling explicit, and to choose the window type (tumbling, sliding, session) for the question being asked. Windowing is a time-semantics problem, and the wrong time produces results that are precisely wrong.

## Core Rules

### Choose Event Time For Correctness And Replayability, Processing Time For Simplicity

The time that drives windowing determines what the results mean. Event time produces deterministic, replayable results that reflect when events occurred; processing time produces results that reflect when the system happened to process them, which shifts under load, delay, and replay.

- **Use event time when correctness and replayability matter.** Aggregations by event time are deterministic: replaying the same events produces the same results, and the results reflect the real-world timing of events (a per-minute count reflects the minute the events occurred, not the minute they were processed).
- **Use processing time for latency-sensitive, approximate, or freshness-driven work** where reflecting "what the system sees right now" matters more than deterministic correctness, and where the cost of event-time machinery (watermarks, state) is not justified.
- **Never use processing time silently for correctness-sensitive work.** A daily revenue total computed by processing time shifts if the pipeline is slow or replayed; event time keeps it correct.
- **Be explicit about which time a window uses.** Mixing time semantics within a pipeline produces confusing, irreproducible results; document and standardize.

### Set The Watermark And Allowed Lateness From The Source's Delay Distribution

The watermark is the system's estimate of event-time completeness — a claim that events earlier than the watermark will not arrive. Setting it requires knowing how late events can actually be in the source, which is a property of the data, not a guess.

- **Measure the source's delay distribution.** How late do events arrive, in practice, relative to their event time? The watermark and allowed lateness should be set from this distribution, covering the vast majority of late events while bounding the wait.
- **Set the watermark to bound the wait.** An aggressive watermark (small allowed lateness) closes windows quickly but drops events later than the bound; a lax watermark (large allowed lateness) captures late events but delays results. Choose the trade based on the latency-vs-completeness need.
- **Make late-event handling explicit.** Events that arrive after the watermark closed the window must be handled deliberately: dropped (with a metric), side-output for separate processing, or used to update the prior result (if the downstream can accept updates).
- **Re-measure as the source evolves.** Delay distributions change (a new source, a network change, a backpressure pattern); the watermark that was right at launch drifts.

### Choose The Window Type For The Question Being Asked

Different window types answer different questions, and the choice should follow the question. Tumbling windows (non-overlapping fixed periods) for distinct-period aggregations; sliding windows (overlapping fixed periods) for moving averages and rolling metrics; session windows (gaps of inactivity) for user-session analysis.

- **Tumbling windows for distinct-period counts and sums** (per-minute, per-hour, per-day aggregations where each event belongs to one period).
- **Sliding windows for rolling/moving aggregations** (a 5-minute average computed every 1 minute), where events belong to multiple overlapping windows.
- **Session windows for activity-based grouping** (a user session defined by a gap of inactivity), where window boundaries depend on the data, not a fixed period.
- **Match the window to the question, not to convenience.** A per-minute tumbling window answers "how many per minute"; a 5-minute sliding window answers "what is the rolling 5-minute average"; conflating them produces the wrong number.

### Define Triggers That Control When A Window Emits

A trigger decides when a window emits its result. The default is to emit when the watermark closes the window, but triggers can emit earlier (on a processing-time timer for early, approximate results) or on other conditions (a count, a data-driven condition).

- **Emit on window close for final, complete results.** This is the default and is correct when the downstream wants the complete aggregation.
- **Use early triggers for approximate, low-latency results.** Emitting periodically before the window closes gives a running approximation, useful when freshness matters more than completeness; the final emission corrects it.
- **Make the trigger's semantics explicit.** A downstream that does not know whether a result is early/approximate or final will treat them inconsistently; communicate the trigger's meaning.

### Handle Late And Out-Of-Order Events Deliberately

Real streams are out of order and late; the system must handle events that arrive after their event-time window would have closed. The handling is a design decision, not a default.

- **Decide per pipeline whether late events update, side-output, or drop.** Updating requires a downstream that accepts corrections; side-output preserves the late data for separate handling; dropping loses it (and should be measured).
- **Measure late-event volume.** A rising late-event rate signals that the watermark is too aggressive or that the source's delay has grown; a metric on late events makes this visible.
- **Test with out-of-order data.** Ordered test data hides late-event bugs; generate realistic out-of-order and late traffic to verify handling.

## Common Traps

### Processing Time Where Event Time Is Required

Using processing time for correctness-sensitive aggregations, producing results that shift under load, delay, or replay. Use event time when correctness and replayability matter; be explicit about which time a window uses.

### Watermark Set By Guess, Not By Measured Delay

A watermark set without measuring the source's delay distribution, either dropping late events (too aggressive) or delaying results indefinitely (too lax). Measure the delay distribution and set the watermark and allowed lateness from data; re-measure as the source evolves.

### Late Events Silently Dropped

Late events arriving after the window closed, dropped without a metric, so the loss is invisible. Make late-event handling explicit (update, side-output, or measured drop) and track late-event volume.

### Windows That Never Close

A watermark too lax or a missing close condition, so windows accumulate state and never emit, growing unbounded. Bound the wait with allowed lateness and verify windows close in practice.

### Wrong Window Type For The Question

A tumbling window where a rolling/sliding aggregation was needed, or a session window where a fixed period was needed, producing the wrong number. Match the window type to the question being asked.

### Untested Out-Of-Order Behavior

Testing only with ordered data, so late-event handling and watermark behavior pass in dev and fail under real out-of-order traffic. Generate realistic out-of-order and late traffic in tests.

### Trigger Semantics Misunderstood Downstream

Early/approximate triggers whose results are treated as final by a downstream that does not know the difference, producing inconsistency. Make trigger semantics explicit and communicate them.

## Self-Check

- [ ] Event time is used for correctness-sensitive and replayable aggregations (deterministic, reflecting real-world event timing), processing time only for latency/freshness-driven approximate work where its cost is justified, and the choice is explicit and standardized within the pipeline (not silently mixed).
- [ ] The watermark and allowed lateness are set from the measured delay distribution of the source (covering the vast majority of late events while bounding the wait), the trade between latency and completeness is deliberate, late-event handling is explicit (update, side-output, or measured drop), and the setting is re-measured as the source evolves.
- [ ] The window type matches the question: tumbling for distinct-period counts/sums, sliding for rolling/moving aggregations, session for activity-based grouping — chosen for the question, not for convenience.
- [ ] Triggers control emission deliberately (on close for final results, early triggers for approximate low-latency results with a final correction), and the trigger's semantics are communicated to downstream consumers so early/approximate and final results are not conflated.
- [ ] Late and out-of-order events are handled deliberately (per-pipeline decision to update, side-output, or drop with measurement), late-event volume is tracked as a metric, and tests use realistic out-of-order and late traffic rather than ordered data.
- [ ] Windows are verified to close in practice (no unbounded state accumulation from a too-lax watermark or missing close condition).
- [ ] The highest-risk cases were verified — an event-time aggregation that replayed deterministically, a watermark tuned to the source's measured delay, late events handled rather than silently dropped, and out-of-order traffic tested — not only the clean ordered-data path.
