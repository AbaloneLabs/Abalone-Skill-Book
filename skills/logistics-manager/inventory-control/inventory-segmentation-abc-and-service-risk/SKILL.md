---
name: inventory_segmentation_abc_and_service_risk.md
description: Use when the agent is segmenting inventory, applying ABC analysis, classifying SKUs by value, velocity, criticality, service risk, margin, lead time, shelf life, storage needs, or deciding differentiated policies for replenishment, counting, slotting, stocking, and service levels.
---

# Inventory Segmentation ABC And Service Risk

Inventory segmentation prevents the operation from managing every SKU the same way. ABC analysis is useful, but value alone does not capture customer impact, lead-time risk, margin, criticality, perishability, substitutability, or operational complexity. Agents often rank items by spend or volume and assume the top group deserves all control. That misses low-cost parts that stop production, slow movers that require special storage, seasonal items that expire, and high-margin items with volatile demand. Strong segmentation creates differentiated policies for service, replenishment, counting, slotting, and escalation.

Use this skill when classifying inventory or designing differentiated inventory policies.

## Core Rules

### Use ABC Analysis As A Starting Point, Not The Whole Policy

ABC analysis usually ranks SKUs by annual usage value, revenue, cost, or movement. It helps focus attention, but it can mislead if treated as complete. A low-cost item can be critical, and a high-value item may have stable demand and long lead time that is easy to plan.

Use ABC to identify where financial exposure is concentrated, then layer additional dimensions. Document what ABC is based on: sales value, cost value, unit volume, margin, or picks. Different bases produce different classifications.

### Add Criticality And Service Consequence

Ask what happens if the item is unavailable. Does it stop a production line, delay surgery, break a kit, cause retail chargebacks, lose a strategic customer, create safety risk, or simply disappoint a low-priority order? Criticality may outweigh dollar value.

Create service-risk categories that reflect failure consequence. Critical low-value items may need high availability, frequent counts, protected stock, or supplier backups. Noncritical high-value items may be managed more cautiously to avoid excess.

### Include Demand And Lead-Time Behavior

SKU policy should reflect demand variability and replenishment reliability. Stable fast movers are different from intermittent items with long supplier lead time. Volatile items require different buffers and monitoring than predictable items.

Segment by demand pattern: fast, slow, intermittent, seasonal, promotional, new, end-of-life, or project-based. Segment by lead time: short, long, variable, import-dependent, capacity-constrained, or single-source. Inventory policy should respond to behavior, not only rank.

### Consider Margin, Obsolescence, And Life Cycle

High-revenue items may have low margin, and low-volume items may be high margin or strategic. Some products become obsolete quickly because of fashion, technology, expiry, model change, packaging updates, or customer program end. Others are durable and safe to hold.

Segment by life-cycle stage: launch, growth, mature, decline, end-of-life, seasonal, or promotional. New items may need cautious ramp policies; end-of-life items need exit discipline. High obsolescence risk should limit overstock even if service is important.

### Account For Operational Complexity

Some SKUs consume disproportionate warehouse effort: oversized, fragile, refrigerated, hazmat, serialized, high-value, lot-controlled, kitted, regulated, theft-prone, or requiring special packaging. These items may need special slotting, counts, security, labor standards, and carrier choices.

Include operational handling in segmentation. A slow-moving hazmat item may not be financially large, but it deserves controls. A bulky low-margin item may strain storage and freight economics.

### Link Segments To Policies

Segmentation has value only if it changes decisions. Define policies for each segment: count frequency, safety stock, reorder method, supplier review, approval threshold, slotting, storage, replenishment priority, allocation, service level, expiry review, and exception escalation.

Avoid creating too many segments that no one uses. A practical segmentation scheme should be understandable, maintainable, and connected to system rules or planning routines.

### Review Segmentation Regularly

SKUs move between segments as demand changes, customers shift, life cycle advances, suppliers improve or deteriorate, and product mix changes. A launch item becomes mature; a mature item becomes end-of-life; a low-risk item becomes critical for a new customer.

Set review cadence and triggers: new product, discontinued product, forecast shift, supplier change, stockout, excess inventory, margin change, customer contract, or regulatory change. Stale segmentation creates stale policy.

### Align Stakeholders On Tradeoffs

Sales, finance, procurement, operations, and customer service may value segments differently. Sales may want availability; finance may want lower working capital; operations may want simpler handling; procurement may face MOQs. Segmentation should make tradeoffs explicit.

Use segmentation as a decision language. When stakeholders disagree, refer to segment rules and adjust them deliberately if business priorities change.

## Common Traps

### Treating ABC As Objective Truth

ABC depends on the metric chosen. Sales value, cost, margin, picks, and criticality can classify the same SKU differently.

### Ignoring Low-Value Critical Items

Cheap parts can stop high-value operations. Criticality must be layered on value.

### Overcomplicating Segments

Too many categories become unmanageable. Segments should drive practical policy differences.

### Failing To Link Segments To Actions

Classification without changed count, replenishment, service, or slotting policy is documentation, not management.

### Forgetting Life Cycle

Launch, seasonal, and end-of-life items need different policy from mature stable items.

### Missing Handling Complexity

Hazmat, cold chain, high-value, oversized, and serialized items need controls beyond value ranking.

### Letting Segmentation Go Stale

Demand, margin, supplier risk, and customer importance change. Review regularly.

### Excluding Stakeholders

Segmentation affects availability, cash, operations, and customer promises. It needs cross-functional agreement.

## Self-Check

- [ ] Is the basis for ABC classification clear: revenue, cost, margin, usage, or picks?
- [ ] Are criticality and consequence of stockout layered on top of ABC value?
- [ ] Are demand pattern and lead-time variability included?
- [ ] Are margin, obsolescence, shelf life, seasonality, and life-cycle stage considered?
- [ ] Are operational handling requirements such as hazmat, cold chain, serial, lot, oversized, fragile, or high-value included?
- [ ] Are segment rules tied to count frequency, safety stock, replenishment, slotting, allocation, service level, and escalation?
- [ ] Is the segmentation simple enough to maintain and use?
- [ ] Is there a review cadence and trigger process?
- [ ] Are sales, finance, procurement, operations, and customer service aligned on tradeoffs?
- [ ] Would a stockout or excess event cause a documented segment policy review?
