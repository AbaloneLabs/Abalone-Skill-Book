---
name: chart_design_and_honest_visualization.md
description: Use when the agent is creating charts or plots for a paper or report, choosing chart types, encoding data visually, scaling axes, selecting colors, avoiding misleading graphics, designing accessible figures, or reviewing whether a visualization honestly represents the data.
---

# Chart Design And Honest Visualization

A chart is an argument rendered in geometry. Every choice of chart type, axis scale, color, and encoding either reveals or distorts the underlying data. Agents often reach for visually striking defaults without considering how those choices manipulate perception, producing charts that exaggerate effects, hide uncertainty, or mislead readers even when no deception is intended. Honest visualization is a discipline of restraint: the chart should make the data's structure legible without amplifying or concealing what is actually there.

The harm this skill prevents is the publication of misleading figures that overstate findings, the perpetuation of visualization conventions that bias interpretation, and accessibility failures that exclude readers. The agent has freedom in aesthetic choices, but the integrity constraints on encoding, scale, and uncertainty are non-negotiable.

## Core Rules

### Match The Chart Type To The Data And Question

Different data structures and questions call for different visual encodings. The wrong chart type can obscure the very pattern the figure is meant to show.

Match type to purpose:

- comparisons between groups: bar or dot plot;
- distributions: histogram, density, box plot, or violin;
- relationships between two variables: scatter plot;
- change over time: line or slope chart;
- composition: stacked bar only when parts sum meaningfully;
- part-to-whole with many categories: treemap or grouped bar, rarely pie.

Avoid 3D charts, which distort perception, and pies with more than a few slices, which are hard to compare.

### Start Bar And Column Axes At Zero

Bar length encodes magnitude, so a truncated y-axis exaggerates differences. This is one of the most common and damaging forms of unintentional deception. If a zero baseline obscures meaningful variation, use a different chart type (dot plot) rather than truncating a bar chart.

Exceptions:

- line charts may use non-zero axes because slope, not length, encodes change;
- dot plots can use non-zero axes because position, not length, encodes value;
- always label the axis range explicitly when not starting at zero.

### Encode Uncertainty Visibly

Point estimates without uncertainty overstate precision. Readers interpret a sharp line or bar as a definitive value. Show confidence intervals, error bars, prediction bands, or shaded ribbons wherever inferential claims are made.

When showing uncertainty:

- choose error bar type deliberately (standard deviation, standard error, confidence interval) and label it;
- consider showing raw data points alongside summaries for small samples;
- use shaded bands for regression confidence intervals;
- avoid implying false precision through overly narrow intervals.

### Use Color To Convey Meaning, Not Decoration

Color is a powerful encoding but is often used carelessly. Rainbow colormaps distort quantitative perception and are inaccessible to colorblind readers.

Color discipline:

- use sequential colormaps for ordered numeric data;
- use diverging colormaps for data with a meaningful midpoint;
- use distinct hues only for categorical data with few groups;
- avoid red-green pairings that exclude colorblind readers;
- ensure sufficient contrast for grayscale printing;
- reserve a highlight color to draw attention to the key comparison.

### Reduce Chartjunk And Maximize Data-Ink

Decorative elements that do not encode data add noise. Gridlines that are too dark, heavy borders, redundant labels, gradients, and shadows compete with the signal.

Apply restraint:

- remove or lighten gridlines;
- drop unnecessary chart borders;
- avoid fill patterns and shadows;
- let whitespace separate groups rather than heavy lines;
- direct-label series instead of relying on a legend when feasible.

### Label Directly And Make Figures Self-Contained

A reader should understand a figure from its caption and labels without returning to the text. Figures are often viewed in isolation, extracted into slides, or skimmed.

Ensure each figure has:

- a descriptive title or caption stating the takeaway;
- labeled axes with units;
- a legend if multiple series are present;
- annotations for key points or events;
- sample size and context where relevant;
- statistical notation explained.

### Design For The Destination Medium

A figure that works on a high-resolution screen may fail in print or on a projector. Consider where the figure will be seen.

Adapt to medium:

- print: high resolution, grayscale legibility, readable at column width;
- presentation: large fonts, high contrast, minimal detail;
- web: accessible color, responsive sizing, alt text;
- colorblind and low-vision readers: redundant encoding (shape, label) beyond color.

### Show The Data, Not Just The Summary

For small samples, plotting only means and error bars hides the distribution and can suggest patterns that vanish when raw points are visible. Show individual data points, beeswarm, or jittered overlays where sample size permits. This exposes outliers, clustering, and overlap that summaries conceal.

## Common Traps

### Truncating Axes To Exaggerate Effects

A y-axis that starts near the data minimum makes small differences look dramatic. This is the most frequent form of misleading visualization.

### Using Dual Axes To Imply Correlation

Two y-axes with independently scaled lines can fabricate the appearance of a relationship. Use dual axes only with clear justification and shared context.

### Overplotting Large Datasets

Thousands of overlapping points become an ink blob. Use transparency, binning, density estimation, or sampling to reveal structure.

### Defaulting To Rainbow Colormaps

Rainbow schemes create artificial boundaries and exclude colorblind readers. Use perceptually uniform sequential or diverging colormaps.

### Hiding Outliers Through Aggregation

A bar chart of means can hide a bimodal distribution or influential outliers. Inspect raw distributions before summarizing.

### Implying Causation Through Trend Lines

A fitted line through observational scatter does not establish causation. Label clearly and avoid causal language unless the design supports it.

### Ignoring Accessibility

Color-only encodings and low-contrast elements exclude readers with visual impairments. Add redundant shape, label, or texture encoding.

## Self-Check

- [ ] Does the chart type match the data structure and the question being asked?
- [ ] Do bar and column charts start their value axis at zero?
- [ ] Is uncertainty (confidence intervals, error bars) shown for inferential claims?
- [ ] Is color used meaningfully with accessible, perceptually uniform palettes?
- [ ] Has chartjunk been removed to maximize the data-to-ink ratio?
- [ ] Can the figure be understood from its caption and labels alone?
- [ ] Is the figure legible in its destination medium (print, screen, projector)?
- [ ] Are raw data points shown for small samples rather than only summaries?
- [ ] Are axes labeled with units and appropriate ranges?
- [ ] Does the visualization avoid exaggerating or minimizing the real effect?
