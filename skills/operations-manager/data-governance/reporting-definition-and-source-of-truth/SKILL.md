---
name: reporting-definition-and-source-of-truth.md
description: Use when the agent is defining operational metrics, source-of-truth rules, dashboard definitions, reporting ownership, KPI calculations, metric changes, data caveats, or conflicting reports used for operating decisions.
---

# Reporting Definition And Source Of Truth

Operational reports influence staffing, escalation, performance judgment, customer promises, and executive decisions. When definitions are unclear or competing reports disagree, teams argue about numbers instead of fixing the operation. This skill helps the agent define reporting source of truth, metric meaning, ownership, and caveats so reports support decisions rather than create confusion.

## Core Rules

### Start with the decision the report supports

Define what decision, review, or action the metric will support: staffing, backlog recovery, service health, quality intervention, vendor escalation, incident response, customer communication, budget review, or performance management. A metric without decision use can become decoration.

Do not design a report around available fields only. Start from the operating question, then identify the data needed and whether it is reliable enough.

### Define each metric precisely

For each metric, specify numerator, denominator, inclusion rules, exclusion rules, time window, timezone, refresh timing, status meanings, segment filters, start and stop events, pause rules, and owner. Define whether the metric is real-time, daily, weekly, monthly, or retrospective.

Terms like "resolved," "open," "handled," "late," "defect," "available," and "active customer" need explicit definitions. Common words are often the source of reporting disputes.

### Establish source-of-truth hierarchy

Name which system or report wins for each decision. A CRM may own customer data, a ticketing system may own case status, finance may own revenue, and workforce tools may own staffing. If reports combine sources, define transformation and reconciliation rules.

When two reports disagree, the team should know whether the issue is timing, filter, definition, data defect, or source hierarchy. Do not let stakeholders choose whichever number supports their position.

### Show caveats where decisions happen

Caveats should be visible in or near the report: refresh delay, excluded segments, manual adjustments, known data gaps, temporary definition changes, migration impacts, and confidence limitations. A caveat hidden in a separate glossary will not protect decision quality.

If a metric is directional rather than exact, say so. If it is not fit for a high-stakes decision, state the boundary.

### Govern metric changes

Changing a definition, filter, source, or calculation can change perceived performance. Define who approves changes, how version history is preserved, how users are notified, and how historical comparisons are handled. Some changes may require parallel reporting during transition.

Do not change metrics silently to make work look better or to align with a preferred narrative. Reporting governance protects trust.

### Segment without losing comparability

Segments help reveal risk by queue, product, region, customer type, vendor, shift, or channel. But too many ad hoc segments can create inconsistent stories. Define standard segments and when custom analysis is allowed.

Ensure segments roll up correctly or explain why they do not. Operations needs both local detail and comparable enterprise view.

### Separate operating, financial, contractual, and regulatory views

The same event may be counted differently for operational control, finance close, customer contract reporting, and regulatory evidence. Define which view applies to each audience and avoid forcing one metric to satisfy every purpose. If numbers differ, explain why and how they reconcile.

This prevents operations from changing a useful live metric to match a finance definition, or finance from relying on an operational snapshot that was never intended for formal reporting.

### Assign ownership and review cadence

Reports need owners for definitions, data quality, access, refresh, issue resolution, user questions, and retirement. Define review cadence to confirm the report still matches current operations. If a report no longer drives decisions, retire or archive it.

Unowned reports persist long after definitions and workflows change, creating false confidence.

## Common Traps

- Treating a dashboard as source of truth without defining the underlying fields and rules.
- Letting different teams use different definitions for the same metric.
- Hiding caveats away from the decision forum.
- Changing report filters or status meanings without version history.
- Using a metric for performance management when it was designed only for trend monitoring.
- Over-segmenting until no one can compare results.
- Keeping legacy reports alive after operations changed.
- Treating operational, financial, contractual, and regulatory metrics as interchangeable; leaving users to infer whether a metric is for live action, trend learning, formal reporting, or performance judgment

## Self-Check

- Is the report tied to a specific operating decision, review, or action?
- Are numerator, denominator, inclusions, exclusions, time window, timezone, refresh, status meaning, segment filters, start, stop, pause rules, and owner defined?
- Is there a source-of-truth hierarchy for conflicting systems and combined reports?
- Are caveats, delays, excluded segments, manual adjustments, and confidence boundaries visible where decisions are made?
- Are metric changes governed with approval, version history, user notification, and historical comparison guidance?
- Are standard segments defined with clear roll-up logic or known limits?
- Are operating, financial, contractual, and regulatory views separated or reconciled where definitions differ?
- Are report ownership and review cadence assigned for definitions, data quality, access, refresh, issue resolution, and retirement?
- Has the report been retired, revised, or caveated if it no longer matches current operations?
- Is each metric labeled by intended use so readers do not apply it to the wrong decision?
