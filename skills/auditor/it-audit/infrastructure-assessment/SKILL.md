---
name: infrastructure-assessment.md
description: Use when the agent is auditing IT infrastructure, assessing data center, cloud, network, and server environments, evaluating capacity and availability controls, reviewing infrastructure change and configuration management, testing backup and disaster recovery, or determining whether the technology foundation supporting business operations is reliable, scalable, secure, and cost-appropriate.
---

# Infrastructure Assessment

An infrastructure assessment determines whether the underlying technology foundation — networks, servers, storage, cloud platforms, data centers, and the processes that operate them — reliably supports the entity's business objectives. The central judgment problem is that infrastructure is judged against multiple competing objectives (availability, performance, scalability, security, cost) that trade off against one another, and a configuration that is excellent on one dimension can be deficient on another. Agents frequently assess infrastructure against generic checklists, accept availability statistics without examining the controls behind them, or evaluate the technology while ignoring the operational processes that keep it running.

## Core Rules

### Align the assessment with business dependency and criticality

Infrastructure does not exist for its own sake; it supports business processes with varying criticality. Begin by establishing which business processes depend on which infrastructure components, and the tolerance for disruption of each. A system supporting real-time trading or life-safety operations demands higher availability and faster recovery than an internal reporting tool. Map infrastructure to business impact and focus assessment effort where downtime or degradation has the greatest consequence. Generic infrastructure scoring disconnected from business impact produces technically interesting but decision-irrelevant conclusions.

### Evaluate availability against defined service levels and the controls behind them

Availability is the most visible infrastructure metric, but a percentage alone is insufficient. Assess:

- the defined service level (SLA) and whether the entity meets it;
- the architecture that supports it (redundancy, failover, clustering, multi-AZ or multi-region design);
- the monitoring and alerting that detects degradation before it becomes outage;
- the incident management process that restores service within target;
- the root-cause analysis that prevents recurrence.

Five-nines availability achieved through heroic manual intervention is fragile; the same number achieved through resilient architecture and automated recovery is robust. The number does not reveal which.

### Assess capacity management against current and projected demand

Infrastructure must handle not only today's load but foreseeable growth. Evaluate whether the entity:

- monitors resource utilization (CPU, memory, storage, network, license seats) against thresholds;
- forecasts demand based on business growth and seasonality;
- provisions capacity ahead of need without excessive over-provisioning;
- handles peak events and known spikes without degradation.

Both chronic under-capacity (performance failure) and chronic over-capacity (wasted spend) are findings. Look for the capacity management process, not just current utilization.

### Examine the change and configuration management discipline

Most infrastructure outages are caused by change, not by steady-state failure. Assess:

- whether changes are requested, assessed, approved, tested, and implemented through a controlled process;
- whether emergency changes are restricted and retrospectively reviewed;
- whether configuration items are recorded in a maintained CMDB with dependencies mapped;
- whether configuration baselines are enforced and drift detected;
- whether roll-back plans exist and have been tested.

Weak change management predicts outages; a CMDB that does not reflect reality makes incident response and impact analysis unreliable.

### Evaluate backup, recovery, and disaster recovery with testing, not just existence

Backup and DR capabilities are only as good as their last successful test. Assess:

- whether backups cover all critical data and systems, with defined retention and immutability where needed;
- whether recovery objectives (RTO and RPO) are defined per system criticality;
- whether restore tests are performed regularly and succeed;
- whether full DR exercises (not just component restores) are conducted periodically;
- whether failover and failback have been tested, including people and process, not just technology.

Untested backups are an aspiration; documented backup schedules without restore evidence provide no assurance that recovery is possible.

### Assess cloud-specific considerations deliberately

Cloud infrastructure introduces distinct assessment dimensions:

- **architecture** — use of managed services, multi-AZ design, auto-scaling, right-sizing;
- **cost management** — detection of waste, reserved capacity strategy, tagging and chargeback;
- **shared responsibility** — clarity on which controls the cloud provider owns versus the entity;
- **identity and access** — IAM policies, privileged access, key management;
- **data residency and sovereignty** — compliance with jurisdictional requirements;
- **provider dependency and portability** — lock-in risk and exit strategy.

Do not assess cloud with the same lens as on-premises; the control models and risk surfaces differ.

### Evaluate network architecture, segmentation, and connectivity

Network infrastructure controls how traffic flows and how threats propagate. Assess:

- segmentation that isolates critical systems and limits lateral movement;
- connectivity (VPN, direct connect, internet exposure) and its controls;
- bandwidth and latency against application requirements;
- redundancy in network paths and internet connectivity;
- monitoring for anomalous traffic and denial-of-service resilience.

Flat networks with excessive trust and single points of failure are common hidden weaknesses.

### Consider operational processes, staffing, and vendor management

Infrastructure reliability depends on people and processes as much as technology. Assess:

- staffing levels, skills, and coverage (including after-hours and on-call);
- operational runbooks and their currency;
- monitoring tooling and the alert-to-action loop;
- vendor and managed-service performance against contracts;
- knowledge concentration risk (single administrators with unique expertise).

Sophisticated technology operated by an overstretched team with undocumented runbooks is operationally fragile regardless of the hardware specifications.

### Balance the competing objectives explicitly

Infrastructure assessment must reconcile competing objectives:

- **availability vs. cost** — higher redundancy costs more;
- **performance vs. cost** — premium resources cost more;
- **security vs. usability** — tighter controls can impede operations;
- **standardization vs. flexibility** — standard builds are easier to manage but may not fit all needs.

Identify where the entity has made these trade-offs deliberately and where they have occurred by accident. Report the trade-offs, not just the resulting configuration, because a deliberate, documented trade-off is a different finding from an unnoticed deficiency.

## Common Traps

- **Checklist without business context.** Scoring infrastructure against generic criteria disconnected from which systems matter most to the business.
- **Availability-statistic credulity.** Accepting high uptime percentages without examining whether they reflect resilient architecture or unsustainable manual heroics.
- **Steady-state focus.** Assessing current operation while ignoring change management, where most outages originate.
- **Untested recovery.** Treating documented backup and DR plans as evidence of recoverability without restore and failover testing.
- **Cloud-as-on-premises.** Applying on-premises assessment lenses to cloud, missing shared responsibility, cost management, and architectural differences.
- **Capacity point-in-time.** Reporting current utilization without assessing the capacity management process or projected demand.
- **CMDB-as-truth.** Assuming the configuration database reflects reality without verifying against the live environment; **Technology-only assessment.** Evaluating hardware and software while ignoring staffing, runbooks, vendor management, and knowledge concentration risk
- **Flat-network blindness.** Overlooking network segmentation and single points of failure because the network "works."; **Trade-off invisibility.** Reporting a configuration as deficient without recognizing it may be a deliberate, documented trade-off against cost or other objectives

## Self-Check

- Have I aligned the assessment with business process criticality, focusing effort where downtime or degradation has the greatest consequence?
- For availability, did I examine the architecture, monitoring, incident management, and root-cause process behind the uptime statistic, not just the number?
- Have I assessed capacity management as a process (monitoring, forecasting, provisioning) against current and projected demand, not just current utilization?
- Did I evaluate change and configuration management, including CMDB accuracy and drift detection, given that change causes most outages?
- For backup and DR, did I confirm that restores and full failover exercises have been tested successfully, not just that plans and schedules exist?
- For cloud infrastructure, did I assess architecture, cost management, shared responsibility, IAM, data residency, and portability using cloud-appropriate criteria?
- Did I assess network segmentation, redundancy, connectivity controls, and single points of failure?
- Have I evaluated operational processes, staffing coverage, runbooks, vendor management, and knowledge concentration risk alongside the technology?
- Have I identified and reported the deliberate versus accidental trade-offs among availability, performance, security, and cost, rather than judging configurations in isolation?
