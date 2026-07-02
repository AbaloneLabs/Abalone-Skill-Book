---
name: probabilistic_and_approximate_algorithms.md
description: Use when the agent is choosing or designing probabilistic data structures such as Bloom filters or HyperLogLog or count-min sketches, setting error bounds and false positive rates, deciding between deterministic and probabilistic approaches, or evaluating when approximate answers are acceptable for counting, membership, cardinality, or frequency estimation.
---

# Probabilistic and Approximate Algorithms

Not every problem needs an exact answer. Counting distinct visitors across billions of events, checking whether an item might be in a massive set, or estimating frequency in a high-volume stream—all of these have exact solutions that require prohibitive memory, and probabilistic solutions that use a fraction of the space at the cost of a bounded, tunable error. The skill is knowing when approximate is acceptable, which structure fits the problem, and how to set the error parameters so the approximation is actually useful rather than misleadingly wrong. The failure mode is reaching for a probabilistic structure where exactness is required, or mis-tuning the error bounds so the result is either useless or no cheaper than the exact approach.

The judgment problem is determining whether the consumer of the result can tolerate approximation, choosing the right probabilistic structure for the query type, and setting the false-positive rate and error bounds to match the use case. The agent should not use a Bloom filter because it sounds clever; it should use it when membership-with-false-positives is acceptable and the memory savings justify the complexity.

This skill applies whenever you are choosing data structures for membership, cardinality, or frequency estimation over large or streaming data, or deciding whether an exact or approximate approach fits the requirement.

## Core Rules

### Confirm approximation is acceptable before choosing a probabilistic structure

The first question is whether the consumer can act on an approximate answer. Approximation is acceptable when:

- The result drives a heuristic or ranking (search relevance, recommendations) where small errors are noise.
- The result is for monitoring or capacity planning, where directional accuracy suffices.
- The result is a pre-filter that is verified exactly later (a Bloom filter that says "maybe in set" before an exact lookup).

Approximation is unacceptable when:

- The result drives a correctness or safety decision (authorization, billing, deduplication that must be exact).
- The result is a user-facing number presented as exact (a balance, a count presented as authoritative).
- Downstream logic cannot tolerate any error.

If exactness is required, use an exact structure. A probabilistic structure in an exactness-required path is a bug.

### Match the structure to the query type

Each probabilistic structure answers a specific class of question. Choose by the query:

- **Membership ("is X in the set?")**: Bloom filter. Returns "possibly in set" (with false positives) or "definitely not in set" (no false negatives). Space-efficient for large sets.
- **Cardinality ("how many distinct items?")**: HyperLogLog. Estimates unique count in small fixed memory (kilobytes for billions of items) with tunable error.
- **Frequency ("how often does X appear?")**: Count-Min Sketch. Estimates per-item counts or heavy hitters in a stream with sublinear space.
- **Top-k / heavy hitters**: Count-Min Sketch with min-heap, or SpaceSaving. Finds the most frequent items approximately.
- **Range/quantile estimation**: t-digest or GK algorithm. Approximates percentiles for monitoring.

Using the wrong structure for the query is a common error: a Bloom filter cannot count, HyperLogLog cannot test membership, a Count-Min Sketch overcounts but never undercounts. Match the structure to the question.

### Understand the direction and tunability of the error

Each structure has a characteristic error profile, and you must know its direction:

- **Bloom filter**: false positives (may claim an absent item is present), never false negatives. The false-positive rate is tunable via bit-array size and hash count; more bits and hashes reduce it at the cost of space.
- **HyperLogLog**: standard error is tunable via the number of registers; more registers reduce error. The estimate is unbiased but noisy; typical accuracy is ~1-2% with modest memory.
- **Count-Min Sketch**: overestimates (never underestimates) frequency, because it takes the minimum of several counts. Error and confidence are tunable via width and depth.

Knowing the error direction is critical: a Bloom filter's "maybe" must be treated as uncertain; a Count-Min Sketch's overcount means thresholds should account for inflation. Misreading the error direction leads to wrong decisions.

### Set error bounds to match the use case's tolerance

Error parameters are not set-and-forget; they must match what the consumer can tolerate:

- For a Bloom filter pre-filtering disk lookups, a 1% false-positive rate may be fine (1% of negative lookups hit disk unnecessarily). For a security-relevant check, even 0.1% may be too high.
- For HyperLogLog driving a dashboard, 2% error is invisible. For driving an alert threshold, 2% may cause flapping.
- Always compute the actual error rate and the resulting space cost, and confirm both are acceptable. Do not pick defaults without understanding them.

Document the chosen error rate and its implications, so future maintainers know the approximation's limits.

### Handle the union/intersection/set-operation properties where relevant

Some probabilistic structures compose; others do not:

- **Bloom filters** are union-friendly: OR-ing two filters' bit arrays yields the union. Intersections are approximate and may produce false negatives.
- **HyperLogLog** is union-friendly: merging registers estimates the union cardinality. This makes it ideal for distributed distinct-count (merge per-shard results).
- **Count-Min Sketches** with matching parameters are addable (sum per-cell), supporting distributed frequency estimation.

If your use case requires set operations across distributed data, choose a structure that composes. If it requires exact intersections or deletions, probabilistic structures are often the wrong tool (standard Bloom filters cannot delete; Counting Bloom filters can, with more space).

### Decide between deterministic and probabilistic per query

For each query, weigh:

- **Exact (deterministic)**: correct, simple, but may require memory or time proportional to the data. Right when the dataset fits or exactness matters.
- **Probabilistic (approximate)**: bounded memory, fast, but approximate. Right when the dataset is too large for exact, or when the answer is heuristic.

A common mistake is using a probabilistic structure for a dataset that fits easily in memory exactly, adding complexity and error for no benefit. Another is forcing an exact structure onto a streaming or multi-terabyte dataset where it cannot fit. Match the choice to the data scale and the accuracy requirement.

### Validate the approximation against ground truth periodically

Probabilistic structures should be validated:

- Compare approximate counts against exact counts on a sample or during low-traffic periods, to confirm the error is within the expected bound.
- Monitor the false-positive rate of Bloom filters in production (track the ratio of "maybe" results that turn out negative on exact verification).
- Alert if the observed error exceeds the theoretical bound, which indicates a tuning or implementation problem.

An unvalidated approximation can silently drift, and consumers will trust numbers that are wrong.

## Common Traps

### Using a probabilistic structure where exactness is required

A Bloom filter for deduplication that must be exact, or HyperLogLog for a billing count, introduces errors the consumer cannot tolerate. Confirm approximation is acceptable first.

### Choosing the wrong structure for the query

A Bloom filter cannot count, HyperLogLog cannot test membership, a Count-Min Sketch overcounts. Match the structure to the query type.

### Misreading the error direction

Treating a Bloom filter's "maybe" as certain, or a Count-Min Sketch's overcount as exact, leads to wrong decisions. Know whether the error is positive-only, negative-only, or bidirectional.

### Using defaults without understanding the error rate

Accepting library defaults for false-positive rate or standard error without computing whether the resulting accuracy and space are acceptable. Always compute and confirm the tradeoff.

### Using a probabilistic structure for data that fits exactly

Adding approximation and complexity to a dataset that fits in memory exactly, gaining nothing. Use exact structures when the data fits.

### Forgetting that standard Bloom filters cannot delete

Adding items to a Bloom filter and later needing to remove them—standard Bloom filters cannot delete without false negatives. Use a Counting Bloom filter or a Cuckoo filter if deletion is required.

### Treating approximate numbers as exact in user-facing output

Displaying a HyperLogLog estimate as an authoritative count, or a Count-Min Sketch frequency as exact, misleads users. Label approximate values or round them to reflect uncertainty.

### Not validating the approximation

Trusting the theoretical error bound without measuring the actual error in production. Implementation bugs, bad hash functions, or skewed data can produce errors worse than the bound.

## Self-Check

- Has it been confirmed that the consumer of the result can tolerate approximation, and that no correctness, safety, or billing decision depends on exactness?
- Is the chosen structure matched to the query type (Bloom for membership, HyperLogLog for cardinality, Count-Min for frequency, t-digest for quantiles)?
- Is the direction of the error understood (Bloom false-positives only, Count-Min overcounts only) and accounted for in downstream decisions?
- Are the error parameters (false-positive rate, standard error) explicitly chosen and computed, with the resulting space cost confirmed acceptable?
- Is the structure's composability (union, merge, addition) matching the use case's need for set operations or distributed aggregation?
- Has the deterministic-versus-probabilistic choice been made based on data scale and accuracy requirement, not on familiarity?
- If deletion is required, is a deletion-capable structure (Counting Bloom, Cuckoo filter) used instead of a standard Bloom filter?
- Are approximate results labeled or rounded to reflect uncertainty in user-facing output, rather than presented as exact?
- Is the approximation validated against ground truth periodically, with the observed error monitored against the theoretical bound?
- Is the complexity of the probabilistic structure justified by the memory or time savings over the exact approach?
