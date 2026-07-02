---
name: data_visualization_design.md
description: Use when the agent is designing a chart, dashboard, report, or data visualization; choosing a chart type for a given data shape and question; setting axis scales (linear vs log, truncated vs zero-based); selecting color palettes including colorblind-safe options; deciding what to emphasize or omit; building a dashboard layout; or reviewing a visualization for accuracy and honesty. Also covers the failure modes of misleading axis truncation, dual-axis charts that imply false correlation, 3D and decorative effects that distort perception, rainbow/color-encoding that is inaccessible and uninterpretable, cherry-picked time windows, and the recurring mistake of choosing a chart because it looks impressive rather than because it answers the question.
---

# Data Visualization Design

A visualization is an argument made in pixels. Every choice — chart type, axis scale, color, what is emphasized, what is omitted — shapes what the viewer concludes, and a visualization that "looks right" can systematically mislead. The judgment problem is that the visually impressive choice is often the misleading one: a 3D pie chart distorts proportions, a truncated y-axis exaggerates a modest change, a dual-axis line chart fabricates a correlation, a rainbow color scale is both inaccessible and perceptually distorted. The honest choice is usually plainer and harder: pick the chart that answers the specific question, scale the axes so the eye reads true magnitudes, encode with color the viewer can actually decode, and strip decoration that competes with the data. Data visualization is not decoration of numbers; it is a second analysis whose errors are as real as a wrong statistic.

Agents tend to miss these problems because the tooling makes impressive charts easy and the defaults are often wrong. A library's default bar chart may not start the y-axis at zero. A "show the trend" instinct reaches for a line chart on categorical data. A desire to "make it pop" reaches for saturated colors and 3D effects. The harm is decisions made on distorted perception: a team panics over a "spike" that is a truncated axis, misallocates resources based on a dual-axis mirage, or excludes colorblind users and print readers with an unreadable palette. The judgment problem is to design each visualization around the question it must answer and the perceptual reality of the viewer, treating honesty and legibility as hard requirements rather than aesthetic preferences.

This skill covers chart-type selection, axis and scale integrity, color and accessibility, decoration and emphasis, and dashboard composition. It complements the statistical-pitfalls skill (which covers the numbers behind the chart) and the experiment-metrics skill (which covers what to measure).

## Core Rules

### Choose The Chart Type By The Question And Data Shape, Not By Habit

Different chart types answer different questions, and the wrong type obscures the answer even when the data is correct:

- **Comparison across categories → bar chart.** Bars compare discrete magnitudes; the eye reads length accurately. Sort categories by value (not alphabetically) unless there is a natural order. Avoid pie charts for comparison: the eye is poor at comparing angles and areas, and pies cannot compare many categories.
- **Trend over time → line chart.** Lines show change across an ordered continuous variable (time). Use points to mark actual observations; connect them only when interpolation is meaningful. For discontinuous categories, do not use a line (it implies a false continuum).
- **Distribution → histogram or box plot.** To see the shape of a single variable (skew, modes, outliers), use a histogram (with appropriate binning) or a box plot. A bar chart of summary statistics hides the distribution.
- **Relationship between two variables → scatter plot.** To see correlation, clustering, and outliers across two continuous variables, use a scatter plot (with transparency or sampling for overplotting).
- **Part-of-whole → stacked bar or proportion, rarely pie.** If part-of-whole matters, a 100% stacked bar or a single stacked bar compares proportions better than a pie; for a few parts, direct labeling works.

State, for each chart: what question does it answer, and why is this type the honest way to answer it? If the answer is "it looks good," the type is wrong.

### Preserve Axis Integrity — Scale Determines Perception

The axis scale is where most visual distortion happens, because the eye reads the visual length/area, not the underlying number:

- **Bar charts must start the y-axis at zero.** Bars encode magnitude by length; a truncated axis (starting at 90 instead of 0) makes a 5% difference look like a doubling. This is the single most common misleading-chart error.
- **Line charts may use a non-zero baseline, but label it clearly.** Lines encode change, not magnitude, so a non-zero baseline can be honest — but the viewer must know the scale, and a truncated line axis still exaggerates volatility. Consider whether the zero-baseline tells a more honest story.
- **Linear vs logarithmic scale must match the phenomenon.** Use linear for additive comparisons (revenue, counts); use log for multiplicative/ratio phenomena (exponential growth, prices across orders of magnitude, skewed distributions). A linear axis on exponential growth flattens the recent explosion; a log axis can hide the magnitude of a spike. Label the scale explicitly.
- **Dual y-axes are almost always misleading.** Two series on two axes invite the viewer to infer correlation that depends entirely on the arbitrary axis scaling. If you must show two series together, use separate panels or index both to a common baseline.
- **Equal aspect ratio for spatial/coordinate data.** For maps, scatter plots of physical quantities, and anything where distance is meaningful, equal aspect ratio preserves geometric truth; stretched axes distort shapes and relationships.

### Encode With Color The Viewer Can Decode

Color is the most abused encoding. Used well, it distinguishes categories or encodes magnitude; used poorly, it is inaccessible, uninterpretable, and distorting:

- **Use colorblind-safe palettes.** The default red/green encoding is unreadable to ~8% of men (deuteranopia/protanopia) and fails in grayscale print. Use perceptually distinct, colorblind-safe palettes (e.g., Okabe-Ito, ColorBrewer's colorblind-safe sets).
- **Match color scale type to the data.** Categorical data needs a qualitative palette (distinct, unordered colors); sequential data (low to high) needs a sequential single-hue or ordered palette; diverging data (negative to positive around a midpoint) needs a diverging palette with a meaningful center. Using a rainbow palette for sequential data is both inaccessible and perceptually distorted (rainbow is not ordered to the eye and creates false bands).
- **Reserve saturated/accent color for what matters.** A chart where everything is brightly colored emphasizes nothing. Use a neutral palette for context and one accent color for the series or point the viewer should attend to.
- **Avoid red/green as the sole distinction, and avoid relying on color alone.** Pair color with shape, label, or position so the chart remains legible without color (for accessibility and for grayscale/print).

### Strip Deception And Decoration — Chartjunk Distorts And Distracts

Visual decoration that does not encode data actively harms comprehension: it distorts perception, competes for attention, and implies precision that is not there:

- **No 3D effects on 2D data.** 3D bars, pies, and lines distort the proportions they are supposed to convey (a 3D pie's foreground slices look larger) and add no information. Flat is more accurate.
- **No gradients, shadows, or glow on data marks.** These are decoration, not encoding; they make values harder to compare and the chart look like an advertisement.
- **Avoid area-encoded quantities unless the area is correct.** Bubble sizes and area charts must scale area (not radius/diameter) to the quantity; scaling the radius makes large values look far larger than they are.
- **Label directly over legends where feasible.** Direct labeling (series name next to its line) beats a legend the eye must map back and forth, especially with many series.
- **Remove gridlines, borders, and tick marks that do not aid reading.** Keep only what helps the viewer decode values; everything else is noise.

### Decide Emphasis And Omission Deliberately — They Are Editorial Choices

What a chart includes, excludes, and emphasizes is an editorial decision with honesty consequences:

- **Do not cherry-pick the time window.** A trend that "always goes up" on a six-month window may be flat or down on a five-year window. Choose the window that represents the phenomenon honestly; if a short window is used, show the longer context alongside.
- **Show uncertainty where it exists.** Confidence intervals, error bars, and bands communicate that estimates are uncertain; omitting them overstates precision. A line without a confidence band looks far more certain than the data supports.
- **Normalize or annotate when comparing unequal groups.** Comparing raw counts across groups of very different sizes is misleading; use per-capita or rate metrics, or annotate the size difference.
- **Decide what the viewer should conclude, and make that the visual focus.** A good chart has a point; arrange scale, color, and annotation so the honest conclusion is the one the eye reaches first. This is not manipulation — it is communication — but the conclusion must be true to the data.

### Compose Dashboards Around Decisions, Not Around Data

A dashboard is not a dump of every metric; it is a decision-support artifact. Its composition determines whether it informs or overwhelms:

- **Lead with the question and the primary metric.** The top-left, largest area answers the main question; supporting detail recedes below and to the right.
- **Limit density.** A dashboard with 30 charts is read as zero charts; the eye cannot find what matters. Group by theme, use the most compact honest chart per metric, and push detail to drill-downs.
- **Use consistent scales and colors across related charts** so the viewer can compare without re-orienting on each panel.
- **Annotate context (targets, baselines, anomalies).** A number without context (target, prior period, expected range) is hard to act on; show the reference the viewer needs to judge it.

## Common Traps

### Truncated Y-Axis On A Bar Chart

Starting a bar chart's axis above zero to exaggerate a modest difference, manufacturing a dramatic visual from a small change. Bar charts start at zero; if the difference is genuinely small, the honest chart shows it as small.

### Dual-Axis Line Chart Implying Correlation

Two series on two independent y-axes, where the apparent correlation depends entirely on arbitrary scaling; the viewer infers a relationship that may not exist. Use separate panels or a common baseline.

### 3D And Decorative Effects Distorting Proportions

3D pies, bars, or gradients that warp the proportions the chart is supposed to convey and add no information. Use flat, undecorated marks.

### Rainbow Or Red/Green Color Encoding

A rainbow palette for sequential data (perceptually disordered and inaccessible) or red/green as the sole distinction (unreadable to colorblind users and in grayscale). Use colorblind-safe, perceptually ordered palettes.

### Pie Chart For Comparison

Using a pie chart to compare many categories, where the eye cannot compare angles/areas accurately. Use a sorted bar chart for comparison.

### Line Chart On Categorical Data

Connecting discrete categories with a line, implying a false continuum and trend. Use a bar chart for unordered categories; reserve lines for ordered continuous variables (time).

### Cherry-Picked Time Window

Choosing a start or end date that manufactures the desired trend, hiding the longer context that contradicts it. Show the window that represents the phenomenon, with longer context where relevant.

### Omitting Uncertainty

Drawing a precise line or bar with no confidence interval or error bar, overstating the certainty of an estimate. Show uncertainty where it exists.

### Area Scaling By Radius Instead Of Area

Bubble or area charts where the radius (not area) scales with the quantity, making large values look disproportionately huge. Scale area to the quantity.

### Dashboard As Metric Dump

A dashboard with dozens of equally-sized charts and no hierarchy, overwhelming the viewer so nothing is read. Lead with the primary question; limit density; group by theme.

## Self-Check

- [ ] The chart type was chosen to answer a specific question for the data shape (bar for category comparison, line for time trend, histogram/box for distribution, scatter for relationship), not by habit or appearance.
- [ ] Bar charts start the y-axis at zero; line charts with non-zero baselines are labeled clearly; linear vs log scale matches the phenomenon and is stated; dual y-axes are avoided or used with explicit warning.
- [ ] Color encoding is colorblind-safe (no red/green-as-sole-distinction, no rainbow for sequential data), matches the data type (categorical/sequential/diverging), and uses an accent only for what matters; color is paired with shape/label so the chart is legible without color.
- [ ] No 3D, gradients, shadows, or decorative effects distort the data; area-encoded quantities scale area (not radius) to the value; direct labeling is used over legends where feasible.
- [ ] The time window is not cherry-picked (longer context shown where a short window could mislead), uncertainty (confidence intervals, error bars) is shown where it exists, and unequal-group comparisons are normalized or annotated.
- [ ] The chart's emphasis (scale, color, annotation) leads the eye to the honest conclusion, and that conclusion is true to the data.
- [ ] Dashboards lead with the primary question and metric, limit density, use consistent scales/colors across related panels, and annotate targets/baselines/anomalies for context.
- [ ] The highest-risk cases were verified — a truncated bar axis, a dual-axis mirage, 3D distortion, an inaccessible palette, a pie used for comparison, a line on categories, a cherry-picked window, and omitted uncertainty — not only the chart that "looks good."
