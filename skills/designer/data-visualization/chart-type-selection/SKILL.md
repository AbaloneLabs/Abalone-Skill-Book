---
name: chart_type_selection.md
description: Use when the agent is choosing a chart type for data, designing reports, building analytics views, selecting between bar, line, pie, scatter, heatmap, table, or area charts, or deciding how to represent a dataset so users can read the right comparison accurately.
---

# Chart Type Selection

The chart is not a decoration for data; it is the argument the data makes. A bar chart and a pie chart of the same numbers tell different stories, support different comparisons, and invite different mistakes. Choosing a chart because it looks modern, or because it is the default in a library, is how dashboards end up beautiful and misleading at the same time.

Use this skill before designing or reviewing any data visualization: reports, analytics dashboards, metric cards, comparison views, trend displays, distribution charts, rankings, and tables. The goal is to prevent the agent from picking a chart by aesthetics or habit and instead matching the chart to the question the user is trying to answer and the comparisons they must make accurately.

## Core Rules

### Start From The User's Question, Not The Data's Shape

The first decision is what the user needs to learn from the chart. Different questions demand different chart families.

Map question to chart:

- comparison across categories: bar chart, column chart;
- change over time: line chart, area chart, column chart;
- part of a whole: stacked bar, treemap, and only sometimes pie;
- distribution: histogram, box plot, density;
- relationship between two variables: scatter plot, bubble chart;
- ranking: sorted bar chart;
- geographic pattern: map with encoded values;
- flow or composition: sankey, funnel, waterfall.

If you cannot state the user's question in one sentence, you do not yet know which chart is right.

### Match The Chart To The Comparison The User Must Make

Users read charts to compare values. The chart should make the needed comparison easy and accurate, and it should not invite comparisons that are meaningless.

Consider:

- bar lengths are compared accurately; pie slice angles and areas are not;
- lines show trend and rate of change; bars show discrete values;
- a shared baseline lets users compare magnitudes; floating elements do not;
- dual-axis charts invite false correlation and should be used rarely and labeled exhaustively;
- stacked bars make totals easy but segments hard to compare;
- 3D charts distort perception and should almost never be used for precise reading.

Choose the chart whose perceptual strengths match the comparison, not the chart whose visual style matches the brand.

### Respect The Data Type And Scale

The measurement level of the data constrains valid chart choices. Treating categorical data as continuous, or continuous data as categorical, produces nonsense.

Rules of thumb:

- categorical data: bars, columns, pies, treemaps;
- ordinal data: bars in a meaningful order, never alphabetical if order carries meaning;
- continuous data: lines, areas, histograms, scatter;
- time series: lines or columns with time on a consistent axis;
- percentages: ensure they sum correctly and label the denominator;
- counts versus rates: never compare raw counts when rates are the fair comparison.

A chart that violates the data type reads as plausible but is wrong.

### Prefer Accuracy Over Decoration

Many visually impressive charts trade accuracy for appeal. Gradients, 3D effects, area encoding, and animation can distort the values the user is trying to read.

Avoid or constrain:

- 3D bar and pie charts, which distort proportions;
- area and volume encoding, which humans read poorly;
- truncated y-axes that exaggerate differences, unless the baseline is clearly justified;
- log scales used without clear labeling, which mislead casual readers;
- excessive animation that obscures the final values;
- decorative gradients that reduce contrast on the data marks.

If a decorative choice makes a value harder to read, it is the wrong choice for an analytical chart.

### Choose Aggregation And Granularity Deliberately

The same raw data can be summarized into very different charts. Aggregation hides detail; too much detail hides pattern. The right granularity depends on the question.

Decide:

- daily, weekly, or monthly buckets for time series;
- top-N versus all categories, with an "other" bucket when needed;
- averages versus totals versus medians, each telling a different story;
- whether outliers should be shown, capped, or annotated;
- whether the user needs raw values available, such as via a table or tooltip.

Aggregation is a judgment that changes the chart's meaning. State the aggregation method, do not hide it.

### Provide A Table Or Detail Path For Precise Reading

Charts are for pattern and comparison; tables are for exact values. Users often need both. A chart that forces users to estimate a precise value from a pixel position is incomplete.

Support precision by:

- showing values on hover or via tooltips;
- offering a linked table or detail view;
- labeling key data points directly;
- providing a data download for analysts;
- annotating important values rather than leaving them implied.

### Account For Edge Cases In The Data

Real data is messy: empty categories, missing values, single data points, negative numbers, and extreme outliers. A chart chosen for clean sample data may collapse on real data.

Test the chart against:

- a single data point, which makes trends meaningless;
- all-zero or null values, which need explicit empty states;
- negative values, which break area charts and pie charts;
- extreme outliers, which compress the rest of the scale;
- very long category labels, which break axes and legends;
- high cardinality, which turns bars into noise.

## Common Traps

### Pie Charts For Comparison

Humans read angles and areas poorly. A pie chart makes close values indistinguishable and invites false precision. Use bars for comparison; reserve pie for simple part-of-whole with few slices.

### Truncated Y-Axis To Exaggerate

Starting a bar chart at 50 instead of 0 makes a 5% difference look like a doubling. This is a common way to mislead, intentionally or not.

### Dual-Axis Charts Implying Causation

Two lines on dual axes that appear to move together suggest correlation that may not exist. Dual-axis charts require heavy labeling and should be used sparingly.

### 3D And Decorative Distortion

3D effects tilt the baseline, skew proportions, and make values harder to read. They add appeal at the cost of accuracy.

### Default Charts From A Library

The library's default chart is chosen for generality, not for your question. Always justify the choice against the user's comparison need.

### Ignoring Aggregation Effects

Averaging can hide variance; summing can hide rate. The same data aggregated differently tells opposite stories. Always disclose the aggregation.

### Forgetting The Empty And Single-Value Cases

A trend chart with one data point is not a trend. A chart with no data is not a chart. These states need explicit design, not a broken render.

## Self-Check

- [ ] The chart type was chosen to answer a specific user question, and that question can be stated in one sentence.
- [ ] The chart's perceptual strengths match the comparison the user must make, such as bars for magnitude, lines for trend.
- [ ] The data type and scale are respected: categorical, ordinal, continuous, time series, percentage, count, or rate.
- [ ] Decorative choices like 3D, gradients, and animation do not reduce the accuracy of value reading.
- [ ] The y-axis baseline and scale are justified, and truncated or log axes are clearly labeled.
- [ ] Aggregation and granularity are deliberate and disclosed, not hidden defaults.
- [ ] A path to precise values exists via tooltips, direct labels, a linked table, or data download.
- [ ] The chart was tested against edge cases: single value, empty data, negatives, outliers, long labels, and high cardinality.
- [ ] Dual-axis and pie charts are used only where justified and are heavily labeled.
- [ ] The chart communicates the intended insight without inviting misleading comparisons.