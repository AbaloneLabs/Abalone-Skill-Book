---
name: slo_and_error_budget.md
description: Use when the agent is defining Service Level Objectives, selecting Service Level Indicators, deciding how to consume an error budget, setting burn-rate alert thresholds, choosing multi-window alerting, or balancing feature velocity against reliability. Also covers the failure mode of SLOs that measure infrastructure uptime rather than user impact, error budgets that are either hoarded (stopping all shipping) or ignored (allowing unbounded unreliability), burn-rate alerts that page on noise or miss real user impact, and the political misuse of SLOs as SLA contract enforcement rather than engineering decision tools.
---

# SLO And Error Budget

An SLO (Service Level Objective) is a measurable reliability target — what fraction of user requests must succeed, at what latency, over what window — and an error budget is the complement: the amount of unreliability you are allowed (1 minus the SLO) before users are meaningfully harmed. Together they convert "how reliable should we be" from a vague aspiration into a quantified engineering decision: as long as you are within budget, ship features; when you burn budget too fast, stop shipping and invest in reliability. The judgment problem is that the SLO is only as good as the indicator it measures, and the easy indicators (CPU, node count, uptime) do not capture user impact. A service whose servers are up but whose p99 latency is ten seconds is "available" by a naive SLI and broken by the user's experience. The discipline is to define SLIs that measure what the user experiences (successful, sufficiently fast requests), to set SLOs based on user behavior and business impact (not on what is convenient to measure), to consume the error budget as a decision tool (when to slow feature velocity and invest in reliability), and to alert on burn rate (how fast budget is being consumed) so that a fast degradation is caught before users are harmed, while avoiding the noise of alerting on every blip.

Agents tend to treat SLOs as a compliance artifact ("we hit 99.9% this quarter") rather than a decision tool, and to pick SLIs that are easy to measure rather than representative of user experience. The harm appears as SLOs that are technically met while users complain (because the SLI measured the wrong thing), as error budgets that are either hoarded (every burn stops shipping, paralyzing the team) or ignored (budget burns unbounded, users suffer), and as burn-rate alerts that page on benign spikes or miss slow, steady degradation. The judgment is to choose SLIs that proxy user happiness (good events over valid events, with latency), to set the SLO by examining the relationship between reliability and user/business outcomes, to use the budget as a shared signal between product and engineering (not a weapon), and to design burn-rate alerts with multi-window thresholds that catch both fast and slow burns without noise. An SLO is a decision tool, not a report card.

## Core Rules

### Define SLIs That Measure User Experience, Not Infrastructure Uptime

An SLI (Service Level Indicator) is the quantity the SLO bounds, and it must proxy what the user experiences. The classic formulation is "good events over valid events" (e.g., requests that succeeded and completed within a latency threshold, over all valid requests), measured at the point closest to the user.

- **Measure at the user boundary, not the server boundary.** A request that succeeds at the server but fails at a proxy, or succeeds but takes 30 seconds, is not a good event from the user's perspective; measure where the user's experience is determined.
- **Include latency, not only success.** A request that succeeds eventually but exceeds the user's patience is a failure of experience; define "good" as both successful and sufficiently fast.
- **Exclude invalid requests from the denominator.** Requests that are malformed, unauthorized, or otherwise not the service's responsibility should not count against reliability; the denominator is valid requests.
- **Choose SLIs per user journey, not only per service.** A service-level SLI misses the composed experience; for critical journeys (checkout, login), measure the end-to-end good-event rate.

### Set The SLO Based On User Behavior And Business Impact

The SLO target should reflect the point at which reliability starts to materially harm users or the business, not a round number chosen for convenience. The way to find it is to examine the relationship between reliability and user behavior.

- **Find the reliability level beyond which users disengage, complain, churn, or revenue drops.** Below that threshold, reliability is a user problem; above it, further reliability may not be worth the cost.
- **Set the SLO just above the user-impact threshold.** The SLO is the floor of acceptable experience; it is not "as reliable as possible," because perfection is unaffordable and users cannot tell the difference between 99.9% and 99.99%.
- **Differentiate SLO by user journey.** Not all journeys deserve the same reliability; checkout and login may warrant a tighter SLO than a recommendations feed. Allocate reliability where it matters.
- **Differentiate SLO from SLA.** The SLA (a contractual commitment with consequences) should be looser than the SLO (the internal target), so that normal SLO misses do not breach contract.

### Consume The Error Budget As A Decision Tool

The error budget (1 minus the SLO) is the amount of unreliability available, and its consumption drives the velocity-vs-reliability decision. Within budget, the team ships features; burning budget too fast, the team slows feature work and invests in reliability. This is what makes the SLO a decision tool rather than a report card.

- **Within budget, prioritize feature velocity.** The budget exists to be spent on shipping; a team that never spends budget is over-investing in reliability the user cannot perceive.
- **When budget burns fast, freeze feature risk.** If the burn rate is high (you are consuming budget faster than it accrues), reduce risk: freeze non-essential changes, prioritize reliability work, and throttle canaries.
- **When budget is exhausted, reliability is the priority.** Once the budget is gone, users are being harmed; further feature work should pause until reliability is restored.
- **Make the budget visible and shared between product and engineering.** The budget is the shared signal that aligns "should we ship" across teams; it should not be a weapon one side wields against the other.

### Alert On Burn Rate, Not On Raw Thresholds

Alerting on raw thresholds ("error rate above X") pages on every blip and misses the trend. Alerting on burn rate (how fast the error budget is being consumed) catches degradations that matter — those that, if continued, will exhaust the budget — while ignoring transient spikes that the budget can absorb.

- **Burn rate is the ratio of current error rate to the budget-sustainable error rate.** A burn rate of 1 means you are consuming budget exactly as fast as it accrues; above 1, you are drawing down; well above 1, you are exhausting it quickly.
- **Use multi-window alerting to balance speed and noise.** A short window (e.g., 5 minutes) catches fast burns; a long window (e.g., 1 hour) confirms sustained burns; requiring both to fire reduces false positives while catching real degradation.
- **Page only on burns that threaten the budget within the alerting window.** A burn that will exhaust budget in an hour is page-worthy; one that will exhaust it in a month is a tracking concern, not a page.
- **Tune to your SLO stringency.** Tighter SLOs (99.99%) have smaller budgets and require more sensitive alerting; looser SLOs tolerate more before alerting.

### Review And Revise SLOs Based On Experience

SLOs are hypotheses about what users need, and they should be revised as you learn. If users complain at an SLO you are meeting, the SLI or the target is wrong; if you consistently exceed the SLO with no user-perceived benefit, the target may be too tight (wasting effort) or the cost of further reliability is worth paying.

- **Review SLOs periodically against user behavior.** If complaints, churn, or support tickets correlate with SLO misses, the SLO is meaningful; if they do not, the SLI may measure the wrong thing.
- **Revise the SLI if it diverges from user experience.** A service meeting its SLO while users suffer means the SLI does not capture the user's reality; fix the indicator.
- **Treat SLO misses as data, not failure.** A miss tells you where to invest; the response is reliability work, not blame.

## Common Traps

### SLI Measuring Infrastructure Uptime Instead Of User Impact

An SLI based on server uptime or CPU that reports the service as available while users experience failures or extreme latency. Define SLIs as good-over-valid events at the user boundary, including latency.

### SLO As A Round Number Or Compliance Artifact

An SLO set to 99.9% because it is round, or used as a quarterly report card, rather than derived from the user-impact threshold and consumed as a decision tool. Set the SLO from user behavior and consume the error budget to drive velocity-vs-reliability decisions.

### Error Budget Hoarded Or Ignored

A budget that is never spent (paralyzing feature work by treating any burn as unacceptable) or ignored (burning unbounded while users suffer). Spend the budget on features within limits; freeze feature risk when it burns fast; prioritize reliability when exhausted.

### Burn-Rate Alerts That Page On Noise Or Miss Degradation

Single-window raw-threshold alerts that page on every transient spike, or thresholds so loose they miss slow steady degradation. Use multi-window burn-rate alerting to catch both fast and sustained burns with low noise.

### SLI Diverging From User Experience

A service meeting its SLO while users complain, because the SLI measures the wrong thing (server-side success, no latency, wrong boundary). Revise the SLI when it diverges from user behavior.

### SLO Used As A Weapon Between Teams

The SLO/budget used by one team to block another rather than as a shared decision signal, eroding its value as a collaborative tool. Make the budget visible and shared; treat it as a signal, not a weapon.

### Treating SLO And SLA As The Same

Setting the SLA at the SLO target, so normal SLO misses breach contract. Differentiate: the SLA should be looser than the SLO so that normal operational misses do not trigger contractual consequences.

## Self-Check

- [ ] SLIs are defined as good events (successful and within latency threshold) over valid events, measured at the user boundary (not the server boundary), including latency, excluding invalid requests, and for critical user journeys measured end-to-end rather than per service.
- [ ] The SLO target is derived from the user-impact threshold (the reliability level beyond which users disengage, complain, churn, or revenue drops), set just above that threshold, differentiated by journey (checkout tighter than a feed), and the SLA is looser than the SLO so normal misses do not breach contract.
- [ ] The error budget is consumed as a decision tool: within budget, feature velocity is prioritized; when burn rate is high, feature risk is frozen and reliability work prioritized; when exhausted, reliability is the priority; and the budget is visible and shared between product and engineering rather than wielded as a weapon.
- [ ] Alerting is on burn rate (current error rate over budget-sustainable rate) with multi-window thresholds (a short window for fast burns, a long window for sustained burns, both required to fire) tuned to the SLO stringency, paging only on burns that threaten the budget within the window.
- [ ] SLOs are reviewed periodically against user behavior (complaints, churn, support tickets), the SLI is revised when it diverges from user experience, and SLO misses are treated as data driving reliability investment rather than as failure.
- [ ] The highest-risk cases were verified — an SLI that captured user impact rather than server uptime, a budget consumption decision that balanced velocity and reliability, a burn-rate alert that caught a fast degradation without paging on noise, and an SLO revised when users complained despite a "passing" metric — not only the clean steady-state path.
