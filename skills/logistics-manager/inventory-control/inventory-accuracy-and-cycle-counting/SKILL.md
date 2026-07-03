---
name: inventory_accuracy_and_cycle_counting.md
description: Use when the agent is managing inventory accuracy, cycle counting, physical counts, stock discrepancies, location accuracy, shrink, inventory adjustments, root-cause analysis, system versus physical stock, and warehouse controls that affect fulfillment, replenishment, finance, and customer service.
---

# Inventory Accuracy And Cycle Counting

Inventory accuracy is the trust layer beneath every logistics decision. If the system says product exists in the wrong quantity, location, status, lot, or condition, the operation will promise orders it cannot ship, buy stock it does not need, miss replenishment, distort financial records, and waste labor searching. Agents often treat inventory counts as administrative cleanup and miss that accuracy is produced by daily process discipline. Strong inventory control uses cycle counting to find errors early, diagnose root causes, and protect service before the annual physical count exposes accumulated drift.

Use this skill when reviewing inventory accuracy, count programs, adjustments, or discrepancy resolution.

## Core Rules

### Define What Accuracy Means

Inventory accuracy has several dimensions: item, quantity, location, status, lot, serial, expiration, ownership, condition, and unit of measure. A SKU can be accurate in total quantity but wrong by location, making pickers search and orders miss cutoff. A pallet can exist physically but be unavailable because it is damaged, quarantined, expired, or allocated.

Decide which dimensions matter for the operation. Regulated, serialized, cold-chain, high-value, or lot-controlled goods require stricter accuracy than generic consumables. Accuracy metrics should reflect business risk, not only total unit count.

### Use Cycle Counting As A Control, Not A Ritual

Cycle counting should detect drift and drive correction. Count frequency should be based on SKU value, velocity, risk, shrink exposure, customer impact, prior errors, and regulatory requirements. High-value or high-velocity items may need frequent counts; slow, low-risk items may need less.

A cycle count program should include count schedule, blind count method, variance thresholds, recount rules, adjustment approval, root-cause coding, and follow-up. Counting without root-cause action only confirms that errors exist.

### Protect Transaction Discipline

Inventory accuracy is created by correct transactions: receiving, putaway, moves, replenishment, picking, packing, shipping, returns, quarantine, damage, scrap, kit build, de-kit, transfer, and adjustment. If workers move product physically without system transactions, accuracy degrades immediately.

Identify where physical flow and system flow diverge. Common weak points include emergency moves, short picks, damaged goods, returns, partial pallets, relabeling, substitutions, and manual workarounds. Improve the process so transactions are easy and required.

### Investigate Variances Before Adjusting

Inventory adjustments affect customer promises and financial records. Before adjusting, check nearby locations, open tasks, in-transit inventory, receiving errors, unit-of-measure issues, pending shipments, returns, damaged or quarantined stock, mislabels, and system timing. A variance may be a process timing issue rather than true shrink.

Use recounts and independent verification for material variances. Approvals should scale with value and risk. Adjustments should have reason codes that reveal patterns, not generic "inventory correction."

### Connect Accuracy To Slotting And Location Control

Poor location discipline creates picking errors and search time. Each location should have clear identity, capacity, status, and rules. Mixed SKUs, mixed lots, unlabeled pallets, overflow locations, and temporary staging can all create errors if not controlled.

Maintain location labeling, license plates, lot segregation, expiration rules, and empty-location discipline. If the warehouse uses temporary overflow, define ownership and system status. "Temporary" locations often become permanent sources of inaccuracy.

### Manage Returns, Damage, And Quarantine Separately

Returns and damaged goods are frequent accuracy traps. Product may physically exist but should not be sellable. Quarantined, expired, recalled, customer-owned, or pending-inspection inventory must be status-controlled.

Do not let returned or damaged product re-enter available inventory without inspection and transaction. For regulated or customer-sensitive goods, status accuracy is as important as quantity accuracy.

### Use Accuracy Data For Root Cause And Prevention

Track variance by SKU, location, process, shift, supplier, customer, worker group, and reason code. Patterns may reveal receiving errors, poor slotting, confusing packaging, training gaps, theft, system integration problems, or process shortcuts.

The goal is prevention. If cycle counts repeatedly find the same variance type, change the process, labels, layout, system rule, or training. Do not simply count more.

### Align Operations And Finance

Inventory adjustments affect financial statements, cost of goods, shrink, reserves, and audit. Finance may need controls over adjustment authority, documentation, count evidence, and physical inventory procedures. Operations needs timely adjustments to avoid service failures.

Define approval thresholds and documentation that satisfy both groups. Avoid delaying operational corrections so long that pickers keep failing, but do not allow uncontrolled adjustments that hide process problems.

## Common Traps

### Measuring Only Total Quantity

Total units can be right while locations, lots, status, or serials are wrong. Pickability matters.

### Counting Without Root Cause

Cycle counting that only adjusts balances does not improve the system. Every material variance needs investigation.

### Allowing Informal Moves

Physical moves without system transactions are a major cause of location inaccuracy. Emergency moves still need controls.

### Treating Returns As Available Too Soon

Returned goods may be damaged, incomplete, expired, wrong item, or customer-owned. Inspect and status-control.

### Using Generic Adjustment Reasons

"Correction" tells nothing. Reason codes should support prevention and accountability.

### Ignoring Unit Of Measure

Each, case, pallet, pack, and weight confusion creates large discrepancies. Check UOM setup and labels.

### Overcounting Low-Risk Items While Missing High-Risk Items

Count effort should follow value, velocity, customer impact, shrink, and regulatory risk.

### Separating Finance And Operations Too Much

Finance control and operational accuracy both matter. Design a process that supports both.

## Self-Check

- [ ] Is accuracy defined by item, quantity, location, status, lot, serial, expiration, condition, ownership, and unit of measure as relevant?
- [ ] Is cycle count frequency based on value, velocity, customer impact, shrink, prior errors, and regulatory risk?
- [ ] Are blind counts, recount thresholds, adjustment approvals, and reason codes defined?
- [ ] Are receiving, moves, replenishment, picking, shipping, returns, damage, quarantine, and adjustments transaction-controlled?
- [ ] Are material variances investigated before adjustment?
- [ ] Are location labels, overflow, mixed SKUs, lots, and empty-location discipline controlled?
- [ ] Are returns, damage, recall, expired, and quarantined goods status-controlled separately from available inventory?
- [ ] Are variance patterns analyzed for root cause by SKU, location, process, shift, supplier, or system?
- [ ] Are adjustment controls acceptable to both operations and finance?
- [ ] Does the count program reduce future errors rather than only correct current balances?
