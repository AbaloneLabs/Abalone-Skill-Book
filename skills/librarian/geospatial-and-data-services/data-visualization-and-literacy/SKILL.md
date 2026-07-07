---
name: data_visualization_and_literacy.md
description: Use when the agent is helping a patron or researcher choose, create, or evaluate data visualizations, selecting chart types for different data and purposes, avoiding misleading visualization choices, using visualization tools (Tableau, R, Python, spreadsheets), or supporting data literacy so patrons can read, interpret, and critically evaluate charts, maps, and graphics they encounter.
---

# Data Visualization And Literacy

Data visualization is a powerful tool for understanding and communicating data, but it is also a powerful tool for misleading, because every visualization encodes choices, what to show, what to omit, what scale to use, what chart type to apply, that shape what the viewer perceives and concludes, and those choices can be made honestly or deceptively, deliberately or accidentally. The defining risk in data visualization support is treating a chart as a neutral rendering of data, when in fact every chart is an argument: a truncated y-axis exaggerates differences, an inappropriate chart type obscures patterns, a cherry-picked time window misleads about trends, and a map with poor binning distorts geographic patterns. A patron who creates a visualization without understanding these choices can produce a chart that misleads themselves and their audience, and a patron who reads a chart without visualization literacy can be misled by charts designed to persuade rather than inform. The discipline is to help patrons choose visualizations honestly matched to their data and purpose, to support the tools and skills to create them, and to build the visualization literacy that lets patrons read charts critically, recognizing both honest communication and misleading design.

Use this skill when helping patrons create, choose, or evaluate data visualizations, or supporting data and visualization literacy. The goal is to prevent the agent from treating visualizations as neutral renderings, and from neglecting the chart-type, scale, and design choices that determine whether a visualization informs or misleads.

## Core Rules

### Match The Chart Type To The Data And The Purpose

Different data and purposes call for different chart types, and the wrong type misleads:

- Comparison: bar charts compare discrete categories; column charts for fewer categories, grouped or stacked for multi-series comparisons.
- Trend over time: line charts show trends; the time scale and granularity must fit the phenomenon (daily, monthly, annual).
- Distribution: histograms show the distribution of a continuous variable; box plots show summary statistics and outliers.
- Composition: stacked bar or area charts show composition, but can obscure component trends; consider small multiples instead.
- Relationship: scatter plots show relationships between two variables; correlation must not be over-read as causation.
- Geographic: maps show spatial patterns, but binning, projection, and normalization choices shape what appears.

Match the chart to what the patron needs to show. A bar chart for a trend, or a pie chart for comparison, misleads because the chart type fights the data's structure.

### Guide Honest Scale And Design Choices

Scale and design choices determine whether a chart informs or misleads:

- Axis scales: truncated y-axes exaggerate differences; start bar charts at zero, and use truncated axes only with clear indication and justification.
- Aspect ratio: the ratio of axes affects the apparent slope of trends; choose ratios that represent the data fairly.
- Color: color should encode meaning, not decorate; use colorblind-friendly palettes and avoid implying ordinal meaning with rainbow palettes on non-ordinal data.
- Bin choice: in maps and histograms, binning (equal interval, quantile, natural breaks) changes the apparent pattern; choose and document the method.
- Omission and context: omitting context (a longer time series, comparison groups, base rates) can mislead as much as active distortion.

Guide patrons toward honest design choices. A chart that uses design to exaggerate or obscure is misleading regardless of the creator's intent.

### Support Visualization Tools And Skills

Patrons create visualizations with various tools, and support must meet their level:

- Spreadsheet tools: Excel and Google Sheets serve basic charting; help patrons use them well, including honest scales and appropriate types.
- Business intelligence tools: Tableau, Power BI, and similar tools support interactive dashboards; help patrons structure data and design clear dashboards.
- Code-based tools: R (ggplot2) and Python (matplotlib, seaborn, plotly) offer precise control for researchers; point patrons to resources for learning.
- Web and library tools: many libraries offer visualization tools, data, and workshops; connect patrons to these resources.

Support the patron's tool without dictating it. The goal is honest, effective visualization in whatever tool the patron uses.

### Build Visualization Literacy For Reading Charts

Visualization literacy, the ability to read charts critically, is as important as the ability to create them:

- Reading axes and scales: teach patrons to check the axes, the scale, and the baseline before interpreting a chart.
- Recognizing misleading design: help patrons spot truncated axes, cherry-picked ranges, inappropriate chart types, and deceptive color use.
- Questioning context: teach patrons to ask what is missing, what time period, what comparison group, what base rate, that the chart does not show.
- Correlation and causation: charts that show relationships do not establish causation; teach patrons to distinguish them.

Visualization literacy protects patrons from being misled by the charts they encounter in news, research, and advocacy. It is a core data literacy skill the library can build.

### Avoid Deceptive Visualization Practices

Whether creating or evaluating, certain practices are deceptive and should be avoided or called out:

- Truncated axes without indication: exaggerating differences by cutting the axis.
- 3D and decorative effects: distorting perception with three-dimensional bars, perspective, and decoration that adds no information.
- Cherry-picked ranges: selecting time periods or subsets that manufacture a desired trend.
- Dual axes misuse: using dual y-axes to imply a relationship that may not exist.
- Misleading maps: poor projection, inappropriate binning, or unnormalized data that distorts geographic patterns.

Identify and avoid these practices in creation, and teach patrons to recognize them in evaluation. Deceptive visualization, intentional or not, undermines the chart's purpose of informing.

### Distinguish Visualization Support From Interpretation

The librarian supports creating and evaluating visualizations, not interpreting the patron's data:

- Creation and evaluation support: help choose chart types, design honestly, use tools, and read charts critically.
- Interpretation boundary: do not interpret what the patron's data means for their research question or decision; that is the patron's analysis.
- Referral: where patrons need statistical interpretation or research methods support, refer to appropriate experts.

Support the visualization without taking over the interpretation. The line is between enabling honest, effective visualization and substituting for the patron's analytical judgment.

## Common Traps

### Treating Charts As Neutral Renderings

Assuming a chart shows the data objectively. The trap is that charts feel factual, but every chart encodes choices that shape perception.

### Wrong Chart Type For The Data

Using a pie chart for comparison or a bar chart for trends. The trap is that the chart type feels flexible, but the wrong type obscures or distorts.

### Deceptive Scale And Design

Truncated axes, 3D effects, or cherry-picked ranges. The trap is that these choices flatter the message, but they mislead the audience.

### Tool-First Rather Than Purpose-First Visualization

Starting from the tool rather than the message. The trap is that the tool feels like the starting point, but the chart must serve the data and purpose.

### Neglecting Visualization Literacy

Helping patrons create charts without building their ability to read them critically. The trap is that creation feels sufficient, but patrons also encounter misleading charts they must evaluate.

### Interpreting The Patron's Data

Drawing conclusions from the patron's visualization. The trap is that interpretation feels helpful, but it substitutes the librarian's judgment for the patron's analysis.

## Self-Check

- Is the chart type matched to the data structure and the purpose (comparison, trend, distribution, composition, relationship, geographic)?
- Are scale and design choices honest, including zero-baselined bars, fair aspect ratios, meaningful color, documented binning, and necessary context?
- Is the patron supported in their tool (spreadsheets, BI tools, code-based tools) with resources for honest, effective visualization?
- Is visualization literacy built, so patrons can read axes, recognize misleading design, question context, and distinguish correlation from causation?
- Are deceptive practices (truncated axes, 3D effects, cherry-picking, dual-axis misuse, misleading maps) avoided in creation and taught for recognition?
- Is the boundary between visualization support and data interpretation maintained, with referral for analytical interpretation?
- Has the patron left with a visualization that honestly represents their data and a literacy that lets them evaluate the charts they encounter?
- Has the patron been enabled to create and read visualizations critically, rather than dependent on the librarian for ongoing chart creation?
