---
name: dashboard_and_metric_design.md
description: Use when the agent is designing dashboards, metric cards, KPI displays, report layouts, monitoring views, analytics surfaces, or any interface where multiple metrics and charts must work together to support decisions and monitoring.
---

# Dashboard And Metric Design

A dashboard is not a grid of charts; it is a decision environment. Each metric, chart, and filter competes for attention, and the arrangement determines whether the user can answer "what needs my attention now" in seconds or drowns in equally weighted numbers. Dashboards fail most often not from missing data but from treating every metric as equally important and every chart as equally urgent.

Use this skill before designing or reviewing dashboards, KPI views, metric cards, monitoring consoles, report layouts, operations centers, or analytics surfaces. The goal is to prevent the agent from assembling charts by availability rather than by decision priority, and from showing numbers without the context needed to judge whether they are good, bad, or alarming.

## Core Rules

### Design Around The Decision, Not The Data Source

A dashboard exists to support a decision or monitoring task. Before placing any chart, state what the user is trying to decide or watch. Every element should serve that goal.

Decision-driven structure:

- lead with the metrics that trigger action, such as status, alerts, and key results;
- place supporting detail below or behind the primary signals;
- exclude metrics that do not change a decision, no matter how available;
- group related metrics so the user can move from signal to cause to action;
- align the dashboard's time window to the decision's cadence.

A dashboard that mirrors a database schema rather than a decision flow is a data dump, not a tool.

### Establish A Clear Priority Ladder Across Metrics

Not all metrics are equal. The dashboard should make instantly clear which numbers matter most, which are context, and which are reference.

Express priority through:

- size and position, with the most important metrics in the primary viewport;
- visual weight, reserving large type and color for headline numbers;
- grouping, separating primary KPIs from secondary detail;
- consistent placement so returning users build muscle memory;
- restraint, avoiding giving every card the same emphasis.

When every metric is large and colorful, none is prioritized, and the user must do the work the dashboard should have done.

### Provide Context For Every Number

A number alone is meaningless. The user needs to know whether it is good or bad, high or low, on track or off track. Context is what turns a metric into a signal.

Give each metric:

- a comparison, such as target, prior period, or benchmark;
- a direction indicator, up or down, with whether that is good or bad;
- a threshold or status band where relevant, such as green, amber, red;
- a unit and a clear definition of what is being measured;
- a time window so the user knows the scope.

"Revenue: $42,000" answers nothing. "Revenue: $42,000, up 12% vs target, on track" answers whether action is needed.

### Define Metrics Precisely And Consistently

Dashboards often show metrics whose definitions drift across teams or over time. The same label can mean different things, leading to conflicting conclusions.

Protect metric integrity:

- define each metric's numerator, denominator, and exclusions;
- keep definitions stable and versioned;
- surface the definition on hover or in a data dictionary;
- align metric names across dashboards so "Active users" means the same thing everywhere;
- document changes to definitions and their effect on historical values.

A metric whose definition is unknown cannot be trusted in a decision.

### Make Status And Exceptions Obvious

The primary job of a monitoring dashboard is to surface what needs attention. The user should see problems without hunting.

Surface exceptions by:

- using status color, paired with text or icon, for alert states;
- sorting or highlighting items that breach thresholds;
- providing a clear "needs attention" region or filter;
- avoiding burying alerts among healthy metrics;
- distinguishing actionable alerts from informational noise.

If the user must scan every chart to find the one problem, the dashboard has failed its core purpose.

### Support Drill-Down From Signal To Cause

A headline metric that signals a problem is only useful if the user can investigate it. The dashboard should link summary to detail.

Provide drill paths:

- from a KPI card to the chart or table behind it;
- from an alert to the relevant time window and segment;
- from a summary to the contributing factors or breakdowns;
- consistent filters that carry context into the detail view;
- a way back to the summary without losing state.

A dashboard that shows a spike but offers no path to explain it forces the user into a separate tool.

### Handle Loading, Empty, And Error States Explicitly

Dashboards depend on data that can be slow, missing, partial, or failed. These states must be designed, not left to default spinners or broken renders.

Design for:

- loading states that show structure so the layout does not jump;
- empty states that explain why there is no data and what to do;
- partial failure where some metrics load and others error;
- stale data indicators showing when the data was last refreshed;
- error states that name the problem and offer a retry.

A dashboard that shows a blank chart with no explanation leaves the user unsure whether the data is zero, missing, or broken.

### Match Density And Refresh To The Use Case

A real-time operations console and a monthly executive report have different needs. Density and refresh cadence should fit the task.

Calibrate by context:

- real-time monitoring: compact, high-density, auto-refreshing, alert-driven;
- operational dashboards: moderate density, frequent refresh, action-oriented;
- executive reports: lower density, periodic, narrative-supported;
- exploratory analytics: flexible, filterable, detail-rich, user-driven.

Applying one style to all dashboards serves none of them well.

## Common Traps

### Metric Grid With Equal Weight

Laying out a dozen metric cards in a uniform grid makes nothing stand out. Priority must be expressed through size, position, and emphasis.

### Numbers Without Context

A KPI card showing only a value gives no signal about whether it is good or bad. Every metric needs comparison, direction, or threshold.

### Decorative Charts That Answer No Question

Gauges, speedometers, and ornamental visuals consume space without supporting decisions. If a chart does not answer a question, remove it.

### Inconsistent Metric Definitions

"Active users" meaning different things on different dashboards destroys trust and causes conflicting reports. Definitions must be shared and documented.

### Alerts Buried Among Healthy Metrics

If problems are not visually distinct, the user misses them until too late. Exceptions need clear visual separation.

### No Path From Summary To Detail

A headline number that signals trouble but offers no drill-down forces users into another tool, breaking the workflow.

### Broken Or Silent Empty States

A blank chart with no message leaves the user unsure whether data is zero, missing, or failed. Every state needs explicit design.

## Self-Check

- [ ] The dashboard is structured around a specific decision or monitoring task, and every element serves that goal.
- [ ] A clear priority ladder distinguishes headline KPIs from supporting detail through size, position, and visual weight.
- [ ] Every metric includes context: comparison, direction, threshold, unit, definition, and time window.
- [ ] Metric definitions are precise, documented, stable, and consistent across all dashboards.
- [ ] Status, exceptions, and threshold breaches are visually obvious and not buried among healthy metrics.
- [ ] Drill-down paths link summary metrics to the charts, tables, and segments that explain them, with consistent filters.
- [ ] Loading, empty, partial, stale, and error states are explicitly designed with explanations and recovery actions.
- [ ] Density and refresh cadence match the use case: real-time monitoring, operational, executive, or exploratory.
- [ ] Decorative charts that answer no question have been removed in favor of decision-relevant visuals.
- [ ] The dashboard was reviewed for the returning user: predictable placement, muscle memory, and fast path to "what needs attention now."