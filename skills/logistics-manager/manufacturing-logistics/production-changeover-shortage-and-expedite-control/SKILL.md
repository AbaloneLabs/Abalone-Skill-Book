---
name: production-changeover-shortage-and-expedite-control.md
description: Use when the agent is managing manufacturing changeovers, material shortages, production expedites, line-down risk, shortage meetings, hot parts, allocation, engineering changes, supplier delays, production sequencing, or logistics decisions that affect manufacturing schedule adherence.
---

# Production Changeover Shortage And Expedite Control

Manufacturing shortages are rarely just missing parts. They are symptoms of planning assumptions, supplier performance, engineering changes, inventory accuracy, quality holds, sequencing, and communication failures. The agent should manage shortages and expedites through structured triage rather than panic. The goal is to protect production flow while preventing expediting from becoming the normal operating model.

## Core Rules

### Separate true shortage from data noise

Before expediting, verify on-hand inventory, WIP, quarantine, receiving, in-transit, supplier ASN, substitute parts, engineering revision, and consumption rate. A system shortage may be caused by wrong location, unposted receipt, quality hold, cycle count error, BOM error, or overconsumption. Expediting without verification can create excess and hide root causes.

Use a standard shortage status: line-down now, line-down within shift, risk within day, risk within week, quality blocked, engineering blocked, supplier late, or data discrepancy. This helps prioritize action.

### Protect changeover readiness

Changeovers need material, tooling, labels, packaging, instructions, quality checks, setup labor, first article approval, and line clearance. Verify readiness before the old run ends. If material arrives at the line after changeover starts, downtime is already built in.

For high-mix operations, create changeover kits or readiness gates. A production sequence should not be released unless critical parts and tools are confirmed. If the schedule changes frequently, logistics needs a controlled resequencing process.

### Triage expedites by production consequence

Expedites should be prioritized by line-down risk, customer criticality, revenue, regulatory obligation, perishability, and recoverability. Not every late part deserves air freight. A part needed next week should not consume the same attention as a part stopping the line in two hours.

Choose expedite mode by total recovery: supplier pickup, courier, air, dedicated truck, hand-carry, local transfer, alternate supplier, substitution, rework, or resequencing. Consider customs, receiving hours, quality release, and line readiness. A part arriving fast but stuck in inspection still fails.

### Run shortage control with explicit cadence

Shortage management needs a working rhythm. Define who updates the shortage list, how often status changes, what evidence is required, and when the issue escalates from planner to production control, procurement, supplier quality, engineering, logistics, or leadership. A stale hot list is worse than no list because teams make decisions from false confidence.

Use time-based commitments rather than vague updates. "Supplier will ship today" is not enough; the useful commitment includes pickup time, mode, tracking or driver identity, transit path, customs exposure, receiving window, inspection requirement, and expected line-ready time. If any step is uncertain, mark the shortage as unresolved. The control room should also show the next best production alternative if the promised recovery misses.

### Communicate consequences without creating panic

Shortage communication should be factual, ranked, and tied to decisions. Production leaders need to know what line, model, order, shift, and customer promise are at risk. Procurement needs to know which supplier action matters now. Logistics needs dimensions, pickup location, trade compliance constraints, and delivery deadline. Finance or leadership needs premium freight exposure and business reason.

Avoid broadcasting every possible risk to every stakeholder with equal urgency. That creates alert fatigue and encourages local optimization. Send the right issue to the role that can change the outcome, and keep a decision log for material allocation, premium freight approval, resequencing, and customer communication.

### Control allocation and substitution

When shortages affect multiple lines or customers, allocation needs decision authority. Do not let whichever supervisor shouts first take scarce material. Allocation should consider customer promises, production constraints, margin, contractual obligations, and downstream effects.

Substitutions require engineering, quality, regulatory, and customer approval where applicable. A physically similar part may not be approved. Track substitutions so inventory, warranty, and traceability remain correct.

### Close the loop after the shortage

Every serious shortage should have root cause and corrective action: supplier miss, forecast error, MOQ issue, engineering change, inventory accuracy, scrap spike, planning parameter, late receipt, quality hold, or logistics failure. Otherwise the same part becomes hot again.

Measure expedite spend, line impact, recurrence, premium freight cause, and recovery effectiveness. Premium freight should be evidence for process improvement, not a hidden budget leak.

## Common Traps
- Expediting based on system shortage without checking physical inventory and holds.
- Treating all shortages as equal.
- Starting changeover before material, tools, labels, and quality checks are ready.
- Air freighting parts that cannot clear customs, receiving, or inspection in time.
- Letting supervisors allocate scarce parts informally.
- Using unapproved substitutions because they look similar; closing a shortage once the part arrives without root cause
- Hiding premium freight as a cost of doing business; running shortage meetings from stale dates, informal messages, or supplier promises with no pickup and line-ready evidence
- Broadcasting every shortage as urgent, which buries the few decisions that actually need escalation; treating resequencing as free even when it creates changeover loss, labor imbalance, packaging conflict, or downstream customer failure

## Self-Check

- Have I verified inventory, WIP, quarantine, receiving, in-transit, supplier ASN, substitutes, revision, and consumption before expediting?
- Is shortage status classified by line-down timing, quality, engineering, supplier, or data cause?
- Are changeover materials, tools, labels, instructions, setup labor, line clearance, and first article checks ready before release?
- Are expedites prioritized by production consequence, customer impact, regulation, revenue, and recoverability?
- Does the expedite mode account for supplier pickup, transit, customs, receiving, quality release, and line readiness?
- Is scarce material allocation governed by clear authority and business rules?
- Are substitutions approved by engineering, quality, regulatory, and customer authorities where needed?
- Are root cause, corrective action, premium freight spend, recurrence, and line impact tracked after recovery?
- Does each hot part have evidence for pickup time, mode, tracking or driver identity, customs risk, receiving window, inspection need, and expected line-ready time?
- Is there a fallback production sequence or customer decision if the promised recovery misses?; have premium freight, allocation, substitution, resequencing, and customer-impact decisions been recorded with owner and reason?
