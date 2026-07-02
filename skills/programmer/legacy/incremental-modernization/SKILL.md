---
name: incremental_modernization.md
description: Use when the agent is planning or executing the incremental modernization of a legacy system, deciding what to replace first and what to leave, applying the strangler pattern or parallel-run migration, mitigating the risk of replacing a system that is imperfect but working, deciding when to cut over and when to retire the old system, or proving the value of modernization before committing to the full effort. Also covers the failure mode of a big-bang rewrite that underestimates accumulated behavior, a parallel run that never converges, modernization that delivers no user value until the end, and the sunk-cost trap of keeping a system alive past its useful life.
---

# Incremental Modernization

Incremental modernization is the work of replacing a legacy system piece by piece while keeping the business running, as opposed to a big-bang rewrite that freezes the world, builds a replacement, and cuts over in a single risky event. The judgment problem is that the legacy system, however ugly, encodes years of accumulated behavior — edge cases, workarounds, compatibility, hard-won correctness — that a rewrite will almost certainly underestimate, and the big-bang approach bets the business on the rewrite rediscovering all of that behavior before money or patience runs out. Incremental modernization, done well, delivers value continuously (each replaced piece is a win), keeps the system working throughout (no flag day), and lets you stop or change direction when you learn that the full rewrite is not worth it. Done poorly, it becomes a permanent parallel run that never converges, with two systems to maintain and the old one never actually retired.

Agents tend to underestimate modernization because the legacy system looks simpler than it is. The harm appears as the rewrite that runs years late (because each "simple" piece turns out to encode a decade of edge cases), as the parallel run that never converges (because the new system never quite matches the old, and no one has the authority to declare "close enough"), and as modernization that delivers no value until the end (so it is the first thing cut when budgets tighten). The discipline is to sequence replacement by value and risk, to use the strangler pattern so the old system keeps working while the new one grows, to prove value early so the effort earns its continued funding, to define the cutover and retirement criteria in advance, and to retire the old system deliberately rather than letting it linger as a permanent second codebase. Modernization is not finished when the new system is built; it is finished when the old system is gone.

## Core Rules

### Sequence Replacement By Value And Risk

Not all parts of a legacy system are equally worth replacing, and the order matters. Sequence replacement so that the highest-value, lowest-risk pieces go first — this delivers value early, builds confidence, and earns the credibility to tackle harder pieces later. Replacing the hardest, riskiest piece first is a common failure: it consumes the budget before any value is shown, and if it goes wrong, the whole effort is discredited.

- **Prioritize by value (what blocks users or developers most) crossed with risk (how dangerous to change).** High-value, low-risk pieces are the first targets; high-value, high-risk pieces come later once the team has built skill and safety nets; low-value pieces may never be worth replacing.
- **Let early wins fund later work.** A successful early replacement that delivers visible value makes it easier to justify the next, harder piece.
- **Avoid the temptation to start with the hardest problem.** The hardest module is often the one with the most accumulated behavior; tackling it first, before the team understands the system and has safety nets, maximizes risk.

### Use The Strangler Pattern So The System Keeps Working

The strangler pattern replaces a legacy system incrementally: the new system is built alongside the old, traffic is routed to it piece by piece, and the old system is retired gradually as its responsibilities shrink. Its virtue is that the system keeps working throughout — there is no flag day — and each increment is independently shippable and reversible.

- **Build the new alongside the old, not instead of it.** The old system keeps serving while the new one grows; a routing layer (facade, proxy, feature flag) directs traffic to whichever system handles a given responsibility.
- **Route incrementally and reversibly.** Move one capability, one route, or one cohort of users at a time, behind a flag so you can revert in minutes if the new path misbehaves.
- **Grow the new system's coverage until it handles everything, then retire the old.** The end state is the old system gone, not two systems running forever.

### Prove Value Early To Earn Continued Funding

Modernization efforts that deliver no value until the end are the first to be cut when budgets tighten or priorities shift. Proving value early — a replaced piece that ships, that users or developers feel, that reduces cost or bugs or toil — earns the credibility and funding to continue. An effort that is "still building" after a year with nothing shipped is politically and financially fragile.

- **Ship the first replacement quickly, even if it is small.** A small piece replaced and shipped proves the approach works and that the team can deliver.
- **Measure and communicate the value of each increment** (bugs fixed, latency reduced, toil eliminated, features unblocked). Modernization is easier to sustain when its value is visible.
- **Let the early increments validate (or invalidate) the cost estimate.** If the first piece took three times longer than estimated, revise the plan before committing to the whole.

### Define Cutover And Retirement Criteria In Advance

A common failure is the parallel run that never converges: the new system is "almost" matching the old, but no one has defined what "close enough" means, so the cutover never happens and two systems run forever. Define the cutover criteria (what must match, within what tolerance) and the retirement plan (when the old system is turned off) in advance, so the decision is made on data rather than deferred indefinitely.

- **Define functional equivalence criteria with tolerances.** The new system must produce the same outputs as the old, within an agreed tolerance; perfect parity is usually impossible (and unnecessary), so agree in advance what divergence is acceptable.
- **Define the retirement plan and date.** The old system will be turned off on a date or when the criteria are met; without a plan, the old system lingers indefinitely as a permanent maintenance burden.
- **Make the cutover reversible, then make it permanent.** Cut over behind a flag, observe, and once stable, remove the flag and retire the old path so there is no path back.

### Mitigate The Risk Of Replacing A Working System

The legacy system is imperfect, but it is working, and its replacement carries risk. Mitigate that risk by running the new and old in parallel during the transition, by shadowing traffic (sending it to both and comparing outputs), and by cutting over incrementally so a problem in the new system affects only a slice of traffic.

- **Shadow traffic before cutting over.** Send production requests to both systems and compare outputs; divergence reveals where the new system has not yet matched the old before users are affected.
- **Cut over a small cohort first, then expand.** Move 1% of traffic, then 5%, then 100%, reverting if the new path misbehaves; never cut over all traffic at once.
- **Have a rollback path at every step.** Each increment must be reversible in minutes, not hours; a flag-day cutover with no rollback is a bet-the-business risk.

### Recognize The Sunk-Cost Trap In Both Directions

Two sunk-cost traps oppose each other in modernization. The first is keeping the legacy system alive past its useful life because of the effort already invested in it. The second is continuing a failing rewrite because of the effort already invested in the rewrite. The judgment is to evaluate each increment on its expected value going forward, not on what has been spent, and to be willing to stop a modernization that is not paying off — or to retire a legacy system whose replacement is ready — regardless of sunk cost.

## Common Traps

### The Big-Bang Rewrite

Replacing the legacy system wholesale in a single risky event, underestimating the accumulated behavior the old system encodes, running years late, and betting the business on a cutover that may fail. Use the strangler pattern; keep the system working throughout.

### Starting With The Hardest Piece

Tackling the highest-risk, most-behavior-laden module first, consuming the budget before any value is shown and discrediting the effort if it goes wrong. Sequence by value crossed with risk; let early wins fund later hard work.

### The Parallel Run That Never Converges

Running old and new in parallel indefinitely because no one defined what "close enough" means or when the old system will be retired. Define cutover criteria with tolerances and a retirement plan in advance.

### No Value Until The End

A modernization that ships nothing until the full replacement is done, making it fragile to budget cuts and priority shifts. Ship the first replacement quickly, measure and communicate value, and let early increments validate the cost estimate.

### Flag-Day Cutover With No Rollback

Cutting over all traffic at once with no way back, so a problem in the new system is a bet-the-business incident. Cut over incrementally behind a flag, shadow traffic first, and keep a rollback path at every step.

### Keeping Two Systems Forever

Building the new system alongside the old but never retiring the old, leaving two codebases to maintain and the modernization never truly finished. Modernization is done when the old system is gone; define and execute the retirement.

### Sunk-Cost In Either Direction

Continuing a failing rewrite because of effort already spent, or keeping a legacy system alive past its useful life for the same reason. Evaluate each increment on expected value going forward and be willing to stop or retire based on that.

## Self-Check

- [ ] Replacement is sequenced by value crossed with risk (high-value low-risk first, high-value high-risk later once skill and safety nets exist, low-value pieces may never be replaced), and early wins are used to fund later harder work rather than starting with the hardest problem.
- [ ] The strangler pattern is used: the new system is built alongside the old, traffic is routed incrementally and reversibly via a routing layer/flag, and the end state is the old system retired — not two systems running forever.
- [ ] Value is proven early: the first replacement ships quickly (even if small), its value is measured and communicated, and the early increments validate or revise the cost estimate before the full effort is committed.
- [ ] Cutover and retirement criteria are defined in advance (functional equivalence with agreed tolerances, a retirement plan and date), so the parallel run converges rather than lingering indefinitely.
- [ ] Risk is mitigated throughout: traffic is shadowed and outputs compared before cutover, traffic is moved incrementally (1% then 5% then 100%) with revert in minutes, and a rollback path exists at every step rather than a flag-day cutover.
- [ ] The old system is retired deliberately once the new handles its responsibilities, with the flag removed and the old path taken away — modernization is treated as finished when the old system is gone, not when the new one is built.
- [ ] Sunk cost is evaluated in both directions: each increment is judged on expected value going forward, and the team is willing to stop a non-paying-off modernization or retire a legacy system whose replacement is ready regardless of past investment.
- [ ] The highest-risk cases were verified — a piece whose accumulated behavior was underestimated, a parallel run that converged rather than lingered, a cutover that was incremental and reversible, and a retirement that actually happened — not only the clean greenfield-style path.
