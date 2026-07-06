---
name: material-flow-line-side-and-kanban-replenishment.md
description: Use when the agent is managing manufacturing material flow, line-side replenishment, kanban, supermarkets, milk runs, point-of-use inventory, kitting to line, material handlers, production feeding, inventory buffers, or logistics processes that prevent line starvation and excess WIP.
---

# Material Flow Line Side And Kanban Replenishment

Manufacturing logistics is judged at the line. A warehouse can look accurate while operators wait for parts, line-side areas overflow, kanban cards are ignored, or material handlers expedite all day. The agent should design material flow around production rhythm, part consumption, replenishment signals, ergonomic line-side storage, and exception response. The goal is not maximum inventory near the line; it is reliable availability with controlled space, quality, and labor.

## Core Rules

### Start from takt, consumption, and line layout

Understand production rate, build sequence, changeovers, bill of materials, part usage, container quantity, line-side space, replenishment frequency, and operator motion. Replenishment should support the line's actual rhythm. A two-bin system, kanban card, e-kanban, sequence cart, or milk run only works if signal timing matches consumption and replenishment lead time.

Do not size line-side inventory by habit. Some parts need point-of-use access; others belong in a supermarket or staged kit. Space at the line is valuable and clutter can create safety, quality, and efficiency problems.

### Design kanban signals with ownership

Kanban requires clear trigger, quantity, location, part identity, owner, and response time. If cards get lost, bins are mixed, barcodes fail, or no one owns replenishment, the system becomes informal expediting. Define who scans, moves, replenishes, audits, and corrects signals.

Set buffer logic by variability. High-runner stable parts can use tight replenishment; variable, imported, quality-risk, or long-lead parts may need different buffers. Avoid blindly applying one kanban size to all materials.

### Control line-side presentation and ergonomics

Parts should be presented in the right container, orientation, height, sequence, and quantity for operators. Poor presentation creates motion waste, picking errors, injuries, and quality defects. Heavy, sharp, fragile, ESD-sensitive, or contamination-sensitive parts need special handling.

Line-side rules should prevent mixed parts, obsolete revisions, damaged containers, blocked aisles, and excess pallets. Visual controls, standard locations, labels, color coding, and 5S matter because operators make fast decisions under production pressure.

### Coordinate material handlers and production

Material handlers need routes, timing, tugger or cart capacity, priority rules, empty return process, supermarket replenishment, battery or fuel plan, and escalation path. Production should not pull material randomly from storage because the system is late. If operators bypass the replenishment process, find the root cause rather than blaming them.

Milk runs should be measured by route adherence, missed signals, late replenishment, emergency calls, and line stoppage risk. A route that looks efficient on paper may fail if aisle congestion, forklift conflicts, or empty container returns are ignored.

### Connect material flow to production governance

Line-side replenishment should be visible in daily production control, not managed as an isolated warehouse chore. Shortage meetings, tier boards, and shift handoffs should distinguish between supplier shortage, supermarket shortage, line-side signal miss, inventory accuracy issue, quality hold, engineering change, and route capacity issue. Each category needs a different fix. Treating all misses as "material late" encourages extra inventory and hides whether planning, purchasing, quality, or logistics owns the correction.

Define escalation thresholds before a line is already down. For example, a missed first replenishment signal may trigger material handler review, a second miss may trigger supervisor support, and any line-down risk within the shift may trigger production control involvement. The escalation path should identify who can resequence production, release emergency material, approve temporary storage, request supplier support, or authorize overtime. Without authority mapping, everyone can see the shortage but no one can change the conditions that created it.

### Protect quality and traceability at the point of use

Point-of-use inventory must preserve traceability, lot control, FIFO, shelf life, revision level, inspection status, and customer restrictions where they apply. The closer material moves to the line, the easier it is for teams to treat it as universally usable. That is dangerous for serialized, regulated, safety-critical, lot-controlled, or customer-specific parts.

Design line-side storage so restricted material cannot be confused with general stock. If line operators need to break packs, decant parts, or move pieces into small bins, define how labels, lot identity, expiration, and remaining quantity are maintained. Replenishment convenience should not erase evidence needed for recalls, warranty analysis, or quality containment.

### Audit for drift and master data errors

Kanban systems drift as demand, BOM, pack quantity, supplier packaging, layout, and product mix change. Audit card counts, bin quantities, part numbers, location master, usage rates, and obsolete material. A wrong pack quantity or unannounced engineering change can destabilize replenishment.

Tie material flow to planning, procurement, quality, engineering, and production control. Line-side logistics cannot fix inaccurate BOMs or late supplier changes alone.

## Common Traps
- Adding more line-side inventory instead of fixing replenishment timing and signal quality.
- Sizing kanban without consumption, lead time, pack quantity, and variability.
- Letting cards, bins, and scans become optional during rush periods.
- Ignoring ergonomics, container orientation, and operator motion.
- Allowing obsolete or mixed-revision parts near the line.
- Measuring material handlers by activity rather than missed signals and line risk; designing milk runs without empty returns, aisle congestion, and route interruptions
- Failing to update kanban after BOM, demand, supplier pack, or layout changes; letting emergency replenishment become the normal method while the formal signal system decays
- Decanting or repacking parts in ways that lose lot, shelf-life, revision, or customer-restriction evidence; escalating shortages without classifying whether the cause is supplier, planning, quality, engineering, route, or inventory accuracy

## Self-Check

- Have I based replenishment on takt, consumption, line layout, pack quantity, and replenishment lead time?
- Is each kanban signal clear on trigger, quantity, location, part identity, owner, and response time?
- Are buffers adjusted for variability, long lead time, quality risk, import risk, and product mix?
- Does line-side presentation support operator ergonomics, correct orientation, safety, ESD, quality, and minimal motion?
- Are standard locations, labels, visual controls, obsolete material control, and blocked aisle prevention in place?
- Do material handlers have route timing, equipment capacity, empty return, priority rules, and escalation paths?
- Are route adherence, missed signals, emergency replenishment, and line stoppage risk measured?
- Are card counts, pack quantities, BOM changes, location master, and demand changes audited for drift?
- Does the escalation path name who can resequence production, authorize temporary buffers, release emergency stock, or involve suppliers?
- Are lot, FIFO, shelf life, revision, inspection status, customer restriction, and traceability requirements preserved after decanting or point-of-use storage?; do shift handoffs and tier meetings separate root causes instead of recording every issue as generic material lateness?
