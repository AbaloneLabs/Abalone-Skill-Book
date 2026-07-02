---
name: data_structure_selection.md
description: Use when the agent is choosing a collection or container type (array, list, map, set, tree, heap, graph, queue), deciding how to represent domain data, optimizing a lookup or traversal pattern, picking between a hash map and a tree, choosing an ordered versus unordered container, deciding whether to normalize or denormalize data in memory, reasoning about concurrency-safe or immutable structures, or diagnosing performance problems caused by a wrong data structure. Also covers access-pattern analysis, memory versus speed tradeoffs, copy-on-write, persistent structures, and the cost of structural mistakes that surface late.
---

# Data Structure Selection

A data structure is a bet about how data will be used. Every container encodes assumptions about which operations are frequent and which are rare: an array bets that access is sequential or by index, a hash map bets that lookup by key dominates, a tree bets that ordered iteration or range queries matter, a heap bets that only the extremal element is needed. When the bet matches the real access pattern, the code is fast and readable. When the bet is wrong, the code pays for the mismatch on every operation — and the cost is often invisible until the dataset grows or the workload shifts.

Agents tend to pick the structure they reached for last, or the one the language makes default, without analyzing the access pattern first. The result is a hash map used for data that is only ever iterated in order, a list used for lookups that should be a set, a flat array used for insertions in the middle, or a mutable structure shared across threads where an immutable one was needed. These mistakes compile and pass tests. They surface later as slowness, accidental quadratic behavior, concurrency bugs, or memory blowups that are expensive to refactor because the structure is now woven through the code.

The judgment problem is to identify the dominant access patterns and constraints — read versus write, lookup versus traversal, ordering, size, concurrency, immutability — and choose the structure whose bets match them. The agent has freedom to choose the simplest sufficient structure, and an obligation to analyze access patterns before defaulting to a familiar one.

## Core Rules

### Analyze The Access Pattern Before Choosing The Structure

The structure follows the access pattern, not the other way around. Before picking a container, enumerate what the code actually does to the data:

- **Read versus write ratio.** Is the data written once and read many times, or mutated constantly? Read-heavy data rewards precomputed indexes and denormalization; write-heavy data rewards structures with cheap updates.
- **Lookup style.** Do you look up by index, by key, by predicate, by range, or by prefix? Each maps to a different structure: index to array, key to hash map, range to a sorted tree or sorted array, prefix to a trie.
- **Traversal order.** Must iteration be insertion order, sorted order, or arbitrary? Needing sorted output is a strong signal for a tree or a sorted array, not a hash map.
- **Membership tests.** Frequent "is this present?" checks call for a set, not repeated linear scans of a list.
- **Insertion and removal location.** Inserts at the end favor arrays and deques; inserts in the middle favor linked structures or trees; removal of the minimum favors a heap.

Write the access pattern down. A structure chosen without naming the dominant operations is a guess, and the guess is usually the author's habit rather than the data's need.

### Match The Structure To The Dominant Operation, Not The Average

Most data is accessed unevenly. One operation runs on every request; another runs once a week. Optimize the structure for the operation that dominates cost — the one that runs most often, on the largest data, or on the most latency-sensitive path — and accept that secondary operations may be slower.

- If a request handler looks up a user by id on every call but iterates all users only in a nightly job, key the working set by id, and rebuild the iteration view in the job.
- If a feature appends constantly but rarely reads, optimize for append (array, ring buffer, log) and pay nothing for read layout.
- If a structure is queried by range in the hot path but updated in a cold path, a sorted structure with slower updates beats a fast-updating structure with slow range queries.

Choosing for the average across operations produces a structure that is mediocre at everything. Choose for the operation whose cost you will pay most often.

### Treat Memory Layout And Locality As A First-Class Criterion

In-memory performance is governed as much by how data is laid out as by asymptotic bounds. Contiguous, cache-friendly structures often beat pointer-chasing structures that look better on paper.

- **Contiguous arrays and vectors** exploit the CPU cache and hardware prefetcher. A linear scan of a contiguous array can outperform a tree or hash map for collections up to thousands of elements, because the constant factor and locality dominate.
- **Pointer-heavy structures** (linked lists, node-based trees, chained hash buckets) pay a cache miss per hop. For small or hot data, this cost can exceed the asymptotic advantage.
- **Struct of arrays versus array of structs** matters when only some fields are touched in the hot path. Laying out the hot fields contiguously improves cache utilization dramatically for numeric or batched workloads.

When the dataset fits in cache tiers, prefer contiguous layouts. Reach for pointer-based structures when their asymptotic advantage (ordered operations, cheap mid-insertion) is real and the dataset is large enough to matter.

### Decide Ordering Semantics Explicitly

Whether a structure preserves order, and which order, is a semantic decision that callers come to depend on. Making it implicitly — by grabbing whatever the language default is — creates subtle bugs when the default changes or when tests happen to pass by accident.

- **Hash maps and sets** are unordered in most languages. If callers depend on iteration order, that dependency is fragile; tests may pass because of insertion order coincidence and fail under resizing. If order matters, use an ordered structure or sort explicitly.
- **Insertion-ordered maps** (linked hash maps) preserve first-insertion order. Use them when the contract is "iterate in the order entries were added."
- **Sorted structures** (balanced trees, sorted arrays) impose a comparison. They are required when the contract is range queries, ordered iteration, or "next greater element," and they are overkill when only equality lookup is needed.

State the ordering contract in the type or documentation. A structure whose ordering is an accident will eventually violate an assumption someone built on it.

### Account For Concurrency And Sharing Constraints

A structure shared across threads, requests, or async tasks has different requirements than one used within a single call. The sharing model must be part of the selection, not bolted on later.

- **Single-threaded, owned locally.** Any mutable structure is fine; choose for speed and simplicity.
- **Shared across threads or requests.** Prefer immutable or copy-on-write structures so readers never block and never see torn state. Where mutation is required, use structures designed for concurrency (lock-free maps, concurrent queues, RCU-style structures) rather than wrapping a plain structure in a lock.
- **Message passing between tasks.** Prefer queues and channels that move ownership, rather than sharing mutable state guarded by locks. Ownership transfer is easier to reason about than shared mutation.
- **Cached across requests.** A structure that leaves a request and enters a shared cache must be safe to hand to the next request: either immutable, or copied, or guarded so no request mutates another's view.

A mutable structure shared without a concurrency strategy is a race condition waiting for the right timing. Choose the sharing model first; the structure follows.

### Weigh Immutability Against Mutation Cost

Immutable and persistent structures make sharing safe and reasoning easy, but they are not free. The tradeoff is deliberate.

- **Immutable structures** (persistent maps, copy-on-write vectors) let you keep old versions, share safely across threads, and avoid defensive copies. They shine for configuration, state snapshots, and shared read-mostly data. Their cost is allocation per update and sometimes worse locality.
- **Mutable structures** are cheaper to update and usually more cache-friendly, but they require discipline or synchronization to share, and they make time-travel (undo, snapshots, replay) hard.

Do not reach for immutability as a default virtue, and do not reject it as inherently slow. Choose it when sharing, snapshotting, or reasoning about state is the dominant concern; choose mutation when update frequency and locality dominate. In performance-sensitive code, a mutable structure confined to one thread often beats a persistent one.

### Prefer The Simplest Sufficient Structure

A more powerful structure is more complex to use, harder to test, and more likely to be wrong for the actual need. A specialized structure (trie, bloom filter, skip list, interval tree) is justified only when its specific capability is required and the simpler alternatives provably cannot meet the constraint.

- Use an array or vector until you have evidence you need random keyed access.
- Use a hash map until you have evidence you need ordering or range queries.
- Use a sorted array plus `binary_search` until you have evidence you need frequent mid-insertion that shifting makes expensive.
- Reach for a trie only when prefix queries are the actual requirement; for plain keyed lookup it is slower and more complex than a hash map.

Each step up in structure power should be triggered by a measured or clearly reasoned need, not by a desire to use an interesting structure. The simplest structure that satisfies the access pattern is the most maintainable one.

### Plan For Growth And Resizing

A structure that is right at one size can be wrong at another. Consider how the choice behaves as the dataset grows.

- **Dynamic arrays and hash maps** resize occasionally, causing an amortized but real spike. If a resize would violate a latency budget, pre-size the structure or choose one with bounded worst-case behavior.
- **Structures with poor scaling** (nested linear scans, O(n) operations hidden in a loop) may be fine at hundreds of elements and catastrophic at millions. Estimate the growth trajectory and the point at which the structure's cost becomes unacceptable.
- **Memory footprint** can become the binding constraint before time does. A structure that holds the full dataset in RAM may be fine at 10k records and impossible at 100M; plan for streaming or external storage before that point.

State the size at which the current choice remains valid, and the structure you would move to when it is exceeded. A data structure choice without a growth plan becomes technical debt at the next order of magnitude.

## Common Traps

### Using A List Where A Set Or Map Is Needed

Storing items in a list and scanning it for membership or lookup, turning an O(1) operation into O(n) and an O(n) loop into O(n^2). This compiles, passes small tests, and becomes a performance cliff as the list grows. If you test membership or look up by key more than rarely, use a set or map.

### Reaching For A Hash Map When Ordering Matters

Choosing a hash map for data that must be iterated in sorted or insertion order, then working around the unordered iteration by sorting on every read or maintaining a parallel list. If order is part of the contract, choose an ordered structure from the start rather than reconstructing order expensively at every use.

### Defaulting To The Language's Pet Structure

Every language pushes a default container (Python's dict/list, JS's object/array, Java's ArrayList/HashMap). Using it without analysis produces a structure optimized for the language's assumptions, not your access pattern. The default is a starting point, not a decision.

### Sharing A Mutable Structure Across Threads Or Requests

Putting a plain hash map or list in a global cache or shared module-level variable and letting concurrent requests read and mutate it. This passes in low-traffic tests and corrupts state under load. If a structure is shared, it must be immutable, copied, or explicitly synchronized — chosen at selection time, not discovered in production.

### Over-Indexing On Asymptotics And Ignoring Locality

Choosing a balanced tree or a chained hash map because its big-O is better, when the dataset is small and a contiguous array would be faster due to cache behavior. Asymptotics predict behavior at infinity; real datasets are finite and locality-sensitive. For small-to-medium in-memory data, measure before trusting the notation.

### Denormalizing For Speed Without A Consistency Strategy

Duplicating data across multiple in-memory structures to speed up different access patterns, then failing to keep them in sync on every mutation. The denormalization buys speed and introduces stale-data bugs. If you denormalize, the update path must update every copy, and that path must be the only way the data changes.

### Choosing A Complex Structure For A Capability You Do Not Need

Using a trie, skip list, bloom filter, or interval tree because it is interesting or "powerful," when the actual access pattern is plain keyed lookup that a hash map serves better. Specialized structures impose complexity and often worse constants for capabilities that go unused. Match the structure's specific strength to a specific, real need.

### Ignoring The Cost Of Copying Large Structures

Passing a large vector or map by value, or cloning it defensively on every call, turning an O(1) operation into an O(n) copy that dominates the work. Know your language's value versus reference semantics, and pass large structures by reference, by move, or by shared immutable pointer unless a copy is genuinely needed.

## Self-Check

- [ ] The dominant access patterns (read/write ratio, lookup style, traversal order, membership tests, insertion/removal location) were enumerated before the structure was chosen, and the structure's strengths match those patterns.
- [ ] The structure is optimized for the operation whose cost is paid most often or on the most latency-sensitive path, not averaged across all operations.
- [ ] Memory layout and cache locality were considered; a contiguous structure was preferred where the dataset is small or hot and a pointer-based structure was chosen only where its asymptotic advantage is real.
- [ ] Ordering semantics are explicit and documented (unordered, insertion-ordered, or sorted), and no caller depends on an iteration order that is an accident of the current implementation.
- [ ] If the structure is shared across threads, requests, async tasks, or a cache, the sharing model is chosen deliberately: immutable or copy-on-write for safe sharing, concurrency-aware structures for shared mutation, or ownership transfer via queues rather than shared mutable state.
- [ ] The immutability versus mutation tradeoff was decided against the real concern (sharing, snapshotting, update frequency, locality), not adopted or rejected as a default.
- [ ] The simplest sufficient structure was chosen; a specialized structure (trie, bloom filter, skip list, interval tree) was adopted only where its specific capability is required and simpler alternatives provably fall short.
- [ ] Resizing and growth behavior is understood: the size at which the current choice stays valid is stated, amortized spikes are acceptable for the latency budget, and there is a plan for the structure that replaces this one at the next order of magnitude.
- [ ] Where data is denormalized across structures for access speed, every mutation path updates every copy, and there is no stale-data path.
- [ ] Large structures are passed by reference, move, or shared pointer rather than copied on every call, consistent with the language's value semantics.
