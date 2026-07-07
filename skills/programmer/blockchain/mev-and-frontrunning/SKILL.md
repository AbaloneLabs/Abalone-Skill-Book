---
name: mev_and_frontrunning.md
description: Use when the agent is designing or reviewing a smart contract, DEX, AMM, lending protocol, NFT mint, auction, or any on-chain transaction whose outcome depends on ordering; defending against frontrunning, sandwich attacks, backrunning, or transaction reordering; exposing or protecting transactions in the mempool; choosing slippage limits, commit-reveal schemes, or private order flow; integrating with Flashbots, MEV-Boost, a searcher, or a private relay; reasoning about maximal extractable value (MEV), the searcher-builder-proposer supply chain, priority fees, or transaction ordering as a source of extractable value; designing an auction or fair-launch mechanism resistant to extraction; or assessing how much value a protocol leaks to MEV and how to return it to users. Covers sandwich attacks, frontrunning, backrunning, just-in-time liquidity, private order flow, commit-reveal, the PBS supply chain, and the tension between transparent mempools and fair transaction ordering.
---

# MEV And Frontrunning

On a public blockchain, transactions are visible before they execute, and the order in which they execute is chosen by whoever builds the block — which means transaction ordering itself is a tradable, extractable resource. Maximal extractable value (MEV) is the value that can be captured by reordering, inserting, or censoring transactions within a block, and it is not an edge case: it is a persistent, sophisticated market in which specialized actors (searchers) compete to extract value from ordinary users' transactions, often at the users' expense. Agents trained on ordinary systems routinely treat transaction ordering as neutral or irrelevant — "the transaction will execute" — when in fact a user's swap can be sandwiched (front-run to push the price up, then back-run to sell at the inflated price), their NFT mint can be front-run, or their liquidation can be stolen by a bot that pays for priority. The result is silent, ongoing value extraction from users who never see the mechanism, only the worse price.

The judgment problem is deciding, for each on-chain action, how much its outcome depends on ordering, how exposed it is to the mempool, and what defense — slippage limits, commit-reveal, private order flow, a fair-ordering mechanism, or accepting and redirecting the MEV — fits the threat model and the user. The harm of ignoring MEV is a protocol that quietly leaks its users' value to extractors, or a mechanism (an auction, a mint, a liquidation) whose fairness is illusory because the first profitable transaction wins. The harm of over-defending is unusable latency or a false sense of protection. MEV is not a bug to be fixed; it is a structural property of public, ordered execution, and the design question is how to live with it honestly.

## Core Rules

### Identify Whether The Action's Outcome Depends On Ordering

Not every transaction is MEV-relevant, and the first judgment is whether ordering matters for this action. A token transfer to a fixed address does not; a swap on an AMM does, because its price depends on pool state that a front-runner can move. A liquidation does, because the first liquidator captures the reward. An NFT mint with a fixed price does not, until it sells out, at which point ordering determines who gets in. Ask, for each user-facing action: does the price, allocation, or reward depend on chain state that other transactions can change between submission and execution? If yes, the action is MEV-exposed and needs a defense. The recurring error is treating a swap or auction as if it executes in isolation, when an extractor will read the pending transaction and act on it first.

### Understand The MEV Supply Chain And Where Value Flows

MEV extraction is now an industrial supply chain, and understanding it determines which defenses work. On Ethereum and similar chains, the pipeline is: a user submits a transaction to the public mempool; a **searcher** detects extractable value and submits a bundled transaction (often the user's tx plus the extractor's front- and back-runs) to a **builder**; the builder assembles the most profitable block and submits it to the **proposer** (validator) via a relay such as MEV-Boost; the proposer picks the highest-bid block. The consequence: the public mempool is observed by adversaries in real time, and value flows from users to searchers, with a portion rebated to builders and proposers via priority fees. Defenses that assume a neutral mempool fail; the relevant levers are avoiding the public mempool (private order flow), making extraction unprofitable (slippage limits, commit-reveal), or redirecting captured MEV back to users (e.g., MEV-aware routers, auction-based order flow). Know which part of the pipeline your transaction enters and who can act on it.

### Set Slippage And Price-Impact Limits Deliberately

The most common and effective defense against sandwich attacks is a slippage limit: the user specifies the minimum acceptable output for a swap, and if front-running has moved the price past it, the transaction reverts. The judgment is the tradeoff: too tight a limit and the transaction reverts on ordinary price movement (and the user still pays gas); too loose and the extractor takes the difference. Set the limit based on realistic expected slippage for the trade size and pool depth, not a default. Beyond per-trade limits, reason about price impact: a large swap in a shallow pool moves the price enough to be self-sandwiching even without an attacker. Cap trade sizes relative to liquidity, or split large trades, to avoid creating the very opportunity extractors exploit. Slippage limits are necessary but not sufficient — they cap the loss per trade; they do not prevent the extraction attempt.

### Use Commit-Reveal Or Private Order Flow When Ordering Fairness Matters

When the outcome must be fair regardless of ordering — an auction, a mint with scarcity, a governance vote where timing matters, or any case where front-running breaks the mechanism — slippage limits are not enough, because the issue is not price movement but who acts first. Two structural defenses:

- **Commit-reveal.** Users first commit a hash of their bid/action (commit phase), then reveal it after the commit window closes (reveal phase), so no one can front-run a bid they cannot see. The cost is latency (two transactions) and the need to penalize non-revealers (a bond) or the scheme is gameable. Use it where fairness is the requirement and the latency is acceptable.
- **Private order flow.** Submit the transaction through a private relay (Flashbots Protect, a trusted builder) that does not broadcast to the public mempool, so searchers cannot see and front-run it. The cost is a new trust assumption (the relay could censor or observe) and reduced decentralization of the order flow. Use it to avoid mempool exposure for sensitive transactions.

Choose based on the threat: commit-reveal for fairness guarantees; private order flow to hide from extractors; both when the stakes warrant. Understand that neither makes MEV disappear — they move where it can be captured and by whom.

### Quantify And Decide What To Do With The MEV Your Protocol Creates

A protocol that routes volume through an AMM, runs liquidations, or holds auctions is generating MEV, and the design choice is whether that value goes to anonymous extractors, to the protocol, or back to users. Quantify the leakage (how much value is extractable from typical user flows), then decide: accept it (documented cost to users), capture it for the protocol (e.g., an auction for liquidation rights, a router that captures its own arbitrage), or return it to users (e.g., MEV-aware routers that rebate captured value). Ignoring the question is the default failure — the value is extracted silently and the protocol's users pay it without knowing. The honest framing: MEV is a tax on your users' transactions; the design decision is who collects it and whether it is disclosed.

## Common Traps

### Treating Transaction Ordering As Neutral

Assuming a transaction executes as submitted, in isolation, when extractors read the pending transaction and reorder, front-run, or sandwich it. Identify which actions are ordering-dependent and defend them.

### A Loose Or Default Slippage Limit That Invites Sandwiching

Accepting a protocol's default slippage (often high) so a sandwich attack stays under the limit and the user silently pays the spread. Set slippage based on realistic trade size and pool depth, not a default.

### Exposing A Sensitive Transaction To The Public Mempool

Broadcasting an auction bid, a liquidation, or a large swap to the public mempool where searchers see and act on it instantly. Use commit-reveal or private order flow for ordering-sensitive actions.

### A Fair-Launch Mechanism That Is Decided By Gas Price

A mint, sale, or auction where the winner is whoever pays the most gas, so the mechanism rewards extractors and bots rather than the intended participants. Use commit-reveal, allow-lists, or fair-ordering where the intended fairness is not "highest gas wins."

### Assuming Private Order Flow Eliminates MEV

Treating a private relay as a complete solution, when it moves the trust assumption (the relay can observe or censor) and does not address MEV extractable by the relay or its builders. Use it to avoid public-mempool exposure, and understand the new assumption it introduces.

### Ignoring Self-Sandwiching From Large Trades In Shallow Pools

A swap so large relative to pool liquidity that it moves the price against the user even without an attacker, creating the same loss a sandwich would. Cap trade size relative to liquidity or split large trades.

### Leaking MEV To Anonymous Extractors Without Disclosing It

A protocol whose user flows are routinely extracted, with no quantification, capture, or disclosure, so users pay an invisible tax. Quantify the leakage and decide whether to accept, capture, or return it.

## Self-Check

- [ ] Each user-facing action was assessed for ordering dependence: actions whose price, allocation, or reward depends on chain state other transactions can change are identified as MEV-exposed and defended.
- [ ] The MEV supply chain (mempool, searcher, builder, proposer, relay) is understood, and it is known which part of the pipeline each transaction enters and who can act on it.
- [ ] Slippage and price-impact limits are set deliberately based on realistic trade size and pool depth — not defaults — and large trades are capped or split to avoid self-sandwiching.
- [ ] Ordering-fairness-critical actions (auctions, mints, governance) use commit-reveal, allow-lists, or fair-ordering rather than being decided by gas-price priority.
- [ ] Sensitive transactions use private order flow where public-mempool exposure would enable frontrunning, and the trust assumption the relay introduces (observation, censorship) is acknowledged.
- [ ] The MEV the protocol generates was quantified, and a deliberate decision was made to accept, capture for the protocol, or return to users — rather than silently leaking it to anonymous extractors.
- [ ] Defenses were chosen to match the threat (slippage for price movement, commit-reveal for fairness, private order flow for mempool exposure) and the tradeoffs (latency, trust assumptions, false protection) are understood.
- [ ] The protocol's MEV exposure and the chosen defenses are documented for users, so the cost of extraction is disclosed rather than invisible.
