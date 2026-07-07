---
name: data_visualization_and_chart_design.md
description: Use when the agent is choosing or designing charts and data visualizations for content, selecting chart types by data relationship, ensuring honest visualization without misleading axes or scales, simplifying charts for lay audiences without distortion, annotating and storytelling with data, or using color in data viz for accessibility and clarity.
---

# Data Visualization And Chart Design

A chart is an argument, not a picture of numbers. The judgment problem is that creators treat data visualization as a presentational step, choosing a chart type that looks appealing and dropping numbers into it, when every visualization decision, chart type, axis, scale, color, and annotation, makes a claim about what the data means. A well-designed chart can reveal a truth the audience would otherwise miss. A poorly designed one can manufacture a falsehood that spreads faster than any correction. The difference is rarely malice; it is usually casual choices made without understanding that each one carries meaning.

The harm is ethical and reputational. A truncated axis exaggerates a trivial difference into a dramatic trend. The wrong chart type obscures the actual relationship in the data. Oversimplification for a lay audience crosses into distortion that misleads. Poor color choices exclude colorblind viewers and confuse the meaning the chart was meant to convey. A shareable but dishonest chart amplifies misinformation at a speed no retraction can match. This skill helps the agent choose chart types that match the data's real relationships, preserve honesty even when simplifying, use annotation to turn data into story, and apply color so it encodes meaning accessibly rather than decorating arbitrarily. The chart's first duty is to the truth; its second duty is to the audience's comprehension, and the two must not conflict.

## Core Rules

### Choose The Chart Type By The Data Relationship

Different chart types answer different questions, and choosing the wrong type distorts meaning before any styling is applied. The chart type must be driven by the relationship in the data and the question the audience needs answered, not by what looks modern.

Match type to relationship by:

- using bar charts for comparison across discrete categories;
- using line or area charts for change over a continuous range, usually time;
- using stacked bars or, rarely and carefully, pie charts for part-of-whole composition;
- using histograms or box plots for distribution;
- using scatter plots for the relationship between two continuous variables;
- using flowcharts or sankeys for process and flow relationships.

A line chart for unrelated categories, or a pie chart for change over time, misleads regardless of how polished it looks. Start from the question, then choose the form.

### Preserve Honesty In Axes And Scale

The most common visualization dishonesty is not fraud but carelessness in axes and scale. Truncated axes, inconsistent scales, and distorted proportions manufacture dramatic trends from trivial differences. Honesty in scale is non-negotiable.

Preserve honesty by:

- starting bar chart axes at zero unless there is a defensible, stated reason not to;
- avoiding truncated y-axes that exaggerate differences out of proportion;
- using consistent scales when comparing charts side by side;
- sizing visual elements, bars, bubbles, areas, proportionally to the values they represent;
- avoiding 3D effects and perspective that distort perception of magnitude.

A chart that looks dramatic by distorting scale is not a better chart; it is a misleading one. Honesty constrains every other design choice.

### Simplify For Lay Audiences Without Distorting

Lay audiences need simplification, but simplification has a line where it becomes distortion. The skill is removing complexity that obscures without removing data that changes the conclusion. Crossing that line turns a helpful chart into a misleading one.

Simplify safely by:

- removing non-essential detail while preserving the core signal;
- grouping related categories explicitly rather than silently dropping them;
- rounding only where precision is not material to the conclusion;
- annotating assumptions, omissions, and grouping decisions openly;
- never removing data points whose inclusion would change the takeaway.

Simplification that clarifies without changing meaning is good design. Simplification that hides inconvenient data is dishonesty dressed as clarity.

### Use Annotation To Turn Data Into Story

Raw charts leave the audience to reverse-engineer the conclusion. Annotation guides the audience to the meaning, turning a chart from a display into an argument. Used well, annotation makes the takeaway undeniable; used badly, it editorializes over the data.

Annotate to tell story by:

- writing titles that state the finding, not just the topic;
- highlighting the key data point with color, callout, or label;
- adding brief notes that explain why a moment matters;
- using direct labels on series rather than forcing legend lookups;
- ensuring the visual emphasis and the verbal claim agree.

A chart titled "Revenue by Quarter" leaves the work to the reader. A chart titled "Revenue Doubled in Q3 After Launch" delivers the message. The title is part of the chart.

### Apply Color To Encode Meaning, Not Decorate

In data visualization, color is a data channel, not decoration. Arbitrary color creates confusion; meaningful color creates clarity. Color choices also determine whether the chart is accessible to colorblind viewers.

Use color by:

- limiting the palette so color encodes meaning rather than ornament;
- using one accent color to highlight the key series and muting the rest;
- never relying on color alone to distinguish categories, pair with labels or patterns;
- choosing colorblind-safe palettes, avoiding red-green as the sole distinction;
- testing the chart in grayscale to confirm meaning survives without color.

Color that decorates confuses. Color that encodes clarifies. Every color in a chart should answer the question of why it is there.

### Design For The Audience's Comprehension Level

A chart must match what its audience can actually parse. A chart that is technically correct but incomprehensible to its audience fails. A chart that is clear but below the audience's level bores them. Match the depth to the real reader.

Match comprehension by:

- knowing whether the audience reads charts fluently or needs guidance;
- providing context and explanation for non-expert audiences;
- avoiding chart types, such as dual-axis or log scale, that confuse lay readers without explanation;
- choosing simpler forms when the audience will not interpret complex ones correctly;
- preserving rigor for expert audiences without dumbing down to the point of distortion.

Clarity and rigor are not opposites. A chart can be honest and accessible at the same time, with deliberate effort.

### Cite Sources And Show Provenance

A chart detached from its source is an assertion. Credible visualization names where the data came from, when it was collected, and how it was processed, so the audience can assess reliability and verify.

Build credibility by:

- naming the data source clearly on or near the chart;
- dating the data and noting the time period covered;
- noting sample size and methodology where they affect interpretation;
- distinguishing the creator's analysis and transformation from the raw data;
- linking to underlying data where possible so claims can be checked.

A chart with provenance invites trust and verification. A chart without it reads as opinion shaped like a graph.

### Guard Against Misleading Shareability

Charts are built to be shared, which amplifies any inaccuracy. A misleading chart that spreads detach from its context reaches audiences who cannot evaluate it and spreads false conclusions faster than any correction. Design for the possibility of detachment.

Guard shareability by:

- ensuring the chart remains honest when detached from its surrounding context;
- making the title and visual agree, with no bait between them;
- avoiding cherry-picked time frames or comparisons that mislead;
- testing whether the takeaway survives scrutiny by a skeptical reader;
- being willing not to publish a chart if honesty cannot be preserved at the desired simplicity.

Shareability multiplies accuracy and inaccuracy alike. A chart designed to spread must be honest enough to survive being shared.

## Common Traps

### Wrong Chart Type For The Relationship

Using a chart type that does not match the data's relationship distorts meaning. Start from the question, then choose the form.

### Truncated Or Inconsistent Axes

Axes manipulated to exaggerate differences manufacture false trends and damage credibility. Preserve honest scale.

### Simplification That Hides The Conclusion

Grouping or omitting data that changes the takeaway is distortion, not simplification. Annotate omissions openly.

### Decoration Over Clarity

Ornament, 3D effects, heavy borders, and arbitrary color impede comprehension. Strip chart junk.

### Implicit Takeaways

Forcing the audience to infer the conclusion reduces impact and invites misreading. State the finding in the title.

### Color Reliance Without Accessibility

Relying on color alone, especially red-green, excludes colorblind viewers and confuses meaning. Pair with labels and patterns.

### Unsourced Or Undated Charts

Charts without sources, dates, or methodology read as assertions and cannot be verified or trusted.

### Shareable But Dishonest

Charts designed to spread that distort the truth amplify misinformation faster than corrections can travel.

## Self-Check

Before approving a chart or data visualization, verify:

- The chart type was chosen to match the data's relationship and the question the audience needs answered.
- Axes, scales, and proportions preserve honesty and do not exaggerate differences.
- Simplification clarifies for the lay audience without distorting or hiding inconvenient data.
- The takeaway is stated explicitly in the title or annotation, not left implicit.
- Annotation and emphasis guide the audience to the meaning and agree with the verbal claim.
- Color encodes meaning rather than decorating, with a colorblind-safe palette and non-color redundancy.
- The chart's complexity matches what the actual audience can comprehend without distortion.
- Sources, dates, sample size, and methodology are cited so the audience can verify.
- The chart remains honest when shared detached from its original context.
- No cherry-picked time frames, comparisons, or omissions mislead the audience.
