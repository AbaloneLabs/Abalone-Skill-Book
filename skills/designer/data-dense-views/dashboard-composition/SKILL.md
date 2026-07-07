---
name: dashboard_composition.md
description: Use when the agent is designing dashboards, composing widgets, cards, and panels into a coherent overview, choosing what metrics and summaries to surface, deciding layout hierarchy, or balancing at-a-glance status against drill-down detail so users can assess situation and act without drowning in widgets.
---

# Dashboard Composition

A dashboard is an overview surface where users assess a situation at a glance and decide where to act. Designing one is not arranging widgets on a grid; it is choosing which signals matter, how they relate, and how a user moves from "something needs attention" to the detail that explains it. Agents tend to fill dashboards with every metric available because each seems important, to give every widget equal weight so nothing reads as primary, and to treat the dashboard as a destination rather than as the start of a workflow. The harm is dashboards that answer no question: users see a wall of equally loud widgets, cannot tell what is urgent, and must open each one to find the one thing that needed action.

Use this skill before composing a dashboard, selecting which metrics and summaries to surface, or defining the path from overview to detail. The goal is to prevent the agent from building a dashboard around available data rather than user decisions, from flattening hierarchy so nothing stands out, or from leaving the user stranded at the overview with no path to act.

## Core Rules

### Start From The Questions The User Needs Answered

A dashboard exists to answer questions: what is the state, what needs attention, what should I do. The widgets should be selected to answer those questions, not to display available data. When the dashboard mirrors the database instead of the user's questions, it becomes a report no one reads. Begin with the questions.

Identify and prioritize:

- what is the user checking first when they open this surface?
- what conditions require their action?
- what context do they need to interpret the state?
- what is the path from a signal to the action it implies?

A dashboard that answers "are we okay, and if not, where" serves the user; one that lists fifty metrics answers nothing.

### Establish Clear Visual Hierarchy By Importance

Not all widgets are equal, and treating them as equal makes the dashboard unreadable. The most important signals, the ones the user checks first and the ones that flag action, must dominate. Secondary context and detail sit smaller and further from the focus. Hierarchy is how a dashboard becomes scannable in seconds.

Build hierarchy by:

- giving primary status and alert widgets the most space and prominence;
- grouping related widgets so the eye reads clusters;
- using size, position, and weight to signal importance, not just color;
- reserving the top-left and above-the-fold region for what matters most.

A grid of equally sized widgets forces the user to scan all of them; a hierarchical layout lets the eye land on what matters.

### Make Status And Alerts Immediately Readable

The core job of many dashboards is to show whether things are okay and to flag what is not. Status and alert widgets must be readable at a glance, with severity and trend clear without study. If the user must parse a chart to know something is wrong, the status communication has failed.

Design status for instant reading:

- use clear status indicators (good, warning, critical) with more than color;
- surface thresholds and breaches prominently, not buried in charts;
- show trend direction so the user sees whether things are improving or worsening;
- lead with the conclusion (needs attention) and support with the data.

A gauge that requires reading the scale to know it is critical is weaker than a gauge that says "critical" and shows the breach.

### Connect Every Signal To Its Action

A dashboard that surfaces a problem but offers no path to address it strands the user. Every alert, status, and metric that might require action should link to the detail or workflow that lets the user act. The dashboard is the start of a workflow, not a dead end.

Connect signals to actions by:

- making widgets and alerts link to the underlying detail or list;
- providing direct paths to common actions from relevant widgets;
- explaining what a signal means and what the user should consider doing;
- avoiding alerts that inform but offer no next step.

An alert that says "queue depth critical" with no link to the queue forces the user to navigate manually to act on it.

### Limit Widgets To What Serves A Decision

Dashboard sprawl is the most common failure. Every team wants their metric on the overview, and over time the dashboard accumulates widgets until it answers no question. The discipline is to include only what serves a user decision, and to move the rest to secondary views or reports.

Control sprawl by:

- including only widgets that answer a prioritized question or flag action;
- moving detailed and rarely-checked metrics to drill-down or secondary tabs;
- reviewing periodically and retiring widgets no one uses;
- resisting the instinct to add a widget for every available metric.

A focused dashboard of eight widgets is more useful than a sprawl of forty; discipline is what keeps dashboards alive.

### Group And Sequence By User Workflow

The arrangement of widgets should reflect how the user works through the situation, not an arbitrary grid. Related widgets sit together so the user can interpret them as a cluster, and the sequence guides the eye from overall status to detail. Layout should follow workflow.

Arrange by workflow:

- lead with overall status and the question "is everything okay";
- follow with the areas that need attention, grouped by domain;
- place supporting context near the widgets it explains;
- keep the path from summary to detail physically adjacent where possible.

### Design For The Range Of States And Time Ranges

Dashboards are not static. Data loads, ranges change, some widgets error, and the whole thing may be empty for a new account. The composition must hold across these states, and time-range control must work consistently across widgets.

Handle state and ranges by:

- designing loading states that hold the layout to avoid shift;
- handling widget-level errors without collapsing the whole dashboard;
- designing the empty state for new or no-data accounts;
- making time-range and filter controls apply consistently across linked widgets.

A dashboard where changing the time range updates some widgets but not others misleads the user about what they are seeing.

### Keep The Overview Separate From The Detail

A dashboard that tries to show full detail for everything becomes a report and loses its at-a-glance function. The overview should summarize; the detail should live one click away. Conflating the two serves neither the glance nor the deep look.

Separate overview from detail by:

- summarizing trends and status in the dashboard widgets;
- linking to tables, records, and full charts in detail views;
- resisting the urge to show every row or data point on the overview;
- letting users drill in when they need the detail, not forcing it on everyone.

## Common Traps

### Mirroring Available Data Instead Of User Questions

Dashboards built from available metrics rather than the questions users need answered become unreadable reports.

### Flat Hierarchy Of Equal Widgets

Grids of equally sized, equally weighted widgets force users to scan everything and let nothing stand out as primary.

### Status That Requires Study To Read

Alerts and gauges that demand chart-reading to determine severity fail the at-a-glance job of a dashboard.

### Signals With No Path To Action

Alerts and metrics that flag a problem but offer no link to the detail or workflow strand the user at the overview.

### Widget Sprawl

Adding a widget for every team's metric until the dashboard answers no question and no one reads it.

### Layout That Ignores Workflow

Widgets arranged in an arbitrary grid rather than grouped and sequenced by how the user works through the situation.

### Inconsistent Time-Range And Filter Behavior

Controls that update some widgets but not others mislead users about what the view represents.

### Conflating Overview With Detail

Dashboards that try to show full detail for everything lose their at-a-glance function and become unwieldy reports.

## Self-Check

- [ ] Widgets are selected to answer the user's prioritized questions (state, attention, action), not to display all available data.
- [ ] Visual hierarchy makes primary status and alert widgets dominate, with secondary context smaller, using size, position, and weight beyond color.
- [ ] Status and alerts are readable at a glance, with severity and trend clear without study, leading with the conclusion.
- [ ] Every signal that may require action links to the underlying detail or workflow, with a clear next step.
- [ ] Widgets are limited to those serving a decision, with detail moved to drill-down views and unused widgets retired periodically.
- [ ] Widgets are grouped and sequenced by user workflow, leading from overall status to areas needing attention with supporting context adjacent.
- [ ] Loading, error, empty, and time-range states are designed, with controls applying consistently across linked widgets.
- [ ] The overview summarizes while detail lives one click away, so the dashboard stays scannable rather than becoming a report.
