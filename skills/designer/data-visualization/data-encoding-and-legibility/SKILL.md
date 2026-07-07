---
name: data_encoding_and_legibility.md
description: Use when the agent is encoding data into visual channels such as color, size, shape, or position, designing chart legends and axes, choosing color palettes for data, ensuring values are readable, or preventing misreading caused by poor encoding, contrast, or scale choices.
---

# Data Encoding And Legibility

Encoding is the translation of numbers into visual properties: position, length, color, size, shape, and texture. Each channel has different perceptual accuracy, and using a weak channel for a precise comparison, or a strong channel for a trivial one, is how charts become technically correct and practically unreadable. Legibility is whether the user can actually extract the intended value from the marks on the screen.

Use this skill before designing or reviewing any visualization where values are encoded visually: charts, maps, heatmaps, status indicators, conditional formatting, scatter plots, and multi-series comparisons. The goal is to prevent the agent from encoding data in ways that look rich but are read inaccurately, and from producing charts where contrast, scale, labels, or color choices hide the very information the chart exists to convey.

## Core Rules

### Match Encoding Channel To Perceptual Accuracy

Humans do not read all visual channels with equal precision. Position on a shared scale is read most accurately; length is strong; angle and area are weaker; color hue is weak for magnitude and strong only for category; size and volume are read poorly.

Channel guidance:

- use position or length for precise quantitative comparison;
- use color hue for categorical distinction, not for ordered magnitude;
- use color lightness or saturation for ordered magnitude, not hue;
- avoid area and volume for values users must compare exactly;
- use shape only for categorical distinction, never for magnitude;
- reserve size encoding for cases where approximate magnitude is acceptable.

Encoding a critical comparison in a weak channel guarantees misreading.

### Choose Color Palettes By Data Type

Color is not a style choice in data visualization; it is part of the encoding. The palette must match whether the data is categorical, sequential, or diverging.

Palette rules:

- categorical data: distinct hues, perceptually balanced, limited in count;
- sequential data: a single hue ramp from light to dark, or a perceptually uniform sequential map;
- diverging data: two hues around a meaningful midpoint, such as red-white-blue for negative-zero-positive;
- never use a rainbow palette for sequential data, it implies an order the data may not have and is unreadable to color-blind users;
- limit categorical palettes to roughly six to eight series before grouping the rest.

The wrong palette type imposes false meaning or hides real gradients.

### Ensure Color Is Distinguishable And Accessible

A palette that looks fine to the designer may be indistinguishable to users with color vision differences, on low-quality displays, in print, or at small sizes. Color should never be the sole carrier of meaning.

Make encoding robust:

- pair color with direct labels, patterns, or position so meaning survives without color;
- test palettes with color-blind simulators;
- ensure adequate contrast between data marks and background;
- avoid relying on subtle hue differences for critical distinctions;
- provide a legend that is close to the data and easy to map.

If removing color removes the meaning, the encoding is too fragile.

### Label Axes, Units, And Scales Explicitly

A chart without labeled axes forces the user to guess what they are seeing. Units, scale type, and baseline must be explicit.

Always provide:

- axis titles with units, such as "Revenue (USD, thousands)";
- tick labels at readable intervals;
- a clear baseline, usually zero for bar charts;
- explicit indication of log, percentage, or indexed scales;
- time axis labels that make the period clear;
- a chart title or caption that states the point, not just the topic.

A chart titled "Sales" answers nothing. A chart titled "Sales rose 18% in Q3 after the pricing change" answers the question.

### Place Legends And Labels Close To The Data

A legend far from the data marks forces the user to look back and forth, increasing error and effort. Direct labeling is almost always better than a separate legend.

Prefer:

- direct labels on series, segments, or points when feasible;
- legends immediately adjacent to the chart when direct labeling is not possible;
- callouts for the specific values or points that matter;
- consistent color-to-label mapping across related charts;
- avoiding legends that require memorizing more than a few mappings.

The closer the label is to the mark, the more accurately it is read.

### Handle Density And Overplotting

Real datasets often have many points or series. Without care, marks overlap into an unreadable blob, hiding distribution and outliers.

Manage density by:

- using transparency or density coloring for overlapping points;
- aggregating or sampling when raw points are too numerous;
- jittering or beeswarm layouts to reveal overlap;
- splitting dense series into small multiples;
- providing zoom and filter so users can isolate regions;
- annotating clusters and outliers rather than letting them hide.

A chart where most points are invisible is not showing the data; it is hiding it.

### Encode Uncertainty And Context

A single value without context, such as a point estimate with no confidence band, invites overconfident interpretation. Charts should show uncertainty, ranges, and reference points where they exist.

Include where relevant:

- confidence intervals or error bars;
- ranges, min-max bands, or quartiles;
- targets, thresholds, or benchmarks as reference lines;
- prior-period comparisons for context;
- annotations explaining anomalies or data quality issues.

A line without a confidence band looks more certain than it is.

## Common Traps

### Hue For Ordered Data

Using a rainbow or random hue palette for sequential data implies categorical breaks where none exist and is unreadable to color-blind users. Use lightness ramps for order.

### Color As The Only Encoding

If the legend is removed and the chart becomes meaningless, the encoding is too fragile. Always pair color with label, position, or pattern.

### Too Many Categorical Colors

Beyond roughly eight categories, distinct hues run out and the palette becomes noise. Group the long tail into "Other" or use a different chart structure.

### Missing Or Implicit Units

A y-axis labeled "Value" with no units leaves the user guessing whether the numbers are dollars, counts, or percentages. Units belong on the axis.

### Legend Far From The Marks

A legend in a corner that requires matching seven colors to seven series is a source of error. Direct labeling removes the effort.

### Overplotting Hiding The Data

Thousands of opaque points stacked into a single dark mass hide the distribution. Transparency, sampling, or density encoding reveals it.

### Point Estimates Without Uncertainty

A clean line with no error band looks precise and certain, encouraging users to over-read small differences. Show the range when it exists.

## Self-Check

- [ ] Quantitative comparisons are encoded in accurate channels such as position or length, not weak channels like area or hue.
- [ ] The color palette matches the data type: categorical hues, sequential lightness ramps, or diverging palettes around a meaningful midpoint.
- [ ] Color is paired with labels, position, or pattern so meaning survives without color and is readable to color-blind users.
- [ ] Axes are titled with units, tick labels are readable, the baseline is clear, and any log or percentage scale is explicitly indicated.
- [ ] The chart title or caption states the insight, not just the topic.
- [ ] Legends are placed close to the data, and direct labeling is used where feasible to reduce lookup effort.
- [ ] Dense data is handled with transparency, sampling, aggregation, jitter, or small multiples so distribution and outliers remain visible.
- [ ] Uncertainty, ranges, thresholds, and reference context are shown where they exist, not hidden behind a single point estimate.
- [ ] The number of categorical series is limited, with a long tail grouped into "Other" rather than an unreadable palette.
- [ ] The encoding was tested for contrast, small-size legibility, color-blind safety, and print or grayscale reproduction.