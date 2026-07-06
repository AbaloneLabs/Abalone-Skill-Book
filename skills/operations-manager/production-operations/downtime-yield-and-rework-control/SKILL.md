---
name: downtime-yield-and-rework-control.md
description: Use when the agent is reviewing production downtime, yield loss, scrap, rework, OEE, line stoppages, maintenance impact, quality loss, defect containment, or production recovery where risks include misleading availability metrics, hidden rework, unsafe restart pressure, weak downtime coding, yield gains that shift defects downstream, or recovery plans that ignore root cause and customer impact.
---

# Downtime Yield And Rework Control

Downtime, yield loss, and rework are tightly connected. A line may run more hours but produce more scrap; a team may recover downtime by rushing and create defects; a yield improvement may simply move inspection failures downstream. Agents often analyze these metrics separately and recommend generic root cause work. This skill helps the agent inspect production losses as an integrated control problem.

Use this skill when reviewing OEE, downtime, scrap, rework, stoppages, yield, quality holds, production recovery, or manufacturing loss reports. The agent should make loss categories accurate enough to drive action.

## Core Rules

### Define loss categories clearly

Separate planned downtime, unplanned downtime, minor stops, speed loss, startup loss, scrap, rework, quality hold, changeover loss, material shortage, no labor, maintenance, utilities, and waiting on decisions. If categories are vague, teams will code losses inconsistently and the data will not support action.

Do not let "other" or "operator issue" become a large bucket. Ambiguous coding hides the real constraint.

### Check data quality before conclusions

Downtime and yield data may come from machine logs, operator entries, quality systems, ERP, spreadsheets, or supervisor notes. Check timestamps, auto/manual entries, category definitions, shift handoffs, rework capture, scrap timing, and whether short stops are recorded.

If data quality is weak, improve capture while using interviews and observation carefully. Do not build a major improvement plan from unreliable loss codes.

### Link downtime to restart quality

Restart after stoppage can create scrap, safety risk, contamination, calibration issues, or first-piece failure. Recovery pressure can make teams skip checks. Review whether downtime recovery includes safe restart criteria and quality verification.

A fast restart is not good if it creates hidden rework or customer defects. The restart process should match the cause and product risk.

### Treat rework as controlled production

Rework is not free correction. It consumes labor, capacity, materials, traceability, quality approval, and sometimes customer confidence. Define what rework is allowed, who approves it, how it is documented, and how reworked product is verified.

Do not let rework become an invisible buffer that protects yield metrics. If rework is not captured, the operation may believe the process is healthier than it is.

### Analyze yield by process step and condition

Overall yield can hide where loss occurs. Review yield by line, product, batch, supplier lot, operator experience, shift, tooling, machine, environmental condition, startup period, and changeover type. Patterns reveal process drift and material issues.

Avoid blaming final inspection for defects created upstream. Trace where the defect should have been prevented or detected earlier.

### Balance recovery with customer and inventory impact

Downtime recovery choices include overtime, resequencing, partial shipments, substitute production, expedited transport, backlog reprioritization, or deferring lower-priority work. Each choice affects cost, service, inventory, and fatigue.

When output is short, communicate affected orders, shortage quantity, confidence, recovery plan, and customer promise impact. Do not hide production loss until shipping fails.

### Separate containment from permanent correction

Short-term containment may keep production moving through sorting, temporary inspection, extra maintenance watch, or controlled rework. That is not the same as eliminating the cause. Label temporary controls, assign expiration or review points, and decide what permanent correction is required before the workaround becomes the new normal.

### Escalate recurring or high-risk losses

Frequent minor stops, repeated scrap, chronic rework, or recurring maintenance issues may deserve formal problem solving, engineering support, supplier action, capital investment, or product design review. Use thresholds to trigger escalation.

Do not normalize losses because they are familiar. Chronic loss is often accepted because it has become part of the daily plan.

### Verify improvement against all loss types

After changes, review availability, performance, quality, scrap, rework, safety, maintenance cost, customer complaints, and inventory effects. An improvement in one metric can harm another.

If the change reduced downtime but increased scrap, the system did not improve. If scrap improved by rejecting more incoming material, supplier and procurement impact must be included.

## Common Traps

- Using broad downtime codes that cannot drive action.
- Treating machine availability as success while yield, rework, or safety worsens.
- Skipping restart checks to recover lost time.
- Hiding rework outside yield or productivity metrics.
- Blaming final inspection instead of tracing where defects were created.
- Reporting production loss too late for customer, logistics, or planning action.
- Treating temporary containment as permanent correction because output recovered.
- Accepting chronic minor stops as normal because they are familiar; proving improvement with one metric while ignoring cost, quality, safety, and service impact

## Self-Check

- Are downtime, speed loss, scrap, rework, changeover, shortage, and decision-wait categories clear?
- Is loss data reliable enough to support the conclusion?
- Are short stops, manual entries, shift handoffs, rework, and scrap timing captured?
- Does restart after downtime include safety and quality verification?
- Is rework approved, documented, traceable, and verified?
- Is yield analyzed by step, line, product, lot, shift, tooling, startup, and changeover where relevant?
- Are recovery plans tied to customer orders, inventory, cost, overtime, and fatigue impact?
- Are temporary containment actions labeled with owner, review point, and permanent correction path?
- Are recurring losses escalated with thresholds and owners?
- Are improvements checked across availability, performance, quality, safety, cost, and customer outcomes?; would the loss report still be credible if challenged by production, quality, maintenance, and planning teams?
