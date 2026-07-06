---
name: production-schedule-and-line-readiness.md
description: Use when the agent is preparing or reviewing production schedules, line readiness, shift starts, material availability, labor coverage, equipment readiness, production release, work order sequencing, daily production plans, or manufacturing execution handoffs where risks include infeasible schedules, missing materials, unready equipment, skill gaps, hidden quality holds, unrealistic output promises, or starting production before constraints are resolved.
---

# Production Schedule And Line Readiness

A production schedule is only useful if the line can actually run it. Agents often treat the schedule as a list of work orders and miss the readiness conditions that make output possible: materials, labor, tooling, equipment, quality release, instructions, utilities, and downstream capacity. This skill helps the agent inspect production readiness before work is released and before missed output becomes an escalation.

Use this skill when reviewing daily production plans, shift readiness, work order release, line startup, schedule sequencing, or handoff between planning and execution. The agent should make constraints visible rather than smoothing them into an optimistic plan.

## Core Rules

### Separate plan from executable schedule

The demand plan or master schedule states what should be produced. The executable schedule states what can run with confirmed materials, labor, equipment, tooling, instructions, quality status, and time. Do not treat planned quantity as executable output until readiness is checked.

If the schedule is infeasible, name the missing constraint and decision required. A schedule that assumes all constraints will resolve by shift start creates hidden risk for supervisors and operators.

### Confirm material and component readiness

Check availability of raw materials, components, packaging, labels, consumables, work-in-progress, lot requirements, expiration dates, substitutions, and quality release. Material "on site" may still be unavailable if it is quarantined, not kitted, short counted, unlabelled, or in the wrong location.

For high-mix production, verify the full bill of materials, not only the main component. A missing label, fastener, cleaning supply, or test fixture can stop the line.

### Validate labor and skill coverage

Production output depends on the right number of trained people in the right roles. Check operator coverage, certifications, supervisors, quality inspectors, maintenance support, material handlers, relief coverage, and overtime limits. Headcount alone is not enough if required skills are missing.

If the plan relies on cross-trained staff or temporary labor, confirm readiness and supervision. Skill gaps often appear as lower yield, slower cycle time, safety issues, or rework rather than immediate stoppage.

### Check equipment, tooling, and utilities

Before release, confirm equipment status, preventive maintenance constraints, tooling availability, calibration, clean-down state, software recipe, fixtures, spare parts, utilities, safety guards, and prior downtime history. A machine that is technically available may not be ready for the planned product.

If equipment is running under temporary workaround or maintenance watch, include that risk in the schedule. Do not hide fragile assets behind normal capacity assumptions.

### Sequence work intentionally

Sequence affects changeover time, cleaning requirements, allergen or contamination risk, color or material transitions, tooling, labor, due dates, shelf life, and downstream shipping. A first-in-first-out work order sequence may be easy but inefficient or risky.

When resequencing, check customer commitments, material expiration, quality holds, and downstream constraints. A sequence change can solve one line problem while creating a shipping or inventory problem elsewhere.

### Use readiness gates before shift start

Define what must be confirmed before production begins: materials staged, equipment cleared, work instructions current, quality checks complete, staffing assigned, safety checks done, and first-piece approval path ready where relevant. A quick readiness meeting can prevent hours of reactive coordination.

Readiness gates should be scaled to risk. Routine work may need a light check; new product, high-risk material, customer-critical order, or post-maintenance startup needs stronger review.

### Communicate schedule risk early

If output is at risk, notify planning, customer operations, logistics, procurement, quality, and leadership as appropriate. Include affected orders, likely shortfall, reason, recovery options, decision needed, and next update time.

Avoid vague updates such as "line issue" or "materials delay." Downstream teams need enough detail to change customer promises, shipments, staffing, or priorities.

### Review schedule attainment honestly

After execution, compare planned output, actual output, downtime, material shortages, labor gaps, quality holds, changeover losses, and schedule changes. Distinguish planning errors from execution errors. A schedule that was impossible should not be reported as execution underperformance.

Use the review to improve readiness checks, planning assumptions, staffing, maintenance, and material staging.

## Common Traps

- Treating the production plan as executable before materials, labor, equipment, and quality are confirmed.
- Counting material as available when it is quarantined, unstaged, expired, mislabeled, or incomplete.
- Planning with headcount but ignoring certifications, role coverage, and supervision.
- Assuming equipment is ready because it is not formally down.
- Sequencing work only by due date while ignoring changeover, cleaning, contamination, or downstream effects.
- Starting production while first-piece, safety, or quality gates are unclear.
- Reporting missed output as poor execution when the schedule was infeasible.
- Communicating schedule risk too late for downstream teams to adjust.

## Self-Check

- Is the executable schedule separated from the demand or master plan?
- Are materials, components, packaging, labels, consumables, and quality release confirmed?
- Is labor coverage checked by role, skill, certification, supervision, and relief need?
- Are equipment, tooling, calibration, cleaning, software, utilities, and safety readiness confirmed?
- Is work sequencing justified by changeover, quality, due date, shelf life, and downstream constraints?
- Are readiness gates scaled to job risk and completed before start?
- Are schedule risks communicated with affected orders, cause, recovery option, owner, and next update?
- Are missed outputs reviewed against plan feasibility as well as execution performance?
- Are readiness failures fed into planning, staffing, maintenance, and material staging improvements?
- Would the line supervisor agree that the schedule is truly runnable?
