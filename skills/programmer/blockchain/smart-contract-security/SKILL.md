---
name: smart_contract_security.md
description: Use when the agent is writing, auditing, or reviewing a smart contract in Solidity, Vyper, Rust/Soroban, Move, or another blockchain language; designing access control, upgrade patterns, or token mechanics; defending against reentrancy, integer overflow/underflow, front-running, or access-control bypass; deciding on an upgradeable proxy pattern; reasoning about immutability and the consequences of post-deployment un-fixability; ensuring auditability; or reviewing a contract for a mainnet deployment. Covers reentrancy, overflow, unchecked external calls, privilege escalation, the illusion of immutability, upgrade-pattern risks, and the irreversible-cost reality of contract bugs.
---

# Smart Contract Security

A smart contract is money whose behavior is enforced by a public, adversarial network, and whose bugs are publicly and irreversibly exploitable. This combination — financial value, public source code, active adversaries, and no ability to push a patch that rewrites deployed state — makes contract security unlike ordinary software security. A bug in a web service can be fixed tonight; a bug in a deployed contract can be drained before the team finishes reading the alert, and the only "fix" may be a migration that abandons the compromised contract. Agents trained on ordinary software routinely apply web-security intuitions that fail here: assuming you can patch after launch, assuming callers are well-behaved, assuming a failed transaction reverts all observable effects, assuming external calls are inert.

The judgment problem is deciding, before deployment, how the contract can be attacked from the outside, what invariants must hold under adversarial input, how privileges are bounded, and what the upgrade and recovery story is when something is wrong. The harm of a casual decision is measured in the contract's total value, often within hours of the mistake becoming visible. Treat contract code as if every line will be read by a motivated attacker with a financial incentive to break it — because it will.

## Core Rules

### Treat Every External Call As A Potential Reentrancy Vector

Reentrancy is the canonical contract vulnerability: an external call to an untrusted contract hands control to code you do not control, and that code can call back into your contract before your function has finished updating state. The famous pattern is a withdrawal function that sends ether *before* zeroing the balance, so the recipient's fallback re-enters and withdraws again against the still-non-zero balance. The defense is the Checks-Effects-Interactions pattern: perform all checks, then write all state changes (effects), and only then make the external call (interaction) last. Even better, use a reentrancy guard (a mutex lock) on functions that make external calls, and prefer pull-over-push patterns where the contract does not initiate transfers to arbitrary addresses. The deeper lesson: any external call is a suspension of your function's control flow to adversary-controlled code. Assume the call can call back, can revert, can consume gas, and can observe your partially-updated state.

### Bound Integer Arithmetic And Do Not Trust Defaults

Integer overflow and underflow produce silent, catastrophic results: a balance that wraps from 0 to the maximum uint, a calculation that silently produces a huge number. Modern Solidity (0.8+) reverts on overflow by default, but older patterns, assembly blocks, and unchecked blocks reintroduce the hazard — and other languages vary. The discipline: never rely on "the language handles it" without confirming the version and the specific operation; use SafeMath or built-in checked arithmetic explicitly where the default is unchecked; and reason about the *range* of every arithmetic result under adversarial input, not just the happy path. A token transfer that underflows on a zero-balance check, or a calculation that overflows when an attacker supplies a crafted amount, is a funds-loss bug. Bound inputs at the entry point and assert invariants that arithmetic cannot violate.

### Make Access Control Explicit, Minimal, And Auditable

Every privileged function is an attack surface, and every privilege grant is a trust assumption. Define roles explicitly (owner, admin, pauser, minter) and scope each to the minimum capability. Avoid a single all-powerful owner where narrower roles suffice; avoid hidden privileges (a function that looks public but is gated by a modifier buried in inheritance). Prefer multisig or DAO governance for high-value privileges so no single key is a single point of failure or compromise. Document every privileged function and who can call it; an unaudited privileged function is a privilege-escalation waiting to be found. The recurring failure is a function intended to be admin-only that is actually callable by anyone, or an owner key held by a single person on a hot wallet.

### Decide The Upgrade Story Deliberately, And Know Its Costs

Immutability is the default and the safety net: a deployed contract cannot be changed, which means an attacker cannot change it either. Upgradeability (proxy patterns) trades that safety for the ability to fix bugs and add features — and introduces a new attack surface: whoever controls the upgrade can change the contract's behavior, including maliciously. The decision and its tradeoffs:

- **Immutable.** Safest against malicious modification; bugs are unfixable. Suitable for simple, well-audited contracts where the logic will not need to change.
- **Upgradeable proxy (UUPS, transparent, beacon).** Allows logic upgrades while preserving storage; the upgrade authority is now the contract's central trust and risk point. Requires careful storage-layout compatibility (never change existing storage slot order or types), and the upgrade mechanism itself is a frequent bug source.
- **Social migration / new deployment.** Keep contracts immutable but plan to deploy new versions and migrate users/state. More work, but avoids the proxy risk.

The common error is adopting upgradeability reflexively "so we can fix bugs," then introducing a proxy vulnerability or a storage-collision bug that is worse than the original risk. If you upgrade, the upgrade path itself must be audited as carefully as the core logic, and storage layout compatibility must be enforced mechanically (tests, layout diffs).

### Reason About The Full Transaction, Not Just Your Function

A transaction is not a single function call; it is a sequence of operations whose observable effects depend on the chain state at execution, the calldata, the other contracts invoked, and the gas available. Adversaries choose all of these. Reason about your contract under adversarial composition: what if the caller is a contract that re-enters, what if the token being transferred is malicious (fee-on-transfer, rebasing, pausable), what if the oracle returns a manipulated value, what if the transaction is front-run or sandwiched. A function that is safe when called by an EOA in isolation may be exploitable when called as part of a crafted multi-contract transaction. Threat-model the transaction, not the function.

### Ensure Auditability And Plan For Review

Contract code that will hold value must be reviewed by someone other than the author, ideally professional auditors, before mainnet deployment. Design for auditability: keep logic simple and minimal (every line is attack surface), avoid clever optimizations that obscure intent, document invariants and assumptions, and write tests that cover the adversarial paths, not just the happy path. Maintain a clear trail of what was reviewed and when. A contract deployed without review is a bet that the author found every bug — a bet that has been lost publicly many times. Treat audit as a prerequisite for mainnet, not an optional polish step.

### Respect The Irreversibility Of Deployment

Deployment to mainnet is, for practical purposes, irreversible. State cannot be silently rewritten, a deployed bytecode cannot be swapped, and funds taken by an exploit are gone. This inverts the usual software risk calculus: a bug that would be a "patch tonight" elsewhere is a catastrophe here. The disciplines that follow: test on testnet under realistic conditions, verify bytecode matches source, use formal verification or invariant testing for high-value logic, stage deployments, and have a documented incident-response plan (pause, migrate, recover) before you need it. The mindset is closer to aerospace than to web development: the cost of a mistake is borne after launch, with no rollback.

## Common Traps

### Sending Before Updating State

Making an external transfer before zeroing a balance or setting a flag, enabling reentrancy. Always apply effects before interactions; add a reentrancy guard as defense in depth.

### Trusting A Token To Behave Like A Vanilla ERC-20

Assuming a `transfer` moves exactly the specified amount and never fails, when fee-on-transfer, deflationary, or pausable tokens behave differently. Account for actual received amounts, not requested amounts, when integrating arbitrary tokens.

### An "Admin-Only" Function That Is Actually Public

A function intended to be restricted but missing its modifier, or inheriting a visibility that exposes it. Audit every function's visibility and access; an unintended public function is a direct exploit path.

### Reflexive Upgradeability With A Weak Proxy

Adding a proxy "to be safe" and introducing a storage collision or an upgrade-authority bug that is worse than the immutability it replaced. If upgrading, audit the proxy and enforce storage-layout compatibility mechanically.

### Overflow In Unchecked Or Assembly Blocks

Using `unchecked` or assembly to save gas and reintroducing overflow, or relying on language defaults without confirming the version. Verify arithmetic bounds on adversarial input explicitly.

### Assuming An Oracle Is Trustworthy

Reading a price or state from an oracle without checking for staleness, manipulation, or deviation across sources. Oracles are a frequent exploit vector; use decentralized oracles, staleness checks, and circuit breakers.

### Front-Running And Sandwich Exposure

Exposing a swap or operation to mempool observation so an attacker can front-run and back-run (sandwich) it for profit at the user's expense. Use commit-reveal, slippage limits, or private order flow where applicable.

### Deploying Without External Review

Shipping to mainnet based on author self-review, then discovering a bug that an auditor would have caught. Independent review is a prerequisite for value-bearing contracts, not a luxury.

## Self-Check

- [ ] Every external call is identified and ordered by Checks-Effects-Interactions; reentrancy guards protect functions that interact with untrusted code; pull-over-push is preferred for transfers to arbitrary addresses.
- [ ] All arithmetic is bounded and uses checked operations (or documented SafeMath) on every path, including unchecked and assembly blocks; input ranges are validated at entry points.
- [ ] Access control is explicit and minimal: every privileged function names its role, privileges are scoped to minimum capability, high-value roles use multisig/governance, and no function is unintentionally public.
- [ ] The upgrade story (immutable, proxy, or migration) was chosen deliberately; if upgradeable, the proxy is audited, storage-layout compatibility is enforced mechanically, and the upgrade authority's risk is documented.
- [ ] The contract was threat-modeled at the transaction level: reentrant callers, malicious tokens, manipulated oracles, and front-running/sandwich attacks were considered and defended.
- [ ] The contract is auditable: logic is minimal and clear, invariants and assumptions are documented, and tests cover adversarial paths (reentrancy, overflow, access bypass, oracle manipulation) not just the happy path.
- [ ] Independent review (professional audit or qualified second party) was completed before mainnet; bytecode was verified against reviewed source.
- [ ] Deployment discipline was followed: testnet testing under realistic conditions, staged rollout, and a documented incident-response plan (pause/migrate/recover) exists before launch.
- [ ] The irreversibility of deployment was respected: the cost of a post-launch bug was weighed, and high-value logic uses invariant tests or formal verification where feasible.
