---
name: cloud_cost_optimization.md
description: Use when the agent is selecting cloud resources, right-sizing instances or containers, choosing between on-demand / Reserved Instances / Savings Plans / Spot, configuring autoscaling for cost, tiering storage, reducing egress or data-transfer cost, identifying idle or orphaned resources, setting up cost tagging and allocation, building a FinOps practice, or deciding whether a resource is needed at all and in what form. Also covers multi-tenancy cost allocation, understanding cloud pricing models (per-second, provisioned vs consumed, storage tiers, egress), cost-vs-performance and cost-vs-reliability tradeoffs, and avoiding lock-in-driven waste. Use when a cloud bill is growing without explanation, when provisioning new infrastructure, when reviewing an architecture for cost, or when balancing cost against performance and reliability.
---

# Cloud Cost Optimization

Cloud spending is the one category of engineering decision that converts directly and continuously into money, and almost every resource choice — the instance type, the region, the storage class, the autoscaling bound, the network path, the retention period — is a recurring charge that compounds for the life of the system. Unlike a one-time design mistake, a wasteful cloud configuration keeps billing every hour, quietly, across every environment, until someone audits it. The result is that cost grows invisibly: a service that was right-sized at launch drifts as traffic changes, an experiment left running bills for months, a default storage tier charges premium rates for data never read, and an architecture that "works" does so at five times the cost of one designed with the pricing model in mind. Cloud cost is not a finance problem handed to engineers after the fact; it is an engineering property of the system, decided at design and provisioning time.

Agents tend to treat cost as secondary to "making it work," because a working but expensive system is rewarded immediately while an efficient system's savings are invisible. The harm is delayed and large: bills that surprise leadership, services that cannot scale because the unit economics break, features that are uneconomic to run, and forced cost-cutting under pressure that cuts into reliability. The judgment problem is not "how do I spend less" but two questions asked together for every resource: *is this resource actually needed, and if so, what is the most cost-efficient form that still meets the performance and reliability requirements?* This skill covers the engineering discipline of cloud cost. Selecting cloud services for capability, and designing for multi-region resilience, are adjacent concerns; here the focus is on the cost dimension of those decisions.

## Core Rules

### Make Cost Visible Before You Try To Optimize It

You cannot optimize what you cannot attribute. The first step in any cost work is visibility: knowing what is spending, by whom, for what purpose, and whether it is justified. Without attribution, "the bill is too high" produces blanket cuts that hit productive resources and miss the waste.

- **Tag everything that costs money** with owner, team, project, environment, and cost center, and enforce tagging at provisioning time (via policy/infrastructure-as-code), not as a voluntary afterthought. An untagged resource is an unattributable cost.
- **Allocate shared spend** (networking, shared databases, platform services) to consumers where practical, so teams see the cost they drive, not just the cost they directly provision.
- **Surface cost in the engineering workflow** — per-service cost dashboards, cost-per-request or cost-per-customer unit metrics, and cost alerts on anomalies — so that a regression in efficiency is noticed the week it happens, not at the next quarterly review.

Visibility is the prerequisite. A team that sees its own spend, in units that map to its work (cost per request, cost per tenant, cost per build), makes better tradeoffs than a team that sees only an aggregate bill. Build the attribution first; the optimizations follow naturally once waste is visible.

### Ask "Is This Needed At All" Before "How Do I Make It Cheaper"

The cheapest resource is the one you do not provision. Before right-sizing or discounting, challenge the existence of each resource:

- **Idle and orphaned resources.** Stopped-but-billed instances, unattached disks, idle load balancers, forgotten dev/staging environments running 24/7, snapshots and backups of deleted systems, oversized queues, idle reserved capacity. These bill continuously while delivering nothing. A recurring cleanup of unattached storage, stopped instances, and stale environments is often the largest single saving available.
- **Duplication.** Multiple services each running their own copy of an expensive dependency (a search cluster, a data warehouse, a NAT gateway) where one shared instance would do.
- **Over-provisioned "just in case."** Resources sized for a peak that never comes, or sized by copying a template rather than measuring demand.

The discipline is to treat every running resource as a hypothesis ("this is needed, at this size, for this purpose") and to revisit the hypothesis as conditions change. A resource that was justified at provisioning may be pure waste a year later, and nothing in the cloud will tell you — only an audit will.

### Right-Size To Measured Demand, Not To Assumptions Or Defaults

The default instance type, the default storage tier, and the default autoscaling bound are almost always larger than needed, because defaults are conservative and assumptions ("we might need headroom") compound. Right-size from data:

- **Measure actual utilization** (CPU, memory, IOPS, throughput, queue depth) over a representative window including peak, and size to comfortably handle observed peak with bounded headroom — not to a guessed multiple.
- **Prefer the resource family that matches the workload.** Compute-bound workloads want CPU-optimized instances; memory-bound ones want memory-optimized; bursty ones want burstable or autoscaling; I/O-heavy ones want the right storage. A general-purpose instance running a specialized workload is paying for capacity it cannot use.
- **Re-right-size periodically.** Utilization drifts as traffic and code change; a size that was right six months ago may now be 3x oversized or hitting saturation. Make right-sizing a recurring review, not a one-time event.

Right-sizing is usually the highest-leverage optimization after deleting waste, because it reduces the unit you pay for continuously. The risk is over-aggressive downsizing that causes saturation or throttling — size to measured peak with headroom, and watch the metrics after a change.

### Match Commitment Strategy To Predictability And Risk

Cloud providers discount committed usage (Reserved Instances, Savings Plans, Committed Use Discounts) versus on-demand, in exchange for a term commitment. The trade is discount versus flexibility, and the mistake is committing either too little (paying on-demand premium forever for baseline load) or too much (committing to capacity you then don't use, paying for nothing):

- **On-demand** for unpredictable, spiky, or short-lived workloads, and for anything you might need to move or shut down. Highest unit cost, maximum flexibility.
- **Reserved / Savings Plans** for stable baseline load you are confident will run for the term. Significant discount, in exchange for a 1- or 3-year commitment. The core rule: commit to the *baseline you are confident in*, not the peak — committing to peak leaves you paying for unused commitment when traffic dips.
- **Spot / preemptible** for fault-tolerant, interruptible, or batch workloads (CI, batch processing, stateless scale-out, training) that can survive termination or checkpoint and restart. Largest discount, but the capacity can be reclaimed with short notice, so it requires the workload and the orchestration to handle interruption.

The blend matters. A mature footprint is mostly committed for the confident baseline, on-demand (or autoscaling) for the variable top, and spot for the interruptible work — each matched to the predictability and criticality of what runs on it. Committing everything to on-demand wastes money on the baseline; committing everything to reserved over-commits on the peak; using spot for stateful critical services invites outages.

### Configure Autoscaling To Scale Down, Not Just Up

Autoscaling is as much a cost control as a performance control, but only if it scales *down* as aggressively as it scales up. A common failure is an autoscaling group that adds capacity under load but has conservative or absent scale-in rules, so capacity added for a peak stays provisioned and bills forever:

- **Set scale-in thresholds and cooldowns deliberately**, symmetric with scale-out, so idle capacity is reclaimed. The fear of thrashing (scaling in then right back out) leads teams to never scale in, which defeats the cost purpose.
- **Schedule-based scaling** for predictable diurnal or weekly patterns — scale down off-hours and weekends for services with business-hours traffic.
- **Scale to zero** for genuinely idle components (dev environments, internal tools, batch workers with a queue) where startup latency is tolerable. A service that runs 24/7 to serve a few requests an hour is almost entirely waste.

The unit economics: a service that scales out for peak but never scales in costs as if it were always at peak. The savings from scaling in often exceed the savings from discounting, because reclaiming an entire idle instance is a 100% reduction on that unit.

### Tier Storage And Plan Data Lifecycle From The Start

Storage is cheap per unit but accumulates without limit, and the default tier is usually the most expensive. Storage cost is governed by access frequency and retention, so tier and lifecycle to match:

- **Hot / frequently accessed** data on premium storage; **infrequently accessed** (backups, archives, old logs, cold analytics) on cheaper tiers; **rarely accessed** (long-term retention, compliance archives) on archival/cold storage. The price difference between tiers is often an order of magnitude.
- **Lifecycle policies** that automatically transition objects to cheaper tiers or delete them after a retention period, so storage self-manages rather than growing forever at premium rates.
- **Versioning and replication multiply cost** — every replicated copy and every object version bills. Enable them where the durability/availability justifies it, and prune versions and old backups on a schedule.
- **Databases** often have the largest storage spend and the most lifecycle opportunity: old partitions, historical data, and logs that could move to cheaper object storage rather than sitting on premium database storage.

The trap is treating storage as "set and forget." Without lifecycle rules, storage grows monotonically and the bill with it, for data that is accessed rarely or never.

### Understand And Minimize Data Transfer (Egress) Cost

Network egress — data leaving a provider's network, or crossing regions or zones — is frequently the most surprising and least-attributed cost, because it is not tied to a visible resource. Architectural choices made for performance or simplicity can generate large recurring transfer bills:

- **Egress to the internet** is usually priced; ingress is usually free. A service that pushes large payloads to external consumers (webhooks, downloads, analytics exports) pays per GB out.
- **Cross-region and cross-zone traffic** is often priced. Multi-region replication, cross-AZ database reads, and architectures that move data between regions for processing can rack up transfer cost that dwarfs compute.
- **NAT gateways** charge per GB processed in addition to hourly — a chatty private-to-internet path through a NAT can be a large, hidden bill.

Design with the transfer model in mind: keep data and its consumers co-located, cache or compress outbound payloads, prefer regional services over cross-region, and scrutinize any architecture that moves large volumes between regions or zones. Egress is where "it works" architectures quietly become expensive.

### Balance Cost Against Performance And Reliability Explicitly

Cost is one axis of a tradeoff, not the objective. The cheapest configuration that violates an SLO, loses data, or fails under peak is more expensive than a slightly pricier one that meets requirements — once you count incident cost, lost revenue, and rework. Make the tradeoff explicit rather than optimizing cost in isolation:

- **Cost vs performance.** A smaller instance that saturates under peak and degrades latency below the SLO is a false economy. Size to the performance requirement, then optimize cost within that constraint.
- **Cost vs reliability.** Single-AZ, no redundancy, spot-only for stateful services — cheap until the outage. Reliability requirements (the SLO) set the floor on redundancy; cost optimization happens above that floor.
- **Cost vs velocity.** Over-engineering for cost (custom scheduling, aggressive multi-tenancy, hand-tuned right-sizing) can slow delivery more than it saves. Some waste is the cost of moving fast; optimize the waste that is large and recurring, not the waste that is small and would cost more to eliminate than it saves.

The framing: define the performance and reliability requirements first, then find the lowest-cost configuration that meets them. Optimizing cost below those requirements is not saving money; it is borrowing incidents.

### Treat Cloud Financial Management (FinOps) As An Ongoing Practice, Not A Project

Cloud cost is not a one-time cleanup; it decays. New resources are provisioned without sizing, traffic patterns shift, services are forgotten, and provider pricing and instance generations change. A one-off cost audit produces a dip in the bill that creeps back up within months. Sustainable cost control requires an ongoing practice:

- **Regular reviews** of top spenders, anomalous growth, and tagging hygiene, at a cadence tied to how fast spend moves.
- **Unit-cost metrics** (cost per request, per customer, per build) tracked over time, so efficiency is visible independent of growth. Aggregate spend will rise as the business grows; unit cost should fall or hold.
- **Ownership and accountability** — each service's cost is owned by the team that runs it, with feedback loops so efficiency is rewarded.
- **Automation** for the repetitive parts: scheduled shutdown of non-production, lifecycle policies, anomaly alerts, and policy-as-code that blocks untagged or oversized provisioning.

The goal is not the lowest possible bill; it is a bill where every charge is intentional, attributed, and justified by the value it produces. A system with that property stays efficient as it grows; one without it does not.

## Common Traps

### Provisioning Defaults And Forgetting Them

Accepting the default instance type, disk size, or storage tier because it works, then never revisiting. Defaults are conservative and bill at premium rates for capacity never used. Right-size from measured demand and choose tiers deliberately.

### Committing To Reserved Capacity At Peak

Buying reservations or savings plans sized for peak traffic, then paying for unused commitment when traffic dips. Commit to the confident baseline; cover the variable top with on-demand or autoscaling.

### Using Spot For Stateful Critical Services

Chasing the spot discount for a database or stateful service that cannot tolerate interruption, then losing data or availability when capacity is reclaimed. Spot is for interruptible, fault-tolerant workloads with orchestration that handles termination; stateful critical services need committed or on-demand capacity.

### Autoscaling Up But Never Down

An autoscaler with aggressive scale-out and timid or absent scale-in, so capacity added for a peak stays provisioned indefinitely. The service bills as if always at peak. Set symmetric scale-in rules and schedule-based or scale-to-zero for idle periods.

### Storage On The Default Tier With No Lifecycle

Putting everything on hot/premium storage with no transition or expiration policy, so logs, backups, and old data accumulate at premium rates forever. Tier by access frequency and set lifecycle rules to transition and expire.

### Ignoring Egress Until The Bill Surprises You

Architecting for simplicity (cross-region replication, cross-AZ reads, large outbound payloads) without considering transfer pricing, then discovering egress is a top-line cost. Design with the transfer model in mind and keep data co-located with its consumers.

### Cost-Cutting Under Pressure That Hits Reliability and untagged Resources That Cannot Be Attributed

A mandate to cut X% leads to dropping redundancy, shrinking below the SLO, or turning off backups — trading a bill reduction for future incidents. Define the performance/reliability floor first and optimize cost above it, not through it.

Resources provisioned without tags, so spend cannot be attributed to a team or purpose, and waste cannot be distinguished from productive use. Enforce tagging at provisioning time; an untagged resource is unmanageable cost.

### Optimizing Small Recurring Costs While Ignoring Large Structural Waste and treating Cost As Finance's Problem

Spending effort on micro-optimizations (a few dollars of small instances) while a multi-region architecture or an oversized database quietly bills thousands. Prioritize by impact: delete idle resources, right-size top spenders, and fix structural waste before micro-tuning.

Engineering provisions, finance receives the bill, and the feedback loop never closes — so engineers never see the cost consequences of their choices and waste accumulates. Cost is an engineering property decided at design time; surface it to the teams that drive it and make them accountable for unit economics.

## Self-Check

- [ ] All cost-bearing resources are tagged (owner, team, project, environment, cost center) with tagging enforced at provisioning time, shared spend is allocated to consumers, and per-service / unit-cost (cost per request, per tenant, per build) dashboards and anomaly alerts exist so cost regressions are noticed within the week.
- [ ] Before optimizing, the existence of each resource was challenged: idle/orphaned resources (stopped instances, unattached disks, stale dev environments, old snapshots, idle load balancers), duplication, and over-provisioned "just in case" capacity were identified and removed.
- [ ] Resources are right-sized to measured utilization (including peak) with bounded headroom, the instance/storage family matches the workload, and right-sizing is a recurring review rather than a one-time event.
- [ ] The commitment blend matches predictability and risk: committed (Reserved/Savings Plans) for the confident baseline, on-demand/autoscaling for the variable top, spot only for interruptible fault-tolerant workloads with termination handling — not committed at peak, not spot for stateful critical services.
- [ ] Autoscaling scales down (symmetric scale-in thresholds, cooldowns, schedule-based scaling, scale-to-zero for idle components), so capacity added for a peak is reclaimed rather than billing as if always at peak.
- [ ] Storage is tiered by access frequency (hot/infrequent/archive) with lifecycle policies that transition and expire, versioning/replication is enabled only where justified and pruned on schedule, and database storage has a lifecycle plan for old data.
- [ ] Data transfer (internet egress, cross-region, cross-zone, NAT-processed) was designed against the pricing model: data co-located with consumers, outbound payloads cached/compressed, cross-region movement minimized — no architecture generates large hidden transfer bills.
- [ ] Cost tradeoffs against performance and reliability are explicit: the SLO/performance floor is defined first, and cost is optimized above it, not by cutting into redundancy, sizing below saturation thresholds, or disabling backups.
- [ ] FinOps is an ongoing practice — regular reviews of top spenders and anomalies, unit-cost metrics tracked over time, per-service cost ownership, and automation (scheduled shutdown, lifecycle policies, anomaly alerts, policy-as-code blocking untagged/oversized provisioning) — not a one-time cleanup whose savings creep back.
