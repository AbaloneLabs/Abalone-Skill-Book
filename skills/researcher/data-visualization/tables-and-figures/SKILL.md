---
name: tables_and_figures.md
description: Use when the agent is building tables for a manuscript, designing figures and diagrams, formatting statistical results in tables, creating descriptive summary tables, deciding between table and figure, writing captions, or ensuring tables and figures meet journal requirements.
---

# Tables And Figures

Tables and figures carry the evidentiary weight of a paper. Readers often scan them before reading the text, and reviewers use them to verify claims. A poorly constructed table buries findings in clutter, a mislabeled figure misleads, and an overcrowded graphic obscures the very result it should reveal. Agents frequently treat these elements as containers for dumping output rather than as designed communication artifacts, which produces tables that are hard to parse and figures that fail to convey their message at a glance.

The harm this skill prevents is evidence that cannot be verified, results that are misread, and rejection driven by figures and tables that do not meet professional standards. The agent has freedom in layout and style, but the requirements for clarity, accuracy, and self-containment are strict.

## Core Rules

### Choose Table Or Figure Based On What The Reader Needs

Tables and figures serve different purposes. Choosing the wrong format wastes the reader's effort.

Use a table when:

- exact values matter and must be looked up;
- many variables must be compared across rows and columns;
- the structure is multidimensional and not easily plotted;
- precision is more important than pattern recognition.

Use a figure when:

- patterns, trends, or relationships are the message;
- a visual comparison communicates faster than numbers;
- distributions or uncertainty need to be shown;
- spatial or temporal structure is central.

### Make Tables Parseable At A Glance

A dense table with no structure forces the reader to hunt. Design tables so the key comparison is immediately visible.

Table design principles:

- put the primary comparison along rows or columns consistently;
- align numbers by decimal point for easy scanning;
- use whitespace and grouping rather than heavy rules;
- round consistently and avoid excessive decimal places;
- order rows by a meaningful variable, not alphabetically by default;
- highlight the key result with bolding or shading sparingly;
- include a clear, self-explanatory title.

### Report Statistics Completely And Consistently

Statistical tables must contain enough information for a reader to interpret and verify the analysis. Omitting elements forces guesswork.

Each statistical table should include:

- the statistic (estimate, mean, coefficient);
- a measure of uncertainty (standard error or confidence interval);
- sample size for each cell or group;
- test statistics and degrees of freedom where relevant;
- p-values reported with effect sizes, never alone;
- consistent rounding and notation;
- footnotes explaining abbreviations and symbols.

### Write Captions That Stand Alone

Captions are read independently of the surrounding text. A caption that merely says "Results of the experiment" fails to convey the figure's message.

A strong caption:

- states the main finding or purpose in the first sentence;
- defines abbreviations and symbols;
- explains the meaning of error bars and statistical markers;
- identifies sample sizes and conditions;
- notes the statistical test where relevant;
- is understandable without reading the body text.

### Number And Reference Every Table And Figure

Every table and figure must be numbered, cited in the text, and placed near its first reference. An unreferenced figure raises questions about why it exists.

Manage numbering and referencing:

- number sequentially in order of first text mention;
- cite each in text as "Figure 1" or "Table 2";
- place as close to the first reference as layout allows;
- ensure no figure or table is orphaned without a text reference;
- check that numbers match after revisions.

### Ensure Reproducibility Of Figures

A figure that cannot be regenerated from data and code is fragile. If the data changes, a hand-edited figure cannot be updated reliably.

For reproducibility:

- generate figures programmatically from data using scripts;
- store the code and data that produce each figure;
- use vector formats (PDF, SVG) for line art where possible;
- use sufficient resolution (300+ dpi) for raster images;
- keep a versioned figure-generation pipeline;
- document any manual post-processing.

### Respect Journal And Venue Requirements

Venues impose specific requirements on dimensions, resolution, file formats, color modes, and caption placement. Ignoring these causes production delays or rejection.

Check requirements for:

- file format (TIFF, EPS, PDF, PNG);
- resolution for raster images;
- dimensions and column width;
- color space (RGB versus CMYK);
- font size and type requirements;
- caption and label placement conventions;
- limits on number of figures or tables.

### Design Diagrams And Schematics Clearly

Conceptual diagrams, flowcharts, and study schematics communicate structure. They fail when cluttered, inconsistent, or unlabeled.

For diagrams:

- use consistent shapes and symbols;
- align elements on a grid;
- limit text to what is necessary;
- use arrows with clear direction and meaning;
- ensure the logic flows readably (top-down or left-right);
- avoid decorative elements that do not aid understanding.

## Common Traps

### Dumping Raw Software Output Into Tables

Copying unformatted console output produces unreadable tables. Reformat for human readers with rounding, alignment, and clear headers.

### Overcrowding A Single Figure

Cramming many panels or series into one figure overwhelms the reader. Split into multiple focused figures when the message fragments.

### Alphabetical Row Ordering By Default

Alphabetical order is rarely meaningful. Sort by effect size, group, or another substantive variable to reveal pattern.

### Inconsistent Decimal Places And Notation

Mixed rounding (3.1, 3.142, 3.14) signals carelessness. Apply consistent rounding and notation throughout.

### Captions That Only Describe, Not Interpret

"Bar chart of scores" tells the reader nothing. State the finding: "Group A scored significantly higher than Group B."

### Forgetting Units And Sample Sizes

Numbers without units or context are uninterpretable. Label units in headers and include n for each cell.

### Hand-Editing Figures That Cannot Be Regenerated

Manual tweaks to a figure break reproducibility. If data changes, the figure cannot be updated cleanly. Generate programmatically.

## Self-Check

- [ ] Is each element a table or a figure based on whether exact values or patterns matter most?
- [ ] Can the key comparison in each table be seen at a glance?
- [ ] Do statistical tables include estimates, uncertainty, sample sizes, and effect sizes?
- [ ] Does each caption state the main finding and stand alone without the body text?
- [ ] Is every table and figure numbered, cited in text, and placed near its first reference?
- [ ] Are figures generated programmatically from versioned data and code?
- [ ] Do figures and tables meet the venue's format, resolution, and dimension requirements?
- [ ] Are rows ordered by a meaningful variable rather than alphabetically by default?
- [ ] Are units, abbreviations, and sample sizes labeled consistently?
- [ ] Has raw software output been reformatted for human readability?
