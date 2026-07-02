---
name: consensus_and_finality.md
description: Use when the agent is building or integrating with a blockchain, choosing a chain for an application, designing a transaction flow that depends on confirmation, reasoning about proof-of-work versus proof-of-stake, evaluating finality guarantees, estimating transaction confirmation latency, assessing reorg (chain reorganization) risk, building bridges or cross-chain interactions, or deciding how many confirmations to wait before treating a transaction as settled. Covers probabilistic vs absolute finality, fork choice rules, reorg depth, the meaning of "confirmed," and the risk of acting on a transaction that can still be reversed.
---

# Consensus And Finality

A blockchain transaction is not "done" when it appears in a block. It is done when the consensus mechanism makes it effectively impossible to reverse — and that point, called finality, arrives at different times and with different strengths depending on the chain's consensus design. The recurring and expensive mistake is treating "included in a block" or "a few confirmations deep" as a universal guarantee of settlement, when in fact some chains offer only probabilistic finality (a transaction can always be reversed in principle, just with vanishing probability) and others offer absolute finality (a finalized transaction cannot be reversed without destroying the network). An agent who does not understand which model a given chain uses will either wait far too long and harm user experience, or act far too soon and lose funds to a reorg.

The judgment problem is deciding, for a given chain and a given transaction value, what finality means, how long to wait, and what the residual reversal risk is — and especially how that risk changes when crossing chains (bridges), where the weakest-link finality governs the whole flow. The harm of getting this wrong is concrete: accepting a payment that gets reorged away, releasing goods against an unconfirmed transaction, or building a bridge whose security assumption the operator does not actually understand.

## Core Rules

### Know Whether The Chain Offers Probabilistic Or Absolute Finality

The two families have fundamentally different settlement semantics, and conflating them is the root error.

- **Probabilistic finality (Nakamoto consensus, proof-of-work style).** A transaction is never absolutely final; it becomes progressively harder to reverse as more blocks are built on top of it. "Finality" is a probability threshold the application chooses (e.g., "6 confirmations makes reversal economically infeasible"). The number is a judgment call balancing latency against reversal risk, and it depends on the chain's block reward, hash rate, and attacker economics.
- **Absolute (deterministic) finality (BFT-style proof-of-stake, e.g., Ethereum post-Merge, Cosmos chains, many AppChains).** A transaction becomes irreversibly final at a specific point (a finalized checkpoint signed by a supermajority of validators). Before that point it can be reorged; after it, it cannot be reversed without a catastrophic failure of a third or more of validators. Finality arrives at a known time, not as a probability curve.

The decision that follows: on a probabilistic chain, you must choose a confirmation depth per transaction value and risk tolerance; on an absolute-finality chain, you wait for the finality checkpoint and then the risk is qualitatively different. Know which chain you are on before reasoning about confirmation.

### Distinguish Inclusion, Confirmation, And Finality

These are three different states, and treating them as one causes both false alarms and lost funds.

- **Inclusion.** The transaction is in a block. It can still be reorged out trivially.
- **Confirmation (depth N).** N blocks have been built on top. On a probabilistic chain, reversal probability decreases with N; on an absolute-finality chain, depth alone does not finalize — you wait for the finality checkpoint.
- **Finality.** The transaction is irreversible by the consensus rules. On probabilistic chains this is a chosen threshold; on absolute-finality chains it is a specific event.

Map your application's logic to these states deliberately: when do you show "pending," when do you release goods, when do you consider the payment settled for accounting. A UI that shows "confirmed" at inclusion misleads users; a backend that ships goods at inclusion accepts reorg risk it may not intend.

### Size Confirmation Depth To Transaction Value And Attacker Economics

On a probabilistic-finality chain, the right confirmation depth is not a constant — it is a function of what an attacker would gain by reversing the transaction versus what it would cost. The classic analysis: to reverse a transaction N blocks deep, an attacker must re-mine those blocks and outpace the honest network, which becomes exponentially expensive in N. For a small payment, a few confirmations suffice because the attack cost exceeds the payment. For a large transfer (an exchange withdrawal, a bridge deposit), the attacker's incentive is higher, so more confirmations are warranted. The error is applying "6 confirmations" universally: it may be excessive for a coffee and insufficient for a million-dollar transfer. Reason about the value at stake and choose the depth that makes attacking it uneconomical.

### Understand Reorg Risk And Its Causes

A reorganization is the consensus mechanism replacing one chain history with another, which can drop or reorder transactions that appeared settled. Reorgs happen for several reasons: two miners/validators find blocks near-simultaneously and one branch wins (benign, shallow), network latency partitions the chain briefly (shallow), or an attacker with sufficient power rewrites history (deep, the dangerous case). Most chains have frequent shallow reorgs of 1-2 blocks; deep reorgs are rare on healthy chains but catastrophic when they occur. When designing around a chain, know its typical reorg depth and the conditions that produce deeper reorgs (low hash rate / validator participation, network partitions, software upgrades). Treat shallow reorgs as expected and design so a 1-2 block reorg does not break your application; treat deep reorgs as the tail risk your confirmation policy defends against.

### Treat Cross-Chain Finality As Weakest-Link

A bridge or cross-chain interaction inherits the finality of every chain it touches, and the whole flow is only as final as the weakest link. A bridge that releases funds on chain B when it sees a deposit on chain A is trusting A's finality — if A's deposit is reorged after the bridge released on B, the bridge is now undercollateralized or the operator eats the loss. The discipline: wait for robust finality on the source chain before acting on the destination, and understand that "robust" may mean many more confirmations on a probabilistic source chain than the bridge's default. Many bridge exploits were not cryptographic breaks but finality mismatches — acting on a source-chain transaction that was not as final as the bridge assumed. For any cross-chain value flow, name the finality assumption on each side and verify the design waits for it.

### Account For Finality Latency In User-Facing Flows

Finality takes time, and that time is a user-experience cost. On an absolute-finality chain, finality may arrive in seconds to minutes (epoch boundaries); on a probabilistic chain, "safe" confirmation depth may take tens of minutes. Design the UX around the real latency: show pending states honestly, do not promise instant settlement the chain cannot deliver, and where latency is unacceptable consider whether a different chain or a layer-2 with faster finality fits. A common failure is a UI that implies instant finality and then leaves users staring at a pending transaction for twenty minutes, eroding trust. Match the interface's promises to the chain's actual settlement time.

### Recognize That Upgrades And Governance Can Change The Rules

Some chains have governance or validator-coordinated upgrades that can, in extremis, alter history or finality (the post-mortem interventions after major exploits are the famous examples). This is a qualitatively different risk than ordinary consensus finality: a "finalized" transaction can in principle be reversed by social consensus, however rarely. For most applications this tail risk is acceptable and not worth designing around; for high-stakes or adversarial contexts, acknowledge that "absolute finality" is absolute only within the chain's governance assumptions, not against a sufficiently motivated coalition. Know your chain's governance model and whether history has ever been altered by intervention.

## Common Traps

### Treating Inclusion As Finality

Acting on a transaction the moment it appears in a block, then losing it to a shallow reorg. Distinguish inclusion, confirmation, and finality; wait for the state your risk requires.

### Applying A Universal Confirmation Depth

Using "6 confirmations" for every transaction regardless of value or chain. Size the depth to the value at stake and the chain's economics; small payments need fewer, large transfers need more.

### Assuming "Finalized" Means The Same Thing On Every Chain

Treating absolute finality and probabilistic finality as interchangeable, or assuming a finalized transaction on chain A has the same irreversibility as on chain B. Know the chain's consensus model and what its finality actually guarantees.

### Bridge Finality Mismatch

A bridge releasing on the destination chain based on a source-chain confirmation that is shallower than the source chain's real finality, so a source reorg leaves the bridge undercollateralized. Wait for robust source-chain finality before acting cross-chain.

### Ignoring Typical Reorg Depth

Designing as if reorgs never happen, then breaking on the routine 1-2 block reorgs every chain experiences. Expect shallow reorgs; design so they cause retries, not failures.

### Promising Instant Settlement The Chain Cannot Deliver

A UI that implies instant finality, then leaves users waiting for the real confirmation/finality time, eroding trust. Match UX promises to actual settlement latency.

### Trusting Finality Against Governance Intervention Without Acknowledging It

Assuming absolute finality is absolute against all adversaries, ignoring that governance or validator coordination has, rarely, altered history. For high-stakes use, acknowledge the governance tail risk.

### Conflating Validator Count With Finality Strength

Assuming more validators means stronger finality, when the relevant metric is the economic stake or Byzantine threshold required to reverse, not the raw count. Reason about the cost to reverse, not the size of the set.

## Self-Check

- [ ] The chain's consensus model (probabilistic vs absolute finality, PoW vs PoS, fork choice) is known, and finality is reasoned about in the correct model, not assumed universal.
- [ ] The application distinguishes inclusion, confirmation, and finality, and each user-visible or value-releasing action waits for the correct state for its risk.
- [ ] Confirmation depth (on probabilistic chains) is sized to transaction value and attacker economics, not applied as a universal constant.
- [ ] Typical reorg depth for the chain is known, and the design tolerates shallow reorgs (retry/idempotency) rather than breaking on them.
- [ ] Cross-chain flows wait for robust finality on the source chain before acting on the destination; the bridge or integration's finality assumption on each side is named and verified, not inherited by default.
- [ ] User-facing flows honestly reflect actual finality latency; the UI does not promise instant settlement the chain cannot deliver.
- [ ] The chain's governance and upgrade model is understood, and for high-stakes use the tail risk of governance-altered history is acknowledged rather than assumed away.
- [ ] For value-bearing decisions, the relevant metric (economic cost to reverse, Byzantine threshold, validator stake distribution) was used to judge finality strength, not a superficial validator count.
- [ ] A test of the flow under adverse conditions (network latency, a shallow reorg, a finality delay) confirmed the application degrades gracefully rather than losing funds or deadlocking.
