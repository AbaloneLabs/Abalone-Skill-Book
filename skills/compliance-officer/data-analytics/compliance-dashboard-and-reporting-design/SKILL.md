---
name: compliance_dashboard_and_reporting_design.md
description: Use when the agent is designing compliance dashboards or reports, building metric hierarchies, tailoring compliance information for different audiences, or ensuring that compliance reporting drives action rather than merely displaying numbers.
---

# Compliance Dashboard And Reporting Design

A compliance dashboard is a decision tool, not a display. Its purpose is to help a specific audience understand whether compliance risk is under control, where it is not, and what to do about it. Most dashboards fail this test: they show too many metrics of the wrong kind, lack thresholds and context, cannot be drilled to root cause, and are presented identically to audiences with very different needs. A dashboard that looks comprehensive but does not change behavior is a failure of design, not a triumph of data. Effective compliance reporting requires deliberate choices about audience, metric hierarchy, thresholds, trend, and actionability.

Use this skill before building a dashboard, redesigning compliance reporting, deciding what to report to a given audience, or evaluating whether current reporting is useful. The goal is to make the agent treat each report as a purpose-built instrument with a defined audience, a clear question it answers, and a path from signal to action.

## Core Rules

### Start From The Audience And The Decision They Must Make

A dashboard designed without a clear audience becomes a dump of everything available. Different audiences need different information.

For each report define:

- the primary audience, such as the board, audit committee, executive management, line-of-business leaders, or the compliance team;
- the decisions that audience is expected to make from the report;
- the level of detail appropriate to their role and time;
- the frequency they need;
- the actions that should follow from reading it.

The board needs a small set of strategic indicators tied to top risks, with trend and threshold context. Line leaders need operational detail tied to their area, with enough granularity to act. The compliance team needs diagnostic depth to investigate and remediate. One report for all audiences serves none of them.

### Build A Metric Hierarchy That Rolls Up Consistently

Reporting should allow a reader to move from a high-level signal to its drivers without losing consistency. A flat list of metrics does not support this.

Structure a hierarchy:

- top-level indicators tied to the organization's top compliance risks;
- supporting indicators that explain movement in the top-level metrics;
- operational drivers that can be drilled to identify the source of a change;
- consistent definitions and calculations at every level so that the numbers reconcile.

If the board metric and the operational metric use different definitions, readers will be confused and trust will erode. Maintain a single source of truth for each metric and document the calculation so that every level rolls up from the same data.

### Lead With Thresholds And Trends, Not Raw Counts

A number without context is hard to interpret. Ten hotline calls could be high or low depending on baseline, population, and history.

Present:

- the current value against a defined threshold or target, with green, amber, and red bands;
- the trend over a meaningful period, not only the latest point;
- comparison to prior period, forecast, or peer benchmark where relevant;
- the direction of concern, so the reader knows whether rising is good or bad;
- context for significant changes, such as population shifts or detection changes.

A dashboard that shows only current values invites misreading. Trend and threshold transform a number into a signal. Where a metric is new and lacks history, state that and avoid premature conclusions.

### Ensure Every Metric Has A Defined Action Path

A metric that does not lead to action is noise. For each indicator, define what should happen when it breaches threshold.

Define:

- the action required at each threshold level;
- the owner accountable for the action;
- the timeframe for response;
- the escalation path if action does not occur;
- how the action and its result are reported back.

Reports should show not only the metric but the status of actions taken in response to prior breaches. A dashboard full of red cells with no corresponding actions is a sign that reporting has become ritual. Action tracking closes the loop and demonstrates that reporting drives management.

### Choose The Right Cadence For Each Report And Audience

Cadence should match how quickly the underlying risk and the audience's decision cycle move. Too frequent wastes attention; too infrequent misses changes.

Set cadence by considering:

- how quickly the risk can change;
- how quickly the audience can and should act;
- the cost of producing the report;
- regulatory or governance expectations for reporting frequency;
- whether the report is for oversight or for operational intervention.

Board and audit committee reporting is typically quarterly or at defined meetings; operational reporting may be weekly or daily for fast-moving risks. Align cadence to the decision, and avoid producing reports on a schedule that no longer matches need simply because it always has.

### Avoid Clutter And Vanity Metrics

Dashboards accumulate metrics over time as stakeholders request additions. Without discipline, they become unreadable.

Apply discipline by:

- including only metrics tied to a defined risk, objective, or decision;
- removing metrics that nobody acts on, even if someone once asked for them;
- avoiding vanity metrics that make the program look good but carry no signal;
- resisting the urge to show every available number;
- periodically reviewing the report with users to confirm what they actually use.

A smaller, sharper report is more effective than a comprehensive one that nobody reads. Quality of signal matters more than quantity of metrics.

### Make The Report Defensible And Traceable

Compliance reporting may be reviewed by regulators, auditors, or investigators. It must be defensible.

Ensure:

- every metric has a documented definition, source, calculation, and owner;
- data quality limitations are disclosed rather than hidden;
- the report is reproducible from the documented source;
- versions are retained so prior reports can be reconstructed;
- significant changes in methodology or data source are noted on the report;
- confidentiality, privilege, and personal data are handled appropriately.

A report that cannot be reproduced or explained undermines credibility. Treat the report's metadata as part of the deliverable.

### Design For Drill-Down And Investigation

A high-level indicator that raises a question should allow the reader to find the answer. Static reports that stop at the top line force separate, slow investigations.

Design so that:

- a breached indicator can be broken down by business unit, geography, product, or time;
- the underlying cases, alerts, or issues can be accessed by authorized users;
- the path from summary to detail is documented and consistent;
- access controls protect sensitive detail while enabling legitimate drill-down.

Not every audience needs drill-down, but the compliance team and senior management should be able to move from signal to cause without rebuilding the analysis from scratch.

## Common Traps

### One Report For All Audiences

A single comprehensive report serves no audience well. Tailor to the audience and decision.

### Raw Counts Without Threshold Or Trend

A number without context is uninterpretable. Add thresholds, trends, and direction of concern.

### Metrics With No Action Path

A metric that triggers no action is decoration. Define the action, owner, and escalation for each.

### Clutter From Accumulated Additions

Reports grow until unreadable. Periodically prune to metrics that inform and are acted upon.

### Inconsistent Definitions Across Levels

Board and operational metrics that do not reconcile erode trust. Maintain a single source of truth.

### Hiding Data Quality Limitations

A clean-looking report built on poor data misleads. Disclose limitations.

### No Drill-Down From Signal To Cause

A top-line breach that cannot be investigated forces slow, separate work. Enable traceable drill-down.

## Self-Check

- Is each report designed for a defined primary audience, their decisions, appropriate detail, frequency, and expected actions?
- Does a metric hierarchy roll up consistently from top-level risk indicators to supporting indicators to operational drivers, with a single source of truth?
- Are metrics presented with thresholds or targets, trend, comparison, direction of concern, and context for significant changes rather than raw counts?
- Does every metric have a defined action, owner, timeframe, escalation path, and feedback of action status into the report?
- Is the cadence matched to how quickly the risk and the audience's decision cycle move, rather than inherited habit?
- Has the report been pruned to metrics tied to defined risks or decisions, with vanity and unused metrics removed?
- Is each metric defensible with documented definition, source, calculation, owner, disclosed data limitations, reproducibility, version retention, and methodology change notes?
- Are confidentiality, privilege, and personal data handled appropriately in the report and its distribution?
- Can authorized users drill from a breached indicator to breakdowns by unit, geography, product, and time, and to underlying cases or issues?
- Is the report periodically reviewed with users to confirm relevance and to retire metrics that no longer inform?
