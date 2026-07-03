---
name: wip-finished-goods-and-plant-warehouse-handoffs.md
description: Use when the agent is managing manufacturing WIP, finished goods handoffs, plant warehouse interfaces, production completion, quality release, inventory transactions, staging, transfer orders, packaging, labeling, shipping readiness, or ownership boundaries between production and logistics.
---

# WIP Finished Goods And Plant Warehouse Handoffs

The boundary between production and warehouse is where inventory accuracy, quality status, and customer shipment readiness often break. Work-in-process, finished goods, quarantine, rework, scrap, and released stock can look physically similar while having very different permissions. The agent should design handoffs so material status, ownership, transactions, labels, and packaging are unambiguous. A pallet in the warehouse is not finished goods unless the system, quality, and documentation agree.

## Core Rules

### Define status before physical movement

Each unit or pallet should have a clear status: WIP, awaiting inspection, quality hold, rework, scrap, released finished goods, customer hold, export hold, or ready to ship. Physical movement should not change status unless the correct transaction and approval occur. Otherwise inventory appears available when it is not.

Use visual controls and system controls together. Labels, colors, locations, scan requirements, and hold areas should match ERP or WMS status. Mixed-status pallets are a common source of shipping errors.

### Make production completion criteria explicit

Finished goods completion may require final assembly, test, inspection, serialization, labeling, packaging, documentation, palletization, and quality release. Define the exact point at which logistics can receive and ship. If production says "done" but quality has not released, the warehouse should not treat inventory as available.

For regulated or serialized products, completion must include records. Missing certificate, batch record, test result, serial upload, or country label can block shipment.

### Control plant warehouse receiving

Plant warehouse receiving from production should have a standard process: scan, count, label, condition check, quality status check, location assignment, and discrepancy handling. Do not let production drop pallets into staging without transaction discipline. Staging areas need capacity rules and aging review.

If material moves between buildings or offsite warehouses, define transfer ownership, trailer seals, temperature or handling requirements, and in-transit visibility. Internal transfers can lose control just like external freight.

### Integrate quality release and inventory cutoffs

Production, quality, and logistics should agree on transaction timing. Backflushing, production receipt, quality release, warehouse putaway, and shipment allocation may happen in different systems or at different times. If the sequence is loose, the warehouse can allocate goods before release, production can close orders before discrepancies are known, or finance can recognize inventory that is not physically controlled.

Define cutoffs for shift end, month end, customer ship windows, and batch close. Late manual transactions, temporary spreadsheets, and after-the-fact corrections can distort ATP, inventory valuation, traceability, and service commitments. The handoff process should state what happens when a pallet is physically complete but documentation, testing, or system posting is delayed.

### Preserve containment and customer restriction controls

Quality containment often fails at handoff points because teams focus on moving pallets out of production space. Hold tags, suspect lots, customer-specific restrictions, export restrictions, and rework instructions must travel with the material and match system status. A warehouse should not depend only on verbal warnings that a pallet is not shippable.

For serialized, lot-controlled, regulated, or high-value goods, preserve chain of custody from production completion through storage and shipment. Define who can break a pallet, relabel material, consolidate lots, split orders, or move stock from hold to release. These permissions should be auditable because mistakes may surface as recalls, claims, chargebacks, or regulatory findings.

### Align packaging and shipping readiness

Finished goods may need customer labels, compliance marks, country of origin, export documents, serialization, pallet patterns, wrapping, temperature controls, or retail routing requirements. Packaging decisions made on the production floor can create shipping rejection or customer chargebacks.

Warehouse and production should agree on standard pack, overpack, dunnage, pallet height, weight, label placement, and damage criteria. If customer-specific packaging is required, make it visible before production completes.

### Reconcile discrepancies quickly

Handoff errors create inventory ghosts: produced not received, received not released, shipped from hold, duplicate transactions, wrong quantity, or wrong item. Track discrepancies by cause and owner. Reconcile daily for fast-moving or high-value items.

Repeated handoff issues usually indicate unclear ownership, rushed production close, bad labels, system timing, or quality bottlenecks. Fix the process, not only the transaction.

## Common Traps
- Treating physical warehouse arrival as finished-goods availability.
- Mixing WIP, quality hold, rework, scrap, and released stock in the same staging area.
- Letting production drop pallets without scans, counts, or status checks.
- Shipping before quality release or documentation completion.
- Missing customer-specific labels, country marks, serialization, or routing requirements.
- Losing control during internal transfers between buildings or offsite warehouses.
- Resolving discrepancies only in systems without checking physical cause.
- Ignoring aging staging inventory until customer orders fail.
- Allowing backflush, production receipt, quality release, and warehouse putaway to occur in an order no one has validated.
- Relying on verbal hold warnings instead of visible and system-enforced containment.
- Splitting, relabeling, or consolidating lots without preserving traceability and customer restrictions.

## Self-Check

- Is every unit or pallet status clear physically and in ERP or WMS before movement?
- Do labels, colors, locations, scans, and hold areas match WIP, quality, rework, scrap, release, and shipping status?
- Are finished goods completion criteria defined for assembly, test, inspection, serialization, packaging, documentation, and quality release?
- Does plant warehouse receiving include scan, count, condition, status, location, and discrepancy handling?
- Are internal transfers controlled for ownership, seals, handling, temperature, in-transit visibility, and receipt confirmation?
- Are customer labels, compliance marks, country of origin, packaging, pallet patterns, routing, and export needs visible before shipment?
- Are handoff discrepancies reconciled quickly with owner, cause, and physical verification?
- Are recurring produced-not-received, hold-shipped, duplicate, wrong-quantity, or aging-staging issues analyzed for root cause?
- Are backflush, production receipt, quality release, putaway, allocation, and shipment cutoffs sequenced so stock cannot become available too early?
- Does the process handle physically complete but documentation-blocked, test-blocked, or system-posting-blocked material without losing control?
- Are permissions clear for breaking pallets, relabeling, splitting lots, consolidating lots, releasing holds, and moving restricted stock?
