---
name: gas_optimization_and_cost.md
description: Use when the agent is writing or optimizing a smart contract for gas efficiency; reducing transaction cost on Ethereum, EVM-compatible chains, or other gas-metered blockchains; deciding between storage layouts, packing structs, caching storage reads, or batch processing; reasoning about gas limits, out-of-gas failures, and DoS via gas exhaustion; balancing user-paid cost against developer convenience; or auditing a contract for excessive gas that makes it unusable or vulnerable. Covers SLOAD/SSTORE costs, computation vs storage tradeoffs, batch and multicall patterns, unbounded-loop gas DoS, and the tension between cheap transactions and maintainable code.
---

# Gas Optimization And Cost

On a gas-metered blockchain, every operation the contract performs is billed to someone, usually the user initiating the transaction. Gas is not a performance metric you can ignore until later — it is a direct, visible cost that determines whether users will use the contract at all, and in the limit it is a correctness concern: a function whose gas consumption grows with state size will become uncallable, or will hit the block gas limit and fail, which is both a denial-of-service and a funds-locking risk. Agents trained on ordinary software optimization routinely treat gas as an afterthought ("optimize later") or, conversely, optimize so aggressively that the code becomes brittle and unauditable. Both extremes are wrong. Gas is a first-class design constraint that must be reasoned about from the first draft, balanced against readability and security.

The judgment problem is deciding, for each operation, where the gas is actually being spent, which optimizations are worth their complexity cost, which gas reductions create security or maintainability risk, and what the failure mode is when gas consumption is unbounded. The harm of ignoring gas is a contract no one can afford to use; the harm of over-optimizing is a contract that is cheap, opaque, and buggy.

## Core Rules

### Know Where Gas Is Spent Before Optimizing

Gas is not uniform. On the EVM, the dominant costs are storage operations: an SSTORE (writing a storage slot) costs orders of magnitude more than an arithmetic operation or a memory write, and an SLOAD (reading a storage slot) is also expensive, especially when cold. The practical hierarchy, from most to least expensive: storage writes, cold storage reads, warm storage reads, external calls, memory operations, stack arithmetic. A contract's gas profile is almost always dominated by storage access, not computation. Profile before optimizing: use the gas report from tests to see which lines consume the budget. Optimizing arithmetic when the cost is in SLOADs wastes effort and obscures the code for no benefit. The first question is always "what is actually consuming the gas," answered by measurement, not intuition.

### Reduce Storage Access As The Primary Lever

Because storage dominates cost, the highest-leverage optimizations reduce storage reads and writes.

- **Cache storage reads in memory.** If a function reads the same storage slot multiple times, read it once into a local (memory/stack) variable and reuse it. Each avoided SLOAD saves meaningful gas, and the pattern is mechanical and safe.
- **Pack structs into single storage slots.** Multiple small fields (uint128, uint64, address) can share a 256-bit slot if ordered correctly, turning several SSTOREs into one. This is a high-value, common optimization, but it couples the struct layout to the gas budget and must be maintained carefully (reordering or resizing fields can break packing silently).
- **Use immutable and constant where possible.** Values set once at construction and never changed should be `immutable` (stored in code, cheap to read); values known at compile time should be `constant` (inlined, free). Distinguishing these from regular storage avoids expensive reads for values that never change.
- **Prefer memory over storage for transient data.** Data that exists only within a transaction should live in memory, not storage.

The discipline: every storage access should be necessary, and repeated accesses to the same slot within a function should be cached. Audit the contract for redundant SLOADs — they are the cheapest gas wins.

### Bound Gas Consumption To Prevent Out-Of-Gas And DoS

A function whose gas grows with array length, number of holders, or any unbounded state will eventually exceed the block gas limit and become uncallable. This is not just a cost issue; it is a correctness and security issue. The classic failure is a token contract that loops over all holders to distribute a dividend, which works at launch and bricks when the holder list grows. The defenses:

- **Avoid unbounded loops over state that can grow.** If you must iterate, bound the iteration (process in batches across multiple transactions, paid for by the user pulling their share).
- **Prefer pull over push.** Instead of the contract looping to send to many recipients, let each recipient call to claim their share. Gas is paid by the recipient, and the contract never iterates unboundedly.
- **Beware of patterns that let an attacker grow the iteration cost.** A contract that iterates over a list anyone can append to is a DoS vector: an attacker appends enough entries to make every call exceed the gas limit.

Reason about the worst-case gas of every function as state grows. A function that is cheap at 100 entries may be uncallable at 10,000, and an attacker may be able to drive it there.

### Batch And Multicall To Amortize Fixed Costs

Every transaction pays a fixed overhead (base gas, calldata, signature verification). For users who need to perform many operations, batching them into one transaction amortizes that overhead and reduces total cost. Patterns include multicall (a single entry point that executes a list of calls), batch transfers, and batching claims. The tradeoff: batching reduces per-operation cost but increases single-transaction gas (which must stay under the block limit) and complicates partial-failure semantics (should the batch revert atomically, or apply successful items and skip failed ones?). Choose the failure semantics deliberately; atomic-all-or-nothing is simplest but can make a batch unusable if one item fails, while partial application requires careful accounting.

### Weigh Optimization Against Readability, Security, And Maintainability

Gas optimization can make code worse. Packing structs, using assembly, replacing clear logic with bit tricks, and caching aggressively all reduce gas and all make the code harder to read, harder to audit, and more prone to subtle bugs. The question is whether the saving justifies the cost. A 5% gas reduction that obscures a transfer function may not be worth the audit risk on a value-bearing contract; a 50% reduction in the hot path of a high-frequency operation may be essential. Apply optimizations proportionally to their impact and to the risk of the code they touch. Document non-obvious optimizations so auditors and future maintainers understand the intent. The worst outcome is a contract that is slightly cheaper and significantly more likely to contain a bug.

### Account For Calldata And L2 Cost Models

Gas is not the only cost, and on different chains the cost model differs. Calldata size matters, especially on rollups where calldata published to the L1 dominates cost — an optimization that adds calldata to save execution gas can be a net loss on an L2. On some chains, storage is cheaper or more expensive relative to computation; on others, different opcodes are priced differently. Know the target chain's cost model before optimizing. An optimization tuned for Ethereum mainnet may be wrong for an L2 rollup, and vice versa. Reason about total user cost (execution gas plus calldata plus any L1 publication) on the actual deployment target.

## Common Traps

### Optimizing Arithmetic While Ignoring Storage

Spending effort squeezing gas out of computation when the cost is dominated by SLOAD/SSTORE. Measure first; storage access is almost always the real cost.

### Unbounded Loops Over Growable State

Iterating over a holder list, queue, or mapping that can grow until the loop exceeds the block gas limit, bricking the function. Bound iteration or switch to pull-based patterns.

### Repeated Cold SLOADs In A Loop

Reading the same storage slot inside a loop body instead of caching it once before the loop, multiplying gas cost by iteration count. Cache storage reads that repeat.

### Breaking Struct Packing By Accident

Reordering struct fields or changing a field type without realizing it breaks slot packing, silently turning one SSTORE into several. Treat struct layout as part of the gas contract; review changes against the packing.

### Over-Optimizing Into Unreadable Assembly

Rewriting clear logic in assembly to save small amounts of gas, producing code that auditors struggle to verify and that is more likely to contain bugs. Weigh the saving against the audit and maintenance cost.

### Ignoring Calldata Cost On Rollups

Optimizing only for execution gas and ignoring calldata, so an L2 deployment is more expensive than expected because calldata dominates. Reason about the target chain's full cost model.

### Atomic Batches That Fail Entirely On One Bad Item

A multicall that reverts the whole batch if any item fails, so a single bad entry makes the entire batch unusable. Choose partial-failure semantics deliberately when batching user-facing operations.

### Assuming "It's Cheap At Launch" Means "It Stays Cheap"

Shipping a function whose gas is acceptable at current state size, then watching it become uncallable as state grows. Project gas consumption forward to realistic future state sizes.

## Self-Check

- [ ] The contract was gas-profiled (test gas report) before optimizing, and optimizations target the actual cost centers (typically storage access), not assumed ones.
- [ ] Repeated storage reads within a function are cached in memory; struct fields are packed into minimal storage slots; immutable/constant are used for values that do not change at runtime.
- [ ] No function has unbounded gas growth: loops over growable state are eliminated, bounded, or converted to pull-based (user-initiated) patterns; an attacker cannot inflate iteration cost.
- [ ] Batching/multicall is used to amortize fixed costs where users perform repeated operations, with deliberate partial-failure semantics.
- [ ] Each optimization was weighed against readability, auditability, and bug risk; non-obvious optimizations are documented; assembly and bit-tricks are justified by material savings, not used reflexively.
- [ ] The cost model of the actual deployment target (mainnet, L2 rollup, other chain) is understood; calldata and L1-publication costs are included in the optimization decision, not only execution gas.
- [ ] Worst-case gas was projected forward to realistic future state sizes (holders, queue length, mapping size), and no function becomes uncallable or unaffordable under growth.
- [ ] Gas reductions do not compromise security: packing does not enable overflow or access bugs, caching does not stale-read, and assembly does not bypass checks.
- [ ] The optimized contract still passes the full adversarial test suite; optimization did not introduce a behavior change that the tests did not catch.
