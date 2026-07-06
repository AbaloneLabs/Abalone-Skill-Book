---
name: data-visualization-choices.md
description: Use when the agent is choosing how to visualize data for a story, selecting chart types and scales, designing graphics that are accurate and not misleading, deciding how much complexity a general audience can handle, or balancing clarity aesthetics and honesty in data presentation.
---

# Data Visualization Choices

A chart can communicate a quantitative truth more powerfully than a thousand words, or it can mislead more effectively than any loaded sentence. Data visualization is not decoration; it is a form of argument, and every choice, chart type, axis scale, color, truncation, aggregation, conveys meaning. The harm this skill prevents is the publication of visualizations that distort the data, whether through intent or carelessness: truncated axes that exaggerate change, misleading proportions, cherry-picked time windows, or aesthetic choices that obscure rather than reveal. Honest visualization serves the reader's understanding; misleading visualization, even unintentional, serves a conclusion the data may not support.

## Core Rules

### Match the chart type to the message and the data
Different chart types suit different purposes: line charts for trends over time, bar charts for comparing categories, scatter plots for relationships, maps for geographic patterns. Choose the form that best reveals the specific point the data supports. A mismatched chart type obscures the signal and can imply a relationship that does not exist.

### Start axes at a meaningful baseline unless there is a justified reason not to
Bar charts with truncated y-axes exaggerate differences and are a leading source of misleading graphics. For quantities, start the axis at zero; for cases where a non-zero baseline is genuinely informative, justify it and make the scale clear. The reader should perceive the magnitude of differences as they actually are.

### Choose scales and ranges that represent the data fairly
The choice of time window, the bin sizes in a histogram, and the scale, linear versus logarithmic, determine what the chart appears to show. Select these to represent the data honestly, and test whether the visual story changes under reasonable alternatives. If the finding depends on one convenient scaling, it is fragile and possibly misleading.

### Label clearly and provide context
A chart should be interpretable without relying on the surrounding text. Label axes with units, title the chart to state what it shows, include a subtitle with the source and time period, and annotate anything non-obvious. Readers frequently encounter charts stripped of their context; clear labeling protects against misreading.

### Use color and emphasis to reveal, not to bias
Color draws the eye and signals importance. Use it to highlight the data's actual story, not to push a conclusion. Avoid color schemes that imply judgment where none is warranted, and ensure the visualization is legible to colorblind readers and in grayscale. Decorative effects that distort perception, 3D bars, disproportionate icons, undermine accuracy.

### Show uncertainty rather than hiding it
Point estimates without error bars or confidence intervals imply a false precision. Where the data carries uncertainty, show it: error bars, ranges, shaded confidence bands. A chart that acknowledges uncertainty is more honest and, to a careful reader, more credible than one that pretends to certainty.

### Avoid charting in ways that invite misinterpretation
Dual-axis charts, area charts that obscure the baseline, pie charts with too many slices, and maps that conflate totals with rates all invite misreading. Recognize the common pitfalls of each chart type and avoid them, or add explicit guidance so the reader interprets the chart correctly.

### Prioritize the reader's understanding over visual impressiveness
The goal is comprehension, not spectacle. A simple, honest chart that readers understand beats an impressive one that distorts or confuses. Resist the pressure to make data dramatic; let the data's actual significance determine the visual treatment.

## Common Traps

### Truncated axes that exaggerate change
Starting a bar chart's axis at a non-zero value to make a small difference look dramatic is the classic misleading-visualization move. It is often done carelessly rather than maliciously, which makes it no less damaging. Audit every chart for whether the visual magnitude matches the real magnitude.

### Conflating totals with rates
Mapping total cases by region, without adjusting for population, makes large areas look worst simply because they have more people. Use rates or per-capita measures when comparing across units of different size, and be explicit about which you are showing.

### Misleading proportional and icon-based charts
Icons that scale in two dimensions, or area-based charts where the visual proportion does not match the data proportion, distort perception. Ensure the visual encoding matches the numerical encoding exactly.

### Cherry-picked time windows
A trend that appears over one chosen period may disappear or reverse over a slightly different one. Show the data over a range that represents the relevant history, and test whether the visual story survives reasonable changes to the window.

### Dual-axis charts implying false correlation
Two lines on dual axes can be scaled to appear correlated regardless of the underlying relationship. Use dual axes rarely and with clear justification, and never to imply a relationship the data does not establish.

### Overloading the chart with complexity
Cramming too many series, dimensions, or annotations into a single chart overwhelms readers and obscures the point. Simplify to the elements that carry the message; if the story is complex, use multiple focused charts rather than one cluttered graphic.

### Aesthetic choices that distort meaning
Gradients, 3D effects, and decorative elements can make data look bigger, smaller, or more dramatic than it is. Every visual element should serve comprehension; anything that distorts perception must be removed.

## Self-Check

- [ ] Does the chart type match the message and the structure of the data?
- [ ] Are axes on a meaningful baseline, with any non-zero start clearly justified?
- [ ] Are scales, ranges, time windows, and bin sizes chosen to represent the data fairly, and does the story survive reasonable alternatives?
- [ ] Is the chart fully labeled, with axes, units, title, source, and time period, so it can be understood without surrounding text?
- [ ] Does color and emphasis reveal the data's actual story rather than pushing a conclusion, and is it legible to all readers?
- [ ] Does the visualization show uncertainty, through error bars, ranges, or confidence bands, where it exists?
- [ ] Have I avoided chart types and configurations that invite misinterpretation, such as misleading dual axes or totals mapped as rates?
- [ ] Does the visual magnitude of differences match the real magnitude in the data?
- [ ] Is the chart simple enough to comprehend, with complexity handled through multiple focused graphics rather than clutter?
- [ ] Before publication, would a skeptical, numerate reader find this chart honest and accurately representative of the data?
