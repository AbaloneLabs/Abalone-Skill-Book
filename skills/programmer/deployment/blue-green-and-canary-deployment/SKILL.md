---
name: blue_green_and_canary_deployment.md
description: Use when the agent is implementing the operational mechanics of blue-green or canary deployment, choosing a traffic-shifting mechanism such as load balancer weighting, service mesh, ingress routing, or DNS, selecting canary metrics and automated rollback thresholds, designing progressive exposure schedules, sizing infrastructure capacity for blue-green or canary, ensuring database and session compatibility during version coexistence, evaluating automated canary analysis tools, or weighing the cost and complexity of progressive delivery. Use when configuring the routing, observability, and threshold machinery rather than choosing the high-level release strategy.
---

# Blue-Green And Canary Deployment

Blue-green and canary are the two progressive-delivery patterns that let you ship a new version while keeping a known-good one to return to. At the strategy level they are simple — blue-green switches all traffic at once between two environments, canary shifts a little traffic first and expands. But the strategy is the easy part. The hard part, and the part agents most often get wrong, is the operational mechanics: how traffic is actually shifted, what metrics decide whether the new version is healthy, what thresholds trigger automatic rollback, what capacity and observability the pattern demands, and what happens to shared state when two versions of the code must coexist. A canary with no meaningful metrics is theater that feels like safety; a blue-green switch with an incompatible migration is a faster way to break everyone.

Agents tend to focus on the topology — "we will run two environments" or "we will send five percent of traffic to the new version" — and treat the surrounding machinery as details to fill in later. The machinery is the entire safety benefit. A canary whose rollback threshold is set by gut feel will either fire on noise (rolling back healthy releases) or miss real regressions (advancing a broken one). A blue-green cutover whose database was migrated in place has no blue to return to, because the green schema is now incompatible with the old code. This skill is the deep-dive implementation of blue-green and canary: traffic-shifting mechanisms, metric and threshold design, capacity and observability requirements, and version-coexistence constraints. The high-level comparison of strategies and the rollback philosophy are covered by the release-strategies skill; this skill assumes that choice is made and covers how to actually build the chosen pattern correctly.

## Core Rules

### Choose The Traffic-Shifting Mechanism By What You Control And What You Need

Traffic shifting is not one mechanism; it is a family of them, and the right choice depends on where you can intercept traffic and how fine-grained the shift must be:

- **Load balancer weighted routing.** The load balancer distributes traffic between target groups or pools by configured weight. Coarse but simple, and available on almost every cloud. Good for blue-green (100/0 to 0/100) and coarse canary steps. Weak for fine-grained or header-based routing.
- **Service mesh routing.** A mesh (Istio, Linkerd) lets you shift traffic by percentage, by header, by user identity, or by request shape at the sidecar. Most expressive, but adds operational complexity and a new failure domain. Worth it when you do progressive delivery often and need fine control.
- **Ingress and API gateway rules.** The ingress or gateway can route by path, host, or header to different backing services. Good for routing specific cohorts (internal users, a beta tenant) onto the new version without a full mesh.
- **DNS-based shifting.** Changing DNS weights to move traffic between endpoints. Simple and infrastructure-light, but slow to propagate (TTL and resolver caching) and slow to reverse, which makes it a poor fit for fast canary rollback. Acceptable for coarse blue-green between regions, risky for fine canary.

The decision criteria are latency of the shift, granularity of control, and the blast radius if the routing layer itself fails. For canary that must reverse in seconds, prefer a mechanism you control in-process or at the edge (load balancer, mesh, gateway), not DNS. Strong choice: a single routing layer that supports both weighted and header-based shifting so coarse and cohort-based canary use the same machinery. Weak choice: DNS for canary, where a rollback takes as long as TTL expiry.

### Select Canary Metrics That Actually Detect Regressions

The metrics that gate a canary are the entire basis on which you decide to advance or roll back, and most teams choose them poorly. The right metrics are the ones that move when something is actually wrong, not the ones that are easy to collect:

- **Error rate is the strongest primary signal.** HTTP 5xx rate, exception rate, or failed-request rate, compared against the baseline, catches most correctness regressions directly. A canary without an error-rate gate is flying blind.
- **Latency, especially tail latency (p95, p99), catches performance regressions that do not show up as errors.** A version that is correct but twice as slow will pass an error-rate check and still harm users. Watch the tail, not just the mean — mean latency hides the worst-affected requests.
- **Business and domain metrics catch harm that technical metrics miss.** Conversion rate, payment success rate, order completion, signup completion. A version can have flat error rate and flat latency and still be broken at the level users care about (a checkout that returns 200 but never completes). For high-stakes services, instrument the business outcome and gate on it.
- **Saturation and resource metrics (CPU, memory, connection pools, queue depth) catch capacity regressions** that will become errors under load even if they have not yet.

Vanity metrics — request count, uptime percentage averaged over the canary window, "no critical alerts" — do not detect regressions and must not be the gate. Strong choice: a canary gated on error rate, p99 latency, and at least one business metric, each with a defined threshold. Weak choice: a canary gated only on "no alerts fired," which advances slow-burning problems to full release.

### Set Rollback Thresholds With Statistical Rigor, Not Gut Feel

A canary at five percent of traffic sees a small sample, and small samples are noisy. Thresholds set without accounting for variance will either fire constantly (rolling back healthy releases on noise) or never fire (advancing broken ones). Design thresholds deliberately:

- **Compare against a baseline, not an absolute number.** The canary's error rate must be compared to the current (baseline) version's error rate over the same window, because absolute thresholds ignore the system's normal error rate. A canary at 0.5% errors looks fine in absolute terms and is catastrophic if the baseline runs at 0.05%.
- **Account for statistical significance and sample size.** At low traffic percentages, a few extra errors can be noise. Use a comparison (statistical test, confidence interval, or a tool that implements one) rather than a raw threshold, and require enough samples before deciding. A canary that decides after thirty seconds at one percent is deciding on noise.
- **Set thresholds before the release, not during it.** Deciding "is this bad enough" while watching a live canary biases toward whatever is easiest under pressure — usually advancing. Define the threshold and the action (roll back automatically, page a human) in advance.
- **Distinguish "advance" from "do not advance" from "roll back now."** Three outcomes, not two. A canary that is statistically indistinguishable from baseline should hold and collect more samples, not auto-advance on "no significant difference."

Strong choice: thresholds defined as a multiple of baseline error rate with a minimum sample size and a confidence interval, automated where the signal is clear and escalated to a human where it is ambiguous. Weak choice: "roll back if error rate exceeds one percent," an absolute threshold that ignores both baseline and sample size.

### Design A Progressive Exposure Schedule, Not A Single Jump

The exposure schedule — the sequence of traffic percentages and the dwell time at each — is what gives a canary its detection power. A single jump from one percent to one hundred percent is a big-bang release with a one-percent preview. Design the schedule as a sequence:

- **Start small enough to bound harm.** One to five percent is typical. Small enough that a catastrophic regression affects few users, large enough to get meaningful samples in a reasonable window.
- **Dwell long enough to collect signal at each step.** The dwell time must produce enough requests to meet the sample-size requirement of the thresholds. For low-traffic services this may be hours, not minutes; advancing before the sample is sufficient means deciding on noise.
- **Expand in increasing steps.** A common shape is 1%, 5%, 25%, 50%, 100%, holding at each until the metrics clear. The early steps catch catastrophic regressions cheaply; the later steps catch regressions that only appear under load.
- **Automate the advance on clear signal, hold for humans on ambiguous signal.** Mechanical advancement reduces operator load; the human checkpoint is reserved for the cases the metrics cannot decide.

The schedule must be written down before the release, with the metric conditions for advancing, holding, and rolling back at each step. A canary schedule that exists only in the operator's head will be improvised under pressure.

### Size Capacity For The Pattern's Real Requirement

Blue-green and canary impose capacity costs that are easy to underestimate:

- **Blue-green needs roughly double steady-state capacity**, because two complete environments must run simultaneously during the cutover (and ideally during warmup and verification before the switch). Provisioning green at less than full capacity means the cutover itself is a capacity event.
- **Canary needs the new version sized for its target traffic at each step**, plus headroom, and the baseline version sized for the remainder. At the final step the new version carries full load; under-provisioning it surfaces as latency or errors that look like a regression and trigger a spurious rollback.
- **Both patterns need capacity for the rollback direction.** If you shift to green and green fails, you must be able to run blue again. If you have already scaled blue down to reclaim capacity, the rollback target is gone. Do not reclaim the old version's capacity until the new version is confirmed stable for a meaningful window.

The cost is real and must be budgeted. A team that adopts blue-green to be safe but refuses to pay for double capacity has a blue-green topology that cannot actually perform a safe cutover.

### Ensure Database And Session Compatibility During Version Coexistence

Both patterns require the old and new versions to run simultaneously, which means they share state. That coexistence is the most common source of subtle failures:

- **The database schema must be compatible with both versions.** A migration that the new version requires but the old version cannot tolerate breaks rollback (the old version cannot read the new schema) and breaks blue-green (both versions hit the same database). Use expansive, backward-compatible migrations within a release; split destructive changes across releases. The release-strategies and migration skills cover this in depth, but the operational consequence here is concrete: if the schema is not compatible, you do not have a blue-green or canary deployment, you have a rolling one with extra steps.
- **Sessions and caches must survive the version mix.** If sessions are encrypted or signed with a version-specific key, or cached objects have a version-specific shape, a user whose request lands on the new version and then the old (or vice versa) gets logged out or errors. Ensure session formats and cache schemas are forward-and-backward compatible across the coexistence window.
- **Background jobs and queues must be consumable by either version.** A message produced by the new version and consumed by the old (or the reverse) must be handled gracefully, or the queue develops poison messages during the rollout.

The rule: during a blue-green or canary, both versions are live against the same state, and the rollout is only safe if that state is compatible with both. Discovering incompatibility mid-cutover is the classic progressive-delivery failure.

### Match Observability Granularity To The Canary Granularity

Canary detection depends on comparing the new version's metrics to the baseline's, which requires the metrics to be labeled by version. If all traffic's metrics are aggregated into one dashboard, you cannot tell whether an elevated error rate comes from the one-percent canary or the ninety-nine-percent baseline:

- **Label every metric by version (and by cohort).** Error rate, latency, business metrics must be filterable by the version that served the request, so the canary and baseline can be compared directly.
- **Make the comparison the default view, not a custom query.** Operators under pressure should see "canary vs baseline" without writing a query. A dashboard that shows only aggregate health hides the signal that matters during a rollout.
- **Ensure the observability pipeline's latency is shorter than the canary dwell time.** If metrics lag by ten minutes and the canary dwells for five, you are advancing on stale data. The observability cadence must be faster than the rollout cadence.

A canary without version-labeled, low-latency, comparison-ready observability is not a canary; it is a release with extra dashboards.

## Common Traps

### DNS-Based Canary With A Long TTL

Shifting canary traffic by changing DNS weights, then discovering rollback takes as long as resolver TTL and caching, by which time the broken version has served far more traffic than intended. Use a routing mechanism you control at the edge for canary; reserve DNS for coarse, slow-moving shifts.

### Canary Gated Only On "No Critical Alerts"

Advancing a canary because no critical alert fired, even though error rate crept up and p99 latency doubled at low rate. The slow-burning regression reaches full release because the gate detected hard failures, not degradation. Gate on error rate, tail latency, and a business metric with explicit thresholds.

### Absolute Thresholds That Ignore Baseline And Sample Size

Rolling back when error rate exceeds a fixed percentage regardless of the baseline, or advancing after a dwell time too short to produce enough samples. The former rolls back healthy releases on the system's normal noise; the latter advances on statistical noise. Compare against baseline and require a sufficient sample.

### Canary Population That Is Not Representative

Routing canary traffic only to internal users or a single region, so the canary passes but the full release fails for real users whose data shape, scale, or geography differs. Make the canary population representative of real traffic on the dimensions that vary.

### Blue-Green With An In-Place Database Migration

Migrating the shared database as part of the green deployment, then discovering the green schema is incompatible with blue and there is no rollback target. Keep migrations expansive and backward-compatible within a release; the database must serve both versions during the cutover and the rollback window.

### Reclaiming The Old Version's Capacity Too Early

Scaling down blue (or the previous canary baseline) the moment green takes traffic, to save cost, then having no capacity to roll back to when green fails. Do not reclaim the old version's capacity until the new version has been stable for a defined window.

### Under-Provisioning The New Version At The Final Canary Step

Sizing the new version for early canary percentages, so at the final step to full traffic it is overloaded and the latency and errors look like a regression, triggering a spurious rollback. Size the new version for its target load plus headroom before advancing.

### Aggregated Metrics That Hide The Canary Signal and auto-Advancing On "No Significant Difference"

A single dashboard showing aggregate error rate across all versions, so an elevated rate cannot be attributed to the canary or the baseline and the operator guesses. Label metrics by version and make the canary-versus-baseline comparison the default view.

Treating a statistically inconclusive canary as a pass and advancing, when the right action is to hold and collect more samples. Inconclusive is not the same as healthy; require positive evidence of health, not merely absence of proven harm.

## Self-Check

- [ ] The traffic-shifting mechanism (load balancer weighting, service mesh, ingress/gateway rules, or DNS) was chosen by latency-of-shift, granularity, and blast-radius-of-routing-failure, and DNS is not used where fast canary rollback is required.
- [ ] Canary metrics include error rate, tail latency (p95/p99), and at least one business or domain metric, and vanity metrics (request count, averaged uptime, "no critical alerts") are not the basis for advance or rollback decisions.
- [ ] Rollback thresholds compare the canary against the baseline version over the same window, account for sample size and statistical significance (not raw absolute percentages), require a minimum sample before deciding, and were defined before the release rather than during it.
- [ ] The progressive exposure schedule is a documented sequence of increasing percentages (e.g., 1%, 5%, 25%, 50%, 100%) with a dwell time at each step long enough to meet the sample-size requirement, and advancement is automated on clear signal while ambiguous signal holds for a human.
- [ ] Capacity is sized for the pattern's real requirement: roughly double steady-state for blue-green, the new version sized for full target load plus headroom for canary, and the old version's capacity is retained until the new version is stable for a defined window so rollback capacity exists.
- [ ] The database schema, session format, cache schema, and queue message format are compatible with both old and new versions during the coexistence window (expansive backward-compatible migrations within a release), so rollback to the old version against current state actually works.
- [ ] Every metric is labeled by version and cohort, the canary-versus-baseline comparison is the default dashboard view (not a custom query), and the observability pipeline latency is shorter than the canary dwell time so decisions are made on fresh data.
- [ ] The canary population is representative of real traffic on the dimensions that vary (tenants, geographies, request shapes), not only internal users or a single non-representative cohort.
