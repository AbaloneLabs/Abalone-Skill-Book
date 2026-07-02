---
name: stream_join_and_aggregation.md
description: Use when the agent is designing stream joins (inner, outer, interval, windowed), implementing stateful aggregations over unbounded streams, reasoning about event ordering and late arrivals in joins, or diagnosing incorrect join or aggregation results. Also covers the failure mode of unbounded state in stream joins, joins that miss matches due to out-of-order events, aggregations that produce wrong results under late data, time-windowed joins with mismatched semantics, and the cost of holding state for the full join window.
---

# Stream Join And Aggregation

Stream joins combine two (or more) unbounded streams based on a condition, and stateful aggregations compute running results (counts, sums, averages, top-N) over unbounded data. The judgment problem is that both require holding state across the stream, and the state's size, lifetime, and correctness depend on design choices that are easy to get wrong. A stream join must hold events from one or both sides until the potential match window passes; without a bound (a time window, a key constraint), the state grows unbounded as every event waits for a match that may never come. An aggregation over unbounded data must define a window or a triggering condition, or it never emits. And both must handle the reality that streams are out of order and late: a join that closes its window too early misses late matches, and an aggregation that emits before the window is complete produces partial results that may be wrong. The discipline is to bound join and aggregation state with time or count windows, to handle late and out-of-order events deliberately, to choose the join type (inner, outer, interval) for the question, and to size and monitor the state these operations hold.

Agents tend to write joins and aggregations as if streams were bounded and ordered. The harm appears as unbounded state growth (a join holding events forever waiting for matches), as missed matches (a window closed before a late event arrived), as wrong aggregations (a result emitted before late data arrived, never corrected), and as surprising cost (a join holding the full window of both sides in state). The judgment is to bound every join and aggregation with a window tied to the data's timing, to handle late events explicitly (update, side-output, or drop), to choose the join type and window for the question, and to monitor state size and correctness under realistic traffic. Stream joins and aggregations are stateful time-windowed operations, and ignoring either property produces results that are wrong in characteristic ways.

## Core Rules

### Bound Join And Aggregation State With A Window

State in a stream join or aggregation must be bounded, or it grows forever. The bound is typically a time window (events held until their match window or aggregation window passes) or a count window, tied to the data's timing.

- **Bound stream joins with a time window (interval join).** Hold events from each side for the duration they could match events on the other side; evict them when the window passes. An unbounded join (hold forever) grows state without limit.
- **Bound aggregations with a window or trigger.** A windowed aggregation (per minute, per session) emits and clears state when the window closes; an unbounded aggregation never emits or grows state forever unless it is a special case (a running total with bounded retention).
- **Size the window to the data, not to a guess.** A join window should reflect how far apart matching events can be in event time; an aggregation window should reflect the period of the question. Too small misses matches or produces partial results; too large holds excessive state.
- **Evict state when the window closes.** Closed-window state must be released; retaining it grows the footprint without benefit.

### Handle Late And Out-Of-Order Events Deliberately

Real streams arrive late and out of order, and joins and aggregations must handle events that arrive after their window would have closed. The handling is a design decision, tied to the watermark and allowed lateness.

- **Decide how late events affect joins and aggregations.** A late event may update a prior result (if the downstream accepts updates), be side-output for separate handling, or be dropped (with a metric). The choice depends on the correctness need.
- **Set the watermark and allowed lateness to cover realistic delay.** A window that closes before late events arrive misses them; allowed lateness should cover the source's actual delay distribution (see the windowing skill).
- **Test with out-of-order data.** Ordered test data hides late-event bugs; generate realistic out-of-order and late traffic to verify join and aggregation behavior.
- **Measure late-event impact.** A rising rate of late events affecting joins or aggregations signals that the watermark or window is too tight.

### Choose The Join Type And Window For The Question

Different join types answer different questions, and the window's semantics must match. Inner, outer, interval, and session joins each have a use case, and conflating them produces the wrong result.

- **Inner join for matches on both sides.** Emits only when both sides have a matching event within the window; unmatched events are dropped.
- **Outer join to preserve unmatched events.** Emits even when one side has no match (with nulls or defaults), useful when the absence is meaningful.
- **Interval join for time-proximity matches.** Matches events whose event times are within a bound of each other, naturally bounded by the interval; efficient for "events within N minutes of each other."
- **Windowed join for fixed-period matches.** Matches events in the same fixed window (per minute, per hour); choose when the question is about co-occurrence in a period.
- **Match the join semantics to the question.** "Did A and B happen in the same session" differs from "did A and B happen within 5 minutes"; choose the join that answers the actual question.

### Make Aggregations Correct Under Streaming Conditions

Aggregations over unbounded streams must define what "the aggregation" means: over what window, with what trigger, and how late events are handled. An aggregation without these definitions is ambiguous.

- **Define the window and trigger explicitly.** Per-minute tumbling, 5-minute sliding, session-based — the window defines what is aggregated, and the trigger defines when it emits (on close, or early for approximation).
- **Handle retractions and updates for early-emitted aggregations.** If an aggregation emits early (approximate) and then late events arrive, the downstream may need a retraction or an updated value; design for this if early emission is used.
- **Be careful with non-associative aggregations.** Average and some other aggregations are not simply composable from sub-aggregations without retaining count/sum; ensure the aggregation is correctly maintained incrementally.
- **Bound top-N and similar heavy aggregations.** A top-N aggregation holding all keys is unbounded; bound it (top-N per window, with eviction) or it grows forever.

### Size And Monitor The State These Operations Hold

Stream joins and aggregations hold state proportional to the window size and the key cardinality, and this state has a real cost (memory/disk, checkpoint time). Size and monitor it.

- **Estimate state size from window size and key cardinality.** A 1-hour join window over a million keys holds far more than a 1-minute window over a thousand; size the backend to the estimate.
- **Monitor state size and checkpoint duration.** Growing state or slowing checkpoints signal that windows are too large, key cardinality is higher than expected, or eviction is insufficient.
- **Prefer keyed state to non-keyed state.** Keyed state (state per key, redistributable) scales across parallel operators; non-keyed state (operator-level) does not and becomes a bottleneck.

## Common Traps

### Unbounded State In A Stream Join

A join holding events forever waiting for matches, growing state without limit. Bound joins with a time window (interval join) or key constraint; evict when the window passes.

### Join Missing Matches Due To Out-Of-Order Events

A join window that closes before a late event arrives, missing the match. Set the watermark and allowed lateness to cover realistic delay; handle late events deliberately.

### Aggregation Emitting Before The Window Is Complete

An aggregation that emits a partial result before late data arrives, producing a wrong number that is never corrected. Define the trigger explicitly; handle retractions/updates if emitting early.

### Wrong Join Type For The Question

An inner join where unmatched events are meaningful (should be outer), or a fixed-window join where time-proximity (interval) was meant. Match the join type and window to the question.

### Non-Associative Aggregation Maintained Wrongly

An average (or similar) maintained incrementally without retaining count/sum, producing wrong results. Maintain aggregations with the state they need for correct incremental updates.

### Unbounded Top-N Or Heavy Aggregation

A top-N or distinct-count aggregation holding all keys, growing without bound. Bound it per window with eviction.

### State Cost Unmonitored

Join or aggregation state that grows larger or checkpoints slower than expected, discovered only when it breaks. Estimate, size the backend, and monitor state size and checkpoint duration.

## Self-Check

- [ ] Join and aggregation state is bounded by a window (time or count) tied to the data's timing — joins use interval or windowed semantics with eviction when the window passes, aggregations define a window and trigger, and window sizes reflect the data rather than guesses.
- [ ] Late and out-of-order events are handled deliberately (update, side-output, or measured drop), the watermark and allowed lateness cover the source's realistic delay distribution, behavior is tested with out-of-order data, and late-event impact is measured.
- [ ] The join type and window match the question (inner for matches, outer to preserve unmatched, interval for time-proximity, windowed for fixed-period co-occurrence), and the semantics are not conflated.
- [ ] Aggregations are correct under streaming conditions: window and trigger are explicit, early-emitted aggregations handle retractions/updates, non-associative aggregations (average, etc.) maintain the state needed for correct incremental updates, and heavy aggregations (top-N, distinct-count) are bounded per window.
- [ ] State size is estimated from window size and key cardinality, the state backend is sized to the estimate, state size and checkpoint duration are monitored, and keyed state is preferred to non-keyed state for scalability.
- [ ] The highest-risk cases were verified — a join that did not grow unbounded because it was windowed, a late event that matched because lateness was allowed, an aggregation that emitted a correct result despite out-of-order data, and state cost that was monitored rather than discovered at failure — not only the clean ordered-bounded path.
