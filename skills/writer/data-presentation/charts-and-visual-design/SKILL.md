---
name: charts_and_visual_design.md
description: Use when the agent is designing charts and data visuals, matching chart types to data, labeling axes and scales honestly, or reviewing visuals for misleading design choices such as truncated axes or distorted proportions.
---

# Charts And Visual Design

A chart is an argument made of geometry instead of grammar, and like any argument it can be honest or misleading, clear or confusing, well-supported or distorted. Readers process visuals faster and more credulously than prose; a chart that truncates an axis, uses an inconsistent scale, or picks the wrong chart type can convince a reader of something the data does not actually show, and the reader rarely stops to check. This is why visual design is an ethical problem as much as an aesthetic one. A misleading chart does not merely look bad; it persuades falsely, and the writer is responsible for the persuasion.

Chart design is a judgment problem because the same data can be visualized in ways that tell opposite stories, and the line between clarity and distortion is a set of deliberate choices rather than a fixed rule. An agent that treats charts as decoration, or as a way to make data look impressive, tends to produce visuals that mislead through truncated axes, mismatched chart types, distorted proportions, or missing context. The craft is matching the visual to the data, labeling honestly, and designing so the reader sees what the data actually shows, no more and no less.

Use this skill when creating or selecting charts and data visuals, when matching chart types to data, when setting scales and axes, or when reviewing visuals for honesty and clarity. The goal is visuals that are accurate, legible, and free of design choices that distort the data's meaning.

## Core Rules

### Match The Chart Type To The Data And The Question

Different chart types answer different questions, and the wrong type obscures or distorts the data. A line chart shows change over time; a bar chart compares discrete categories; a scatter plot reveals relationships between variables; a pie chart shows parts of a whole, poorly. Choosing the chart type is the first and most consequential design decision, because it determines what the reader can and cannot see.

Match type to purpose:

- line charts for trends over a continuous variable, usually time;
- bar charts for comparing discrete categories or groups;
- scatter plots for relationships or correlations between two variables;
- histograms for the distribution of a single variable;
- tables when precise values matter more than visual patterns.

Avoid chart types that fight the data. Pie charts are rarely the right choice because humans compare angles poorly; 3D charts distort perception of magnitude; area charts can mislead when overlapping. Choose the simplest type that shows what the reader needs to see.

### Label Everything The Reader Needs

A chart without clear labels forces the reader to guess, and guessing readers often guess wrong. Every axis, every series, every unit, and every relevant context must be labeled so the reader can interpret the visual without external help. Unlabeled or cryptically labeled charts are not minimalist; they are incomplete, and they invite misreading.

Label completely:

- both axes, with the variable and the unit of measurement;
- every series or category, clearly and legibly;
- the time period, geography, or population the data covers;
- the source, so the reader can trace and assess the data;
- any transformation, such as log scale, adjusted values, or rolling averages.

When in doubt, over-label. The cost of an extra label is small; the cost of a reader misinterpreting an unlabeled axis is large. A chart should be self-explanatory to a reader who encounters it outside its surrounding prose.

### Set Axes And Scales Honestly

The axis is where most chart distortion happens, deliberately or accidentally. A truncated y-axis exaggerates differences; an inconsistent scale breaks comparison; a non-zero baseline on a bar chart misleads the eye. Set axes so the visual impression matches the data's actual meaning, and flag any choice that deviates from the default a reader would expect.

Set axes honestly by:

- starting bar chart axes at zero, since bar lengths are read as magnitude;
- truncating line chart axes only when the variation is meaningful, and flagging it clearly;
- using consistent scales across compared charts so readers can compare fairly;
- choosing linear or log scales based on the data, and labeling which is used;
- avoiding dual axes that imply relationships the data may not support.

If a chart's effect depends on a non-obvious axis choice, that choice is doing the persuading, not the data. A reader who discovers a truncated axis after being persuaded feels manipulated, and the chart's credibility is destroyed along with the argument it supported.

### Avoid Distorting Proportions And Magnitudes

Visual elements should represent magnitudes faithfully. A 3D bar that appears twice as tall should represent twice the value; an icon scaled to represent a figure should scale by area or volume consistently, not arbitrarily. Humans read visual proportion instinctively, and any mismatch between visual size and actual value produces false persuasion.

Avoid distortion by:

- scaling pictograms by area consistently, not by height alone, when representing size;
- avoiding 3D effects that exaggerate or minimize perceived magnitude;
- keeping bar widths and spacing consistent so density does not mislead;
- not using area or volume to represent one-dimensional quantities unless carefully scaled.

The temptation to make a chart dramatic by exaggerating the visual is the temptation to mislead. A chart that needs distortion to be persuasive is making a claim the data does not support, and the honest fix is a weaker chart or a weaker claim.

### Provide Context So The Visual Means Something

A chart in isolation rarely means anything; its significance depends on context. A line going up is significant only against a baseline, a norm, or an expectation. A number is large or small only against a comparison. Provide the context the reader needs to judge whether the visualized pattern matters, rather than presenting the data as self-evidently significant.

Provide context by:

- including baselines, averages, or benchmarks for comparison;
- showing trends over enough time to distinguish signal from noise;
- noting the population or denominator that gives the figures meaning;
- flagging external events that explain the pattern, so causation is not implied by the visual alone.

A chart without context can imply more than the data shows, because the reader supplies their own context, which may be wrong. The writer's job is to supply the right context so the reader judges the data accurately.

### Design For The Reader's Comprehension, Not The Designer's Toolkit

A chart's purpose is communication, not demonstration of capability. Effects, colors, and complexity that do not serve comprehension should be removed, no matter how impressive they look. A simple chart the reader understands beats an elaborate chart the reader cannot parse. Design from the reader's needs backward, not from the available features forward.

Design for comprehension by:

- using color to distinguish, not to decorate, and ensuring colorblind-safe palettes;
- removing gridlines, borders, and effects that do not aid reading;
- keeping the number of series small enough to track without a legend struggle;
- ordering categories meaningfully, such as by value rather than alphabetically, when it aids comparison;
- making text large enough to read at the size the chart will appear.

Every design element should earn its place by helping the reader. If it does not, it is noise, and noise in a chart is not neutral; it competes with the signal and degrades comprehension.

### Review Every Visual For Misleading Choices

Before publishing, review each visual as a skeptical reader would, asking what impression it gives and whether that impression is honest. Many misleading charts are produced carelessly rather than maliciously, by writers who did not step back and ask whether the visual's effect matches the data's meaning. A deliberate review catches what an unconscious design choice created.

Review by asking:

- what conclusion would a casual reader draw, and is that conclusion supported by the data;
- would a different, equally valid visualization tell a different story;
- are axis choices, color, or emphasis steering the reader toward a stronger impression than warranted;
- does the chart need context, baselines, or caveats that are currently missing.

If an honest review reveals that the chart persuades beyond what the data supports, redesign it. The writer's responsibility is to the data's meaning, not to the chart's effect.

## Common Traps

### Wrong Chart Type For The Data

A pie chart for trends, a line chart for discrete categories, or a bar chart for distributions obscures the data. Match the type to the question the visual must answer.

### Truncated Or Inconsistent Axes

A y-axis that does not start at zero on a bar chart, or inconsistent scales across comparisons, exaggerates differences. Set axes so visual impression matches meaning.

### Unlabeled Or Cryptic Elements

Axes, units, series, and sources left unlabeled force the reader to guess. Label completely so the chart is self-explanatory.

### Distorted Proportions And 3D Effects

Visual size that does not match actual magnitude persuades falsely. Scale faithfully and avoid effects that exaggerate.

### Missing Context That Inflates Significance

A chart without baseline, comparison, or time depth implies more than the data shows. Provide the context the reader needs to judge significance.

### Decoration That Competes With Comprehension

Color, effects, and complexity that do not serve reading are noise. Design for the reader's understanding, not the designer's toolkit.

## Self-Check

Before treating chart and visual design as complete, verify:

- The chart type matches the data and the question the visual must answer.
- Both axes are labeled with variables and units, and every series is clearly identified.
- Axes start at zero for bar charts, and any truncation or non-standard scale is flagged.
- Visual proportions faithfully represent magnitudes, with no 3D or scaling distortion.
- Context such as baselines, comparisons, and time depth is provided so the visual's significance is clear.
- Color and design choices aid comprehension rather than decorate.
- The source is cited so the reader can trace and assess the data.
- Any transformation (log scale, adjusted values, rolling averages) is labeled.
- A casual reader's impression would match what the data actually shows; an equally valid alternative visualization would not tell a contradictory story
- Text is legible at the size the chart will appear; the chart could stand alone, understood by a reader who sees it outside its prose
