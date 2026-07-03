---
name: logistics_kpis_and_service_performance.md
description: Use when the agent is defining logistics KPIs, service metrics, transportation or warehouse scorecards, on-time performance, order accuracy, fill rate, cost metrics, operational dashboards, performance reviews, and tradeoffs between service, cost, quality, and customer experience.
---

# Logistics KPIs And Service Performance

Logistics KPIs shape what teams optimize. Poor metrics can make an operation look efficient while customers wait, inventory is wrong, carriers reject freight, or costs shift to another department. Agents often list common KPIs without defining ownership, data source, thresholds, and tradeoffs. Strong performance management measures service, cost, quality, speed, reliability, and exception behavior in a way that helps teams make better decisions.

Use this skill when building logistics dashboards, scorecards, service reviews, or KPI definitions.

## Core Rules

### Tie KPIs To Service Promises And Operating Decisions

Metrics should reflect what the operation promises and what managers can change. If customers are promised same-day shipment, measure order release, pick completion, pack completion, carrier handoff, and delivery separately. If the business sells high availability, measure fill rate, backorders, and stockout recovery.

Avoid measuring what is easy instead of what matters. A warehouse can have high units per hour while missing priority orders. Transportation can have low cost per mile while customers receive late deliveries. Start from the promise and trace the process.

### Define Metrics Precisely

Every KPI needs a definition: numerator, denominator, timestamp, exclusions, data source, owner, update cadence, and target. "On time" is ambiguous unless it states whether it means ship by promise, pickup, arrival, delivery appointment, or signed proof of delivery.

Ambiguous KPIs cause arguments and gaming. Define before comparing teams, carriers, or facilities. If data is weak, improve data governance before using metrics for penalties or bonuses.

### Balance Cost, Service, Quality, And Risk

Single metrics drive distortion. Cost per order may improve by cutting labor and increasing errors. On-time shipping may improve through premium freight. Inventory turns may improve while stockouts rise. Productivity may improve while safety worsens.

Use balanced scorecards. Pair cost with service, service with quality, speed with safety, and utilization with resilience. The right set should expose tradeoffs rather than hide them.

### Separate Leading, Lagging, And Diagnostic Metrics

Lagging metrics show what happened: delivery performance, claims, cost, customer complaints. Leading metrics show risk before failure: backlog age, dock dwell, tender rejection, inventory below safety stock, labor absence, order hold aging. Diagnostic metrics explain cause: scan compliance, ASN accuracy, pick errors, lane rejection reasons.

Use each type appropriately. Executives need concise outcomes and risks; operations needs diagnostic detail for daily action. A dashboard overloaded with all metrics helps no one.

### Segment Performance By Meaningful Context

Averages hide problems. Segment by customer, channel, lane, facility, carrier, SKU class, service level, order type, region, and season where relevant. A 95 percent service level may hide chronic misses for strategic accounts or remote regions.

Do not oversegment until analysis becomes noise, but always check whether averages are masking high-impact failures. Segment metrics should support decisions, not just curiosity.

### Include Exception And Recovery Performance

Operations are judged not only by normal flow but by how exceptions are handled. Track expedite reason codes, exception age, customer communication timeliness, claims closure, backorder recovery, missed appointment recovery, and root-cause closure.

This prevents teams from celebrating average performance while expensive exceptions pile up. Recovery metrics show resilience and process discipline.

### Assign Ownership And Action Thresholds

A metric without an owner is a statistic. Define who reviews it, what threshold triggers action, and what action is expected. For example, tender rejection above a threshold triggers routing guide review; backlog age triggers overtime or cutoff change; claims spike triggers packaging review.

Targets should be realistic and tied to business value. Perfect performance may be too expensive; weak targets may tolerate customer harm. Use thresholds that drive decisions.

### Review Metrics For Unintended Behavior

People optimize what is measured. If a KPI encourages skipped scans, rejected difficult orders, inflated forecasts, delayed receipts, or premium freight, revise it. Metrics should encourage system performance, not local gaming.

Periodically ask whether the KPI still reflects current strategy, customer promises, and operating reality. As the network changes, metrics should evolve.

## Common Traps

### Using Undefined "On Time"

On-time ship, pickup, arrival, and delivery are different. Define the event.

### Measuring Only Cost

Cost metrics without service and quality can reward harmful cuts.

### Measuring Only Averages

Averages can hide high-impact customer, lane, or SKU failures.

### Confusing Activity With Outcome

Orders processed, calls made, or miles driven may not show whether service improved.

### Ignoring Leading Indicators

Waiting for late deliveries or complaints is too slow. Monitor risk before failure.

### Creating Too Many KPIs

Dashboards with dozens of equal metrics dilute attention. Prioritize decision-driving metrics.

### Failing To Assign Ownership

Metrics need owners, thresholds, and actions. Otherwise they become reports.

### Letting Metrics Drive Unsafe Behavior

Productivity and speed targets must not undermine safety, accuracy, or compliance.

## Self-Check

- [ ] Are KPIs tied to customer promises and operating decisions?
- [ ] Is each metric defined with numerator, denominator, timestamp, source, exclusions, owner, cadence, and target?
- [ ] Are cost, service, quality, speed, safety, resilience, and risk balanced?
- [ ] Are leading, lagging, and diagnostic metrics distinguished?
- [ ] Is performance segmented by customer, channel, lane, facility, carrier, SKU, service level, or region where useful?
- [ ] Are exception and recovery metrics included?
- [ ] Does each KPI have an owner and action threshold?
- [ ] Are targets realistic and linked to business value?
- [ ] Are metrics reviewed for gaming or unintended behavior?
- [ ] Could a manager use the dashboard to decide what to do next?
