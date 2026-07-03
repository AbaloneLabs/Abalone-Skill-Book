---
name: replenishment_safety_stock_and_stockouts.md
description: Use when the agent is planning replenishment, safety stock, reorder points, stockout response, allocation, inventory buffers, supplier lead times, demand variability, service levels, and tradeoffs between working capital, availability, expediting, obsolescence, and customer commitments.
---

# Replenishment Safety Stock And Stockouts

Replenishment decisions turn demand uncertainty into inventory commitments. Too little stock creates stockouts, expedites, lost sales, production stops, and customer churn. Too much stock ties up cash, fills warehouses, increases obsolescence, and hides planning problems. Agents often treat safety stock as a formula or a fixed buffer and miss lead-time variability, demand spikes, supplier reliability, service-level promises, order minimums, shelf life, and criticality. Strong replenishment planning makes the tradeoff between availability and inventory cost explicit.

Use this skill when reviewing reorder policies, safety stock, stockout response, or inventory availability strategy.

## Core Rules

### Tie Inventory Policy To Service Promise

Inventory levels should reflect the service promise and consequence of failure. A critical spare part, healthcare item, high-margin fast mover, or contractual retail SKU deserves different protection than a slow-moving accessory. Do not apply one blanket safety-stock rule across all items.

Define target service levels by SKU segment, customer tier, channel, and criticality. If the business promises same-day shipment, inventory must support that promise. If customers can tolerate lead time, lower stock may be appropriate.

### Separate Demand Variability From Lead-Time Variability

Stockouts can come from demand spikes, supplier delays, transportation disruption, receiving backlog, quality holds, or inaccurate inventory. Safety stock should reflect both demand variability and replenishment lead-time variability.

Measure actual lead time, not only supplier quoted lead time. Include order approval, supplier processing, production, pickup, transit, customs, receiving, inspection, putaway, and system availability. Inventory is not usable until it is available for fulfillment or production.

### Segment SKUs For Different Policies

Use segmentation by velocity, margin, criticality, lead time, variability, substitutability, shelf life, storage requirement, value, and obsolescence risk. ABC value analysis is useful but incomplete; a low-value item can be critical if it stops production or completes a kit.

Design policies by segment: min-max, reorder point, periodic review, make-to-order, buy-to-order, vendor-managed inventory, consignment, forward stocking, or central stocking. Policy should match item behavior and business risk.

### Include Order Constraints And Practical Replenishment

Replenishment is constrained by minimum order quantities, case packs, pallet quantities, supplier schedules, container fills, freight economics, shelf life, warehouse capacity, cash, and purchase approvals. A mathematically ideal reorder quantity may be impossible or uneconomic.

Consider lot size, order frequency, inbound capacity, and transportation mode. For imported goods, container economics may push larger orders, but that increases inventory risk. For perishables, smaller frequent orders may protect freshness.

### Manage Stockouts With Priority Rules

When stock is short, decide allocation deliberately. Which customers, orders, regions, channels, kits, or production lines receive limited stock? First-come-first-served may be fair in some cases and harmful in others. Critical orders, contractual commitments, safety needs, or strategic accounts may need priority.

Create stockout response rules: allocate, substitute, split, backorder, expedite, transfer, cancel, or communicate delay. Customer service and sales should know the rule before they promise recovery.

### Monitor Early Warning Signals

Stockout prevention requires signals before zero inventory. Track days of supply, demand spike, supplier delay, open PO risk, inbound ETA, receiving backlog, allocation pressure, forecast bias, backorder growth, and inventory accuracy issues. Use exception alerts for items approaching risk thresholds.

Do not wait until orders fail. If inbound supply is late, decide whether to expedite, transfer, ration, adjust promises, or notify customers while options remain.

### Balance Safety Stock Against Obsolescence And Capacity

Safety stock is not free. It consumes cash and space and can become obsolete, expired, damaged, or markdown-prone. High buffers may hide poor forecasts or unreliable suppliers. For short life-cycle, fashion, electronics, seasonal, or perishable items, excess stock may be worse than occasional stockouts.

Review slow-moving and aging inventory. Adjust policy when demand changes. Safety stock should be a conscious buffer, not inventory that no one wants to question.

### Feed Stockout Root Causes Back Into Planning

A stockout may not mean safety stock was too low. It may come from bad forecast, late supplier, inaccurate inventory, receiving delay, system allocation, promotion not communicated, unexpected customer order, or policy mismatch. Root cause determines the fix.

Track stockouts by cause and cost: lost sale, expedite, substitution, customer penalty, production downtime, or churn. Use that evidence to improve supplier management, forecasting, order policy, and service promises.

## Common Traps

### Using One Safety Stock Rule For Everything

Different items carry different service, lead-time, value, and obsolescence risk. Segment policy.

### Ignoring True Lead Time

Supplier lead time does not end at shipment. Include transit, customs, receiving, inspection, and putaway.

### Treating Forecast As Fact

Forecasts are uncertain. Use variability and error, not only expected demand.

### Allocating Scarce Stock By Accident

If no allocation rule exists, the system may consume limited stock in order sequence and miss strategic priorities.

### Expediting Every Stockout

Expedite may be justified, but frequent premium freight indicates planning or supplier problems.

### Hiding Obsolescence Behind High Service Goals

High availability can create excess and obsolete inventory if life cycle and demand decline are ignored.

### Ignoring Inventory Accuracy

Replenishment policies fail if system stock is wrong. Accuracy and replenishment are linked.

### Failing To Communicate Supply Risk

Sales and customer service need early warning before they promise unavailable product.

## Self-Check

- [ ] Are inventory policies tied to service promises, customer commitments, and failure consequence?
- [ ] Are demand variability and true replenishment lead-time variability both considered?
- [ ] Are SKUs segmented by velocity, value, criticality, lead time, variability, substitutability, shelf life, and obsolescence risk?
- [ ] Are reorder quantities aligned with MOQs, case packs, freight, inbound capacity, cash, and storage constraints?
- [ ] Are stockout allocation, substitution, split, backorder, transfer, expedite, and cancellation rules defined?
- [ ] Are early warning signals monitored before inventory reaches zero?
- [ ] Is safety stock balanced against cash, space, shelf life, markdown, and obsolescence?
- [ ] Are stockout causes tracked separately from symptoms?
- [ ] Are sales, customer service, procurement, and operations aligned on supply risk?
- [ ] Would the policy change if demand, lead time, supplier reliability, or service promise changes?
