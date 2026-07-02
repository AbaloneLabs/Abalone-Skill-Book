---
name: service_levels_and_credits.md
description: Use when the agent is reviewing or drafting service level agreements, SLA definitions and measurement, service credit remedies for underperformance, reporting obligations, exclusions and force majeure, termination rights for persistent failure, and remedy limitations in services or cloud contracts.
---

# Service Levels And Credits

Service level agreements translate the fuzzy promise of "good service" into measurable obligations with consequences. The judgment problem is that an SLA looks like a commitment to quality, but its real effect depends entirely on how the service is measured, what counts as downtime or failure, what credits the customer actually receives, and whether the credits are the sole remedy. Agents often summarize an SLA as "99.9 percent uptime with credits" while missing that the measurement window excludes maintenance, that the credit is capped at a fraction of monthly fees, that the customer must affirmatively request credits, and that persistent failure rarely gives a termination right. An SLA that looks protective can leave the customer paying full price for broken service.

Use this skill before reviewing, drafting, or negotiating service levels, credits, reporting, exclusions, or remedy limitations in any services, SaaS, cloud, or outsourcing agreement. The goal is to make the agent define service levels measurably, design a credit mechanism that has teeth, allocate exclusions fairly, build escalation and termination for persistent failure, and check the remedy limitation that may cap the whole structure. This is contract support; final SLA structures, especially for mission-critical or regulated services, require qualified counsel.

## Core Rules

### Define Service Levels In Measurable, Verifiable Terms

A service level is only useful if it can be measured and verified by both parties. Define each metric precisely: what is being measured, the measurement method, the measurement window, the data source, and the calculation. For uptime, define what counts as available, what counts as downtime, whether it is measured monthly or annually, and how partial outages are treated. For response and resolution times, define the severity tiers, the clock start and stop, and the business hours. Vague levels like "commercially reasonable efforts" or "industry standard" are unenforceable and leave disputes unresolved. Specify the metric and the math.

### Align Measurement With Business Impact

Measurement should reflect the impact that matters to the customer. An uptime percentage calculated across all features may hide critical-feature outages; a response time measured during business hours may ignore the customer's 24/7 needs. Align the measurement windows, severity tiers, and excluded periods with the customer's actual operations, so that a passing SLA reflects real service quality. A 99.9 percent uptime that excludes the customer's peak hours is less valuable than it appears.

### Design Service Credits That Have Real Value

Service credits are the primary remedy for SLA failure, and their value depends on their structure. Check the credit percentage per level of failure, the cap on total credits, whether credits are automatic or must be requested, the timeframe for claiming, and whether they offset future fees or require cash refund. A credit of a few percent of monthly fees, capped at a small amount, may be commercially trivial compared to the customer's loss. Determine whether the credit is meaningful relative to the service's importance, and consider stepped credits that increase with the severity or duration of failure.

### Allocate Exclusions And Force Majeure Fairly

SLAs exclude certain periods from the measurement, and the exclusions determine the real obligation. Common exclusions are scheduled maintenance, customer-caused issues, third-party outages, force majeure, and issues outside the provider's control. Check that exclusions are specific and limited, that scheduled maintenance is noticed and timed to minimize impact, and that force majeure is not so broad it swallows the SLA. Over-broad exclusions can make a 99.9 percent commitment effectively much lower. Confirm the customer cannot be excluded for issues the provider caused or contributed to.

### Build Escalation And Termination For Persistent Failure

Credits compensate for individual failures, but persistent failure may require exit. Build an escalation path for repeated or prolonged failures, with management notice, remediation plans, and defined thresholds. Include a termination right for persistent failure, such as failure to meet a level for a defined number of months or failure to remedy within a cure period, so the customer is not trapped in a permanently broken service. Without escalation and termination, the customer's only remedy is a stream of inadequate credits. Define the trigger, the cure, and the consequence.

### Coordinate Credits With The Sole Remedy And Limitation Clauses

SLA credits often operate as the sole remedy for service level failure, and the limitation of liability may cap total liability including credits. Check whether the credit is the exclusive remedy, whether it sits inside or outside the liability cap, and whether consequential damages are excluded. A credit that is the sole remedy and capped by the limitation clause can leave the customer unable to recover real losses. Determine whether SLA failures should carve out of the liability cap, especially for critical services, and coordinate the credit, sole remedy, and limitation language consistently.

### Require Transparent Reporting And Audit Rights

The customer needs data to verify SLA performance. Require the provider to report performance against each level, on a defined cadence, with the underlying data or a portal the customer can check. Consider audit or verification rights, especially where the provider self-reports. Without reporting, the customer cannot know whether levels are met or claim credits. Specify the report content, frequency, and the consequences of failing to report.

### Account For Dependencies, Subcontractors, And Third Parties

Services often depend on subcontractors, cloud infrastructure, telecommunications, and third-party APIs. Determine whether the provider remains responsible for SLA performance when a dependency fails, or whether third-party failures are excluded. A provider that passes through third-party outages as exclusions can effectively disclaim responsibility for the service. Push for the provider to remain responsible for its chosen dependencies, with rights against those third parties flowing to the provider, not the customer.

### Match SLA Strictness To The Service's Criticality

Not every service needs the strictest SLA. Match the levels, credits, and termination rights to the service's importance to the customer's business. Mission-critical, regulated, or safety-related services warrant stricter levels, higher credits, and easier termination; commodity or low-impact services may accept standard levels. Calibrating the SLA to criticality avoids overpaying for unnecessary strictness on minor services and under-protecting on critical ones.

## Common Traps

### Vague Or Unmeasurable Service Levels

"Commercially reasonable" is not a metric. Define the measurement method and math.

### Measurement That Hides Critical Failures

An uptime percentage across all features can mask critical outages. Align measurement to business impact.

### Credits Too Small Or Capped To Matter

A trivial credit capped at a fraction of fees does not incentivize performance. Size credits to impact.

### Over-Broad Exclusions And Force Majeure

Broad exclusions gut the SLA. Keep them specific and limited.

### No Termination Right For Persistent Failure

Without exit, the customer is stuck with credits for broken service. Build escalation and termination.

### Sole Remedy And Liability Cap Swallowing Real Losses

A credit that is the sole remedy and capped can bar recovery of real damages. Coordinate the clauses.

### No Reporting Or Audit Rights

Without data, the customer cannot verify performance or claim credits. Require transparent reporting.

### Passing Through Third-Party Failures As Exclusions

Excluding dependency failures lets the provider disclaim responsibility. Keep the provider accountable for its dependencies.

## Self-Check

- Are service levels defined with precise metrics, measurement methods, windows, data sources, and calculations?
- Does the measurement align with the customer's business impact, including critical features and operating hours?
- Are service credits structured with meaningful percentages, caps, claim process, and stepped increases for severity or duration?
- Are exclusions and force majeure specific, limited, noticed, and not so broad as to gut the SLA?
- Is there an escalation path and a termination right for persistent or prolonged failure, with defined triggers and cure?
- Are credits coordinated with the sole remedy and limitation of liability clauses, with consideration of carve-outs for critical services?
- Does the provider report performance transparently, with content, cadence, and audit or verification rights?
- Is the provider accountable for subcontractor, cloud, and third-party dependencies rather than passing them through as exclusions?
- Is the SLA strictness calibrated to the service's criticality, with stricter terms for mission-critical or regulated services?
- Does the output flag that final SLA structures for mission-critical or regulated services require qualified counsel?
