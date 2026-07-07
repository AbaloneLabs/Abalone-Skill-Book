---
name: token_design_and_economics.md
description: Use when the agent is designing a token, writing or reviewing a token contract (ERC-20, ERC-721, ERC-1155, SPL, CW20, Move), deciding supply mechanics, mint and burn policy, vesting and unlock schedules, airdrop or distribution allocation, governance rights attached to a token, fee distribution, staking yields, rebasing or deflationary mechanics, or a tokenomics model; classifying a token as utility, governance, or security and assessing regulatory risk; evaluating incentive alignment, inflation/deflation dynamics, or concentration and fairness of a distribution; advising on a DAO treasury, token launch, TGE, liquidity bootstrapping, or a vesting cliff; or reviewing whether a token's mechanics survive adversarial conditions such as governance capture, whale dumping, or a bank run on a redeemable token. Covers tokenomics, token engineering, vesting cliffs, mint authority risk, governance token capture, Howey/security-token classification, and the tension between incentive alignment and extractability.
---

# Token Design And Economics

A token is a coordination mechanism enforced by code: it encodes who gets what, when they can take it, and what the rules let them do to each other. Unlike ordinary software, the rules are adversarial and financial — every participant is incentivized to find the strategy the token rewards, even if that strategy is dumping, farming-and-exiting, capturing governance, or front-running the unlock schedule. Agents trained on feature design routinely treat a token as "an ERC-20 plus a number," choosing a supply and a distribution because they look reasonable, without modeling how the mechanics behave under self-interest, concentration, time-locked unlocks, or regulatory scrutiny. The result is a token whose economics collapse on a predictable schedule, whose governance is captured by the largest holder, or whose distribution reads as an unregistered securities offering.

The judgment problem is deciding, before launch, how the token's mechanics align incentives across holders who do not share the team's goals, how supply and unlock schedules behave over time, what rights the token grants and what those rights expose the project to, and whether the design crosses from a utility or governance instrument into something a regulator will call a security. The harm of a casual decision is measured in market cap, legal exposure, and the project's survival: a vesting cliff that triggers a coordinated dump, a mint authority that becomes a single point of trust failure, or a "utility" token whose promised yield is, in the regulator's view, a dividend on a common enterprise. Token design is not graphic design with a number; it is mechanism design with money and law attached.

## Core Rules

### Model The Token As An Adversarial Game, Not A Feature List

Every token mechanic is a rule in a game played by self-interested actors, and the equilibrium of that game is what actually happens — not the team's intent. Before choosing a mechanic, ask what strategy it rewards and whether that strategy is the one you want. A governance token with no quorum rewards apathy and lets a minority capture votes; a staking yield funded by inflation rewards early stakers at the expense of late holders; a fee-share token rewards holding only until someone extracts the yield and exits. Model the flows: who receives tokens, who is dilutive to whom, what happens at each unlock, what happens if the largest holder acts against the project. The discipline is to assume holders will optimize for themselves and to verify the token's intended behavior survives that assumption. If the mechanic only works when everyone cooperates, it does not work.

### Make Supply, Mint, And Burn Policy Explicit And Trust-Bounded

Supply mechanics are the core economic lever, and vagueness here is the most common failure. Decide and document: is supply fixed, mintable, or deflationary; who can mint and under what cap; what burns tokens and whether burns are real (sent to a dead address) or accounting tricks; whether there is a max supply and whether it is enforceable. A mint authority is a centralized trust point — whoever holds it can dilute every holder at will — so for any non-trivial token, bound the mint cap, time-lock it, place it behind a multisig or governance, or remove it entirely. The recurring disaster is a "fixed supply" token whose contract still has a mint function with an unbounded or admin-controlled cap, which is either an oversight or a backdoor. Make the supply policy legible, enforce it in code, and treat the mint key as the most sensitive credential in the project.

### Design Vesting And Distribution For The Time Horizon, Not The Launch

Token distributions fail on a schedule, and the schedule is set at launch. Vesting (linear or cliff-plus-linear unlocks for team, investors, advisors) exists to align recipients with the project's long-term success, but a poorly designed schedule creates predictable sell pressure the moment unlocks begin — often a cliff at 12 months that becomes a coordinated dump. Design for the actual time horizon: long vesting with gradual unlocks, cliffs matched to product milestones rather than calendar dates, and an awareness of the aggregate unlock calendar across all recipient classes. Examine distribution concentration: if a small number of addresses (team, a single investor, an exchange) hold enough to move the market, the token's decentralization claim is cosmetic and the token is exposed to a single actor's exit. Fairness and survival are not just about total allocation percentages; they are about who can sell what, when, and how much damage that does.

### Classify The Token Honestly And Assess Regulatory Exposure

Token classification is not a marketing decision; it is a legal determination that varies by jurisdiction and carries enforcement consequences. The trap is designing a token that functions as a security (an investment of money in a common enterprise with expectation of profit derived from the efforts of others, per the Howey test and analogous frameworks) while labeling it a "utility" or "governance" token to avoid registration. The factors regulators examine are concrete: is profit promised or implied, is value derived from the team's ongoing efforts, is there a common enterprise, is the token purchased for consumption or speculation. A token whose primary pitch is price appreciation, yield, or revenue share is at high risk of being deemed a security regardless of its label. The judgment: design the token's actual rights and economics first, then assess the classification honestly, and involve qualified legal counsel before launch — not after an enforcement letter. Do not let the token's mechanics imply a profit expectation the project cannot legally sustain.

### Scope Governance Rights To What The Token Can Legitimately Decide

Attaching governance rights to a token is a common way to claim decentralization, but it creates two risks the team often ignores: governance capture and governance theater. Capture occurs when voting power concentrates (whales, exchanges holding custodied tokens, borrowable governance tokens) and a minority controls outcomes; the defense is quorum requirements, delegation, time-locked voting, and sometimes vote weighting that resists plutocracy. Governance theater occurs when the token can vote but the vote is non-binding or the team retains effective control — which both undermines the decentralization claim and, in some regulatory views, confirms the team is the "efforts of others" on which profit depends. Decide what the token actually governs (parameter changes, treasury allocation, protocol upgrades) and whether that control is real and enforceable. A governance token that governs nothing is a marketing token; one that governs everything is a target for capture.

## Common Traps

### A "Fixed Supply" Contract That Still Has An Unbounded Mint

Leaving a mint function in the contract with a high or admin-set cap, then marketing the token as fixed supply. Holders price it as scarce; the admin can dilute them. Either remove mint capability, bound it with a hard cap, or disclose it — the mismatch between the claim and the code is the trap.

### A Vesting Cliff That Becomes A Coordinated Dump

Designing a large cliff unlock (e.g., 12 months) for team and investors without modeling the sell pressure when it hits, so the token's price collapses on a predictable date. Spread unlocks, align cliffs to milestones, and model the aggregate unlock calendar before launch.

### Yield Funded By Inflation Presented As "Staking Rewards"

Offering staking yields paid in newly minted tokens so the nominal APY looks attractive, while every non-staking holder is silently diluted. The yield is real only until inflation outpaces value accrual; design yields from real revenue or fee share, not from a money printer.

### A "Utility" Token Whose Pitch Is Price Appreciation

Marketing a token as utility or governance while the entire value proposition is that its price will rise due to the team's efforts — which is the core of a security offering. The label does not control the classification; the economics and promises do. Assess honestly and get legal review.

### Governance Capture By The Largest Holder Or An Exchange

A 1-token-1-vote system where a single whale, a large investor, or an exchange voting custodied tokens can pass any proposal, making "decentralized governance" a facade. Add quorum, delegation, vote delays, and concentration-resistant mechanisms where capture is a real risk.

### Concentrated Distribution Disguised As Decentralization

Allocating a large share to team and a few investors while claiming broad distribution, so a handful of addresses can crash the market on unlock. Examine the actual concentration (including related addresses) and design distribution to survive the largest holders exiting.

## Self-Check

- [ ] The token's mechanics were modeled as an adversarial game: the strategy each rule rewards was identified, and the intended behavior survives the assumption that holders optimize for themselves.
- [ ] Supply policy is explicit and enforceable in code: fixed/mintable/burning is documented, any mint authority is bounded by a hard cap and time-lock/multisig/governance, and there is no hidden or unbounded mint masquerading as a fixed supply.
- [ ] Vesting and unlock schedules are designed for the project's time horizon: cliffs align to milestones, unlocks are gradual, and the aggregate unlock calendar across all recipient classes was modeled for sell pressure.
- [ ] Distribution concentration was examined (team, investors, exchanges, related addresses), and no small set of holders can unilaterally crash the market or capture governance.
- [ ] The token's classification (utility, governance, security) was assessed against its actual economics and promises — not its label — and qualified legal counsel reviewed it before launch where profit expectation or common-enterprise factors are present.
- [ ] Governance rights are scoped to real, enforceable decisions, and capture resistance (quorum, delegation, vote delays, concentration-aware weighting) is in place where voting power could concentrate.
- [ ] Any yield, fee share, or revenue distribution is funded by real protocol revenue, not by inflation that silently dilutes non-participating holders.
- [ ] The token's design was stress-tested against adversarial conditions (a coordinated dump at unlock, governance capture, a bank run on a redeemable token, a whale exit) and the intended behavior survives or degrades gracefully.
