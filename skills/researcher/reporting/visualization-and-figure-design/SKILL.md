---
name: visualization_and_figure_design.md
description: Use when the agent is designing data visualizations for research reporting, choosing chart types by data and message, avoiding misleading visuals such as truncated axes 3D charts or dual axes, ensuring figure accessibility with colorblind-safe palettes and alt text, deciding between tables and figures, or producing reproducible figure generation pipelines.
---

# Visualization And Figure Design

A figure is an argument compressed into an image. Because readers scan figures before, and sometimes instead of, reading the text, a visualization can persuade where prose would be questioned. This power is precisely why figures are easy to misuse, often unintentionally. A truncated y-axis, a misleading dual axis, an inappropriate chart type, or an inaccessible color scheme can make a weak result look strong, a null result look significant, or a finding invisible to readers who cannot distinguish the chosen colors. The judgment problem is to design figures that are honest about uncertainty, matched to the data and the message, accessible to the intended audience, and reproducible from the underlying data.

Use this skill when creating figures for a paper, report, presentation, or dashboard; when choosing between tables and figures; when appraising a visualization for honesty; when ensuring accessibility; or when building a reproducible figure pipeline. The goal is to keep the agent from treating visualization as decoration and from letting design choices, conscious or not, exaggerate or obscure the evidence. The agent has freedom to choose aesthetics, but must subordinate them to honesty, clarity, and accessibility.

## Core Rules

### Match The Chart Type To The Data And The Message

Different chart types encode different relationships, and a mismatch between type and message produces confusion or misreading. The first decision is what the figure must communicate, and the chart type follows from that.

Match type to purpose:

- comparisons between groups, use bar charts for counts or means, with care about baselines;
- distributions, use histograms, density plots, box plots, or, preferably, plots showing individual points;
- relationships between two continuous variables, use scatter plots, with transparency or jittering for overplotting;
- change over time, use line charts with consistent time spacing;
- composition, use stacked bars or area charts cautiously, since they impede comparison of components;
- multivariate data, use small multiples, heatmaps, or faceted plots rather than cramming many variables into one panel.

Avoid chart types whose primary effect is novelty, such as radar, polar area, or 3D charts, unless they genuinely serve the message. A chart that requires explanation of how to read it has failed its first job.

### Choose Honesty Over Impact

The most common visualization failures are not outright lies but small exaggerations that accumulate. Each is individually defensible and collectively misleading.

Guard against:

- truncated y-axes on bar charts that magnify small differences;
- inconsistent axis scales across panels that invite false comparison;
- dual y-axes that imply a relationship between unrelated quantities;
- area or volume encodings, as in 3D or bubble charts, where the eye reads area but the data encodes radius;
- aspect ratios that exaggerate or flatten trends;
- color or emphasis that draws the eye to favorable results and away from others.

When in doubt, prefer the plainer chart that shows the data faithfully over the dramatic one that shows it favorably.

### Show The Data And The Uncertainty

A mean with a confidence interval can hide bimodality, outliers, and subgroups. Where sample sizes allow, show individual data points alongside summary statistics, so readers can see the distribution that produced the summary.

Convey uncertainty honestly:

- use error bars, confidence intervals, or credible intervals appropriate to the inference;
- state what the error bar represents, since standard deviation, standard error, and confidence interval are different;
- avoid hiding weak or variable results behind clean summary lines;
- show the spread of the raw data where it matters to the interpretation.

A figure that reports only a fitted line with no underlying points invites the reader to trust a relationship the data may not support.

### Decide Between Tables And Figures Deliberately

Tables and figures serve different purposes, and the choice should be intentional. Figures excel at showing patterns, trends, and relationships; tables excel at presenting exact values that readers must look up or compare precisely.

Choose a table when:

- readers need exact numerical values;
- the data are small enough to read at a glance;
- precision matters more than pattern;
- the audience may want to extract specific numbers for further use.

Choose a figure when:

- the message is a pattern, trend, comparison, or relationship;
- the data are too numerous to parse as a table;
- visual comparison is the point;
- uncertainty or distribution is part of the message.

Do not duplicate the same content in both a table and a figure merely to fill space. Choose the format that serves the reader.

### Make Figures Accessible

Inaccessible figures exclude readers with color vision deficiencies, low vision, or those using screen readers, and they fail in grayscale printing. Accessibility is not optional polish; it is part of honest reporting.

Ensure accessibility by:

- using colorblind-safe palettes, and avoiding red-green encodings as the sole distinguisher;
- distinguishing groups by shape, pattern, or label in addition to color;
- testing figures in grayscale to confirm they remain legible;
- providing sufficient contrast and font size for readability;
- writing alt text that conveys the figure's message and key values for screen readers;
- avoiding reliance on color alone to encode critical information.

An accessible figure is usually also a clearer figure for all readers, since redundant encoding strengthens comprehension.

### Make The Caption A Self-Contained Unit

A reader should understand a figure from its caption alone, without hunting through the text. The caption should state what is shown, the axes, the sample, the key finding, and the meaning of symbols and error bars.

A strong caption includes:

- a title sentence stating the figure's message, not just its topic;
- a description of what is plotted and the units;
- the sample size, groups, or conditions;
- the meaning of symbols, colors, and error bars;
- statistical annotations, such as significance markers, with the test used;
- any abbreviations defined.

A caption that reads only Effect of X on Y forces the reader to infer everything else and invites misreading.

### Preserve Proportionality And Avoid Distortion

The visual encoding should be proportional to the quantity encoded. Distortions arise when the geometry of the chart does not match the geometry of the data.

Check:

- that bar chart baselines start at a meaningful zero unless a non-zero baseline is explicitly justified;
- that the area or length encoding matches the numerical proportion;
- that axis breaks, if used, are clearly marked;
- that 3D effects, which distort perception of length and volume, are avoided;
- that the aspect ratio does not artificially steepen or flatten a trend.

### Build Reproducible Figure Pipelines

A figure should be regenerable from its data and code. Hand-edited figures, proprietary formats, or undocumented transformations make the figure impossible to check, update, or trust.

Make figures reproducible by:

- generating them from the analysis data using scripts, not manual editing in a graphics program;
- versioning the data and code that produced each figure;
- recording software, package versions, and random seeds where relevant;
- storing the source data alongside the figure, or providing access through a repository;
- ensuring the figure in the paper matches the figure the code produces.

A figure that cannot be reproduced is a figure that cannot be audited, and over time it becomes untrustworthy.

## Common Traps

### Truncated Axes That Magnify Differences

Starting a bar chart y-axis above zero can make a trivial difference look dramatic. This is the most common form of visual exaggeration and is often unintentional.

### Dual Axes Implying False Relationships

Plotting two series on different y-axes can manufacture the visual appearance of correlation where none exists, by independently scaling each axis to align the curves.

### 3D And Decorative Charts Distorting Perception

Three-dimensional bars, pies, and surfaces distort the perception of length, area, and volume, and they add no information. They are almost always the wrong choice.

### Color As The Sole Encoding

Encoding critical distinctions only by color excludes readers with color vision deficiencies and fails in grayscale. Always add shape, pattern, or label redundancy.

### Hiding Weak Data Behind Clean Summaries

Reporting only fitted lines or means with error bars can conceal sparse, variable, or bimodal underlying data. Show the points where feasible.

### Error Bars Without Definition

Error bars could represent standard deviation, standard error, or a confidence interval, and these are not interchangeable. An undefined error bar invites misinterpretation.

### Caption That Does Not Stand Alone

A caption that labels only the topic forces the reader to search the text for context, and readers who scan figures first will misread or skip the figure.

### Non-Reproducible Figures

Figures edited by hand or produced by undocumented steps cannot be checked, updated, or trusted over time, and they undermine the reproducibility of the entire report.

## Self-Check

- Is the chart type chosen to match the data structure and the message, rather than for novelty or impact?
- Are honesty checks satisfied, including non-truncated baselines, consistent scales across panels, no misleading dual axes, and proportional area or length encoding?
- Are individual data points shown alongside summaries where sample sizes allow, and is uncertainty represented with clearly defined error bars?
- Is the choice between table and figure deliberate, based on whether pattern or exact values matter, without redundant duplication?
- Is the figure accessible, using colorblind-safe palettes, redundant shape or label encoding, grayscale legibility, sufficient contrast, and alt text?
- Does the caption stand alone, stating the message, axes, sample, symbols, error bar definition, and statistical annotations?
- Are 3D effects, decorative elements, and disproportionate encodings avoided?
- Is the figure generated from versioned data and code, with software, package versions, and seeds recorded, so it can be reproduced and audited?
- Does the figure avoid drawing the eye selectively to favorable results while burying inconvenient ones?
- When reviewed as a whole, does the set of figures tell the honest story of the evidence, including uncertainty and limitations, rather than a cleaner version of it?
