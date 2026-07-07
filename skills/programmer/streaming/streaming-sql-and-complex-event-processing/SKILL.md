---
name: streaming_sql_and_complex_event_processing.md
description: Use when the agent is designing stream processing with SQL (streaming SQL, continuous queries) or complex event processing (CEP) for pattern detection over event streams, defining windows and time semantics in SQL, reasoning about stream-table joins and temporal joins, handling late and out-of-order events in pattern matching, or diagnosing incorrect pattern matches (false positives from too-loose patterns, missed matches from too-strict time windows, non-deterministic results from wall-clock time). Also covers the failure mode of streaming SQL hiding streaming complexity behind familiar SQL syntax (state growth, watermarks, late data) that an agent ignores because the query looks like batch SQL, patterns that match spuriously on noise, and the recurring mistake of treating a continuous query like a batch query when its results are incremental, time-sensitive, and stateful.
---

# Streaming SQL And Complex Event Processing

Streaming SQL brings the familiar SQL interface to unbounded streams, and complex event processing (CEP) detects patterns across sequences of events (e.g., "a login followed by a failed payment within 5 minutes"). The judgment problem is that the familiar interface hides streaming complexity. A streaming SQL query looks like batch SQL, but its execution is incremental, time-sensitive, and stateful: it maintains state across windows, depends on time semantics and watermarks, and produces results as events arrive rather than once at the end. An agent who treats it like batch SQL ignores the state growth, the watermark and late-data behavior, and the time semantics that determine what "within 5 minutes" means. CEP patterns match spuriously on noise if too loose, or miss matches if the time window is too strict, and wall-clock time makes results non-deterministic. The discipline is to treat streaming SQL as streaming with a SQL interface (not batch SQL that happens to run on a stream), to define windows and time semantics explicitly, to design stream-table and temporal joins for their state and correctness, and to tune CEP patterns against false positives and missed matches.

Agents tend to write streaming SQL as if it were batch SQL, because the syntax is familiar and the demo query returns results. The harm appears as unbounded state growth (a window or join with no eviction accumulates forever), as incorrect results from wall-clock time (a query using processing time produces different results on re-run), as missed pattern matches (a strict window dropped a slightly-late event), and as false-positive pattern matches (a loose pattern fired on routine noise). The judgment is to make time semantics and windows explicit, to bound state, to design joins for streaming correctness, and to validate patterns against real event sequences. A streaming SQL query that looks correct can produce wrong, late, or unbounded results because the streaming mechanics behind the SQL were not respected.

This skill covers streaming SQL time semantics and windows, stream-table and temporal joins, CEP pattern design, and state and late-data handling. It complements the windowing-and-watermarks skill (time and windows in general stream processing), the stream-join-and-aggregation skill (joins and aggregations), and the exactly-once-and-state skill (state and delivery). Here the focus is the SQL/CEP interface and the streaming complexity it hides.

## Core Rules

### Treat Streaming SQL As Streaming, Not Batch SQL With A Stream

A streaming SQL query is a continuous query: it runs indefinitely, produces incremental results, and maintains state. Treating it as batch SQL ignores the streaming mechanics:

- **Results are incremental, not final.** A streaming aggregation emits updated results as events arrive; a late event can change an already-emitted result (or be dropped, depending on semantics). Know whether your query emits retractions/updates or only appends, and design downstream consumers for the emission mode.
- **State is maintained and must be bounded.** Windows, joins, and aggregations maintain state; without eviction or time bounds, state grows unbounded. Every stateful streaming SQL construct needs a defined retention (window length, join time range, aggregation retention) or it will eventually fail.
- **Time semantics determine correctness.** "Within 5 minutes" means different things in event time (when the event occurred) vs processing time (when the stream processor saw it); using processing time makes results depend on system load and re-runs non-deterministic. Use event time with watermarks for deterministic, meaningful results (see the windowing-and-watermarks skill).

### Define Windows And Time Semantics Explicitly

Windows group unbounded streams into bounded chunks for aggregation, and the window and time semantics must be explicit and intentional:

- **Choose window type for the question.** Tumbling (non-overlapping fixed windows), hopping/sliding (overlapping fixed windows), and session (gaps of inactivity) windows answer different questions; choose the type that matches the aggregation's intent.
- **Use event time, not processing time, for meaningful results.** Event time reflects when the event occurred (robust to delay and reprocessing); processing time reflects when the system processed it (varies with load). Event time requires watermarks to signal window closure (see below).
- **Set the watermark and allowed lateness deliberately.** The watermark tells the system how long to wait for late events before closing a window; too aggressive closes windows before late events arrive (missed results), too conservative delays results and grows state. Allowed lateness defines how long a late event can update an already-emitted result. Tune both for the data's delay distribution.

### Design Stream-Table And Temporal Joins For Streaming Correctness

Joins in streaming SQL differ from batch joins, and their streaming characteristics must be designed:

- **Stream-table joins enrich with current dimension state.** Joining a stream to a table (dimension) enriches events with the current dimension value; the join is point-in-time and the dimension may change. Know whether the join uses the dimension's state at the event's time (temporal join) or its current state (enrichment), as they produce different results.
- **Temporal joins respect event-time consistency.** A temporal join uses the dimension's version as of the event's event-time, producing consistent results even if the dimension changed; this requires the dimension's history (a versioned table or changelog stream) and is more correct but more expensive.
- **Stream-stream joins need a time bound.** Joining two streams (e.g., "a login within 5 minutes of a payment") requires a time bound and window, or the join state grows unbounded waiting for a match that never comes. Define the join's time range and window.
- **Bound join state.** Every join maintains state (the build side, the pending matches); without a time bound or retention, state grows unbounded. Ensure every streaming join has a defined retention.

### Tune CEP Patterns Against False Positives And Missed Matches

Complex event processing detects patterns across event sequences, and patterns must be tuned to match the intended behavior, not fire on noise or miss real matches:

- **Define the pattern precisely.** A pattern (event A followed by event B within T) must specify the events, their conditions, the sequence (followed-by vs not-followed-by), and the time window; vague patterns match spuriously.
- **Set the time window for the real behavior.** Too strict a window misses matches where the events are slightly more separated than expected; too loose a window matches unrelated events as if correlated. Set the window from the actual temporal distribution of the pattern's events.
- **Handle optional and repeating events explicitly.** Patterns with optional (event A then optionally B) or repeating (3 failed logins) sub-patterns must define the optionality and repetition precisely, or they match unintended sequences.
- **Validate patterns against real event sequences.** Test the pattern against labeled positive and negative examples (real matches and noise) to measure false positives and missed matches before production; a pattern that looks right can fire on routine noise.

### Handle Late And Out-Of-Order Events In Patterns And Queries

Events arrive late and out of order, and streaming SQL and CEP must handle them or produce wrong results:

- **Use watermarks to define "how late is too late."** A watermark lets the system close windows and finalize results after a delay; events later than the watermark are dropped or handled as late. Set the watermark for the data's delay distribution.
- **Decide how to handle late events.** Late events can update already-emitted results (if allowed lateness permits), be dropped, or be routed to a side output; choose the handling for the use case's correctness needs.
- **Do not rely on processing time for pattern timing.** A pattern "within 5 minutes" using processing time fires based on when the system saw the events, not when they occurred; use event time so the pattern reflects real temporal relationships.

## Common Traps

### Treating Streaming SQL As Batch SQL

Writing a streaming SQL query as if it were batch, ignoring incremental results, state growth, and time semantics, producing unbounded state or wrong results. Treat it as streaming with a SQL interface.

### Wall-Clock Time Making Results Non-Deterministic

Using processing time for windows or patterns, making results depend on system load and non-reproducible. Use event time with watermarks.

### Unbounded State From Windows Or Joins Without Retention

A window, join, or aggregation with no eviction or time bound accumulating state until failure. Define retention for every stateful construct.

### Too-Strict Time Windows Missing Real Matches

A pattern or join window too strict for the data's delay distribution, dropping slightly-late events and missing real matches. Set windows from the actual temporal distribution.

### Too-Loose CEP Patterns Matching Noise

A pattern so loose it fires on routine, unrelated event sequences, producing false positives. Validate patterns against labeled positive and negative examples.

### Temporal Joins Using Current Instead Of Point-In-Time Dimension State

A stream-table join using the dimension's current state rather than its state at the event's time, producing inconsistent results when the dimension changed. Use temporal joins for event-time consistency.

## Self-Check

- [ ] Streaming SQL is treated as streaming (incremental results, bounded state, explicit time semantics), not batch SQL; the emission mode (appends vs updates/retractions) is known and downstream consumers handle it.
- [ ] Windows and time semantics are explicit: window type matches the question, event time (not processing time) is used with watermarks, and the watermark and allowed lateness are tuned for the data's delay distribution.
- [ ] Joins are designed for streaming correctness: stream-table joins use point-in-time/temporal semantics (not current state for event-time consistency), stream-stream joins have a time bound, and every join's state has defined retention.
- [ ] CEP patterns are precisely defined (events, conditions, sequence, time window, optionality/repetition), the time window is set from the real temporal distribution, and patterns are validated against labeled positive and negative examples for false positives and missed matches.
- [ ] Late and out-of-order events are handled via watermarks (set for the delay distribution) with a defined late-event policy (update, drop, or side output), and pattern timing uses event time.
- [ ] The highest-risk cases were verified — unbounded state from a retentionless join, non-deterministic results from processing time, a missed match from a too-strict window, a false positive from a loose pattern, and inconsistent results from a non-temporal join — not only the clean demo query.
