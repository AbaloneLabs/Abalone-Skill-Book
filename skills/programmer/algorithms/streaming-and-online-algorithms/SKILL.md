---
name: streaming_and_online_algorithms.md
description: Use when the agent is processing data as a stream (unbounded, arriving over time) rather than a bounded batch; implementing online algorithms that update incrementally as data arrives; computing approximate aggregates (count-distinct via HyperLogLog, heavy hitters via Count-Min Sketch, quantiles); managing sliding windows and windowed aggregations; handling out-of-order and late data; or choosing between batch and stream processing for a workload. Covers the stream-vs-batch decision, online vs offline computation, approximate algorithms and their error bounds, windowing semantics (tumbling, sliding, session), watermarking and late-data handling, and the tradeoff between latency and correctness in stream processing.
---

# Streaming And Online Algorithms

A batch algorithm processes a bounded dataset: all the data is available, the algorithm reads it, computes a result, and finishes. A streaming algorithm processes an unbounded dataset: data arrives continuously over time, the algorithm sees each element once (or a bounded number of times), and must maintain its state and produce results incrementally, without the luxury of re-reading the full dataset. The shift from batch to streaming changes what is possible and what algorithms are needed. An exact count of distinct elements in a batch is straightforward (a hash set); in a stream, storing every element to check distinctness is infeasible at high volume, so an approximate algorithm (HyperLogLog) is used, trading a small error bound for bounded memory. The median of a batch is found by sorting; the median of a stream requires specialized data structures or approximation, because storing the full stream is infeasible. Streaming and online algorithms are the tools for real-time, high-volume, unbounded data, and their defining tradeoffs are memory boundedness (process unbounded data with bounded memory), single-pass or few-pass processing, and the frequent reliance on approximation with understood error bounds.

Agents tend to apply batch algorithms to streaming problems (buffering the stream to process it "later," defeating the real-time goal), to expect exact results where approximation is the practical option, and to mishandle the temporal aspects of streams (out-of-order arrival, late data, windowing). The judgment problem is recognizing that streaming is a different computational model with its own algorithms and tradeoffs, that approximation with understood error bounds is often the right answer where exact computation is infeasible, and that windowing, watermarks, and late-data handling are the mechanisms that make temporal aggregation correct. This skill covers the discipline of streaming and online algorithms: the stream-vs-batch decision, online computation, approximate algorithms, windowing, watermarks, and the latency-vs-correctness tradeoff.

## Core Rules

### Choose Streaming Versus Batch Based On Latency And Data Characteristics

Streaming and batch are suited to different needs. The choice depends on the required latency and the data's boundedness.

- **Batch for bounded data and latency-tolerant computation.** When all the data is available (a file, a day's logs) and the result is not needed in real-time (a nightly report, a periodic model retrain), batch processing is simpler, more efficient per-element, and allows exact computation and multiple passes.
- **Streaming for unbounded data and low-latency computation.** When data arrives continuously (events, transactions, sensor readings) and the result is needed in near-real-time (a live dashboard, fraud detection, alerting), streaming processes data as it arrives, with latency of seconds rather than hours.
- **Streaming for data too large to store.** Even without a latency requirement, data too large to hold in memory (a multi-terabyte log) may require a streaming or few-pass algorithm, because batch algorithms that assume random access to all data are infeasible.
- **Do not force batch where streaming is needed (or vice versa).** Buffering a stream to process it in batches adds latency and memory pressure; streaming a bounded dataset that fits in memory adds complexity. Match the model to the data and latency.

### Use Online Algorithms For Incremental Computation

An online algorithm updates its result incrementally as each data element arrives, without reprocessing the full dataset. This is the foundation of stream processing.

- **Use online algorithms for aggregates that can be updated incrementally.** A running sum, count, average, min, max can be updated per element (sum += x; count++) without reprocessing. These are naturally online; use them for streaming aggregates.
- **Recognize when an aggregate is not naturally online and use a specialized algorithm.** The median, distinct count, and quantiles are not naturally online (they require the full dataset to compute exactly). Use specialized streaming algorithms (approximate medians via t-digest, distinct count via HyperLogLog, quantiles via GK or KLL algorithms) that maintain bounded state and provide approximate results.
- **Understand the algorithm's memory and time complexity per element.** A streaming algorithm must process each element quickly (to keep up with the stream's rate) and with bounded memory (to run indefinitely). An algorithm that accumulates state per element (growing memory) is not suitable for unbounded streams.
- **Handle the cold-start and warm-up.** Some online algorithms (exponential moving averages, approximate quantiles) need time to warm up before their results are accurate. Account for the warm-up in the result's interpretation.

### Use Approximate Algorithms With Understood Error Bounds

Many streaming computations (distinct count, frequency estimation, quantiles) are infeasible to compute exactly with bounded memory. Approximate algorithms provide a result within a known error bound, using bounded memory.

- **HyperLogLog for approximate count-distinct.** Counts the number of distinct elements in a stream using bounded memory (a few KB for ~1-2% error), where exact counting (a hash set of all elements) is infeasible at high cardinality. The error bound is understood and tunable (more memory, less error).
- **Count-Min Sketch for frequency estimation and heavy hitters.** Estimates the frequency of elements in a stream (and identifies the heavy hitters — the most frequent elements) using bounded memory, with an understood error bound. Used for top-K, trending, and anomaly detection.
- **T-digest or KLL for approximate quantiles.** Estimates the median, percentiles, and other quantiles of a stream using bounded memory, with understood error. Used for latency percentiles, distributions.
- **Bloom filters for membership testing.** Tests whether an element is possibly in a set (with no false negatives, possible false positives) using bounded memory. Used for deduplication, filtering, and "have I seen this before" checks.
- **Understand and document the error bounds.** Approximate algorithms trade accuracy for memory/speed; the tradeoff is quantified (e.g., "1% error with 95% confidence"). Document the bound so consumers interpret the result correctly; an approximate count presented as exact misleads.

### Define Windowing Semantics For Temporal Aggregation

Stream aggregations are typically over windows (time-based subsets of the stream), and the windowing semantics determine what data is included in each aggregation.

- **Tumbling windows: fixed-size, non-overlapping.** The stream is divided into fixed intervals (every minute, every hour); each element belongs to exactly one window; the aggregation is computed per window and emitted when the window closes. Simple and common for periodic aggregates (requests per minute).
- **Sliding windows: fixed-size, overlapping.** Windows of a fixed size that slide by an interval (a 10-minute window emitted every minute); an element may belong to multiple windows. Used for moving averages and smoothed aggregates.
- **Session windows: variable-size, grouped by activity.** Windows defined by gaps in activity (a user session ends after N minutes of inactivity); variable-size, grouping a user's continuous interaction. Used for session-based analysis (sessions per user).
- **Choose the windowing by the aggregation's meaning.** A "requests per minute" metric uses a tumbling window; a "rolling 7-day active users" uses a sliding window; a "sessions per user" uses session windows. The windowing must match the question being asked.

### Handle Out-Of-Order And Late Data With Watermarks

In real streams, data arrives out of order (an event generated at T1 arrives after an event generated at T2 > T1) and late (an event arrives long after its generation time). Watermarks and late-data handling manage this.

- **Distinguish event time (when the event occurred) from processing time (when the system processes it).** Aggregations over event time (the meaningful time for the business) must handle out-of-order arrival; aggregations over processing time are simpler but may not reflect business reality.
- **Use watermarks to signal event-time progress.** A watermark is the system's assertion that it has seen all events up to a certain event time (no events with earlier timestamps are expected). It allows the system to close event-time windows (finalize the aggregation for a time range) once the watermark passes.
- **Handle late data (events arriving after the watermark).** An event arriving after its window's watermark has passed is "late." Define a policy: update the closed window (if the aggregation allows), route to a side output for separate handling, or drop. Choose based on the data's importance and the aggregation's idempotence.
- **Set the watermark delay based on the expected out-of-orderness.** A watermark that is too aggressive (close windows early) drops late data; too conservative (wait too long) increases latency. Set the delay based on the observed out-of-order distribution (how late events typically arrive).

### Reason About The Latency-Versus-Correctness Tradeoff

Stream processing involves a tradeoff between latency (how fast results are available) and correctness (how complete and accurate they are). Understand and choose the tradeoff.

- **Lower latency means less complete results.** Emitting results as events arrive (low latency) means the result does not include late-arriving events; the result is preliminary and may be updated. Suitable for real-time monitoring where a preliminary result is valuable.
- **Higher correctness means higher latency.** Waiting for watermarks and late data (higher latency) produces complete, correct results, but delays them. Suitable for billing, reporting, and other cases where correctness matters more than immediacy.
- **Choose the tradeoff per computation.** A live dashboard may accept approximate, preliminary results (low latency); a billing aggregate must be exact and complete (higher latency). Match the tradeoff to the use case.
- **Be explicit about whether a result is preliminary or final.** A streaming result that will be updated as late data arrives should be marked preliminary; a final result (window closed, watermark passed) is final. Consumers must know which they are getting.

## Common Traps

### Batch Algorithm Applied To A Streaming Problem

Buffering a stream to process it in batches, adding latency and memory pressure, when an online or streaming algorithm would process it incrementally. Match the model to the data.

### Expecting Exact Results Where Approximation Is Practical

Insisting on exact distinct count or quantiles in a high-volume stream, using infeasible memory, when HyperLogLog or t-digest would provide a result within a known error bound. Use approximation with documented bounds.

### Ignoring Event Time, Processing By Arrival Time

Aggregating by processing time (when the event arrives) rather than event time (when it occurred), producing incorrect temporal aggregates when data is out of order. Use event time with watermarks.

### Windowing Semantics Mismatched To The Question

A tumbling window for a "rolling 7-day" metric (should be sliding), or a session window for a "per minute" metric (should be tumbling). Match the windowing to the aggregation's meaning.

### No Handling Of Late Data

Events arriving after the watermark dropped or mishandled, losing data or corrupting aggregates. Define a late-data policy (update, side output, or drop).

### Watermark Too Aggressive Or Too Conservative

Closing windows too early (dropping late data) or waiting too long (excessive latency). Set the watermark delay based on observed out-of-orderness.

### Approximate Result Presented As Exact

An approximate count or quantile presented without its error bound, misleading consumers who treat it as exact. Document and communicate the error bounds.

### Unbounded Memory Accumulation

A streaming algorithm that accumulates state per element (growing memory), failing on unbounded streams. Use bounded-memory algorithms.

## Self-Check

- [ ] Streaming vs batch is chosen based on data boundedness and latency requirements — streaming for unbounded data or low-latency results, batch for bounded data or latency-tolerant computation — and the model is not forced where it does not fit.
- [ ] Online algorithms are used for incremental computation (running aggregates updated per element), with specialized streaming algorithms (t-digest, GK/KLL for quantiles) for aggregates that are not naturally online.
- [ ] Approximate algorithms (HyperLogLog for distinct count, Count-Min Sketch for frequency/heavy hitters, t-digest/KLL for quantiles, Bloom filters for membership) are used where exact computation is infeasible, with understood and documented error bounds, and approximate results are not presented as exact.
- [ ] Windowing semantics (tumbling, sliding, session) are chosen to match the aggregation's meaning, and the choice is deliberate rather than defaulting to tumbling.
- [ ] Event time is distinguished from processing time, aggregations use event time with watermarks to signal progress and close windows, and late data (arriving after the watermark) has a defined policy (update, side output, or drop).
- [ ] The watermark delay is set based on the observed out-of-order distribution (how late events typically arrive), balancing late-data completeness against latency.
- [ ] The latency-vs-correctness tradeoff is chosen per computation — low latency with preliminary results for monitoring, higher latency with complete results for billing/reporting — and results are marked preliminary or final so consumers interpret them correctly.
- [ ] The streaming algorithm's memory and time complexity per element are bounded (it can keep up with the stream's rate and run indefinitely without unbounded memory growth), and the algorithm has been tested under realistic stream rates and out-of-order patterns.
