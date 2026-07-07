---
name: bridge_and_cross_chain_security.md
description: Use when the agent is building, integrating with, or auditing a cross-chain bridge, interoperability protocol, or message-passing layer; designing a wrapped-asset (WETH, wrapped BTC) or canonical-asset issuance flow; selecting a validator set, multi-sig custodian, or relayer model for cross-chain transfers; verifying cross-chain messages, signatures, or Merkle proofs; reasoning about lock-and-mint versus burn-and-mint versus liquidity-pool bridge models; assessing the trust assumptions of a bridge that spans chains with different finality (probabilistic vs absolute); defending against replay attacks, signature forgery, or finality-mismatch exploits; evaluating the honeypot risk of a bridge holding billions in custody; or reviewing an IBC, LayerZero, Wormhole, Nomad, or arbitrary-message bridge for mainnet deployment. Covers validator-set collusion, multi-sig custody risk, canonical vs non-canonical wrapped assets, message-passing verification, the bridge-as-honeypot problem, replay protection, and the weakest-link finality that governs every cross-chain flow.
---

# Bridge And Cross-Chain Security

A cross-chain bridge is a system that holds assets on one chain and issues a claim on those assets on another — which makes it, in aggregate, the largest concentrated honeypot in crypto. Bridges routinely custody billions of dollars, and they are correspondingly the most-hacked category of blockchain infrastructure: the largest exploits in the space's history have almost all been bridges, and the losses were not cryptographic breaks of the underlying chains but failures of the bridge's own trust model — a forged signature, a misconfigured validator set, a message-verification bug, or a finality assumption that did not hold. Agents trained on single-chain security routinely treat a bridge as "two contracts and a relayer," under-modeling the trust that the bridge introduces. The bridge's security is not the security of either chain it connects; it is a separate, usually weaker, assumption layered on top.

The judgment problem is deciding, before deployment, what trust the bridge actually requires (a validator set, a multi-sig, an honest-majority assumption, a light-client proof), how that trust behaves under collusion and under chains with different finality models, what happens to the wrapped assets if the bridge is compromised or the source chain reorgs, and whether the design is a canonical bridge (one source of truth) or a fragmented one whose wrapped tokens can diverge. The harm of a casual decision is the full value the bridge custodies, taken in a single transaction, often irreversibly. Treat a bridge as what it is: a custodian whose vault is public and whose keyholders are adversarially motivated.

## Core Rules

### Identify The Bridge's Trust Model And Make The Assumption Explicit

Every bridge rests on a trust assumption, and the first task is to name it, because the assumption is where the exploits happen. The common models, in roughly increasing security:

- **Centralized custodian / multi-sig.** A set of signers holds the locked assets and authorizes releases. Security equals the threshold and the honesty of the signers; a collusion of threshold signers, or a key compromise, drains the bridge. Simple, and the most common honeypot.
- **Validator set with on-chain light client.** A rotating or fixed validator set signs messages, and the destination chain verifies a supermajority of signatures against a validator set it tracks. Stronger, but only as strong as the validator-set update logic and the stake/bond backing the validators.
- **Optimistic or fraud-proof based.** Messages are assumed valid unless challenged within a window by a watcher that posts a fraud proof. Strong against single faults but assumes an active, honest watcher and a functioning challenge game.
- **ZK or cryptographic proof.** The source chain's state or a specific message is proven cryptographically to the destination. Strongest in principle, but the proof system and the light-client implementation become the attack surface.

The trap is treating the model as a label ("it's decentralized") rather than a concrete assumption ("2/3 of this validator set must be honest, and the set updates by this governance path"). Write down the exact assumption, the threshold, who the participants are, and how the set changes — because that is what an attacker will target.

### Treat The Bridge As A Honeypot And Size The Risk Accordingly

Bridges concentrate value by design: assets locked on the source chain back the wrapped assets on the destination, so the locked pool grows until it is an attractive target. The security of the bridge must scale with the value it holds, and most do not — a multi-sig threshold that was adequate at $10M becomes a liability at $1B. The discipline: know the total value locked (TVL) the bridge can reach, and require a trust model whose cost-to-attack scales with that TVL (economic stake, slashing, fraud-proof bonds) rather than a fixed key set whose cost-to-attack is constant. If the bridge's security does not scale with its honeypot, cap the TVL, rate-limit withdrawals, or add a delay-and-challenge window so a compromise is not instantly total. A bridge whose security is static while its custody grows is an exploit waiting for the right price.

### Resolve Finality Across Chains As Weakest-Link

A bridge inherits the finality of every chain it touches, and the whole flow is only as final as the weakest link — but bridges add a second failure mode: the bridge's own confirmation logic may act on a source-chain state that is not yet final. If the source chain is probabilistic-finality (PoW-style) and the bridge releases on the destination after a few confirmations, a deep source-chain reorg leaves the bridge releasing assets against a deposit that no longer exists. The rule: wait for robust, value-appropriate finality on the source chain before minting or releasing on the destination, and understand that "robust" on a probabilistic chain means many more confirmations than on an absolute-finality chain. Many of the worst bridge losses were not cryptographic breaks but finality mismatches — the bridge trusted a source state that was not as final as assumed. Name the finality assumption on each side and verify the design waits for it (see the consensus-and-finality skill for the underlying finality reasoning).

### Distinguish Canonical And Non-Canonical Wrapped Assets

Wrapped assets (wrapped BTC, WETH on a foreign chain) are not all equivalent, and conflating them creates silent risk. A **canonical** wrapped asset is backed by a single, authoritative bridge or minting contract — there is one source of truth for the asset on that chain, and it is the asset that liquidity, integrations, and prices assume. A **non-canonical** wrapped asset is one of several competing representations, each backed by a different bridge, and they can trade at different prices, fragment liquidity, and — critically — diverge or become unredeemable if their backing bridge is compromised. The judgment: know whether the asset you are integrating is the canonical representation or a non-canonical one, who controls its mint/burn, and what happens to its value if that bridge is exploited. A non-canonical wrapped asset whose bridge is drained can go to zero while the canonical version is unaffected; treat each wrapped asset as a claim on a specific bridge, not as fungible with the underlying.

### Verify Every Cross-Chain Message, Not Just The Asset Transfer

Bridges increasingly pass arbitrary messages (governance votes, contract calls, state updates), not just token transfers, and an arbitrary-message bridge (AMB) is a far larger attack surface: a forged or replayed message can call into any contract the bridge is authorized to touch. Message verification must address replay (the same valid message being executed twice or on a different chain), ordering, and authorization (is the sender on the source chain permitted to trigger this action on the destination). Use nonces, chain IDs, and domain separators to bind a message to a specific source, destination, and execution; require explicit allow-listing of what a message can do rather than granting the bridge blanket call authority. The recurring failure is a message-verification bug (a zero-check, an uninitialized root, an unguarded function) that lets an attacker forge a message that mints or releases assets. Treat the message layer with the same rigor as the asset layer — a forged message is as good as a stolen key.

## Common Traps

### A Multi-sig Threshold That Does Not Scale With TVL

A 3-of-5 or 5-of-7 multi-sig that was reasonable at launch but becomes the single point of failure as the bridge custodies billions, because the cost to compromise the keys is fixed while the payoff grows. Require security that scales with TVL, or cap the locked value.

### Acting On Source-Chain State Before Finality

Releasing or minting on the destination based on a source-chain confirmation shallower than the source chain's real finality, so a source reorg leaves the bridge undercollateralized. Wait for value-appropriate, robust finality on the source before acting cross-chain.

### Treating Wrapped Assets As Fungible With The Underlying

Integrating a wrapped asset as if it were the native asset, ignoring that it is a claim on a specific bridge that can be compromised, de-peg, or fragment from the canonical version. Know which bridge backs each wrapped asset and what its failure mode is.

### A Validator-Set Update Logic That Is The Real Attack Surface

Focusing on the signature threshold while the validator-set membership update path (governance, a quorum vote, an admin key) is the actual vulnerability, letting an attacker swap in their own validators. Audit the set-update logic as carefully as the signing logic.

### Replayable Or Forged Arbitrary Messages

An arbitrary-message bridge that does not bind messages to a specific source/destination/nonce, or that grants the message layer blanket call authority, so a replayed or forged message mints or releases assets. Use domain separation, nonces, and explicit allow-listing of message effects.

### Assuming "Decentralized Validators" Means Uncolludable

Treating a validator set as a strong guarantee because it is "decentralized," when a small or related set of validators can collude above the threshold, or when the validators are not economically bonded/slashed. Reason about the cost-to-collude, not the validator count.

### Ignoring The Watcher Assumption In Optimistic Bridges

An optimistic or fraud-proof bridge that assumes an active, honest watcher will challenge invalid messages within the window, when no one is actually watching or the watcher is economically unaligned. Verify the watcher exists, is incentivized, and the challenge window is realistic.

## Self-Check

- [ ] The bridge's trust model is explicitly named (custodian/multi-sig, validator set with light client, optimistic/fraud-proof, or ZK), with the exact threshold, participants, and set-update logic documented — not reduced to a "decentralized" label.
- [ ] The bridge is treated as a honeypot: TVL the bridge can reach is known, and the security model's cost-to-attack scales with that TVL (economic stake, slashing, bonds), or the TVL is capped and withdrawals are rate-limited/delayed.
- [ ] Finality is resolved weakest-link: the bridge waits for robust, value-appropriate finality on the source chain before minting/releasing on the destination, accounting for whether each chain is probabilistic or absolute finality.
- [ ] Wrapped assets are distinguished as canonical vs non-canonical; each integrated wrapped asset's backing bridge, mint/burn authority, and failure mode (de-peg, drain, fragmentation) is known.
- [ ] Arbitrary messages are verified against replay and forgery: domain separation, chain IDs, nonces bind each message to a specific source/destination/execution, and message effects are explicitly allow-listed rather than blanket-authorized.
- [ ] The validator-set update logic (membership, governance path, admin keys) was audited as carefully as the signing threshold, since it is the realistic attack surface for set-based bridges.
- [ ] For optimistic/fraud-proof bridges, an incentivized, active watcher exists and the challenge window is realistic; the assumption is not made silently.
- [ ] The bridge was threat-modeled end-to-end (source lock, message/proof, destination release, set updates, watcher) and the cost-to-collude or cost-to-compromise was compared against the value the bridge custodies.
