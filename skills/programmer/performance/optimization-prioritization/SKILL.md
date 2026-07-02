---
name: optimization_prioritization.md
description: Use when the agent is deciding what to optimize, prioritizing performance work, choosing between candidate optimizations, justifying an optimization's return on investment, or reviewing whether a proposed optimization is worth its complexity. Covers bottleneck identification with Amdahl's law, measurement-driven optimization, avoiding premature optimization, cost/benefit framing, user-perceived vs system-internal performance, the latency-throughput tradeoff, and the discipline of optimizing only what the data shows matters. Also use when a teammate proposes optimizing a component, when evaluating whether to rewrite for speed, or when balancing performance against readability, cost, and shipping time.
---

# Optimization Prioritization

Performance work fails in two opposite ways. The first is optimizing the wrong thing: a team spends weeks shaving microseconds off a function that runs once per request, while a database query that accounts for most of the latency goes untouched. The second is not optimizing at all, on the grounds that "premature optimization is the root of all evil," until the system is too slow to ship. Both failures share a root cause: optimization decisions made by intuition about where time goes, instead of by measurement of where time actually goes. The discipline of optimization prioritization is replacing that intuition with a measured profile, a quantified share of total cost, and a cost/benefit comparison — then acting only where the data justifies action.

Agents tend to drift toward whichever failure is more comfortable. Given a "make it faster" task, the common drift is toward the optimization that is technically interesting (a clever algorithm, a lock-free data structure, a custom allocator) rather than the one that moves the user-visible metric, because interesting work feels like progress and the interesting component is usually not the bottleneck. Given a "should we optimize this" question, the common drift is toward a generic caution against premature optimization that forecloses work the data would support. The judgment problem is neither "optimize everything" nor "optimize nothing," but a cold comparison of where the time and money actually go, what each candidate change would save, and whether that saving justifies the complexity, risk, and maintenance cost.

This skill is about deciding what to optimize and in what order. It complements the profiling skill (which covers how to find where time goes) and the benchmarking skill (which covers how to measure a change); here the question is the meta-decision of which change is worth making at all.

## Core Rules

### Optimize The Bottleneck, Quantified By Share Of Total Cost

Amdahl's law is the governing constraint: the speedup achievable by improving a component is limited by the fraction of total time that component consumes. Speeding up a function that is 1% of runtime to zero yields at most a 1% improvement, no matter how clever the optimization. Speeding up a function that is 50% of runtime by half yields a 25% improvement. The implication is blunt: if you cannot point to a profile showing the target accounts for a meaningful share of total cost, you are almost certainly optimizing the wrong thing.

- **Profile before optimizing, and profile the real workload.** A profile of a synthetic workload may point you at code that is cheap in production. Profile representative production traffic (or the closest faithful reproduction), at the granularity that attributes time to components.
- **Express the target as a share of the whole.** "This query takes 200ms" is not actionable without knowing the request takes 800ms total. "This query is 25% of request time" tells you the ceiling on improvement and whether it is worth pursuing.
- **Re-profile after each win.** Once you optimize the top bottleneck, the next-largest component becomes the new top. Optimization is iterative on a moving target; do not assume the original profile still holds after changes.

### Frame Every Candidate As Cost Versus Benefit

An optimization is worth doing only if its benefit exceeds its cost. Both sides must be explicit, not felt.

- **Benefit is the user-visible or system-level improvement** — reduced latency, increased throughput, lower cost, smaller footprint — expressed in the units that matter to the decision (ms saved, requests/sec gained, dollars/month reduced). A micro-optimization with no measurable user impact has near-zero benefit regardless of its cleverness.
- **Cost includes the obvious and the hidden.** Obvious: engineering time to implement. Hidden: added complexity that slows future work, new failure modes, harder testing, deeper coupling, ongoing maintenance, the risk of a correctness bug introduced for speed. An optimization that shaves 5% but makes the code unreadable has a large ongoing cost.
- **Prefer high-benefit, low-complexity changes first.** An index that halves query time for a day of work beats a lock-free rewrite that shaves 10% for a month and introduces subtle races. Order candidates by benefit-to-cost ratio, not by technical interest.
- **Reject optimizations whose benefit does not justify their complexity.** It is correct to decline a 2% improvement that adds a maintenance burden forever. "Could be faster" is not "should be made faster."

### Distinguish User-Perceived Performance From System-Internal Performance

The metric that matters is the one the user experiences. Optimizing a metric the user does not perceive is work that does not matter to them, however real the improvement.

- **Identify the user-perceived metric.** For a request, it is end-to-end latency as the user experiences it (including network, TLS, server queueing, processing, rendering). For a batch job, it may be time-to-result. For a UI, it may be interaction responsiveness. Optimize the metric the user feels, not a sub-component in isolation.
- **Watch for optimizations that improve an internal metric but not the user metric.** Cutting server processing time by 30% may not move end-to-end latency if the request was network-bound; the internal win is real but invisible to the user. Before celebrating, confirm the user metric moved.
- **Account for the full path.** A backend optimization that halves processing time but is dwarfed by a frontend waterfall of blocking requests delivers no user benefit until the frontend is fixed. Prioritize along the path that constrains the user metric.

### Use Measurement To Decide, Not To Confirm

The role of measurement is to change the decision, not to ratify a decision already made. If you would optimize regardless of what the measurement shows, the measurement is theater.

- **Measure the baseline before changing anything.** Without a baseline you cannot know if you improved anything, by how much, or whether you regressed something else.
- **Measure the change in isolation.** A/B the optimized and unoptimized paths, or benchmark before and after on identical conditions. An "improvement" that coincided with a load change or a data change is not an improvement you caused.
- **Beware confirmation bias.** It is easy to run benchmarks until one shows the desired result and then stop. Predefine what you are measuring and what threshold constitutes success, then report honestly whether the threshold was met.
- **Measure at the level of the decision.** A micro-benchmark of the changed function confirms the function got faster; only a system-level benchmark confirms the system got faster. The latter is what justifies the change.

### Separate Latency Goals From Throughput Goals

Latency and throughput are related but distinct, and optimizing one can harm the other. Confusing them produces work that moves the wrong metric.

- **Latency** is how long one operation takes (per-request time). Throughput is how many operations complete per unit time (requests/sec). Improving latency often improves throughput, but the relationship is not linear and the bottlenecks differ.
- **Know which you are optimizing and why.** A user-facing API with a latency SLO should be optimized for tail latency; a batch pipeline should be optimized for throughput. The right tool differs (e.g., reducing queueing for latency; increasing parallelism or batching for throughput).
- **Watch for latency-throughput tradeoffs.** Batching increases throughput but adds per-request latency (waiting for the batch to fill). Caching improves both until it does not (cache misses under load). Increased concurrency can improve throughput but add queueing latency. Make the tradeoff explicit and choose for the goal.

### Treat Premature Optimization And Excessive Caution As Symmetric Errors

"Premature optimization is the root of all evil" is routinely misused to mean "never optimize," which is not what it says. It says do not optimize before you know it matters. The symmetric error — refusing to optimize even after measurement shows it matters — is just as damaging.

- **Premature optimization is optimizing without evidence of benefit.** The cure is measurement and cost/benefit, not inaction.
- **Excessive caution is declining optimization despite evidence of benefit.** The cure is the same measurement and cost/benefit, applied honestly.
- **The dividing line is the data.** With a profile showing a bottleneck and a cost/benefit that clears the bar, optimize. Without it, do not. The rule is "measure, then decide," not "optimize" or "don't optimize" as a default.

## Common Traps

### Optimizing The Interesting Component, Not The Dominant One

Spending effort on a clever algorithm or data structure because it is intellectually appealing, while the actual bottleneck (often I/O, a database query, or serialization) goes unaddressed. Profile first; optimize the component the profile identifies as dominant.

### Reporting An Internal Win That Did Not Move The User Metric

Celebrating a 40% reduction in server processing time when end-to-end user latency is unchanged because the request was network- or frontend-bound. Confirm the user-perceived metric moved before claiming success.

### Amdahl Blindness

Pursuing a 1% improvement with the same effort as a 30% improvement, because the 1% component is the one you happen to be looking at. Rank candidates by share of total cost; the ceiling on each improvement is its share.

### No Baseline, So No Way To Know If It Worked

Optimizing without measuring the before state, then asserting improvement based on a feeling or a single noisy run. Measure the baseline first; compare after on identical conditions.

### Optimizing To Confirm A Decision Already Made

Running benchmarks with the intent to find one that justifies the optimization, stopping at the first favorable result. Predefine the metric and threshold; report honestly.

### Confusing Latency And Throughput

Adding batching or parallelism that improves throughput but worsens per-request latency, when the goal was a latency SLO. Name the goal (latency or throughput) before choosing the technique.

### Accepting Permanent Complexity For A Temporary Gain

A 5% optimization that introduces a subtle concurrency bug or makes the code unreadable, costing far more in maintenance and incidents than it ever saved. Weigh ongoing complexity cost against the benefit; some optimizations are not worth their complexity.

### Treating "Premature Optimization" As "Never Optimize"

Invoking the quote to decline optimization work even after a profile identifies a dominant bottleneck with clear cost/benefit. The quote is a warning against unmeasured optimization, not a prohibition on measured optimization.

## Self-Check

- [ ] The optimization target was identified from a profile of representative production (or faithfully reproduced) traffic, and its share of total cost is known — not chosen by intuition or technical interest.
- [ ] The candidate optimization has an explicit cost/benefit: a quantified expected benefit (ms saved, throughput gained, cost reduced, in the user-relevant unit), weighed against implementation time, added complexity, new failure modes, and ongoing maintenance.
- [ ] Candidates are ordered by benefit-to-cost ratio, and high-benefit/low-complexity changes (e.g., an index) are preferred over low-benefit/high-complexity rewrites regardless of technical interest.
- [ ] The user-perceived metric (end-to-end latency, time-to-result, interaction responsiveness) was named as the goal, and the optimization was confirmed to move that metric — not just an internal sub-component metric.
- [ ] A baseline was measured before the change, the change was measured in isolation on identical conditions, and the measurement was predefined (metric and success threshold) rather than run until a favorable result appeared.
- [ ] Latency and throughput goals were distinguished, the relevant one for the task was named, and latency-throughput tradeoffs (batching, parallelism, caching) were made explicit rather than assumed away.
- [ ] The decision to optimize (or not) was driven by the data — a profile showing a dominant bottleneck and a cost/benefit that clears the bar — not by a default of "optimize" or "don't optimize"; neither premature optimization nor excessive caution was applied.
- [ ] After each optimization, the system was re-profiled to find the new bottleneck, since optimization is iterative on a moving target.
